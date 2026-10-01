use super::{operation_quality, OpQuality, BREP_KERNEL_OPERATIONS, OPERATION_QUALITY};
use std::collections::HashSet;

#[test]
fn operation_quality_table_covers_every_brep_kernel_method_with_no_duplicates() {
    let table_names: Vec<&str> = OPERATION_QUALITY.iter().map(|(name, _)| *name).collect();
    let unique_table_names: HashSet<&str> = table_names.iter().copied().collect();
    assert_eq!(table_names.len(), unique_table_names.len(), "OPERATION_QUALITY has duplicate method names");
    for method in BREP_KERNEL_OPERATIONS {
        assert!(unique_table_names.contains(method), "OPERATION_QUALITY is missing BrepKernel method {method:?}");
        assert_ne!(operation_quality(method), OpQuality::Unsupported, "BrepKernel method {method:?} resolved to Unsupported — add it to OPERATION_QUALITY");
    }
    let known_methods: HashSet<&str> = BREP_KERNEL_OPERATIONS.iter().copied().collect();
    for name in &table_names {
        assert!(known_methods.contains(name), "OPERATION_QUALITY names {name:?}, which is not a BrepKernel trait method");
    }
}

#[test]
fn unknown_operation_reports_unsupported() {
    assert_eq!(operation_quality("not_a_real_method"), OpQuality::Unsupported);
}
