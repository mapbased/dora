use std::collections::HashMap;
use std::ops::Index;
use std::sync::Arc;

use crate::bytecode::BytecodeType;
use crate::language::sem_analysis::{
    get_tuple_subtypes, ClassDefinition, ClassDefinitionId, EnumDefinition, EnumDefinitionId,
    FctDefinition, SemAnalysis, StructDefinitionId, TraitDefinitionId, TupleId, TypeParam,
    TypeParamId,
};

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub enum SourceType {
    // couldn't determine type because of error
    Error,

    // Allow any type here, used for type inference
    Any,

    // type with only one value: ()
    Unit,

    // primitives
    Bool,
    Char,
    UInt8,
    Int32,
    Int64,
    Float32,
    Float64,

    // pointer to object, only used internally
    Ptr,

    // self type
    This,

    // some class
    Class(ClassDefinitionId, SourceTypeArray),

    // some struct
    Struct(StructDefinitionId, SourceTypeArray),

    // some tuple
    Tuple(TupleId),

    // some trait object
    Trait(TraitDefinitionId, SourceTypeArray),

    // some type variable
    TypeParam(TypeParamId),

    // some lambda
    Lambda(LambdaId),

    // some enum
    Enum(EnumDefinitionId, SourceTypeArray),
}

impl SourceType {
    pub fn is_error(&self) -> bool {
        match self {
            SourceType::Error => true,
            _ => false,
        }
    }

    pub fn is_enum(&self) -> bool {
        match self {
            SourceType::Enum(_, _) => true,
            _ => false,
        }
    }

    pub fn is_enum_id(&self, enum_id: EnumDefinitionId) -> bool {
        match self {
            SourceType::Enum(id, _) => *id == enum_id,
            _ => false,
        }
    }

    pub fn is_unit(&self) -> bool {
        match self {
            SourceType::Unit => true,
            _ => false,
        }
    }

    pub fn is_self(&self) -> bool {
        match self {
            SourceType::This => true,
            _ => false,
        }
    }

    pub fn is_cls(&self) -> bool {
        match self {
            SourceType::Class(_, _) => true,
            _ => false,
        }
    }

    pub fn is_cls_id(&self, cls_id: ClassDefinitionId) -> bool {
        match self {
            SourceType::Class(id, _) => *id == cls_id,
            _ => false,
        }
    }

    pub fn is_trait(&self) -> bool {
        match self {
            SourceType::Trait(_, _) => true,
            _ => false,
        }
    }

    pub fn is_float(&self) -> bool {
        match self {
            &SourceType::Float32 | &SourceType::Float64 => true,
            _ => false,
        }
    }

    pub fn is_int32(&self) -> bool {
        match self {
            &SourceType::Int32 => true,
            _ => false,
        }
    }

    pub fn is_bool(&self) -> bool {
        match self {
            &SourceType::Bool => true,
            _ => false,
        }
    }

    pub fn is_type_param(&self) -> bool {
        match self {
            &SourceType::TypeParam(_) => true,
            _ => false,
        }
    }

    pub fn is_tuple(&self) -> bool {
        match self {
            &SourceType::Tuple(_) => true,
            _ => false,
        }
    }

    pub fn is_struct(&self) -> bool {
        match self {
            &SourceType::Struct(_, _) => true,
            _ => false,
        }
    }

    pub fn is_primitive(&self) -> bool {
        match self {
            &SourceType::Bool
            | &SourceType::UInt8
            | &SourceType::Char
            | &SourceType::Int32
            | &SourceType::Int64
            | &SourceType::Float32
            | &SourceType::Float64 => true,
            _ => false,
        }
    }

    pub fn is_tuple_or_unit(&self) -> bool {
        match self {
            &SourceType::Tuple(_) => true,
            &SourceType::Unit => true,
            _ => false,
        }
    }

    pub fn is_lambda(&self) -> bool {
        match self {
            &SourceType::Lambda(_) => true,
            _ => false,
        }
    }

    pub fn lambda_id(&self) -> Option<LambdaId> {
        match self {
            &SourceType::Lambda(lambda_id) => Some(lambda_id),
            _ => None,
        }
    }

    pub fn cls_id(&self) -> Option<ClassDefinitionId> {
        match self {
            SourceType::Class(cls_id, _) => Some(*cls_id),
            _ => None,
        }
    }

