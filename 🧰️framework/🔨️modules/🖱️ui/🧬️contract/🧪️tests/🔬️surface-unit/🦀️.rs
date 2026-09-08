use super::*;

fn ui_text(value: &str) -> crate::UiText {
    crate::UiText::try_from_str(value).expect("bounded fixture text")
}

fn fixed_bytes(value: &[u8]) -> crate::UiFixedBytes {
    crate::UiFixedBytes::try_from_vec(value.to_vec()).unwrap_or_else(|_| panic!("bounded fixture bytes"))
}

fn fixed_list<T, const N: usize>(values: impl IntoIterator<Item = T>) -> crate::UiFixedList<T, N> {
    let mut list = crate::UiFixedList::default();
    for value in values {
        list.try_push(value).unwrap_or_else(|_| panic!("bounded fixture list"));
    }
    list
}

fn surface_id(value: &str) -> crate::SurfaceId {
    crate::SurfaceId::try_from(value).expect("bounded surface id")
}

fn node_record(component: crate::Component) -> crate::UiNodeRecord {
    crate::UiNodeRecord {
        id: crate::UiNodeId(0),
        key: ui_text("root"),
        component,
        layout: Default::default(),
        style: Default::default(),
        activity: Default::default(),
        disabled: false,
        transition: None,
        accessibility: Default::default(),
        bindings: Default::default(),
        menu: None,
        children: Default::default(),
    }
}

//#region 🔖️Rename
#[test]
fn surface_kind_wire_names_are_all_kebab_case() {
    assert_eq!(serde_json::to_string(&SurfaceKind::World3d).expect("serialize"), "\"world-3d\"");
    assert_eq!(serde_json::to_string(&SurfaceKind::VirtualFileSystem).expect("serialize"), "\"virtual-file-system\"", "the wire-name inconsistency this packet fixes: was camelCase \"virtualFileSystem\"");
}
//#endregion 🔖️Rename

//#region 🔖️SurfaceProps
#[test]
fn empty_surface_props_round_trip_on_the_json_control_plane() {
    let props = SurfaceProps { kind: SurfaceKind::World3d, doc_schema: ui_text("world3d@1"), doc: SurfaceDoc::default(), bindings: Default::default() };
    let json = serde_json::to_string(&props).expect("serialize");
    let back: SurfaceProps = serde_json::from_str(&json).expect("deserialize");
    assert_eq!(props, back);
}

#[test]
fn credited_clone_retains_non_empty_doc_and_bindings() {
    let props = SurfaceProps {
        kind: SurfaceKind::World3d,
        doc_schema: ui_text("world3d@1"),
        doc: SurfaceDoc { bytes: fixed_bytes(&[1, 2, 3, 4, 5]) },
        bindings: fixed_list([crate::ActionBinding { trigger: crate::Trigger::Activate, action: crate::ActionId::try_v1("scope", "reset-view").expect("bounded action id"), args: None, capability: None }]),
    };
    let clone = props.credited_clone().expect("credited surface clone");
    assert_eq!(props, clone);
    assert_eq!(clone.doc.bytes.as_slice(), &[1, 2, 3, 4, 5]);
    assert_eq!(clone.bindings.len(), 1);
}

#[test]
fn surface_props_omits_empty_bindings_on_the_wire() {
    let props = SurfaceProps { kind: SurfaceKind::Canvas2d, doc_schema: ui_text("canvas2d@1"), doc: SurfaceDoc::default(), bindings: Default::default() };
    let json = serde_json::to_value(&props).expect("serialize");
    assert!(json.get("bindings").is_none());
}

/// 🧬️ The property this packet's whole "opaque blob" rule rests on: two `SurfaceProps` differing
/// only in one byte of `doc.bytes` are unequal via plain derived `PartialEq` — the reconciler needs
/// nothing schema-aware to notice a changed scene, structural equality already carries it.
#[test]
fn differing_only_in_one_doc_byte_makes_surface_props_unequal() {
    let base = SurfaceProps { kind: SurfaceKind::World3d, doc_schema: ui_text("world3d@1"), doc: SurfaceDoc { bytes: fixed_bytes(&[1, 2, 3]) }, bindings: Default::default() };
    let changed = SurfaceProps { kind: SurfaceKind::World3d, doc_schema: ui_text("world3d@1"), doc: SurfaceDoc { bytes: fixed_bytes(&[1, 99, 3]) }, bindings: Default::default() };
    assert_ne!(base, changed, "a one-byte doc change must make the whole SurfaceProps compare unequal");
    assert_eq!(base, base.credited_clone().expect("credited surface clone"), "an identical doc must compare equal");
}
//#endregion 🔖️SurfaceProps

