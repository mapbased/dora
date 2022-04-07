use std::collections::{HashMap, HashSet};
use std::sync::Arc;

use parking_lot::RwLock;

use dora_parser::ast;
use dora_parser::interner::Name;
use dora_parser::Position;

use crate::language::sem_analysis::{
    extension_matches, namespace_path, ExtensionId, FctDefinitionId, NamespaceId, TraitDefinitionId,
};
use crate::language::sym::SymTable;
use crate::language::ty::{SourceType, SourceTypeArray};
use crate::utils::GrowableVec;
use crate::vm::{
    impl_matches, replace_type_param, ClassInstanceId, Field, FieldId, FileId, ImplId, VM,
};

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct ClassDefinitionId(usize);

impl ClassDefinitionId {
    pub fn max() -> ClassDefinitionId {
        ClassDefinitionId(usize::max_value())
    }

    pub fn to_usize(self) -> usize {
        self.0
    }
}

impl From<ClassDefinitionId> for usize {
    fn from(data: ClassDefinitionId) -> usize {
        data.0
    }
}

impl From<usize> for ClassDefinitionId {
    fn from(data: usize) -> ClassDefinitionId {
        ClassDefinitionId(data)
    }
}

impl GrowableVec<RwLock<ClassDefinition>> {
    pub fn idx(&self, index: ClassDefinitionId) -> Arc<RwLock<ClassDefinition>> {
        self.idx_usize(index.0)
    }
}

#[derive(Debug)]
pub struct ClassDefinition {
    pub id: ClassDefinitionId,
    pub file_id: FileId,
    pub ast: Arc<ast::Class>,
    pub namespace_id: NamespaceId,
    pub pos: Position,
    pub name: Name,
    pub primitive_type: Option<SourceType>,
    pub ty: Option<SourceType>,
    pub parent_class: Option<SourceType>,
    pub is_open: bool,
    pub is_abstract: bool,
    pub internal: bool,
    pub internal_resolved: bool,
    pub has_constructor: bool,
    pub is_pub: bool,
    pub table: SymTable,

    pub constructor: Option<FctDefinitionId>,
    pub fields: Vec<Field>,
    pub methods: Vec<FctDefinitionId>,
    pub virtual_fcts: Vec<FctDefinitionId>,

    pub impls: Vec<ImplId>,
    pub extensions: Vec<ExtensionId>,

    pub type_params: Vec<TypeParam>,
    pub type_params2: TypeParamDefinition,

    pub specializations: RwLock<HashMap<SourceTypeArray, ClassInstanceId>>,

    // true if this class is the generic Array class
    pub is_array: bool,
    pub is_str: bool,
}

impl ClassDefinition {
    pub fn new(
        _vm: &VM,
        id: ClassDefinitionId,
        file_id: FileId,
        ast: &Arc<ast::Class>,
        namespace_id: NamespaceId,
    ) -> ClassDefinition {
        let type_params = ast.type_params.as_ref().map_or(Vec::new(), |type_params| {
            type_params
                .iter()
                .map(|type_param| TypeParam::new(type_param.name))
                .collect()
        });
        ClassDefinition {
            id,
            file_id,
            ast: ast.clone(),
            namespace_id: namespace_id,
            pos: ast.pos,
            name: ast.name,
            ty: None,
            parent_class: None,
            is_open: ast.is_open,
            is_abstract: ast.is_abstract,
            internal: ast.internal,
            internal_resolved: false,
            has_constructor: ast.has_constructor,
            is_pub: ast.is_pub,
            table: SymTable::new(),

            constructor: None,
            fields: Vec::new(),
            methods: Vec::new(),
            virtual_fcts: Vec::new(),

            impls: Vec::new(),
            extensions: Vec::new(),

            type_params,
            type_params2: TypeParamDefinition::new(),
            specializations: RwLock::new(HashMap::new()),

            is_array: false,
            is_str: false,
            primitive_type: None,
        }
    }

    pub fn is_generic(&self) -> bool {
        self.type_params.len() > 0
    }

    pub fn type_param(&self, id: TypeParamId) -> &TypeParam {
        &self.type_params[id.to_usize()]
    }

    pub fn type_param_ty(&self, ty: SourceType) -> &TypeParam {
        let id = match ty {
            SourceType::TypeParam(id) => id,
            _ => unimplemented!(),
        };

        &self.type_params[id.to_usize()]
    }

    pub fn ty(&self) -> SourceType {
        if let Some(ref primitive_ty) = self.primitive_type {
            primitive_ty.clone()
        } else {
            self.ty.clone().expect("not initialized")
        }
    }

