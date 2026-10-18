/// 🌳️ Mounted tree publication laws: the tracker stack budget, one reconcile opportunity per grant, document tree, settings controls and catalogue publication, producer failure reporting, reconcile capacity and the window and panel sources.

#[test]
fn tracker_initialization_fits_the_component_stack_budget() {
    let mut original_caller=semio_framework_plugin_testing_caller::OriginalFixtureCaller::admit().expect("independently authored original patch fixture caller");
    let mut original_context=original_caller.context();let cx=&mut original_context;
    let result=(||{
    let _guard = semio_framework_ui_runtime::surface_reconcile_registry_test_guard();
    let bytes = size_of::<PatchTrackerState>();
    assert!(bytes <= 256, "PatchTrackerState requires {bytes} bytes");
    let state = PatchTrackerState::default();
    assert_eq!(state.slots.len(), SURFACE_RECONCILE_ADMISSION_SLOTS);
    assert_eq!(state.unadmitted.len(), SURFACE_RECONCILE_ADMISSION_SLOTS + 1);
    assert_eq!(state.ready.len(), READY_PATCH_CAPACITY);

    })();
    let fuel=original_context.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original patch fixture caller paid source close");result
}

#[test]
fn mounted_path_advances_one_reconcile_opportunity_per_grant() {
    let mut original_caller=semio_framework_plugin_testing_caller::OriginalFixtureCaller::admit().expect("independently authored original patch fixture caller");
    let mut original_context=original_caller.context();let cx=&mut original_context;
    let result=(||{
    let _guard = semio_framework_ui_runtime::surface_reconcile_registry_test_guard();
    let tracker = PatchTracker::new();
    tracker.begin("main".into(), leaf("root", "a")).expect("admitted");
    assert!(tracker.take_ready_patch().is_none());
    assert!(tracker.drive_one(cx).expect("original patch fixture semantic step"));
    assert!(tracker.take_ready_patch().is_none());
    let patch = finish(&tracker,cx).expect("eventual patch");
    assert_eq!(patch.base_revision, ui_contract::UiRevision(0));

    })();
    let fuel=original_context.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original patch fixture caller paid source close");result
}

