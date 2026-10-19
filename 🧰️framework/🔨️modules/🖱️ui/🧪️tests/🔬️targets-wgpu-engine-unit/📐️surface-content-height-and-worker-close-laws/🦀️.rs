/// 📐️ Surface content height and worker close laws: the ui surface slot table, the intrinsic surface content height, the discarded candidate, the retained input discard and the original layout admission and worker close.

/// 🧱️ The `boxed_fixed_slots` law for this module's fixed slot tables, against the one committed
/// budget every implementation of it reads (`the committed fixed-slot fixture`).
///
/// Asserts the measured shape of each table (capacity, one slot's bytes, the owner's own bytes)
/// against that record, that each owner is smaller than the table it owns — the structural proof the
/// slots are heap-first rather than an inline `[T; N]` field — and then constructs them on a thread
/// holding only the fixture's `boundedThreadStackBytes`. `Builder::stack_size` overrides
/// `RUST_MIN_STACK`, so the repo runner's 128 MiB floor cannot hide a re-inflated frame here.
#[test]
fn ui_surface_slot_table_is_heap_first_and_fits_a_bounded_thread_stack() {
    let fixture: serde_json::Value = serde_json::from_str(fixture_text!("../../../../../../../🧪️tests/🧫️fixtures/🧱️boxed-fixed-slots/🔣️.json")).expect("🧱️ the committed fixed-slot-table budget parses");
    let declared: Vec<semio_framework_async::FixedSlotTableBudget> = fixture["tables"]
        .as_array()
        .expect("🧱️ the budget lists its tables")
        .iter()
        .filter(|table| table["guard"] == "ui::wgpu_engine")
        .map(|table| {
            semio_framework_async::FixedSlotTableBudget::new(
                table["owner"].as_str().expect("owner"),
                table["capacity"].as_u64().expect("capacity") as usize,
                table["elementSizeBytes"].as_u64().expect("element bytes") as usize,
                table["ownerSizeBytes"].as_u64().expect("owner bytes") as usize,
            )
        })
        .collect();
    let measured = vec![semio_framework_async::FixedSlotTableBudget::new("wgpu::engine::UiSurfaceRegistry", UI_LAYOUT_SURFACE_SLOTS, size_of::<Option<UiSurfaceSlot>>(), size_of::<UiSurfaceRegistry>())];
    semio_framework_async::assert_fixed_slot_tables(
        "ui::wgpu_engine",
        fixture["boundedThreadStackBytes"].as_u64().expect("bounded stack budget") as usize,
        fixture["conversionThresholdBytes"].as_u64().expect("conversion threshold") as usize,
        &declared,
        &measured,
        || {
            drop(UiSurfaceRegistry::default());
        },
    );
}

/// 📐️ `surface_content_height` answers the DOCUMENT's own extent, never the viewport it was laid out
/// against — it is the measure a floating panel hugs its content with (`anchor_panel_rect`'s
/// `content_h`), so answering the caller's own input makes that hug a no-op and leaves every panel at
/// its full column band. Measured live on 6118: the `framework.panel.toolRun` root reported 781.6
/// while its one run group was 233.96 tall, and the Tool-runs panel covered the whole right column
/// and the 3D preview under it (ticket 26/09/17/WGPU-RENDERER-REACT-PARITY,
/// `📓️w14b-generation3d-labels-preview-layout.md`).
#[test]
fn surface_content_height_measures_the_document_not_the_viewport_it_was_given() {
    let mut ui = Ui::new(semio_framework_ui_locale::Locale::En);
    let mut atlas = FontAtlas::builtin();
    ui.apply_tree("panel", &stack_ui(vec![button_ui("one", "One"), button_ui("two", "Two")]));
    drive_layout(&mut ui, "panel", 300.0, 780.0, &mut atlas);

    let short = ui.surface_content_height("panel").expect("a laid-out surface answers its content height");
    assert!(short > 0.0, "two buttons take some height, got {short}");
    assert!(short < 780.0, "two buttons do not take a 780 px column, got {short}");

    drive_layout(&mut ui, "panel", 300.0, 1_400.0, &mut atlas);
    let tall = ui.surface_content_height("panel").expect("content height");
    assert!((tall - short).abs() <= 1.0, "the document's extent is viewport-independent, got {short} then {tall}");
}

