//! 🫧️ Laws of the whole-root transient declarations (`transient_root!`, `window_transient_owners!`, audit K3) over a probe
//! state: the wire of the one mutation, its diff and inverse, every codec round trip, the footprint, and the window owners'
//! registration, snapshot read and addressing.

use crate::{protocol, store};

/// 🧪️ A probe window-transient root.
#[derive(Clone, Debug, Default, PartialEq, crate::ToValue, crate::FromValue)]
#[value(rename_all = "camelCase")]
pub struct ProbeTransient {
    pub label: String,
    pub count: u32,
}

semio_framework_value::artifact_retire_struct!(ProbeTransient { label, count });

crate::transient_root! {
    state: ProbeTransient,
    mutation: ProbeTransientMutation,
    diff: ProbeTransientDiff,
    owner: "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🪟️window/🫧️transient/🧪️tests/🧪️transient-root",
    kind: "set-window-transient",
    display_name: "Set Probe Window Transient",
    payload_schema: "probe.windowtransient",
    envelope: "s.test.probe.windowtransient",
    extension: "probewindowtransient",
    fields: { label: String, count: u32 },
}

crate::window_transient_owners! {
    state: ProbeTransient,
    mutation: ProbeTransientMutation,
    windows: { ProbeMainOwner => "probe.main", ProbeSideOwner => "probe.side" },
}

fn probe() -> ProbeTransient {
    ProbeTransient { label: "drag".into(), count: 3 }
}

/// ⚖️ LAW: the one mutation's wire is `{"kind":"snapshot","transient":…}` in text and binary alike, it parses back, installs
/// its root over any base, and its inverse restores that base; a wire naming one member twice is refused.
#[test]
fn the_snapshot_mutation_replaces_the_whole_root_and_inverts_to_the_base() {
    use protocol::{Mutation, MutationDiff, OpBinary, OpText};
    let mutation = ProbeTransientMutation::Snapshot { transient: probe() };
    let text = mutation.print_op();
    assert_eq!(text, r#"{"kind":"snapshot","transient":{"label":"drag","count":3}}"#, "the wire of the one mutation");
    assert_eq!(ProbeTransientMutation::parse_op(&text).expect("the text parses"), mutation);
    assert_eq!(ProbeTransientMutation::decode_op(&mutation.encode_op().expect("encodes")).expect("decodes"), mutation);
    let base = ProbeTransient { label: "base".into(), count: 1 };
    assert_eq!(protocol::apply_diff(mutation.diff(&base).diff(), &base).expect("applies"), probe(), "the root is replaced whole");
    assert_eq!(mutation.inverse(&base).expect("inverts"), vec![ProbeTransientMutation::Snapshot { transient: base }], "the inverse restores the base");
    assert_eq!(mutation.descriptor().semantic_kind, "set-window-transient");
    assert!(ProbeTransientMutation::parse_op(r#"{"kind":"snapshot","transient":{"label":"x","count":1,"count":2}}"#).is_err(), "a repeated member is refused");
}

/// ⚖️ LAW: the snapshot mutation's diff is sparse — only the slots where the requested root differs from the base — it is empty for an
/// identical root, and its inverse diff restores exactly those slots.
#[test]
fn the_snapshot_diff_sets_only_the_differing_fields() {
    use protocol::{DiffAlgebra, Mutation};
    let base = ProbeTransient { label: "drag".into(), count: 1 };
    let mutation = ProbeTransientMutation::Snapshot { transient: probe() };
    let diff = mutation.diff(&base).diff().clone();
    assert_eq!((diff.label.as_ref(), diff.count), (None, Some(3)), "only the changed field is carried");
    assert_eq!(diff.inverse(&base), ProbeTransientDiff { label: None, count: Some(1) }, "the inverse is the base value of the set slots");
    assert!(mutation.diff(&probe()).diff().is_empty(), "an identical root changes nothing");
}

/// ⚖️ LAW: the root's DSL and pack round trip through its semio envelope, an empty body or pack reads the default root, and a
/// pack of another envelope is refused.
#[test]
fn the_root_codecs_round_trip_through_their_envelope() {
    use store::{ArtifactDsl, ArtifactPack};
    assert_eq!(ProbeTransient::parse_dsl(&probe().print_dsl()).expect("the DSL parses"), probe());
    assert_eq!(ProbeTransient::decode_pack(&probe().encode_pack()).expect("the pack decodes"), probe());
    assert_eq!(ProbeTransient::decode_pack(&[]).expect("an empty pack"), ProbeTransient::default());
    assert_eq!(<ProbeTransient as ArtifactDsl>::EXTENSION, "probewindowtransient");
    let foreign = store::semio_format::wrap_binary(&store::semio_format::SemioEnvelope::from_envelope_id("s.test.other.windowtransient", store::semio_format::Component::Pack, 1).expect("envelope"), b"{}");
    assert!(ProbeTransient::decode_pack(&foreign).is_err(), "a pack of another envelope is refused");
}

/// ⚖️ LAW: one publication is one admitted item sized by the encoded root, and the transfer moves the root out unchanged.
#[test]
fn the_footprint_is_one_item_of_the_encoded_root() {
    let mutation = ProbeTransientMutation::Snapshot { transient: probe() };
    let footprint = mutation.footprint().expect("an admissible footprint");
    assert_eq!((footprint.work_items, footprint.retained_bytes), (1, r#"{"label":"drag","count":3}"#.len()));
    assert_eq!(mutation.into_state(), probe());
}

/// ⚖️ LAW: every declared window kind registers once (a second registration is refused), and `addressed` installs the root in
/// the view's own window of a declared kind, refusing a view without a window, a stale window and an undeclared kind.
#[test]
fn the_window_owners_register_once_and_address_the_views_own_window() {
    let mut registry = crate::WindowTransientOwnerRegistry::default();
    register(&mut registry).expect("every owner registers");
    assert!(register(&mut registry).is_err(), "a kind registers once");
    let window = |id: &str, kind: &str| semio_framework::ViewWindowInstance { id: id.into(), window_kind_id: kind.into() };
    let view = |window_id: Option<&str>| crate::ViewModel {
        window_id: window_id.map(str::to_string),
        window_instances: vec![window("w-main", "probe.main"), window("w-side", "probe.side"), window("w-other", "other.kind")],
        ..crate::ViewModel::new(semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native)
    };
    for (id, kind) in [("w-main", "probe.main"), ("w-side", "probe.side")] {
        let mutation = addressed(&view(Some(id)), probe()).expect("a declared window");
        assert_eq!((mutation.window_id(), mutation.window_kind_id()), (id, kind));
    }
    let code = |result: Result<crate::WindowTransientMutation, crate::Fault>| result.err().map(|fault| fault.code.0);
    assert_eq!(code(addressed(&view(None), probe())).as_deref(), Some("window-transient.window-required"));
    assert_eq!(code(addressed(&view(Some("gone")), probe())).as_deref(), Some("window-transient.window-stale"));
    assert_eq!(code(addressed(&view(Some("w-other")), probe())).as_deref(), Some("window-transient.kind-unknown"));
    assert_eq!(from_snapshot(None), ProbeTransient::default(), "no snapshot reads the default root");
}