    pub fn primitive_struct_id(&self, sa: &SemAnalysis) -> Option<StructDefinitionId> {
        match self {
            SourceType::Bool => Some(sa.known.structs.bool()),
            SourceType::UInt8 => Some(sa.known.structs.uint8()),
            SourceType::Char => Some(sa.known.structs.char()),
            SourceType::Int32 => Some(sa.known.structs.int32()),
            SourceType::Int64 => Some(sa.known.structs.int64()),
            SourceType::Float32 => Some(sa.known.structs.float32()),
            SourceType::Float64 => Some(sa.known.structs.float64()),
            _ => None,
        }
    }

    pub fn from_cls(cls_id: ClassDefinitionId) -> SourceType {
        SourceType::Class(cls_id, SourceTypeArray::empty())
    }

    pub fn enum_id(&self) -> Option<EnumDefinitionId> {
        match self {
            SourceType::Enum(enum_id, _) => Some(*enum_id),
            _ => None,
        }
    }

    pub fn struct_id(&self) -> Option<StructDefinitionId> {
        match self {
            SourceType::Struct(struct_id, _) => Some(*struct_id),
            _ => None,
        }
    }

    pub fn tuple_id(&self) -> Option<TupleId> {
        match self {
            SourceType::Tuple(tuple_id) => Some(*tuple_id),
            _ => None,
        }
    }

    pub fn type_param_id(&self) -> Option<TypeParamId> {
        match self {
            SourceType::TypeParam(id) => Some(*id),
            _ => None,
        }
    }

    pub fn type_params(&self) -> SourceTypeArray {
        match self {
            SourceType::Class(_, params)
            | SourceType::Enum(_, params)
            | SourceType::Struct(_, params)
            | SourceType::Trait(_, params) => params.clone(),
            _ => SourceTypeArray::empty(),
        }
    }

    pub fn reference_type(&self) -> bool {
        match self {
            SourceType::Ptr => true,
            SourceType::Class(_, _) => true,
            SourceType::Trait(_, _) => true,
            _ => false,
        }
    }

    pub fn value_type(&self) -> bool {
        match self {
            SourceType::Unit
            | SourceType::Bool
            | SourceType::UInt8
            | SourceType::Int32
            | SourceType::Int64
            | SourceType::Float32
            | SourceType::Float64 => true,
            _ => false,
        }
    }

    pub fn subclass_from(&self, sa: &SemAnalysis, ty: SourceType) -> bool {
        if !self.is_cls() {
            return false;
        }
        if !ty.is_cls() {
            return false;
        }

        let cls_id = self.cls_id().unwrap();
        let cls = sa.classes.idx(cls_id);
        let cls = cls.read();
        cls.subclass_from(sa, ty.cls_id().unwrap())
    }

    pub fn name(&self, sa: &SemAnalysis) -> String {
        let writer = SourceTypePrinter {
            sa,
            type_params: None,
        };

        writer.name(self.clone())
    }

    pub fn name_with_params(&self, sa: &SemAnalysis, type_params: &[TypeParam]) -> String {
        let writer = SourceTypePrinter {
            sa,
            type_params: Some(type_params),
        };

        writer.name(self.clone())
    }

    pub fn name_fct(&self, sa: &SemAnalysis, fct: &FctDefinition) -> String {
        let writer = SourceTypePrinter {
            sa,
            type_params: Some(&fct.type_params),
        };

        writer.name(self.clone())
    }

    pub fn name_cls(&self, sa: &SemAnalysis, cls: &ClassDefinition) -> String {
        let writer = SourceTypePrinter {
            sa,
            type_params: Some(&cls.type_params),
        };

        writer.name(self.clone())
    }

    pub fn name_enum(&self, sa: &SemAnalysis, enum_: &EnumDefinition) -> String {
        let writer = SourceTypePrinter {
            sa,
            type_params: Some(&enum_.type_params),
        };

        writer.name(self.clone())
    }