/// 🌲️ Compact content keeps its intrinsic extent when presentation swaps its accepted tree.
#[test]
fn compact_tree_content_height_survives_presentation_and_viewport_changes() {
    let law: serde_json::Value = serde_json::from_str(fixture_text!("../../../../../../../🧪️tests/🧫️fixtures/🌳️compact-tree-intrinsic/🔣️.json")).expect("compact tree height fixture");
    let surface = law["surface"].as_str().unwrap();
    let nodes = law["nodes"].as_array().unwrap();
    let mut document = UiDocumentTree::new(UiDocumentLeaseHeader { generation: 1, surface: SurfaceId::try_from(surface).unwrap(), revision: UiRevision(1), root: UiNodeId(1), layout_epoch: 0, node_count: nodes.len() }).unwrap();
    for node in nodes {
        document.try_upsert_record(serde_json::from_value(node.clone()).expect("compact row record")).unwrap();
    }
    let mut ui = Ui::new(semio_framework_ui_locale::Locale::En);
    let mut atlas = FontAtlas::builtin();
    assert!(ui.publish_document(surface, document));
    drive_scene_lifetime_reconcile(&mut ui, surface, 1);
    let expected = law["expectedHeight"].as_f64().unwrap() as f32;
    for (index, height) in law["viewports"].as_array().unwrap().iter().enumerate() {
        drive_scene_lifetime_reconcile(&mut ui, surface, 1);
        drive_layout(&mut ui, surface, law["width"].as_f64().unwrap() as f32, height.as_f64().unwrap() as f32, &mut atlas);
        let before = ui.surface_content_height(surface).expect("candidate content extent");
        assert!((before - expected).abs() < 0.02, "candidate content height {before} != {expected}");
        acknowledge_scene_lifetime_candidate(&mut ui, surface, index as u64 + 31);
        let after = ui.surface_content_height(surface).expect("presentation preserves content extent");
        assert!((after - expected).abs() < 0.02, "presented content height {after} != {expected}");
    }
    let mut oracle = taffy::TaffyTree::<()>::new();
    oracle.disable_rounding();
    let children = law["visibleRowHeights"]
        .as_array()
        .unwrap()
        .iter()
        .map(|height| {
            oracle.new_leaf(taffy::Style { size: taffy::geometry::Size { width: taffy::style::Dimension::length(300.0), height: taffy::style::Dimension::length(height.as_f64().unwrap() as f32) }, flex_shrink: 0.0, ..Default::default() }).unwrap()
        })
        .collect::<Vec<_>>();
    let root = oracle.new_with_children(taffy::Style { flex_direction: taffy::style::FlexDirection::Column, ..Default::default() }, &children).unwrap();
    oracle.compute_layout(root, taffy::geometry::Size { width: taffy::style::AvailableSpace::MaxContent, height: taffy::style::AvailableSpace::MaxContent }).unwrap();
    assert!((oracle.layout(root).unwrap().size.height - expected).abs() < 0.02, "independent flex oracle agrees with the measured React row sum");
}

/// 🧾️ A discarded layout cannot lend its height witness to the still-presented document.
#[test]
fn intrinsic_content_height_cannot_leak_from_a_discarded_candidate() {
    let law: serde_json::Value = serde_json::from_str(fixture_text!("../../../../../../../🧪️tests/🧫️fixtures/🌳️compact-tree-intrinsic/🔣️.json")).unwrap();
    let surface = law["surface"].as_str().unwrap();
    let sequence = law["publicationSequence"].as_array().unwrap();
    let document = |generation: u64, presentation: &str| {
        let nodes = law["nodes"].as_array().unwrap();
        let mut document = UiDocumentTree::new(UiDocumentLeaseHeader { generation, surface: SurfaceId::try_from(surface).unwrap(), revision: UiRevision(generation), root: UiNodeId(1), layout_epoch: 0, node_count: nodes.len() }).unwrap();
        for node in nodes {
            let mut node = node.clone();
            if node["id"] == 1 {
                node["component"]["presentation"] = presentation.into();
            }
            document.try_upsert_record(serde_json::from_value(node).unwrap()).unwrap();
        }
        document
    };
    let mut ui = Ui::new(semio_framework_ui_locale::Locale::En);
    let mut atlas = FontAtlas::builtin();
    assert!(ui.publish_document(surface, document(sequence[0]["generation"].as_u64().unwrap(), sequence[0]["presentation"].as_str().unwrap())));
    drive_scene_lifetime_reconcile(&mut ui, surface, 1);
    drive_layout(&mut ui, surface, 300.0, 720.0, &mut atlas);
    acknowledge_scene_lifetime_candidate(&mut ui, surface, 61);
    let presented = ui.surface_content_height(surface).expect("A is presented");
    drive_scene_lifetime_reconcile(&mut ui, surface, 1);
    assert!(ui.publish_document(surface, document(sequence[1]["generation"].as_u64().unwrap(), sequence[1]["presentation"].as_str().unwrap())));
    drive_scene_lifetime_reconcile(&mut ui, surface, 2);
    drive_layout(&mut ui, surface, 300.0, 720.0, &mut atlas);
    let discarded = ui.surface_content_height(surface).expect("B owns a distinct candidate extent");
    assert!((presented - sequence[0]["height"].as_f64().unwrap() as f32).abs() < 0.02);
    assert!((discarded - sequence[1]["height"].as_f64().unwrap() as f32).abs() < 0.02);
    assert!(ui.seal_presented_input_candidate(62, &[surface.to_string()]));
    assert!(ui.discard_presented_input_candidate(62));
    assert!(ui.publish_document(surface, document(sequence[2]["generation"].as_u64().unwrap(), sequence[2]["presentation"].as_str().unwrap())));
    let before_layout = ui.surface_content_height(surface);
    let expected = sequence[2]["heightBeforeLayout"].as_f64().unwrap() as f32;
    assert!(before_layout.is_none_or(|height| (height - expected).abs() < 0.02), "C has no layout; A's {presented} may be reused, never discarded B's {discarded}: {before_layout:?}");
}