#[test]
fn mounted_document_tree_publishes_nested_interactive_rows() {
    let mut original_caller=semio_framework_plugin_testing_caller::OriginalFixtureCaller::admit().expect("independently authored original patch fixture caller");
    let mut original_context=original_caller.context();let cx=&mut original_context;
    let result=(||{
    let _guard = semio_framework_ui_runtime::surface_reconcile_registry_test_guard();
    use ui_contract::{Buildable, HasBase, HasChildren};
    fn row(value: &serde_json::Value) -> ui_contract::BuiltNode {
        let id = value["id"].as_str().unwrap();
        let target = serde_json::from_value(serde_json::json!({ "scope": "fixture", "version": 1, "args": { "domainId": "fixture", "merge": "replace", "method": "pick", "targets": id }, "activation": "select" })).unwrap();
        let mut builder = ui_contract::tree_item(ui_contract::Label(ui_contract::UiText::try_from_str(id).unwrap())).try_id(id).ok().unwrap().target(target);
        for name in value["rowActions"].as_array().into_iter().flatten() {
            let name = name.as_str().unwrap();
            builder = builder
                .try_row_action(ui_contract::RowAction {
                    icon: ui_contract::UiText::try_from_str(name).unwrap(),
                    label: Some(ui_contract::Label(ui_contract::UiText::try_from_str(name).unwrap())),
                    verb: ui_contract::UiText::try_from_str(name).unwrap(),
                    placement: ui_contract::RowActionPlacement::Row,
                    disabled: false,
                    reason: None,
                })
                .ok()
                .unwrap();
        }
        builder.try_children(value["children"].as_array().into_iter().flatten().map(row)).ok().unwrap().try_build().unwrap()
    }
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/📃️document-surface.json")).unwrap();
    assert_eq!(ui_contract::UI_NODE_BINDINGS as u64, fixture["limits"]["nodeBindings"].as_u64().unwrap());
    assert_eq!(semio_framework_ui_runtime::SurfaceReconcileLimits::default().max_bytes as u64, fixture["limits"]["surfaceBytes"].as_u64().unwrap());
    assert_eq!(semio_framework_ui_runtime::SURFACE_RECONCILE_PAGE_BYTES as u64, fixture["limits"]["pageBytes"].as_u64().unwrap());
    assert_eq!(semio_framework_ui_runtime::SURFACE_RECONCILE_AGGREGATE_BYTES as u64, fixture["limits"]["aggregateBytes"].as_u64().unwrap());
    assert_eq!(
        semio_framework_ui_runtime::SURFACE_RECONCILE_AGGREGATE_BYTES,
        ui_contract::UI_RESIDENT_SLOTS * ui_contract::UI_DOCUMENT_NODES * size_of::<ui_contract::UiNodeRecord>(),
        "the aggregate budget is exactly sixty-four full-document record baselines — a drift in either factor moves the recorded literal above"
    );
    let sections = fixture["sections"].as_array().unwrap().iter().map(|section| {
        ui_contract::tree_section(ui_contract::Label(ui_contract::UiText::try_from_str(section["id"].as_str().unwrap()).unwrap()))
            .try_id(section["id"].as_str().unwrap())
            .ok()
            .unwrap()
            .try_children(section["rows"].as_array().unwrap().iter().map(row))
            .ok()
            .unwrap()
            .try_build()
            .unwrap()
    });
    let root = ui_contract::tree().try_id("document").ok().unwrap().try_children(sections).ok().unwrap().try_build().unwrap();
    let tracker = PatchTracker::new();
    {let mut grant=reserve(&tracker, ui_contract::SurfaceId::try_from(fixture["surface"].as_str().unwrap()).unwrap()).unwrap();let mut source=Some(ComponentTree::new(root));assert!(grant.commit_source(&mut source,cx).unwrap(), "actual original source admitted");assert!(source.is_none());}
    let patch = finish(&tracker,cx).unwrap_or_else(|| panic!("document never published: {:?}", tracker.state.borrow().terminals.iter().flatten().map(|slot| slot.authority.fault()).collect::<Vec<_>>()));
    let nodes = patch.ops.iter().filter_map(|op| if let ui_contract::UiPatchOp::Upsert(node) = op { Some(node) } else { None }).collect::<Vec<_>>();
    assert_eq!(nodes.len(), fixture["nodes"].as_u64().unwrap() as usize);
    let activates = |node: &&&ui_contract::UiNodeRecord| matches!(&node.component, ui_contract::Component::TreeItem(props) if props.target.as_ref().is_some_and(|target| target.activation.is_some()));
    assert_eq!(nodes.iter().filter(activates).count(), fixture["interactiveRows"].as_u64().unwrap() as usize);

    })();
    let fuel=original_context.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original patch fixture caller paid source close");result
}

#[test]
fn mounted_settings_controls_publish_with_authored_fields() {
    let mut original_caller=semio_framework_plugin_testing_caller::OriginalFixtureCaller::admit().expect("independently authored original patch fixture caller");
    let mut original_context=original_caller.context();let cx=&mut original_context;
    let result=(||{
    let _guard = semio_framework_ui_runtime::surface_reconcile_registry_test_guard();
    use ui_contract::{Buildable, HasBase, HasChildren};
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🎚️settings-surface.json")).expect("language-neutral settings");
    let fields = fixture["fields"].as_array().unwrap();
    let children = fields.iter().map(|field| {
        let id = field["id"].as_str().unwrap();
        let action = serde_json::from_value(serde_json::json!({ "scope": "fixture", "name": field["action"], "version": 1 })).unwrap();
        let mut control = ui_contract::BuiltNode::try_new(
            format!("{id}.control"),
            ui_contract::Component::NumberStepper(ui_contract::NumberStepperProps { value: field["value"].as_f64().unwrap(), step: field["step"].as_f64().unwrap(), uniform: false, ..Default::default() }),
        )
        .ok()
        .unwrap();
        control.bindings.try_push(ui_contract::ActionBinding { trigger: ui_contract::Trigger::Change, action, args: None, capability: None }).ok().unwrap();
        ui_contract::field(ui_contract::Label(ui_contract::UiText::try_from_str(field["label"].as_str().unwrap()).unwrap())).try_id(id).ok().unwrap().try_child(control).ok().unwrap().try_build().unwrap()
    });
    let root = ui_contract::section(ui_contract::Label(ui_contract::UiText::try_from_str(fixture["label"].as_str().unwrap()).unwrap())).try_id("settings").ok().unwrap().default_open(true).try_children(children).ok().unwrap().try_build().unwrap();
    let tracker = PatchTracker::new();
    {let mut grant=reserve(&tracker, ui_contract::SurfaceId::try_from(fixture["surface"].as_str().unwrap()).unwrap()).unwrap();let mut source=Some(ComponentTree::new(root));assert!(grant.commit_source(&mut source,cx).unwrap(), "actual original source admitted");assert!(source.is_none());}
    let patch = finish(&tracker,cx).unwrap_or_else(|| panic!("settings never published: {:?}", tracker.state.borrow().slots.iter().flatten().map(|slot| &slot.job).collect::<Vec<_>>()));
    let controls = patch
        .ops
        .iter()
        .filter_map(|op| match op {
            ui_contract::UiPatchOp::Upsert(node) => match &node.component {
                ui_contract::Component::NumberStepper(props) => Some(serde_json::json!({ "value": props.value, "step": props.step, "action": node.bindings[0].action.name.as_str() })),
                _ => None,
            },
            _ => None,
        })
        .collect::<Vec<_>>();
    assert_eq!(controls, fields.iter().map(|field| serde_json::json!({ "value": field["value"], "step": field["step"], "action": field["action"] })).collect::<Vec<_>>());

    })();
    let fuel=original_context.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original patch fixture caller paid source close");result
}