    pub fn allows(&self, sa: &SemAnalysis, other: SourceType) -> bool {
        match self {
            // allow all types for Error, there is already an error,
            // don't report too many messages for the same error
            SourceType::Error => true,

            // Any allows all other types
            SourceType::Any => true,

            SourceType::Unit
            | SourceType::Bool
            | SourceType::UInt8
            | SourceType::Char
            | SourceType::Struct(_, _)
            | SourceType::Enum(_, _)
            | SourceType::Trait(_, _) => *self == other,
            SourceType::Int32 | SourceType::Int64 | SourceType::Float32 | SourceType::Float64 => {
                *self == other
            }
            SourceType::Ptr => panic!("ptr does not allow any other types"),
            SourceType::This => unreachable!(),
            SourceType::Class(self_cls_id, self_list) => {
                if *self == other {
                    return true;
                }

                let (other_cls_id, other_list) = match other {
                    SourceType::Class(cls_id, ref other_list) => (cls_id, other_list.clone()),
                    _ => {
                        return false;
                    }
                };

                if *self_cls_id == other_cls_id {
                    self_list == &other_list
                } else {
                    other.subclass_from(sa, self.clone())
                }
            }
            SourceType::Tuple(tuple_id) => match other {
                SourceType::Tuple(other_tuple_id) => {
                    if *tuple_id == other_tuple_id {
                        return true;
                    }

                    let subtypes = get_tuple_subtypes(sa, *tuple_id);
                    let other_subtypes = get_tuple_subtypes(sa, other_tuple_id);

                    if subtypes.len() != other_subtypes.len() {
                        return false;
                    }

                    let len = subtypes.len();

                    for idx in 0..len {
                        let ty = subtypes[idx].clone();
                        let other_ty = other_subtypes[idx].clone();

                        if !ty.allows(sa, other_ty) {
                            return false;
                        }
                    }

                    true
                }

                _ => false,
            },

            SourceType::TypeParam(_) => *self == other,

            SourceType::Lambda(_) => {
                // for now expect the exact same params and return types
                // possible improvement: allow super classes for params,
                //                             sub class for return type
                *self == other
            }
        }
    }

    pub fn is_defined_type(&self, sa: &SemAnalysis) -> bool {
        match self {
            SourceType::Error | SourceType::This | SourceType::Any | SourceType::Ptr => false,
            SourceType::Unit
            | SourceType::Bool
            | SourceType::UInt8
            | SourceType::Char
            | SourceType::Int32
            | SourceType::Int64
            | SourceType::Float32
            | SourceType::Float64
            | SourceType::Trait(_, _)
            | SourceType::Lambda(_)
            | SourceType::TypeParam(_) => true,
            SourceType::Enum(_, params)
            | SourceType::Class(_, params)
            | SourceType::Struct(_, params) => {
                for param in params.iter() {
                    if !param.is_defined_type(sa) {
                        return false;
                    }
                }

                true
            }
            SourceType::Tuple(tuple_id) => {
                let subtypes = get_tuple_subtypes(sa, *tuple_id);

                for ty in subtypes.iter() {
                    if !ty.is_defined_type(sa) {
                        return false;
                    }
                }

                true
            }
        }
    }

    pub fn is_concrete_type(&self, sa: &SemAnalysis) -> bool {
        match self {
            SourceType::Error | SourceType::This | SourceType::Any => false,
            SourceType::Unit
            | SourceType::Bool
            | SourceType::UInt8
            | SourceType::Char
            | SourceType::Int32
            | SourceType::Int64
            | SourceType::Float32
            | SourceType::Float64
            | SourceType::Ptr => true,
            SourceType::Class(_, params)
            | SourceType::Enum(_, params)
            | SourceType::Struct(_, params)
            | SourceType::Trait(_, params) => {
                for param in params.iter() {
                    if !param.is_concrete_type(sa) {
                        return false;
                    }
                }

                true
            }

            SourceType::Tuple(tuple_id) => {
                let subtypes = get_tuple_subtypes(sa, *tuple_id);
                for subtype in subtypes.iter() {
                    if !subtype.is_concrete_type(sa) {
                        return false;
                    }
                }

                true
            }
            SourceType::Lambda(_) => unimplemented!(),
            SourceType::TypeParam(_) => false,
        }
    }