#[test]
fn retained_input_discard_projects_the_shared_draft_law_and_schedules_paint() {
    let corpus: serde_json::Value = serde_json::from_str(fixture_text!("../../../../../../../../../🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/🧫️fixtures/🚦️input-draft-disposition/🔣️.json")).unwrap();
    for (index, law) in corpus["cases"].as_array().unwrap().iter().enumerate() {
        let window = format!("retained-draft-{index}");
        let records = serde_json::json!([
            {"id":0,"key":"draft/root","component":{"type":"container"},"layout":{"kind":"stack","axis":"vertical","gap":"none","padding":{"all":"none"},"align":"stretch","justify":"start","grow":true,"wrap":false},"style":{},"activity":"idle","accessibility":{},"children":[1]},
            {"id":1,"key":"draft/input","component":{"type":"input","kind":law["kind"],"value":law["base"],"commit":"blur"},"layout":{"kind":"leaf","width":"fill","height":"hug"},"style":{},"activity":"idle","accessibility":{"label":"Draft input"},"bindings":[{"trigger":"commit","action":{"scope":"fixture","name":"setValue","version":1}}]}
        ]);
        let mut document = UiDocumentTree::new(UiDocumentLeaseHeader { generation:1, surface:SurfaceId::try_from(window.as_str()).unwrap(), revision:UiRevision(1), root:UiNodeId(0), layout_epoch:1, node_count:2 }).unwrap();
        for record in records.as_array().unwrap() { document.try_upsert_record(serde_json::from_value(record.clone()).unwrap()).unwrap(); }
        let mut ui = Ui::new(semio_framework_ui_locale::Locale::En);
        assert!(ui.publish_document(&window, document));
        drive_scene_lifetime_reconcile(&mut ui, &window, 1);
        acknowledge_scene_lifetime_candidate(&mut ui, &window, index as u64 + 1);
        let key = |name:&str, ctrl:bool| UiEvent::KeyDown { key:name.into(), modifiers:semio_framework_ui_wgpu_events::EventModifiers { ctrl, ..Default::default() } };
        ui.dispatch_event(&window, key("Tab",false));
        ui.dispatch_event(&window, key("a",true));
        ui.dispatch_event(&window, UiEvent::TextInput { text:law["submitted"].as_str().unwrap().into() });
        let commands = ui.dispatch_event(&window, key("Enter",law["kind"] == "longText"));
        let intent = commands.iter().find_map(|command| match command { UiCommand::App { intent, .. } => Some(intent), _ => None }).unwrap();
        let submitted = ui.retained_input_draft(&window,intent).unwrap();
        if law["draft"] != law["submitted"] {
            ui.dispatch_event(&window,key("a",true));
            ui.dispatch_event(&window,UiEvent::TextInput { text:law["draft"].as_str().unwrap().into() });
        }
        drive_scene_lifetime_reconcile(&mut ui, &window, 1);
        let owned = ui.windows.get_mut(&window).unwrap();
        let root = owned.tree.root.unwrap();
        for flag in [NodeFlags::DIRTY_PAINT, NodeFlags::DIRTY_LAYOUT, NodeFlags::SUBTREE_DIRTY] { owned.tree.node_mut(root).unwrap().flags.set(flag, false); }
        assert!(!ui.needs_frame());
        if law["cancelled"] != true && law["disposition"] == "discard" {
            let restored = ui.discard_retained_input_draft(&window,&submitted);
            assert_eq!(restored,law["draft"] == law["submitted"]);
            assert_eq!(ui.needs_frame(),restored,"a discarded focused input schedules its next painted frame");
            if restored {
                drive_scene_lifetime_reconcile(&mut ui, &window, 1);
                acknowledge_scene_lifetime_candidate(&mut ui, &window, index as u64 + 101);
            }
        }
        let visible = semio_framework_ui_wgpu_hosting::accessibility::accessibility_projection(ui.tree(&window).unwrap()).into_iter().find(|node|node.key == "draft/input").unwrap().value_text.unwrap();
        assert_eq!(visible,law["expected"].as_str().unwrap(),"{}",law["name"]);
    }
}

