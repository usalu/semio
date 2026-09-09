mod app_commands_tests {
    use crate::{ArtifactView, ConfigView, Emit, HistoryView, NoConfigMutation};

    mod add_widget {
        use semio_framework_value_derive::{FromValue, ToValue};
        #[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
        pub struct AddWidget {
            pub kind: String,
            pub x: f64,
        }

        /// 🎯️ Mirrors the shape a real `🎮️commands/add_widget/🦀️.rs::handle` will have —
        /// `(payload, doc, cfg) -> ArtifactMutationOutcome<Mutation, ConfigMutation>`.
        pub fn handle(payload: &AddWidget, _doc: &crate::ArtifactView<'_, u32>, _cfg: &crate::ConfigView<'_, ()>) -> Result<crate::Emit<String, crate::NoConfigMutation>, crate::Fault> {
            Ok(crate::Emit::mutations(vec![format!("add:{}:{}", payload.kind, payload.x)]))
        }
    }

    mod delete_selection {
        use semio_framework_value_derive::{FromValue, ToValue};
        #[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
        pub struct DeleteSelection {
            pub id: String,
        }

        pub fn handle(payload: &DeleteSelection, _doc: &crate::ArtifactView<'_, u32>, _cfg: &crate::ConfigView<'_, ()>) -> Result<crate::Emit<String, crate::NoConfigMutation>, crate::Fault> {
            Ok(crate::Emit::mutations(vec![format!("delete:{}", payload.id)]))
        }
    }

    app_commands! {
        pub enum TestFakeCommand for u32, String, (), NoConfigMutation {
            "addWidget" => add_widget::AddWidget,
            "deleteSelection" => delete_selection::DeleteSelection,
        }
    }

    #[semio_framework_async_macros::async_test]
    async fn command_id_matches_declared_row() {
        assert_eq!(TestFakeCommand::AddWidget(add_widget::AddWidget { kind: "inputSlider".into(), x: 1.5 }).command_id(), "addWidget");
        assert_eq!(TestFakeCommand::DeleteSelection(delete_selection::DeleteSelection { id: "n1".into() }).command_id(), "deleteSelection");
    }

    #[semio_framework_async_macros::async_test]
    async fn generated_tool_job_catalog_is_an_exact_bijection_with_rows() {
        assert_eq!(TestFakeCommand::TOOL_JOB_IDS, &["addWidget", "deleteSelection"]);
        assert_eq!(<TestFakeCommand as crate::ToolCommandCatalog>::TOOL_JOB_IDS, TestFakeCommand::TOOL_JOB_IDS);
        assert_eq!(<TestFakeCommand as ::protocol::OpBinary>::TOOL_JOB_IDS, TestFakeCommand::TOOL_JOB_IDS);
    }

    #[semio_framework_async_macros::async_test]
    async fn dispatch_forwards_to_the_payload_modules_own_handle() {
        let snapshot = 0u32;
        let config = ();
        let history = HistoryView::empty();
        let doc = ArtifactView::new(&snapshot, &history);
        let cfg = ConfigView { snapshot: &config, window: None };

        let emit: Emit<String, NoConfigMutation> = TestFakeCommand::AddWidget(add_widget::AddWidget { kind: "inputSlider".into(), x: 1.5 }).dispatch(&doc, &cfg).expect("dispatch add-widget");
        assert_eq!(emit.artifact_mutations, vec!["add:inputSlider:1.5".to_string()]);

        let emit: Emit<String, NoConfigMutation> = TestFakeCommand::DeleteSelection(delete_selection::DeleteSelection { id: "n1".into() }).dispatch(&doc, &cfg).expect("dispatch delete-selection");
        assert_eq!(emit.artifact_mutations, vec!["delete:n1".to_string()]);
    }

    #[semio_framework_async_macros::async_test]
    async fn wire_round_trips_through_dsl_ops_op_text_and_op_binary() {
        store::os_store::test_support::assert_op_text_binary_equivalence(&TestFakeCommand::AddWidget(add_widget::AddWidget { kind: "neuron".into(), x: 2.0 }));
        store::os_store::test_support::assert_op_text_binary_equivalence(&TestFakeCommand::DeleteSelection(delete_selection::DeleteSelection { id: "n1".into() }));
    }

    mod keyed {
        use semio_framework_value_derive::{FromValue, ToValue};
        #[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
        pub struct AddWidget {
            pub kind: String,
        }

        pub fn handle(payload: &AddWidget, _doc: &crate::ArtifactView<'_, u32>, _cfg: &crate::ConfigView<'_, ()>, ctx: &mut u32) -> Result<crate::Emit<String, crate::NoConfigMutation>, crate::Fault> {
            *ctx += 1;
            Ok(crate::Emit::mutations(vec![format!("add:{}:{ctx}", payload.kind)]))
        }
    }

    mod keyed_unit {
        use semio_framework_value_derive::{FromValue, ToValue};
        #[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
        pub struct DeleteSelection {}

        pub fn handle(_payload: &DeleteSelection, _doc: &crate::ArtifactView<'_, u32>, _cfg: &crate::ConfigView<'_, ()>, _ctx: &mut u32) -> Result<crate::Emit<String, crate::NoConfigMutation>, crate::Fault> {
            Ok(crate::Emit::mutations(vec!["delete".to_string()]))
        }
    }

    app_commands! {
        pub enum TestKeyedCommand for u32, String, (), NoConfigMutation, ctx = u32 {
            "addWidget" as "add-widget" => keyed::AddWidget,
            "deleteSelection" as "delete-selection" => keyed_unit::DeleteSelection,
        }
    }

    /// 🧪️ The keyed arm must keep `command_id()` (manifest action id) and the `dsl` wire keyword
    /// independent — the exact split every hand-written `*_protocol` Command enum already has.
    #[semio_framework_async_macros::async_test]
    async fn keyed_rows_separate_the_command_id_from_the_wire_keyword() {
        let command = TestKeyedCommand::AddWidget(keyed::AddWidget { kind: "inputSlider".into() });
        assert_eq!(command.command_id(), "addWidget");
        assert!(protocol::OpText::print_op(&command).starts_with("add-widget "), "wire keyword must be the kebab `as` literal, got {:?}", protocol::OpText::print_op(&command));
        store::os_store::test_support::assert_op_text_binary_equivalence(&command);
    }

    /// 🧪️ A fieldless payload struct must print/encode exactly like the unit variant it replaces —
    /// the migration-safety property for every `DeleteSelection`-style bare variant.
    #[semio_framework_async_macros::async_test]
    async fn fieldless_payload_matches_a_unit_variants_wire_form() {
        let command = TestKeyedCommand::DeleteSelection(keyed_unit::DeleteSelection {});
        assert_eq!(protocol::OpText::print_op(&command), "delete-selection");
        assert_eq!(protocol::OpBinary::encode_op(&command).expect("encode"), vec![1u8, 1, 0, 0]);
        store::os_store::test_support::assert_op_text_binary_equivalence(&command);
    }

    #[semio_framework_async_macros::async_test]
    async fn ctx_is_threaded_through_dispatch_into_every_handler() {
        let snapshot = 0u32;
        let config = ();
        let history = HistoryView::empty();
        let doc = ArtifactView::new(&snapshot, &history);
        let cfg = ConfigView { snapshot: &config, window: None };
        let mut ctx = 41u32;
        let emit: Emit<String, NoConfigMutation> = TestKeyedCommand::AddWidget(keyed::AddWidget { kind: "neuron".into() }).dispatch(&doc, &cfg, &mut ctx).expect("dispatch");
        assert_eq!(emit.artifact_mutations, vec!["add:neuron:42".to_string()]);
        assert_eq!(ctx, 42);
    }
}