    pub fn from_bytecode(ty: BytecodeType) -> SourceType {
        match ty {
            BytecodeType::Bool => SourceType::Bool,
            BytecodeType::Char => SourceType::Char,
            BytecodeType::Float32 => SourceType::Float32,
            BytecodeType::Float64 => SourceType::Float64,
            BytecodeType::Int32 => SourceType::Int32,
            BytecodeType::Int64 => SourceType::Int64,
            BytecodeType::Ptr => SourceType::Ptr,
            BytecodeType::UInt8 => SourceType::UInt8,
            BytecodeType::TypeParam(id) => SourceType::TypeParam(TypeParamId(id as usize)),
            BytecodeType::Struct(struct_id, params) => SourceType::Struct(struct_id, params),
            BytecodeType::Tuple(tuple_id) => SourceType::Tuple(tuple_id),
            BytecodeType::Enum(enum_id, params) => SourceType::Enum(enum_id, params),
        }
    }
}

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub enum SourceTypeArray {
    Empty,
    List(Arc<Vec<SourceType>>),
}

impl SourceTypeArray {
    pub fn empty() -> SourceTypeArray {
        SourceTypeArray::Empty
    }

    pub fn single(ty: SourceType) -> SourceTypeArray {
        SourceTypeArray::List(Arc::new(vec![ty]))
    }

    pub fn with(type_params: Vec<SourceType>) -> SourceTypeArray {
        if type_params.len() == 0 {
            SourceTypeArray::Empty
        } else {
            SourceTypeArray::List(Arc::new(type_params))
        }
    }

    pub fn connect(&self, other: &SourceTypeArray) -> SourceTypeArray {
        if self.is_empty() {
            return other.clone();
        }

        if other.is_empty() {
            return self.clone();
        }

        let mut params = self.types().to_vec();
        params.extend_from_slice(other.types());

        SourceTypeArray::List(Arc::new(params))
    }

    pub fn connect_single(&self, other: SourceType) -> SourceTypeArray {
        if self.is_empty() {
            return SourceTypeArray::single(other);
        }

        let mut params = self.types().to_vec();
        params.push(other);

        SourceTypeArray::List(Arc::new(params))
    }

    pub fn types(&self) -> &[SourceType] {
        match self {
            SourceTypeArray::Empty => &[],
            SourceTypeArray::List(ref params) => (**params).as_slice(),
        }
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn len(&self) -> usize {
        match self {
            &SourceTypeArray::Empty => 0,
            &SourceTypeArray::List(ref params) => params.len(),
        }
    }

    pub fn iter(&self) -> SourceTypeArrayIter {
        SourceTypeArrayIter {
            params: self,
            idx: 0,
        }
    }

    pub fn name(&self, sa: &SemAnalysis) -> String {
        let mut result = String::new();
        let mut first = true;
        result.push('[');

        for ty in self.iter() {
            if !first {
                result.push_str(", ");
            }
            result.push_str(&ty.name(sa));
            first = false;
        }

        result.push(']');

        result
    }
}

impl Index<usize> for SourceTypeArray {
    type Output = SourceType;

    fn index(&self, idx: usize) -> &SourceType {
        match self {
            &SourceTypeArray::Empty => panic!("type list index out-of-bounds"),
            &SourceTypeArray::List(ref params) => &params[idx],
        }
    }
}

pub struct SourceTypeArrayIter<'a> {
    params: &'a SourceTypeArray,
    idx: usize,
}

impl<'a> Iterator for SourceTypeArrayIter<'a> {
    type Item = SourceType;