    pub fn field_by_name(&self, name: Name) -> FieldId {
        for field in &self.fields {
            if field.name == name {
                return field.id;
            }
        }

        panic!("field not found!")
    }

    pub fn name(&self, vm: &VM) -> String {
        let mut name = namespace_path(vm, self.namespace_id, self.name);

        if self.type_params.len() > 0 {
            let type_params = self
                .type_params
                .iter()
                .map(|p| vm.interner.str(p.name).to_string())
                .collect::<Vec<_>>()
                .join(", ");
            name.push('[');
            name.push_str(&type_params);
            name.push(']');
        }

        name
    }

    pub fn name_with_params(&self, vm: &VM, type_list: &SourceTypeArray) -> String {
        let name = vm.interner.str(self.name);

        if type_list.len() > 0 {
            let type_list = type_list
                .iter()
                .map(|p| p.name(vm))
                .collect::<Vec<_>>()
                .join(", ");

            format!("{}[{}]", name, type_list)
        } else {
            name.to_string()
        }
    }

    pub fn find_method(&self, vm: &VM, name: Name, is_static: bool) -> Option<FctDefinitionId> {
        let mut classid = self.id;

        loop {
            let cls = vm.classes.idx(classid);
            let cls = cls.read();

            for &method in &cls.methods {
                let method = vm.fcts.idx(method);
                let method = method.read();

                if method.name == name && method.is_static == is_static {
                    return Some(method.id);
                }
            }

            if let Some(ref parent_class) = cls.parent_class {
                classid = parent_class.cls_id().expect("no class");
            } else {
                return None;
            }
        }
    }

    pub fn find_trait_method(
        &self,
        vm: &VM,
        trait_id: TraitDefinitionId,
        name: Name,
        is_static: bool,
    ) -> Option<FctDefinitionId> {
        for &impl_id in &self.impls {
            let ximpl = vm.impls[impl_id].read();

            if ximpl.trait_id != Some(trait_id) {
                continue;
            }

            let table = if is_static {
                &ximpl.static_names
            } else {
                &ximpl.instance_names
            };

            return table.get(&name).cloned();
        }

        None
    }

    pub fn subclass_from(&self, vm: &VM, super_id: ClassDefinitionId) -> bool {
        let mut cls_id = self.id;

        loop {
            if cls_id == super_id {
                return true;
            }

            let cls = vm.classes.idx(cls_id);
            let cls = cls.read();

            match cls.parent_class {
                Some(ref parent_class) => {
                    cls_id = parent_class.cls_id().expect("no class");
                }

                None => {
                    return false;
                }
            }
        }
    }
}

pub fn find_field_in_class(
    vm: &VM,
    mut class: SourceType,
    name: Name,
) -> Option<(SourceType, FieldId, SourceType)> {
    if class.cls_id().is_none() {
        return None;
    }

    loop {
        let cls_id = class.cls_id().expect("no class");
        let cls = vm.classes.idx(cls_id);
        let cls = cls.read();

        let type_list = class.type_params();

        for field in &cls.fields {
            if field.name == name {
                return Some((
                    class,
                    field.id,
                    replace_type_param(vm, field.ty.clone(), &type_list, None),
                ));
            }
        }

        if let Some(ref parent_class) = cls.parent_class {
            let type_list = parent_class.type_params();
            class = replace_type_param(vm, parent_class.clone(), &type_list, None);
        } else {
            return None;
        }
    }
}

pub fn find_method_in_class(
    vm: &VM,
    mut class: SourceType,
    name: Name,
) -> Option<(SourceType, FctDefinitionId)> {
    loop {
        let cls_id = class.cls_id().expect("no class");
        let cls = vm.classes.idx(cls_id);
        let cls = cls.read();

        for &method in &cls.methods {
            let method = vm.fcts.idx(method);
            let method = method.read();

            if method.name == name && method.is_static == false {
                return Some((class, method.id));
            }
        }

        if let Some(ref parent_class) = cls.parent_class {
            let type_list = parent_class.type_params();
            class = replace_type_param(vm, parent_class.clone(), &type_list, None);
        } else {
            return None;
        }
    }
}

pub struct Candidate {
    pub object_type: SourceType,
    pub container_type_params: SourceTypeArray,
    pub fct_id: FctDefinitionId,
}

