use std::cmp::max;

use crate::language::sem_analysis::{get_tuple_subtypes, TupleId};
use crate::language::ty::SourceType;
use crate::mem;
use crate::vm::{specialize_enum_id_params, EnumLayout, VM};

#[derive(Clone)]
pub struct ConcreteTuple {
    offsets: Vec<i32>,
    references: Vec<i32>,
    size: i32,
    align: i32,
}

impl ConcreteTuple {
    pub fn offsets(&self) -> &[i32] {
        &self.offsets
    }

    pub fn contains_references(&self) -> bool {
        !self.references.is_empty()
    }

    pub fn references(&self) -> &[i32] {
        &self.references
    }

    pub fn size(&self) -> i32 {
        self.size
    }

    pub fn align(&self) -> i32 {
        self.align
    }
}

fn determine_tuple_size(vm: &VM, subtypes: &[SourceType]) -> ConcreteTuple {
    let mut size = 0;
    let mut offsets = Vec::new();
    let mut references = Vec::new();
    let mut align = 0;

    for ty in subtypes {
        assert!(ty.is_concrete_type(vm));

        let element_size;
        let element_align;
        let element_ty;

        if let Some(tuple_id) = ty.tuple_id() {
            let concrete = get_concrete_tuple(vm, tuple_id);

            element_size = concrete.size;
            element_align = concrete.align;

            let element_offset = mem::align_i32(size, element_align);
            offsets.push(element_offset);

            for &ref_offset in &concrete.references {
                offsets.push(element_offset + ref_offset);
            }

            size = element_offset + element_size;
            align = max(align, element_align);

            continue;
        } else if let SourceType::Enum(enum_id, type_params) = ty {
            let edef_id = specialize_enum_id_params(vm, *enum_id, type_params.clone());
            let edef = vm.enum_defs.idx(edef_id);

            match edef.layout {
                EnumLayout::Int => {
                    element_size = 4;
                    element_align = 4;
                    element_ty = SourceType::Int32;
                }
                EnumLayout::Ptr | EnumLayout::Tagged => {
                    element_size = mem::ptr_width();
                    element_align = mem::ptr_width();
                    element_ty = SourceType::Ptr;
                }
            }
        } else {
            element_size = ty.size(vm);
            element_align = ty.align(vm);
            element_ty = ty.clone();
        }

        let element_offset = mem::align_i32(size, element_align);
        offsets.push(element_offset);

        if element_ty.reference_type() {
            references.push(element_offset);
        }

        size = element_offset + element_size;
        align = max(align, element_align);
    }

    size = mem::align_i32(size, align);
    ConcreteTuple {
        offsets,
        references,
        size,
        align,
    }
}

pub fn get_concrete_tuple(vm: &VM, id: TupleId) -> ConcreteTuple {
    let subtypes = get_tuple_subtypes(vm, id);
    determine_tuple_size(vm, &*subtypes)
}
