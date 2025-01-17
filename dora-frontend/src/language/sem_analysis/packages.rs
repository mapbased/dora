use std::collections::HashMap;
use std::sync::Arc;

use dora_parser::interner::Name;

use parking_lot::RwLock;

use crate::language::sem_analysis::ModuleDefinitionId;
use crate::language::sym::{Sym, SymTable};
use crate::Id;

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct PackageDefinitionId(pub usize);

impl PackageDefinitionId {
    pub fn to_usize(self) -> usize {
        self.0
    }
}

impl Id for PackageDefinition {
    type IdType = PackageDefinitionId;

    fn id_to_usize(id: PackageDefinitionId) -> usize {
        id.0
    }

    fn usize_to_id(value: usize) -> PackageDefinitionId {
        PackageDefinitionId(value)
    }

    fn store_id(value: &mut PackageDefinition, id: PackageDefinitionId) {
        value.id = Some(id);
    }
}

#[derive(Debug)]
pub struct PackageDefinition {
    pub id: Option<PackageDefinitionId>,
    pub name: PackageName,
    pub top_level_module_id: Option<ModuleDefinitionId>,
    pub dependencies: Vec<PackageDependency>,
    pub dependency_names: HashMap<Name, PackageDefinitionId>,
    pub table: Arc<RwLock<SymTable>>,
}

impl PackageDefinition {
    pub fn new(name: PackageName, module_id: ModuleDefinitionId) -> PackageDefinition {
        PackageDefinition {
            id: None,
            name,
            top_level_module_id: Some(module_id),
            dependencies: Vec::new(),
            dependency_names: HashMap::new(),
            table: Arc::new(RwLock::new(SymTable::new())),
        }
    }

    pub fn iname(&self) -> Option<Name> {
        match self.name {
            PackageName::External(name) => Some(name),
            _ => None,
        }
    }

    pub fn top_level_module_id(&self) -> ModuleDefinitionId {
        self.top_level_module_id.expect("uninitialized module id")
    }

    pub fn add_dependency(
        &mut self,
        name: Name,
        package_id: PackageDefinitionId,
        top_level_module_id: ModuleDefinitionId,
    ) -> bool {
        let table = self.table.write();

        if table.get(name).is_some() {
            false
        } else {
            let old_value = self
                .table
                .write()
                .insert(name, Sym::Module(top_level_module_id));
            assert!(old_value.is_none());
            self.dependencies
                .push(PackageDependency { name, package_id });
            true
        }
    }
}

#[derive(Debug)]
pub struct PackageDependency {
    pub name: Name,
    pub package_id: PackageDefinitionId,
}

#[derive(Debug)]
pub enum PackageName {
    Stdlib,
    Boots,
    Program,
    External(Name),
}