pub fn find_methods_in_class(
    vm: &VM,
    object_type: SourceType,
    type_param_defs: &[TypeParam],
    type_param_defs2: Option<&TypeParamDefinition>,
    name: Name,
    is_static: bool,
) -> Vec<Candidate> {
    let mut candidates = Vec::new();
    let mut ignores = HashSet::new();

    let mut class_type = object_type.clone();

    loop {
        let cls_id = class_type.cls_id().expect("no class");
        let cls = vm.classes.idx(cls_id);
        let cls = cls.read();

        for &method in &cls.methods {
            let method = vm.fcts.idx(method);
            let method = method.read();

            if method.name == name && method.is_static == is_static {
                if let Some(overrides) = method.overrides {
                    ignores.insert(overrides);
                }

                if !ignores.contains(&method.id) {
                    return vec![Candidate {
                        object_type: class_type.clone(),
                        container_type_params: class_type.type_params(),
                        fct_id: method.id,
                    }];
                }
            }
        }

        if let Some(ref parent_class) = cls.parent_class {
            let type_list = class_type.type_params();
            class_type = replace_type_param(vm, parent_class.clone(), &type_list, None);
        } else {
            break;
        }
    }

    // Find extension methods
    {
        let cls_id = object_type.cls_id().expect("no class");
        let cls = vm.classes.idx(cls_id);
        let cls = cls.read();

        for &extension_id in &cls.extensions {
            if let Some(bindings) = extension_matches(
                vm,
                object_type.clone(),
                type_param_defs,
                type_param_defs2,
                extension_id,
            ) {
                let extension = vm.extensions[extension_id].read();

                let table = if is_static {
                    &extension.static_names
                } else {
                    &extension.instance_names
                };

                if let Some(&fct_id) = table.get(&name) {
                    return vec![Candidate {
                        object_type,
                        container_type_params: bindings,
                        fct_id: fct_id,
                    }];
                }
            }
        }
    }

    let mut class_type = object_type;

    loop {
        let cls_id = class_type.cls_id().expect("no class");
        let cls = vm.classes.idx(cls_id);
        let cls = cls.read();

        for &impl_id in &cls.impls {
            if let Some(bindings) = impl_matches(
                vm,
                class_type.clone(),
                type_param_defs,
                type_param_defs2,
                impl_id,
            ) {
                let ximpl = vm.impls[impl_id].read();

                let table = if is_static {
                    &ximpl.static_names
                } else {
                    &ximpl.instance_names
                };

                if let Some(&method_id) = table.get(&name) {
                    candidates.push(Candidate {
                        object_type: class_type.clone(),
                        container_type_params: bindings.clone(),
                        fct_id: method_id,
                    });
                }
            }
        }

        if let Some(parent_class) = cls.parent_class.clone() {
            let type_list = class_type.type_params();
            class_type = replace_type_param(vm, parent_class, &type_list, None);
        } else {
            break;
        }
    }

    candidates
}

#[derive(Clone, Debug)]
pub struct TypeParam {
    pub name: Name,
    pub trait_bounds: HashSet<TraitDefinitionId>,
}

impl TypeParam {
    pub fn new(name: Name) -> TypeParam {
        TypeParam {
            name,
            trait_bounds: HashSet::new(),
        }
    }
}

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct TypeParamId(pub usize);

impl TypeParamId {
    pub fn to_usize(self) -> usize {
        self.0
    }
}

#[derive(Debug)]
pub struct TypeParamDefinition {
    pub names: Vec<Name>,
    pub bounds: Vec<TypeParamBound>,
}

impl TypeParamDefinition {
    pub fn new() -> TypeParamDefinition {
        TypeParamDefinition {
            names: Vec::new(),
            bounds: Vec::new(),
        }
    }

    pub fn len(&self) -> usize {
        self.names.len()
    }

    pub fn add_bound(&mut self, type_param_id: TypeParamId, trait_id: TraitDefinitionId) {
        self.bounds.push(TypeParamBound {
            type_param_id,
            trait_id,
        });
    }

    pub fn iter(&self) -> TypeParamIter {
        TypeParamIter {
            next: 0,
            limit: self.len(),
        }
    }
}

pub struct TypeParamIter {
    next: usize,
    limit: usize,
}

impl Iterator for TypeParamIter {
    type Item = TypeParamId;

    fn next(&mut self) -> Option<TypeParamId> {
        if self.next < self.limit {
            let current = self.next;
            self.next += 1;
            Some(TypeParamId(current))
        } else {
            None
        }
    }
}

#[derive(Debug)]
pub struct TypeParamBound {
    pub type_param_id: TypeParamId,
    pub trait_id: TraitDefinitionId,
}