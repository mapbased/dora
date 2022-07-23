use std::collections::HashSet;

use dora_parser::ast;
use dora_parser::Position;

use crate::language::error::msg::SemError;
use crate::language::readty::read_type_unchecked;
use crate::language::sem_analysis::{SemAnalysis, SourceFileId, TypeParamDefinition, TypeParamId};
use crate::language::sym::{NestedSymTable, Sym};

pub fn check(sa: &SemAnalysis) {
    check_traits(sa);
    check_impls(sa);
    check_classes(sa);
}

fn check_traits(sa: &SemAnalysis) {
    for trait_ in sa.traits.iter() {
        let trait_ = trait_.write();
        let mut symtable = NestedSymTable::new(sa, trait_.module_id);
        symtable.push_level();

        let _type_param_definition = read_type_param_definition(
            sa,
            trait_.ast.type_params.as_ref(),
            &mut symtable,
            trait_.file_id,
            trait_.pos,
        );

        symtable.pop_level();
    }
}

fn check_impls(sa: &SemAnalysis) {
    for impl_ in sa.impls.iter() {
        let impl_ = impl_.write();
        let mut symtable = NestedSymTable::new(sa, impl_.module_id);
        symtable.push_level();

        let _type_param_definition = read_type_param_definition(
            sa,
            impl_.ast.type_params.as_ref(),
            &mut symtable,
            impl_.file_id,
            impl_.pos,
        );

        read_type_unchecked(sa, &symtable, impl_.file_id, &impl_.ast.extended_type);

        symtable.pop_level();
    }
}

fn check_classes(sa: &SemAnalysis) {
    for cls in sa.classes.iter() {
        let cls = cls.write();
        let mut symtable = NestedSymTable::new(sa, cls.module_id);
        symtable.push_level();

        let _type_param_definition = read_type_param_definition(
            sa,
            cls.ast().type_params.as_ref(),
            &mut symtable,
            cls.file_id(),
            cls.pos(),
        );

        symtable.pop_level();
    }
}

fn read_type_param_definition(
    sa: &SemAnalysis,
    ast_type_params: Option<&Vec<ast::TypeParam>>,
    symtable: &mut NestedSymTable,
    file_id: SourceFileId,
    pos: Position,
) -> TypeParamDefinition {
    if ast_type_params.is_none() {
        return TypeParamDefinition::new();
    }

    let ast_type_params = ast_type_params.expect("type params expected");

    if ast_type_params.len() == 0 {
        let msg = SemError::TypeParamsExpected;
        sa.diag.lock().report(file_id, pos, msg);

        return TypeParamDefinition::new();
    }

    let mut names = HashSet::new();
    let mut result_type_params = TypeParamDefinition::new();

    // 1) Discover all type parameters.

    for (id, type_param) in ast_type_params.iter().enumerate() {
        let id = TypeParamId(id);

        if !names.insert(type_param.name) {
            let name = sa.interner.str(type_param.name).to_string();
            let msg = SemError::TypeParamNameNotUnique(name);
            sa.diag.lock().report(file_id, type_param.pos, msg);
        }

        let sym = Sym::TypeParam(id);
        symtable.insert(type_param.name, sym);
    }

    // 2) Read bounds for type parameters.

    for (id, type_param) in ast_type_params.iter().enumerate() {
        let id = TypeParamId(id);

        for bound in &type_param.bounds {
            let ty = read_type_unchecked(sa, &symtable, file_id, bound);

            if ty.is_trait() {
                if !result_type_params.add_bound(id, ty) {
                    let msg = SemError::DuplicateTraitBound;
                    sa.diag.lock().report(file_id, type_param.pos, msg);
                }
            } else if !ty.is_error() {
                let msg = SemError::BoundExpected;
                sa.diag.lock().report(file_id, bound.pos(), msg);
            }
        }
    }

    result_type_params
}
