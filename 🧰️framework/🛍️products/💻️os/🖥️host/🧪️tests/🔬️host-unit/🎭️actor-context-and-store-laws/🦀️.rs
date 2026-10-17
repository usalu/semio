    /// 🎭️ Actor context and store laws: the retained actor context, the workflow store catalog wire and the backbone round trips with their cursor.

    /// 🧾️ The opened host actor is retained across edits and persistence reopen.
    #[test]
    fn workflow_host_preserves_the_opened_actor_context() {
        test_identity!(identity);
        let fixture:serde_json::Value=serde_json::from_str(include_str!("../../🧫️fixtures/🧾️actor-context/🔣️.json")).expect("neutral actor fixture");
        for case in fixture["cases"].as_array().expect("cases") {
            let actor=protocol::ActorId(case["actor"].as_str().expect("actor").into());
            let reopened_actor=protocol::ActorId(case["reopenActor"].as_str().expect("reopened actor").into());
            let document=create_backbone_document(workflow::S_WORKFLOW_SCHEMA,"workflow-actor","Workflow",resolve_kernel_future(workflow::empty_workflow_snapshot()));
            let mut store=OsWorkflowStore::new(document,actor.clone()).expect("opened store");
            assert_eq!(store.inner.local_actor_id(),&actor);
            store.add_parameter(&workflow::WorkflowParameterType::Numeric,"First", &mut identity).expect("first edit");
            let document=store.document();
            let encoded=encode_backbone_payload(&document).expect("persist host document");
            close_backbone_envelope(take_backbone_envelope(document).1).expect("persisted copy retires");
            close_workflow_fixture(&mut store);
            let reopened=decode_backbone_payload(&encoded,workflow::S_WORKFLOW_SCHEMA).expect("reopen host document");
            let mut store=OsWorkflowStore::new(reopened,reopened_actor.clone()).expect("reopened store");
            assert_eq!(store.inner.local_actor_id(),&reopened_actor);
            store.add_parameter(&workflow::WorkflowParameterType::Numeric,"Second", &mut identity).expect("second edit");
            let document=store.document();
            let actors:Vec<_>=document.vcs.edits.iter().map(|edit|edit.actor.as_deref().expect("attributed edit")).collect();
            assert_eq!(serde_json::to_value(actors).expect("independent Serde actor projection"),case["expectedActors"]);
            close_backbone_envelope(take_backbone_envelope(document).1).expect("actor projection copy retires");
            close_workflow_fixture(&mut store);
        }
    }

    /// ✍️ Law: the OS workflow store authors an edit, its undo and its redo through the canonical wire its catalog carries, and each authored row is the independent serde_json document of its operation.
    #[test]
    fn the_os_workflow_store_authors_apply_undo_redo_and_its_rows_through_its_catalog_wire() {
        test_identity!(identity);
        let mut store = test_workflow_store();
        let base = store.snapshot().expect("base snapshot").parameters.len();
        store.add_parameter(&workflow::WorkflowParameterType::Numeric, "Authored", &mut identity).expect("apply authors through the catalog wire");
        assert_eq!(store.snapshot().expect("applied snapshot").parameters.len(), base + 1);
        let (forward_rows, inverse_rows) = store.inner.authored_edit_operation_bytes(&mut identity).expect("rows author").expect("the applied edit has rows");
        let (forwards, inverses, _) = store.inner.edit_mutations().expect("the applied edit");
        let document = |mutation: &workflow::WorkflowMutation| serde_json::to_value(semio_framework_value::ToValue::to_value(mutation)).expect("independent serde_json document");
        let parsed = |row: &[u8]| { assert!(row.starts_with(b"op"), "the canonical wire carries its operation header"); serde_json::from_slice::<serde_json::Value>(&row[2..]).expect("the wire body is JSON an independent parser accepts") };
        assert_eq!(forward_rows.iter().map(|row| parsed(row)).collect::<Vec<_>>(), forwards.iter().map(document).collect::<Vec<_>>());
        assert_eq!(inverse_rows.iter().map(|row| parsed(row)).collect::<Vec<_>>(), (0..inverses.len()).map(|index| document(inverses.get(index).expect("inverse row"))).collect::<Vec<_>>());
        resolve_kernel_future(store.inner.dispatch(ArtifactCommand::Undo, &mut identity)).expect("undo authors");
        assert_eq!(store.snapshot().expect("undone snapshot").parameters.len(), base);
        resolve_kernel_future(store.inner.dispatch(ArtifactCommand::Redo, &mut identity)).expect("redo authors");
        assert_eq!(store.snapshot().expect("redone snapshot").parameters.len(), base + 1);
        close_workflow_fixture(&mut store);
    }

    /// 🧮️ The complete history position (applied, redo, checkpoint, alternative) the event log of `edits` + `transitions` folds to.
    fn workflow_history<'a>(document_id: &str, edits: impl IntoIterator<Item = &'a store::Edit<workflow::WorkflowMutation>>, transitions: &[protocol::MutationEnvelope], conflicts: &[protocol::Conflict]) -> protocol::HistoryFold {
        let head = protocol::ViewerHead::canonical_trunk(&protocol::ArtifactId(document_id.to_string()));
        store::os_store::ledger::fold_event_log::<workflow::WorkflowSnapshot, workflow::WorkflowMutation>(document_id, &edits.into_iter().collect::<Vec<_>>(), transitions, conflicts, &head).expect("event log folds")
    }

    /// 🧮️ [`workflow_history`] of one host backbone document.
    fn document_history(document: &OsWorkflowArtifactDocument) -> protocol::HistoryFold {
        workflow_history(&document.id, document.vcs.edits.iter(), &document.transitions, &document.conflicts)
    }

    #[test]
    fn backbone_and_workflow_store_round_trips_preserve_outcomes_and_conflicts() {
        test_identity!(identity);
        let mut store = test_workflow_store();
        store.add_parameter(&workflow::WorkflowParameterType::Numeric, "Durable", &mut identity).expect("create one edit");
        let mut document = store.document();
        let edit_id = document_history(&document).applied.last().expect("one applied edit").clone();
        let edit = document.vcs.edits.iter().find(|edit| edit.id == edit_id).expect("applied edit is persisted");
        let messages = vec![protocol::MutationMessage::warning("mutation.clamped", "durable host outcome").at(["parameters", "0"]).at_op(0)];
        document.edit_messages = vec![protocol::EditMessages { edit_id: edit_id.clone(), messages: messages.clone() }];
        let kind = protocol::ConflictKind::Degraded { edit_ids: vec![edit_id] };
        let timestamp = edit.mutation_meta.first().expect("operation metadata").timestamp;
        let mutation_ids = edit.mutation_meta.iter().map(|meta| meta.mutation_id.clone().expect("stable mutation identity")).collect::<Vec<_>>();
        let actors = vec![protocol::ActorId(edit.actor.clone().expect("stable edit actor"))];
        document.conflicts =
            vec![protocol::Conflict { id: resolve_kernel_future(protocol::ConflictId::new(&kind, &protocol::ArtifactId(document.id.clone()), &mutation_ids, &timestamp)), kind, status: protocol::ConflictStatus::Open, messages, actors, timestamp }];

        let payload = encode_backbone_payload(&document).expect("backbone payload encodes");
        let decoded: OsWorkflowArtifactDocument = decode_backbone_payload(&payload, workflow::S_WORKFLOW_SCHEMA).expect("backbone payload decodes");
        assert_eq!(document_history(&decoded), document_history(&document));
        assert_eq!(decoded.edit_messages, document.edit_messages);
        assert_eq!(decoded.conflicts, document.conflicts);

        let rebuilt = OsWorkflowStore::new(decoded, store.inner.local_actor_id().clone()).expect("workflow store rebuilds");
        let rebuilt_document = rebuilt.document();
        assert_eq!(rebuilt_document.edit_messages, document.edit_messages);
        assert_eq!(rebuilt_document.conflicts, document.conflicts);

        let text = export_backbone_dsl(&document).expect("backbone text encodes");
        let parsed = resolve_kernel_future(store::os_store::text_format::parse_document_text::<workflow::WorkflowSnapshot, workflow::WorkflowMutation>(&text.dsl, &text.ops, &::semio_framework_os_kernel::os_vcs::io::binary::genesis::ArtifactPackGenesisProducer)).expect("backbone text decodes");
        assert!(parsed.envelope.edit_messages.iter().eq(document.edit_messages.iter()));
        assert_eq!(parsed.envelope.conflicts, document.conflicts);
        assert_eq!(workflow_history(&parsed.envelope.id, parsed.envelope.vcs.edits.iter(), &parsed.envelope.transitions, &parsed.envelope.conflicts), document_history(&document));

        let mut invalid = document.clone();
        invalid.conflicts[0].id = protocol::ConflictId("conflict-invalid".into());
        assert!(encode_backbone_payload(&invalid).is_err(), "host binary persistence must reject a non-content-addressed conflict id");
        let malformed_text = text.ops.replacen(&document.conflicts[0].id.0, "conflict-invalid", 1);
        assert!(resolve_kernel_future(store::os_store::text_format::parse_document_text::<workflow::WorkflowSnapshot, workflow::WorkflowMutation>(&text.dsl, &malformed_text, &::semio_framework_os_kernel::os_vcs::io::binary::genesis::ArtifactPackGenesisProducer)).is_err(), "host text persistence must reject a non-content-addressed conflict id");
    }

    #[test]
    fn backbone_binary_text_and_workflow_store_preserve_the_complete_cursor() {
        test_identity!(identity);
        let mut store = test_workflow_store();
        store.add_parameter(&workflow::WorkflowParameterType::Numeric, "Committed", &mut identity).expect("first edit");
        resolve_kernel_future(store.inner.dispatch(ArtifactCommand::CommitCheckpoint { message: Some("cursor checkpoint".into()), authors: Vec::new() }, &mut identity)).expect("checkpoint");
        store.add_parameter(&workflow::WorkflowParameterType::Numeric, "Undone", &mut identity).expect("second edit");
        store.dispatch_text("undo", &mut identity).expect("undo second edit");
        let document = store.document();
        let history = document_history(&document);
        assert!(!history.redo.is_empty(), "precondition: redo lane is populated");
        assert!(history.checkpoint.is_some(), "precondition: checkpoint is populated");

        let payload = encode_backbone_payload(&document).expect("binary encode");
        let decoded: OsWorkflowArtifactDocument = decode_backbone_payload(&payload, workflow::S_WORKFLOW_SCHEMA).expect("binary decode");
        assert_eq!(document_history(&decoded), history);
        assert_eq!(document_history(&OsWorkflowStore::new(decoded, store.inner.local_actor_id().clone()).expect("workflow rebuild").document()), history);

        let text = export_backbone_dsl(&document).expect("text encode");
        let parsed = resolve_kernel_future(store::os_store::text_format::parse_document_text::<workflow::WorkflowSnapshot, workflow::WorkflowMutation>(&text.dsl, &text.ops, &::semio_framework_os_kernel::os_vcs::io::binary::genesis::ArtifactPackGenesisProducer)).expect("text decode");
        assert_eq!(workflow_history(&parsed.envelope.id, parsed.envelope.vcs.edits.iter(), &parsed.envelope.transitions, &parsed.envelope.conflicts), history);
    }
