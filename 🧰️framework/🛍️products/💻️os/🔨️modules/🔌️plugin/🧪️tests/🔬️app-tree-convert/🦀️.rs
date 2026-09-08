mod tree_convert_tests {
    use super::*;

    #[test]
    fn dynamic_text_root_reports_fixed_capacity_admission_failure() {
        let error = built_text_to_component_tree(ui_wgpu::wgpu::Label::data("x".repeat(UI_TEXT_MAX_BYTES + 1))).expect_err("oversized dynamic text must not panic or truncate");
        assert_eq!(error.code, "ui.fixed-capacity");
    }
}