#[test]
fn original_layout_admission_params_cancel_and_full_error_close_with_system_receipts(){
 use semio_framework_job::{OperationId,Generation,InteractiveStage,RetainedCloneGrant};
use semio_framework_job_worker_authority::{BatchJobParams,BatchDriveConfig};
 use semio_framework_value::{ValueError,ValueRefusalKind};
 use crate::physical_allocator::measured;
 let law:serde_json::Value=serde_json::from_str(fixture_text!("../../../../../../../🧪️tests/🔬️targets-wgpu-mounted-layout-unit/🧫️fixtures/🫴️receiving.json")).unwrap();
 for copy in law["admission"]["copies"].as_array().unwrap().iter().map(|row|row.as_u64().unwrap()as usize){
  let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:copy,maximum_capacity_bytes:law["grant"]["maximumCapacityBytes"].as_u64().unwrap()as usize,maximum_release_bytes:law["grant"]["maximumReleaseBytes"].as_u64().unwrap()as usize,maximum_depth:law["grant"]["maximumDepth"].as_u64().unwrap()as usize};
  let((root,receipt),root_born,root_free)=measured(||semio_framework_job::CancelToken::admit_root(grant).unwrap().unwrap());assert!(receipt.fits(grant));assert_eq!((root_born,root_free),(receipt.retained_capacity_bytes,receipt.released_bytes));
  let((mut original,pointer),born,free)=measured(||{let error=ValueError::new(ValueRefusalKind::InvariantViolated,"original owned layout refusal 雪🌊️");let pointer=error.message.as_ptr();let mut original=LayoutWorkerAdmission::default();*original.params=Some(BatchJobParams{operation:OperationId(law["operation"].as_u64().unwrap()),generation:Generation(law["admission"]["callerGeneration"].as_u64().unwrap()),cancel:root.clone(),config:BatchDriveConfig{retained:grant,site:"ui.layout-original-receipt",stage:InteractiveStage::UserVisibleSimStep,fuel_per_step:1,step_budget_us:1000},now_us:semio_framework_job::default_now_us});*original.failure=Some(error);(original,pointer)});
  let owned=born-free;let(mut births,mut releases,mut idle)=(0,0,0);
  for turn in 0..100000{
   if original.terminal_is_empty(){break}
   let(step,born,free)=measured(||original.close_step(RetainedCloneGrant{maximum_items:0,..grant},Some(&root)).unwrap());assert_eq!(step,Some(Default::default()));assert_eq!((born,free),(0,0));
   if let Some(error)=original.failure.as_ref(){assert_eq!(error.message.as_ptr(),pointer)}
   if let Some(error)=original.failure_close.as_ref().and_then(|owner|owner.original()){assert_eq!(error.message.as_ptr(),pointer)}
   if let Some(cancel)=original.cancel.as_ref(){if !cancel.terminal_is_empty(){assert!(cancel.is_original_alias_witness(&root))}}
   let(step,born,free)=measured(||original.close_step(grant,Some(&root)).unwrap());let receipt=step.expect("same borrowed root funds original alias return");assert!(receipt.fits(grant));assert_eq!((born,free),(receipt.retained_capacity_bytes,receipt.released_bytes));births+=born;releases+=free;if receipt==Default::default(){idle+=1}else{idle=0}assert!(idle<law["admission"]["maximumZeroTurns"].as_u64().unwrap()as usize,"original layout source stalled copy={copy} turn={turn}");
  }
  assert!(original.terminal_is_empty());assert_eq!(owned+births,releases);assert_eq!(measured(||drop(original)),((),0,0));assert!(!root.is_cancelled_now());
  let mut root=semio_framework_async::CancelTokenRetirement::from_token(root);for _ in 0..1000{if root.terminal_is_empty(){break}let(step,born,free)=measured(||root.close_step(grant).unwrap());let receipt=step.progress();assert!(receipt.fits(grant));assert_eq!((born,free),(receipt.retained_capacity_bytes,receipt.released_bytes));births+=born;releases+=free;}assert!(root.terminal_is_empty());assert_eq!(root_born+owned+births,releases);assert_eq!(measured(||drop(root)),((),0,0));
 }
}