#[test]
fn mounted_catalogue_publishes_every_section_beyond_thirty_two_nodes() {
    let mut original_caller=semio_framework_plugin_testing_caller::OriginalFixtureCaller::admit().expect("independently authored original patch fixture caller");
    let mut original_context=original_caller.context();let cx=&mut original_context;
    let result=(||{
    let _guard = semio_framework_ui_runtime::surface_reconcile_registry_test_guard();
    use ui_contract::{Buildable, HasBase, HasChildren};
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🗂️catalogue-surface.json")).unwrap();
    let rows = fixture["rowsPerSection"].as_u64().unwrap();
    let mut expected = std::collections::BTreeSet::new();
    let sections = fixture["sections"].as_array().unwrap().iter().map(|section| {
        let section = section.as_str().unwrap();
        let children = (0..rows).map(|row| {
            let key = format!("{section}.{row}");
            expected.insert(key.clone());
            leaf(&key, &key).root
        });
        ui_contract::tree_section(ui_contract::Label(ui_contract::UiText::try_from_str(section).unwrap())).try_id(section).ok().unwrap().try_children(children).ok().unwrap().try_build().unwrap()
    });
    let root = ui_contract::tree().try_id("catalogue").ok().unwrap().try_children(sections).ok().unwrap().try_build().unwrap();
    assert_eq!(expected.len() as u64, fixture["expectedRows"].as_u64().unwrap());
    let tracker = PatchTracker::new();
    {let mut grant=reserve(&tracker, ui_contract::SurfaceId::try_from(fixture["surface"].as_str().unwrap()).unwrap()).unwrap();let mut source=Some(ComponentTree::new(root));assert!(grant.commit_source(&mut source,cx).unwrap(), "actual original source admitted");assert!(source.is_none());}
    let mut published = None;
    for _ in 0..65_536 {
        tracker.drive_one(cx).expect("original patch fixture semantic step");
        if let Some(owner) = tracker.take_ready_patch() {
            let (patch, authority) = publish_test(owner);
            let mut authority = Some(authority);
            let mut acknowledgement = None;
            assert!(semio_framework_ui_runtime::SurfaceReconcilePublishedPatch::acknowledge_into(
                &mut authority,
                &mut acknowledgement,
                fixture["surface"].as_str().unwrap(),
                patch.revision.0,
                semio_framework_ui_runtime::SurfaceReconcilePublishedPatch::required_acknowledge_bytes()
            )
            .unwrap());
            assert!(tracker.mark_published_ack(acknowledgement.as_ref().unwrap()).unwrap());
            close_ack(acknowledgement.take().unwrap());
            published = Some(patch);
            break;
        }
        if !tracker.has_work() {
            break;
        }
    }
    let fault = format!("{:?}", tracker.state.borrow().terminals.iter().flatten().map(|slot| slot.authority.fault()).collect::<Vec<_>>());
    close_instance_to_empty(&tracker, 1,cx);
    let patch = published.unwrap_or_else(|| panic!("catalogue did not publish: {fault}"));
    let nodes = patch.ops.iter().filter_map(|op| if let ui_contract::UiPatchOp::Upsert(node) = op { Some(node) } else { None }).collect::<Vec<_>>();
    assert_eq!(nodes.len() as u64, fixture["expectedNodes"].as_u64().unwrap());
    assert_eq!(nodes.iter().filter(|node| matches!(node.component, ui_contract::Component::Text(_))).map(|node| node.key.to_string()).collect::<std::collections::BTreeSet<_>>(), expected);

    })();
    let fuel=original_context.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original patch fixture caller paid source close");result
}

