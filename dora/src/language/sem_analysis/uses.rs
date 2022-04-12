use std::sync::Arc;

use crate::language::sem_analysis::NamespaceDefinitionId;
use crate::vm::SourceFileId;
use dora_parser::ast;

pub struct UseDefinition {
    pub ast: Arc<ast::Use>,
    pub namespace_id: NamespaceDefinitionId,
    pub file_id: SourceFileId,
}

impl UseDefinition {
    pub fn new(
        file_id: SourceFileId,
        namespace_id: NamespaceDefinitionId,
        node: &Arc<ast::Use>,
    ) -> UseDefinition {
        UseDefinition {
            namespace_id,
            file_id,
            ast: node.clone(),
        }
    }
}