/// 🧳️ The real window keeps both failed worker sources through denied and fully funded original close.
#[test]
fn original_ui_worker_close_retains_whole_failure_and_original_cx(){
 let law:serde_json::Value=serde_json::from_str(fixture_text!("../../../../../../../🧪️tests/🔬️targets-wgpu-mounted-layout-unit/🧫️fixtures/🧵️original-close.json")).unwrap();
 for copy in law["copyGrants"].as_array().unwrap(){let mut ui=Ui::new(semio_framework_ui_locale::Locale::En);let id="original-worker-failure";let token=ui.try_admit_surface(id).unwrap();assert!(ui.begin_surface_close(token));let((failure,pointers),birth)=semio_framework_trace::observe_heap_allocations_on_this_thread(||{let primary=semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::UnsupportedOwner,"original UI worker cause 源");let receipt=semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvariantViolated,"original separate UI receipt 源");let pointers=(primary.message.as_ptr(),receipt.message.as_ptr());({let original=semio_framework_job::OriginalTurnFailure::from(primary);semio_framework_job::OriginalTurnFailure{receipt_refusal:Some(receipt),context_refusal:None,..original}},pointers)});
  let(_,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||ui.windows.get_token_mut(token).unwrap().retain_layout_close_failure("Layout.CloseSession",failure));assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));let grant=semio_framework_job::RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:copy.as_u64().unwrap()as usize,maximum_capacity_bytes:262144,maximum_release_bytes:262144,maximum_depth:128};let mut sequence=0;let cancel=semio_framework_job::root_cancel_token();
  for denial in law["denials"].as_array().unwrap(){let denial=denial.as_str().unwrap();let mut receipt=Default::default();let mut cx=semio_framework_job::StepContext::new(semio_framework_job::OperationId(17),semio_framework_job::Generation(9),semio_framework_job::StepBudget::new(if denial=="work"{0}else{1},if denial=="deadline"{0}else{12345},semio_framework_job::RetainedCloneGrant{maximum_items:usize::from(denial!="items"),..grant}),cancel.clone(),||Some(1),&mut sequence,&mut receipt);let(result,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||ui.close_surface_one(token,&mut cx));assert!(matches!(result.unwrap(),UiSurfaceCloseStep::Pending));assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));assert_eq!(cx.retained_progress(),Default::default());let original=ui.layout_retirement_failure(id).unwrap();assert_eq!((original.primary_value().unwrap().message.as_ptr(),original.receipt_refusal.as_ref().unwrap().message.as_ptr()),pointers);}
  let(mut born,mut freed)=(birth.requested_bytes,birth.released_bytes);for turn in 0..100000{if !ui.layout_retirement_failure_pending(id){break}let mut receipt=Default::default();let mut cx=semio_framework_job::StepContext::new(semio_framework_job::OperationId(17),semio_framework_job::Generation(9),semio_framework_job::StepBudget::new(1,12345,grant),cancel.clone(),||Some(1),&mut sequence,&mut receipt);let(result,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||ui.close_surface_one(token,&mut cx));assert!(matches!(result.unwrap(),UiSurfaceCloseStep::Pending));assert_eq!((heap.requested_bytes,heap.released_bytes),(cx.retained_progress().retained_capacity_bytes,cx.retained_progress().released_bytes));born+=heap.requested_bytes;freed+=heap.released_bytes;assert!(turn<99999)}assert_eq!(born,freed);assert!(!ui.layout_retirement_failure_pending(id));assert!(ui.layout_retirement_refusal(id).is_none());close_surface_to_terminal(&mut ui,token);
 }
}