#[test]
fn mounted_catalogue_reports_producer_failure_once_before_cleanup() {
    let mut original_caller=semio_framework_plugin_testing_caller::OriginalFixtureCaller::admit().expect("independently authored original patch fixture caller");
    let mut original_context=original_caller.context();let cx=&mut original_context;
    let result=(||{
    let _guard = semio_framework_ui_runtime::surface_reconcile_registry_test_guard();
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🗂️catalogue-surface.json")).unwrap();
    let failure = &fixture["failure"];
    let key = failure["key"].as_str().unwrap();
    let mut root = leaf("catalogue", "Catalogue").root;
    root.children.try_push(leaf(key, key).root).unwrap();
    root.children.try_push(leaf(key, key).root).unwrap();
    let tracker = PatchTracker::new();
    {let mut grant=reserve(&tracker, ui_contract::SurfaceId::try_from(failure["surface"].as_str().unwrap()).unwrap()).unwrap();let mut source=Some(ComponentTree::new(root));assert!(grant.commit_source(&mut source,cx).unwrap(), "actual original source admitted");assert!(source.is_none());}
    let mut reported = None;
    for _ in 0..4096 {
        tracker.drive_one(cx).expect("original patch fixture semantic step");
        if let Some(fault) = tracker.take_render_fault() {
            reported = Some(fault);
            break;
        }
    }
    assert!(tracker.take_ready_patch().is_none());
    assert!(tracker.take_render_fault().is_none());
    close_instance_to_empty(&tracker, failure["instance"].as_u64().unwrap() as u32,cx);
    let (instance, reason) = reported.expect("producer fault must not disappear during cleanup");
    assert_eq!(instance as u64, failure["instance"].as_u64().unwrap());
    assert!(reason.contains(failure["surface"].as_str().unwrap()));
    assert!(reason.contains(failure["reason"].as_str().unwrap()));
    assert!(reason.contains(failure["authority"].as_str().unwrap()), "the report names the authority that read the fault, so an exact attribution is distinguishable from a generation-derived one: {reason}");
    assert!(reason.contains(&format!("parent={} key={}", failure["parent"].as_str().unwrap(), failure["key"].as_str().unwrap())), "a duplicate sibling key names the parent and the key it refused, not only the surface: {reason}");

    })();
    let fuel=original_context.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original patch fixture caller paid source close");result
}

#[test]
fn mounted_catalogue_reports_reconcile_capacity_without_leaking_owners() {
    let mut original_caller=semio_framework_plugin_testing_caller::OriginalFixtureCaller::admit().expect("independently authored original patch fixture caller");
    let mut original_context=original_caller.context();let cx=&mut original_context;
    let result=(||{
    let _guard = semio_framework_ui_runtime::surface_reconcile_registry_test_guard();
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🗂️catalogue-surface.json")).unwrap();
    let failure = &fixture["capacityFailure"];
    assert_eq!(ui_contract::UI_DOCUMENT_NODES as u64, failure["nodeLimit"].as_u64().unwrap());
    let mut root = leaf("catalogue", "Catalogue").root;
    for section in 0..failure["sections"].as_u64().unwrap() {
        let mut child = leaf(&section.to_string(), "Section").root;
        for row in 0..failure["rowsPerSection"].as_u64().unwrap() {
            child.children.try_push(leaf(&row.to_string(), "Row").root).unwrap();
        }
        root.children.try_push(child).unwrap();
    }
    let tracker = PatchTracker::new();
    {let mut grant=reserve(&tracker, ui_contract::SurfaceId::try_from(failure["surface"].as_str().unwrap()).unwrap()).unwrap();let mut source=Some(ComponentTree::new(root));assert!(grant.commit_source(&mut source,cx).unwrap(), "actual original source admitted");assert!(source.is_none());}
    let mut reported = None;
    for _ in 0..65_536 {
        tracker.drive_one(cx).expect("original patch fixture semantic step");
        if let Some(fault) = tracker.take_render_fault() {
            reported = Some(fault);
            break;
        }
    }
    assert!(tracker.take_ready_patch().is_none());
    assert!(tracker.take_render_fault().is_none());
    close_instance_to_empty(&tracker, failure["instance"].as_u64().unwrap() as u32,cx);
    let (instance, reason) = reported.expect("reconcile failure must not become an idle empty surface");
    assert_eq!(instance as u64, failure["instance"].as_u64().unwrap());
    assert!(reason.contains(failure["surface"].as_str().unwrap()));
    assert!(reason.contains(failure["reason"].as_str().unwrap()));

    })();
    let fuel=original_context.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original patch fixture caller paid source close");result
}