//#region 🔖️SchemaParsing
#[test]
fn parse_doc_schema_splits_kind_and_version() {
    assert_eq!(parse_doc_schema("world3d@1"), Ok(SurfaceSchema { kind: "world3d", version: 1 }));
    assert_eq!(parse_doc_schema("node-graph@42"), Ok(SurfaceSchema { kind: "node-graph", version: 42 }));
}

/// 🚧️ Every malformed shape returns a typed fault, never a panic — exercised explicitly rather than
/// merely trusted, since this is the exact guarantee an unknown/malformed `doc_schema` depends on to
/// avoid taking down reconciliation.
#[test]
fn parse_doc_schema_never_panics_and_returns_a_typed_fault_for_every_malformed_shape() {
    assert_eq!(parse_doc_schema(""), Err(SurfaceSchemaFault::Empty));
    assert_eq!(parse_doc_schema("world3d"), Err(SurfaceSchemaFault::MissingVersionSeparator));
    assert_eq!(parse_doc_schema("@1"), Err(SurfaceSchemaFault::EmptyKind));
    assert_eq!(parse_doc_schema("world3d@not-a-number"), Err(SurfaceSchemaFault::InvalidVersion));
    assert_eq!(parse_doc_schema("world3d@"), Err(SurfaceSchemaFault::InvalidVersion));
    assert_eq!(parse_doc_schema("@"), Err(SurfaceSchemaFault::EmptyKind));
    assert_eq!(parse_doc_schema("a@b@1"), Err(SurfaceSchemaFault::InvalidVersion), "split_once takes only the FIRST '@'; \"b@1\" is what gets parsed as the version half and fails to parse as u32");
}
//#endregion 🔖️SchemaParsing

//#region 🔖️UnknownSchemaNeverRejects
/// 🛡️ The contract-side half of "an unknown `doc_schema` must never panic or drop the surrounding
/// patch": a `Component::Surface` carrying a schema no renderer recognises still passes
/// `validate_snapshot` cleanly and applies through `apply_patch` like any other component change —
/// this crate never validates `doc_schema` against a known set, because it can never own that set
/// (every product crate that embeds a surface adds its own kinds).
#[test]
fn validate_snapshot_never_rejects_an_unrecognised_doc_schema() {
    let surface = crate::Component::Surface(SurfaceProps { kind: SurfaceKind::World3d, doc_schema: ui_text("totally-unknown-schema-nobody-registered@999"), doc: SurfaceDoc { bytes: fixed_bytes(&[9, 9, 9]) }, bindings: Default::default() });
    let snapshot = crate::UiSnapshot { surface: surface_id("s"), revision: crate::UiRevision(0), root: crate::UiNodeId(0), nodes: fixed_list([node_record(surface)]), layout_epoch: 0 };
    assert_eq!(crate::validate_snapshot(&snapshot, &crate::UiDocumentLimits::default()), Ok(()), "an unrecognised doc_schema must never be a validation violation");
}

#[test]
fn apply_patch_accepts_a_set_component_carrying_an_unrecognised_doc_schema() {
    let mut state = crate::UiSnapshotState::new(surface_id("s"));
    state.root = Some(crate::UiNodeId(0));
    state.nodes.try_insert(node_record(crate::Component::Separator(crate::SeparatorProps {}))).unwrap_or_else(|_| panic!("bounded node table"));
    let limits = crate::UiDocumentLimits::default();

    let surface = crate::Component::Surface(SurfaceProps { kind: SurfaceKind::Canvas2d, doc_schema: ui_text("nonsense@not-a-version"), doc: SurfaceDoc::default(), bindings: Default::default() });
    let mut ops = crate::UiPatchOps::default();
    ops.try_push(crate::UiPatchOp::SetComponent { id: crate::UiNodeId(0), component: surface }).unwrap();
    let patch = crate::UiPatch { surface: state.surface.clone(), base_revision: state.revision, revision: state.revision.try_next().expect("bounded revision"), ops };

    crate::apply_patch(&mut state, &patch, &limits).expect("a patch carrying an unrecognised/unparseable doc_schema must still apply — it is not this crate's job to reject it");
}
//#endregion 🔖️UnknownSchemaNeverRejects