    fn next(&mut self) -> Option<SourceType> {
        match self.params {
            &SourceTypeArray::Empty => None,

            &SourceTypeArray::List(ref params) => {
                if self.idx < params.len() {
                    let ret = params[self.idx].clone();
                    self.idx += 1;

                    Some(ret)
                } else {
                    None
                }
            }
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct LambdaId(usize);

impl From<usize> for LambdaId {
    fn from(val: usize) -> LambdaId {
        LambdaId(val)
    }
}

pub struct LambdaTypes {
    types: HashMap<Arc<LambdaType>, LambdaId>,
    values: Vec<Arc<LambdaType>>,
    next_lambda_id: usize,
}

impl LambdaTypes {
    pub fn new() -> LambdaTypes {
        LambdaTypes {
            types: HashMap::new(),
            values: Vec::new(),
            next_lambda_id: 0,
        }
    }

    pub fn len(&self) -> usize {
        self.values.len()
    }

    pub fn insert(&mut self, params: Vec<SourceType>, ret: SourceType) -> LambdaId {
        let ty = LambdaType { params, ret };

        if let Some(&val) = self.types.get(&ty) {
            return val;
        }

        let id = LambdaId(self.next_lambda_id);
        let ty = Arc::new(ty);
        self.types.insert(ty.clone(), id);

        self.values.push(ty);

        self.next_lambda_id += 1;

        id
    }

    pub fn get(&self, id: LambdaId) -> Arc<LambdaType> {
        self.values[id.0].clone()
    }
}

struct SourceTypePrinter<'a> {
    sa: &'a SemAnalysis,
    type_params: Option<&'a [TypeParam]>,
}

impl<'a> SourceTypePrinter<'a> {
    pub fn name(&self, ty: SourceType) -> String {
        match ty {
            SourceType::Error => "<error>".into(),
            SourceType::Any => "Any".into(),
            SourceType::Unit => "()".into(),
            SourceType::UInt8 => "UInt8".into(),
            SourceType::Char => "Char".into(),
            SourceType::Int32 => "Int32".into(),
            SourceType::Int64 => "Int64".into(),
            SourceType::Float32 => "Float32".into(),
            SourceType::Float64 => "Float64".into(),
            SourceType::Bool => "Bool".into(),
            SourceType::Ptr => panic!("type Ptr only for internal use."),
            SourceType::This => "Self".into(),
            SourceType::Class(id, params) => {
                let cls = self.sa.classes.idx(id);
                let cls = cls.read();
                let base = self.sa.interner.str(cls.name);

                if params.len() == 0 {
                    base.to_string()
                } else {
                    let params = params
                        .iter()
                        .map(|ty| self.name(ty))
                        .collect::<Vec<_>>()
                        .join(", ");

                    format!("{}[{}]", base, params)
                }
            }
            SourceType::Struct(sid, params) => {
                let struc = self.sa.structs.idx(sid);
                let struc = struc.read();
                let name = struc.name;
                let name = self.sa.interner.str(name).to_string();

                if params.len() == 0 {
                    name
                } else {
                    let params = params
                        .iter()
                        .map(|ty| self.name(ty))
                        .collect::<Vec<_>>()
                        .join(", ");

                    format!("{}[{}]", name, params)
                }
            }
            SourceType::Trait(tid, params) => {
                let trait_ = self.sa.traits[tid].read();
                let name = self.sa.interner.str(trait_.name).to_string();

                if params.len() == 0 {
                    name
                } else {
                    let params = params
                        .iter()
                        .map(|ty| self.name(ty))
                        .collect::<Vec<_>>()
                        .join(", ");

                    format!("{}[{}]", name, params)
                }
            }
            SourceType::Enum(id, params) => {
                let enum_ = self.sa.enums[id].read();
                let name = self.sa.interner.str(enum_.name).to_string();

                if params.len() == 0 {
                    name
                } else {
                    let params = params
                        .iter()
                        .map(|ty| self.name(ty))
                        .collect::<Vec<_>>()
                        .join(", ");

                    format!("{}[{}]", name, params)
                }
            }

            SourceType::TypeParam(idx) => {
                if let Some(type_params) = self.type_params {
                    self.sa
                        .interner
                        .str(type_params[idx.to_usize()].name)
                        .to_string()
                } else {
                    format!("TypeParam({})", idx.to_usize())
                }
            }

            SourceType::Lambda(id) => {
                let lambda = self.sa.lambda_types.lock().get(id);
                let params = lambda
                    .params
                    .iter()
                    .map(|ty| self.name(ty.clone()))
                    .collect::<Vec<_>>()
                    .join(", ");
                let ret = self.name(lambda.ret.clone());

                format!("({}) -> {}", params, ret)
            }

            SourceType::Tuple(tuple_id) => {
                let types = get_tuple_subtypes(self.sa, tuple_id);

                let types = types
                    .iter()
                    .map(|ty| self.name(ty.clone()))
                    .collect::<Vec<_>>()
                    .join(", ");

                format!("({})", types)
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct LambdaType {
    pub params: Vec<SourceType>,
    pub ret: SourceType,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn append_type_lists() {
        let e1 = SourceTypeArray::empty();
        let e2 = SourceTypeArray::single(SourceType::Int32);
        assert_eq!(e1.connect(&e2).types(), &[SourceType::Int32]);

        let e1 = SourceTypeArray::single(SourceType::Float32);
        let e2 = SourceTypeArray::single(SourceType::Int32);
        assert_eq!(
            e1.connect(&e2).types(),
            &[SourceType::Float32, SourceType::Int32]
        );
    }
}