#[test]
fn mounted_sources_publish_every_window_and_panel_tree() {
    let mut original_caller=semio_framework_plugin_testing_caller::OriginalFixtureCaller::admit().expect("independently authored original patch fixture caller");
    let mut original_context=original_caller.context();let cx=&mut original_context;
    let result=(||{
    let _guard = semio_framework_ui_runtime::surface_reconcile_registry_test_guard();
    use ui_contract::{Buildable, HasBase, HasChildren};
    let fixtures: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🪟️mounted-surfaces.json")).expect("language-neutral surface fixtures");
    let fixtures = fixtures.as_array().expect("surface list");
    let tracker = PatchTracker::new();
    for fixture in fixtures {
        let surface = fixture["surface"].as_str().expect("surface key");
        let children = fixture["children"].as_array().expect("child keys").iter().map(|key| {
            let mut node = leaf(key.as_str().expect("child key"), "content").root;
            let action = serde_json::from_value(fixture["action"].clone()).expect("language-neutral action");
            node.bindings.try_push(ui_contract::ActionBinding { trigger: ui_contract::Trigger::Activate, action, args: None, capability: None }).expect("bounded binding");
            node
        });
        let root = ui_contract::column().try_id(surface).ok().expect("bounded key").try_children(children).ok().expect("bounded children").try_build().expect("bounded root");
    {let mut grant=reserve(&tracker, ui_contract::SurfaceId::try_from(surface).expect("bounded surface")).expect("mounted reservation");let mut source=Some(ComponentTree::new(root));assert!(grant.commit_source(&mut source,cx).expect("mounted producer"), "actual original source admitted");assert!(source.is_none());}
        let patch = finish(&tracker,cx).unwrap_or_else(|| {
            let state = tracker.state.borrow();
            panic!(
                "surface {} never published: {:?}; terminals={}, rejected={}",
                fixture["surface"],
                state.slots.iter().flatten().map(|slot| (&slot.surface, slot.producer.as_ref().map(|producer| producer.authority.fault()), &slot.job, slot.reconciler.is_some())).collect::<Vec<_>>(),
                state.terminals.iter().flatten().count(),
                state.rejected.iter().flatten().count()
            );
        });
        assert_eq!(patch.surface.0.as_str(), fixture["surface"].as_str().unwrap());
        assert_eq!(patch.ops.iter().filter(|op| matches!(op, ui_contract::UiPatchOp::Upsert(..))).count(), 1 + fixture["children"].as_array().unwrap().len());
        for op in &patch.ops {
            if let ui_contract::UiPatchOp::Upsert(record) = op {
                if record.key.as_str() != surface {
                    assert_eq!(serde_json::to_value(&record.bindings[0].action).expect("action wire oracle"), fixture["action"]);
                }
            }
        }
    }

    })();
    let fuel=original_context.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original patch fixture caller paid source close");result
}

#[test]
fn one_active_surface_does_not_wait_behind_sixty_three_empty_slots_between_steps() {
    let mut original_caller=semio_framework_plugin_testing_caller::OriginalFixtureCaller::admit().expect("independently authored original patch fixture caller");
    let mut original_context=original_caller.context();let cx=&mut original_context;
    let result=(||{
    let _guard = semio_framework_ui_runtime::surface_reconcile_registry_test_guard();
    let steps = [0, SURFACE_RECONCILE_ADMISSION_SLOTS - 1].map(|index| {
        let tracker = PatchTracker::new();
        tracker.begin("main".into(), leaf("root", "a")).expect("admitted");
        tracker.state.borrow_mut().slots.swap(0, index);
        for step in 1..=4_096 {
            tracker.drive_one(cx).expect("original patch fixture semantic step");
            if let Some(owner) = tracker.take_ready_patch() {
                let (patch, published) = publish_test(owner);
                close_test_patch(patch);
                close_published(published);
                return step;
            }
        }
        panic!("active surface never published");
    });
    assert_eq!(steps[0], steps[1], "empty slots consume no reconcile opportunities");

    })();
    let fuel=original_context.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original patch fixture caller paid source close");result
}
