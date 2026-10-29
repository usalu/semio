    /// 🧾️ Source rejection, anchors and host lane: generic reducer rejection, the fixture contract anchors, the bounded renderer page and ack and the bounded event sourced host configuration lane.

    /// ⚖️ The generic command is constructed to be ENCODED: `admit_command_wire` admits the command's
    /// own wire, so the decoder necessarily precedes its admission. What the clause has always stood
    /// for — no generically constructed command reaches the reducer without the complete pipeline —
    /// is asserted directly, as the order decoder → gate → reducer entry.
#[test]
    fn full_operation_source_rejects_generic_reducers_and_old_monolithic_shells() {
        let source = [module_source("🧵️retained-command/🔄️full-operation/🦀️.rs"), module_source("👷️job/🛬️lifted/🔄️full-operation/🦀️.rs"), instance_source()].concat();
        let start = source.find("impl<A: ArtifactApp> semio_framework_job::InteractiveJob for TypedCommandFullOperationJob<A>").expect("typed full-operation job");
        let end = source[start..].find("struct TypedCommandFullOperationJobFactory").map(|offset| start + offset).expect("typed full-operation factory");
        let job = &source[start..end];
        for stage in ["typed-command-prepare", "typed-command-reducer"] {
            assert!(job.contains(stage));
        }
        for forbidden in ["A::handle", "A::ephemeral", "bounded_command_output_bytes", "serde_json::to_vec", "fault.message.as_bytes().to_vec()", "for child in emit.child_emits", "ActiveToolCommand", "BoundedFirstStepCommandJob"] {
            assert!(!job.contains(forbidden), "forbidden typed-command shortcut returned: {forbidden}");
        }
        let route = source.rfind("async fn dispatch_typed_command_inner").expect("typed command route");
        let gate = source[route..].find("self.media.require_complete_tool_operation_pipeline(&admission)?").expect("fail-closed full-operation gate");
        let queue = source[route..].find("self.tools.latest_wins_commands.insert_admitted(").expect("latest-wins admission");
        assert!(gate < queue, "incomplete typed command must fail before it is admitted into the latest-wins queue");
        assert!(!source[route..route + queue].contains("self.refresh_cache().await"), "typed dispatch performs no legacy preparation before admission");
        for decoder in ["let command=A::command_from_action(action,if source.assembled.is_some()", "let command=A::command_from_action(command_id,if assembled.is_some()"] {
            let decoder = source.find(decoder).expect("typed route decoder");
            let gate = source[decoder..].find("self.media.require_complete_tool_operation_pipeline(&admission)?").expect("typed route full-operation gate") + decoder;
            let reducer = source[decoder..].find("self.as_dispatch().dispatch_typed_command_inner(").expect("typed route reducer entry") + decoder;
            assert!(gate < reducer, "generic command construction must not reach the reducer before full-operation admission");
        }
        let intent = source.find("async fn handle_intent_frame(&mut self, intent: &UiIntent").expect("intent route");
        let intent_end = source[intent..].find("async fn resume_task_emit").map(|offset| intent + offset).expect("intent route end");
        let intent_route = &source[intent..intent_end];
        for forbidden in ["serde_json::to_value", "serde_json::to_vec", "A::command_from_intent"] {
            assert!(!intent_route.contains(forbidden), "intent route performed pre-admission work: {forbidden}");
        }
        assert!(source.contains("pub fn dispatch_typed"), "typed-value route must remain available to the dependency testkit");
    }

    /// ♻️ The seven per-lane `Closing` arms collapsed into ONE shared
    /// `MountedTypedCommandFullOperation::granted_retirement_step` (ticket 26/09/09/PROCEDURAL-3D-END-TO-END),
    /// so the retained one-item retirement seam is that call, not a per-lane `close_step`.
    ///
    /// 🧵️ The one-page publisher suspends in EXACTLY one place: the task lane's spawn. Every
    /// other lane stays synchronous, so the awaits are enumerated and the one that is allowed is
    /// named by its own call — a new `.await` anywhere in the publisher fails here by name.
    #[test]
    fn fixture_contract_is_anchored_to_the_production_retained_factory_publisher_and_host_receivers() {
        let source = instance_source();
        let admission_marker = ["async fn admit_command_json_with_", "proof(&self"].concat();
        let admission_start = source.rfind(&admission_marker).expect("production raw JSON admission");
        let admission_end = source[admission_start..].find("pub async fn new(app: A, actor").map(|offset| admission_start + offset).expect("admission route end");
        let admission = &source[admission_start..admission_end];
        let reserve = admission.find(".begin_exact_wire").expect("maximum extent authority");
        let encode = admission.find("semio_framework_pack_json::to_json_string(&(verb, wire_args))").expect("bounded wire encoder");
        let seal = admission.find("finish_admitted_prefix").expect("truthful exact prefix");
        assert!(reserve < encode && encode < seal);

        let dispatch_start = source.rfind("async fn start_typed_command_operation").expect("production typed operation setup");
        let dispatch_end = source[dispatch_start..].find("pub fn dispatch_typed").map(|offset| dispatch_start + offset).expect("production setup end");
        let dispatch = &source[dispatch_start..dispatch_end];
        assert!(dispatch.contains("PendingLatestWinsCommand {"));
        assert!(dispatch.contains("admission: Some(admission)"));
        assert!(dispatch.contains("source_identity:(cx.operation(),cx.generation())"));

        let publisher_start = source.rfind("fn publish_mounted_typed_operation_unit").expect("production one-page publisher");
        let publisher_end = source[publisher_start..].find("pub async fn advance_typed_operation_publication_one").map(|offset| publisher_start + offset).expect("publisher end");
        let publisher = &source[publisher_start..publisher_end];
        for retained_seam in ["pending_artifact_publication", "admit_apply_batch", "advance_apply_batch", "ArtifactStoreOneItemAdvance::Published"] {
            assert!(publisher.contains(retained_seam), "production publisher lost its retained one-item seam: {retained_seam}");
        }
        for forbidden in [".apply_one(", "artifact_mutations.last().cloned()", "config_mutations.last().cloned()", "draft_mutations.last().cloned()", "presence.last().cloned()", "transient.last().cloned()"] {
            assert!(!publisher.contains(forbidden), "production publisher restored a monolithic or clone-then-pop shortcut: {forbidden}");
        }
        let freshness = publisher.find("typed-operation pending publication rejected a stale immutable document root").expect("per-turn document freshness gate");
        let pending_advance = publisher[freshness..].find("\n        if let Some(pending) = mounted.pending_artifact_publication.as_mut() {\n            let grant=").map(|offset| freshness + offset).expect("pending publication advance");
        assert!(freshness < pending_advance, "document freshness must fail closed before every retained lane advance");
        assert!(publisher[..pending_advance].contains("pending.begin_close()"));
        assert!(publisher[..pending_advance].contains("self.stores.handoff_mounted_durable_publication(mounted)?"));
        assert!(!publisher.contains("dispatch_emit_group("));
        assert!(!publisher.contains(".await"), "the production one-page publisher is synchronous: its task lane spawns without suspending");
        assert!(publisher.contains("tasks_task_lane::spawn_task(mounted.meta.instance_id, &mounted.meta, task)?"));
        assert!(publisher.contains("typed_effect_outbox.push"));
        assert!(publisher.contains("typed_event_outbox.push"));
        assert!(publisher.contains("typed_ui_outbox.push"));
        assert!(source.contains("typed operation can retain exactly one unacknowledged result page"));

        let reactor = semio_framework_plugin_test_support::read_source_family_source(&fixture_path!("../../../..").join("⚛️reactor")).expect("the reactor family reads");
        assert!(reactor.contains("take_typed_operation_effect"));
        assert!(reactor.contains("take_typed_operation_event"));
        assert!(reactor.contains("take_typed_operation_ui_scope"));
        let receiving=semio_framework_test_fixtures::fixture_text!("../../../../⚛️reactor/🏃️runtime/📮️publication/🦀️.rs");
        assert!(receiving.contains("ResultPageWireReceiver"));
        assert!(receiving.contains("cx.consume_retained(p)"));
        assert!(!reactor.contains("page.renderer_exchange_bytes()"));
        assert!(reactor.contains("plugin_drive_result_pages"));
        assert!(reactor.contains("TypedOperationResultPage::renderer_ack_token"));
        assert!(reactor.contains("plugin_acknowledge_typed_operation_result(runtime, token)"));
        assert!(!reactor.contains("typed_operation_results: _"), "the old typed-result drop route must remain unreachable");
    }

    #[test]
    fn language_neutral_renderer_page_and_exact_ack_have_bounded_stable_wire_fields() {
        let token = TypedOperationResultToken { receiver: u32::MAX, operation: u64::MAX, generation: u64::MAX - 1, sequence: u32::MAX, attempt: 2 };
        let page = TypedOperationResultPage::try_new(token, TypedOperationResultLane::Terminal, &[0x5a; TYPED_OPERATION_RESULT_PAGE_BYTES]).expect("exact renderer page maximum");
        let wire = page.renderer_exchange_bytes();
        assert!(wire.starts_with(TypedOperationResultPage::RENDERER_PAGE_MAGIC));
        assert_eq!(&wire[wire.len() - TYPED_OPERATION_RESULT_PAGE_BYTES..], &[0x5a; TYPED_OPERATION_RESULT_PAGE_BYTES]);

        let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../../../../../🔨️modules/🎭️actor/🧪️tests/🗞️typed-operation-page/🧫️fixtures/🔣️.json")).expect("neutral result lanes");
        for row in fixture["lanes"].as_array().expect("result lanes") {
            let lane: TypedOperationResultLane = semio_framework_pack_json::from_json_str(&serde_json::to_string(&row["name"]).unwrap(), semio_framework_pack_json::JsonMemberPolicy::Reject).expect("own lane decoder");
            let page = TypedOperationResultPage::try_new(token, lane, &[0x5a]).expect("declared result lane");
            let bytes = page.renderer_exchange_bytes();
            assert_eq!(bytes[TypedOperationResultPage::RENDERER_PAGE_MAGIC.len() + 25], row["tag"].as_u64().unwrap() as u8);
        }

        let mut ack = Vec::from(TypedOperationResultPage::RENDERER_ACK_MAGIC);
        ack.extend_from_slice(&token.receiver.to_le_bytes());
        ack.extend_from_slice(&token.operation.to_le_bytes());
        ack.extend_from_slice(&token.generation.to_le_bytes());
        ack.extend_from_slice(&token.sequence.to_le_bytes());
        ack.push(token.attempt);
        assert_eq!(TypedOperationResultPage::renderer_ack_token(&ack), Some(token));
        ack.push(0);
        assert_eq!(TypedOperationResultPage::renderer_ack_token(&ack), None);
    }

    #[test]
    fn host_configuration_uses_one_bounded_event_sourced_lane_before_the_generic_gate() {
        let source=instance_source();
        let start=source.find("async fn drive_original_host_configuration").expect("original host source driver");
        let end=source[start..].find("async fn drive_original_emission").map(|offset|start+offset).expect("original mounted emission driver");
        let host=&source[start..end];
        let classify=host.find("A::original_host_configuration(action)").expect("classification before producer");
        let admit=host.find("self.original_emission.admit_host(").expect("actual original source admission");
        let control=host.find("prepare_original_tool_dispatch(").expect("retained control decision");
        let produce=host.find("self.original_emission.advance_host(cx)").expect("native configuration producer");
        let settle=host.find("render.settle_press(").expect("actual retained press settlement");
        let finish=host.find("self.original_emission.finish_host(").expect("original metadata and emission handoff");
        assert!(classify<admit&&admit<control&&control<produce&&produce<settle&&settle<finish);
        assert!(host.contains("&mut source.original_tool_dispatch"));
        assert!(host.contains("&mut source.original_press_result"));
        assert!(host.contains("&meta.actor"));
        assert!(host.contains("self.original_emission.host_base.as_ref()"));
        assert!(host.contains("finish_empty_host(action,metadata,cx)"));
        assert!(host.contains("self.drive_original_emission(cx).await"));
        let action=source.find("async fn dispatch_action_received").expect("action driver");
        let command=source[action..].find("async fn dispatch_command(").map(|offset|action+offset).expect("manifest command driver");
        assert!(source[action..command].contains("A::original_host_configuration(action).is_some()"));
        assert!(source[command..].contains("A::original_host_configuration(command_id).is_some()"));
        for cold in ["A::host_configuration_mutation(","admit_host_configuration_json(","OpBinary::encode_op(&config_mutation)","dispatch_emit(command_id, Emit::config"]{assert!(!source.contains(cold),"cold host producer route returned: {cold}");}
    }
