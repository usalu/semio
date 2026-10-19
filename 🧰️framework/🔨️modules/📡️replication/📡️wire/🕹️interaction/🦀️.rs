//! 🕹️ Interaction vocabulary shared by every pointer and selection surface: hover and selection specs, the topology, the selection and hover machines and the state validation.

use std::collections::{BTreeMap,HashMap,HashSet};

//#region 🔖️Interaction
// 🕹️ ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM: relocated here (from
// `semio-framework`'s `🔨️modules/🕹️interaction/{🦀️.rs,🧬️schema/🦀️component.rs}`) to
// unblock naming `InteractionState`/`PresenceInteraction` from `store`/`sync` — `semio-framework`
// depends on this crate, never the reverse, so nothing under `os_spr` could previously name a
// framework-defined type. `InteractionDefinition`/`GranularityDefinition`/`InteractionRef` stay in
// `semio-framework` (their `label`/`icon_id` fields pull in `ui_wgpu::LocalizedLabel`/`IconName`,
// which this wasm-safe kernel crate does not and must not depend on) and now `pub use` everything
// below instead of redefining it — see that module's own header comment. Every wave-0 test covering
// this code moved with it, verbatim except `validate_state`'s fixtures (see `InteractionOutline`).
/// 🐁️ One domain's hover behavior — see `semio_framework::InteractionDefinition::hover`.
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HoverSpec {
    #[serde(default = "default_true")]
    pub enabled: bool,
    /// 🌳️ Whether hovering a target expands to its descendant closure (root first) — requires
    /// `hierarchy != HierarchyProvider::Flat`.
    #[serde(default)]
    pub transitive: bool,
    /// 📡️ Named hover channels this domain accepts (e.g. `["pointer"]`); the shared cursor throttle
    /// keys off the same channel names.
    #[serde(default = "default_pointer_channels")]
    pub channels: Vec<String>,
    /// 📣️ Whether this domain's own hover mirrors into `PresenceInteraction` for peers.
    #[serde(default = "default_true")]
    pub broadcast: bool,
}

impl Default for HoverSpec {
    // 🚫️async: E1 impl of externally-declared `Default` trait
    fn default() -> Self {
        Self { enabled: true, transitive: false, channels: default_pointer_channels(), broadcast: true }
    }
}

/// 🌱️ Hand-written, not derived — same DAG reason as `SelectionMode` above. Mirrors the
/// pre-existing `#[serde(rename_all = "camelCase", default = "…")]` wire shape byte-for-byte:
/// every field is sparse-optional on read (missing key falls back to `default()`), always emitted
/// on write.
impl semio_framework_value::ToValue for HoverSpec {
    fn to_value(&self) -> semio_framework_value::DslValue {
        semio_framework_value::DslValue::object(vec![
            ("enabled".to_string(), semio_framework_value::ToValue::to_value(&self.enabled)),
            ("transitive".to_string(), semio_framework_value::ToValue::to_value(&self.transitive)),
            ("channels".to_string(), semio_framework_value::ToValue::to_value(&self.channels)),
            ("broadcast".to_string(), semio_framework_value::ToValue::to_value(&self.broadcast)),
        ])
    }
}
impl semio_framework_value::FromValue for HoverSpec {
    fn from_value(value: semio_framework_value::DslValue) -> Result<Self, semio_framework_value::ValueError> {
        let semio_framework_value::DslValue::Object(fields) = value else {
            return Err(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("expected an object for HoverSpec, found {value:?}")));
        };
        let mut out = HoverSpec::default();
        for (key, entry) in fields {
            match key.as_str() {
                "enabled" => out.enabled = <bool as semio_framework_value::FromValue>::from_value(entry).map_err(|e| e.under("enabled"))?,
                "transitive" => out.transitive = <bool as semio_framework_value::FromValue>::from_value(entry).map_err(|e| e.under("transitive"))?,
                "channels" => out.channels = <Vec<String> as semio_framework_value::FromValue>::from_value(entry).map_err(|e| e.under("channels"))?,
                "broadcast" => out.broadcast = <bool as semio_framework_value::FromValue>::from_value(entry).map_err(|e| e.under("broadcast"))?,
                _ => {}
            }
        }
        Ok(out)
    }
}

// 🚫️async: E1 — called by name from `#[serde(default = "...")]`, whose generated call site is sync.
fn default_true() -> bool {
    true
}

/// 🖱️ One domain's selection behavior — see `semio_framework::InteractionDefinition::selection`.
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SelectionSpec {
    /// 🪜️ Non-empty; the first entry is the domain's default mode.
    pub modes: Vec<SelectionMode>,
    pub methods: Vec<SelectionMethod>,
    pub merges: Vec<MergeMode>,
    /// 🌳️ Whether selecting a target expands to its descendant closure — requires
    /// `hierarchy != HierarchyProvider::Flat`.
    #[serde(default)]
    pub transitive: bool,
    /// 📣️ Whether this domain's own selection mirrors into `PresenceInteraction` for peers.
    #[serde(default = "default_true")]
    pub broadcast: bool,
}

/// 🌱️ Hand-written, not derived — same DAG reason as `SelectionMode` above.
impl semio_framework_value::ToValue for SelectionSpec {
    fn to_value(&self) -> semio_framework_value::DslValue {
        semio_framework_value::DslValue::object(vec![
            ("modes".to_string(), semio_framework_value::ToValue::to_value(&self.modes)),
            ("methods".to_string(), semio_framework_value::ToValue::to_value(&self.methods)),
            ("merges".to_string(), semio_framework_value::ToValue::to_value(&self.merges)),
            ("transitive".to_string(), semio_framework_value::ToValue::to_value(&self.transitive)),
            ("broadcast".to_string(), semio_framework_value::ToValue::to_value(&self.broadcast)),
        ])
    }
}
impl semio_framework_value::FromValue for SelectionSpec {
    fn from_value(value: semio_framework_value::DslValue) -> Result<Self, semio_framework_value::ValueError> {
        let semio_framework_value::DslValue::Object(fields) = value else {
            return Err(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("expected an object for SelectionSpec, found {value:?}")));
        };
        let mut modes = None;
        let mut methods = None;
        let mut merges = None;
        let mut transitive = false;
        let mut broadcast = true;
        for (key, entry) in fields {
            match key.as_str() {
                "modes" => modes = Some(<Vec<SelectionMode> as semio_framework_value::FromValue>::from_value(entry).map_err(|e| e.under("modes"))?),
                "methods" => methods = Some(<Vec<SelectionMethod> as semio_framework_value::FromValue>::from_value(entry).map_err(|e| e.under("methods"))?),
                "merges" => merges = Some(<Vec<MergeMode> as semio_framework_value::FromValue>::from_value(entry).map_err(|e| e.under("merges"))?),
                "transitive" => transitive = <bool as semio_framework_value::FromValue>::from_value(entry).map_err(|e| e.under("transitive"))?,
                "broadcast" => broadcast = <bool as semio_framework_value::FromValue>::from_value(entry).map_err(|e| e.under("broadcast"))?,
                _ => {}
            }
        }
        Ok(SelectionSpec {
            modes: modes.ok_or_else(|| semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "SelectionSpec missing modes"))?,
            methods: methods.ok_or_else(|| semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "SelectionSpec missing methods"))?,
            merges: merges.ok_or_else(|| semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "SelectionSpec missing merges"))?,
            transitive,
            broadcast,
        })
    }
}

/// 🌳️ Where a domain's target ids come from, and thus what `DomainTopology` (if any) is available for
/// range selection and transitive hover/select closures.
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(tag = "kind", rename_all = "camelCase", rename_all_fields = "camelCase")]
pub enum HierarchyProvider {
    /// 🪨️ No parent/child structure — range and transitive closures degrade to the single target.
    Flat,
    /// 🕸️ App-supplied topology (`ArtifactApp::interaction_topology`), e.g. a DAG or scene graph.
    Topology,
    /// 🌲️ Derived from the rendered `UiTree` shape for this domain.
    UiTree,
    /// 🧵️ Derived from splitting each target id on `delimiter` (e.g. `♾️infinite`'s `"surfaceId/id"`).
    PathDelimited { delimiter: String },
}

/// 🌱️ Hand-written, not derived — same DAG reason as `SelectionMode` above. Internally tagged on
/// `"kind"`, mirroring `#[serde(tag = "kind", rename_all = "camelCase")]` byte-for-byte
/// (`rename_all_fields` dropped per the derive's own doc: redundant when it matches `rename_all`).
impl semio_framework_value::ToValue for HierarchyProvider {
    fn to_value(&self) -> semio_framework_value::DslValue {
        match self {
            HierarchyProvider::Flat => semio_framework_value::DslValue::object(vec![("kind".to_string(), semio_framework_value::DslValue::String("flat".to_string()))]),
            HierarchyProvider::Topology => semio_framework_value::DslValue::object(vec![("kind".to_string(), semio_framework_value::DslValue::String("topology".to_string()))]),
            HierarchyProvider::UiTree => semio_framework_value::DslValue::object(vec![("kind".to_string(), semio_framework_value::DslValue::String("uiTree".to_string()))]),
            HierarchyProvider::PathDelimited { delimiter } => {
                semio_framework_value::DslValue::object(vec![("kind".to_string(), semio_framework_value::DslValue::String("pathDelimited".to_string())), ("delimiter".to_string(), semio_framework_value::ToValue::to_value(delimiter))])
            }
        }
    }
}
impl semio_framework_value::FromValue for HierarchyProvider {
    fn from_value(value: semio_framework_value::DslValue) -> Result<Self, semio_framework_value::ValueError> {
        let semio_framework_value::DslValue::Object(fields) = value else {
            return Err(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("expected an object for HierarchyProvider, found {value:?}")));
        };
        let kind = fields.iter().find(|(k, _)| k == "kind").map(|(_, v)| v.clone()).ok_or_else(|| semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "HierarchyProvider missing kind"))?;
        let semio_framework_value::DslValue::String(kind) = kind else {
            return Err(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "HierarchyProvider kind must be a string"));
        };
        match kind.as_str() {
            "flat" => Ok(HierarchyProvider::Flat),
            "topology" => Ok(HierarchyProvider::Topology),
            "uiTree" => Ok(HierarchyProvider::UiTree),
            "pathDelimited" => {
                let delimiter = fields.into_iter().find(|(k, _)| k == "delimiter").map(|(_, v)| v).ok_or_else(|| semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "HierarchyProvider.pathDelimited missing delimiter"))?;
                Ok(HierarchyProvider::PathDelimited { delimiter: <String as semio_framework_value::FromValue>::from_value(delimiter).map_err(|e| e.under("delimiter"))? })
            }
            other => Err(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("unknown HierarchyProvider kind `{other}`"))),
        }
    }
}

/// 🔢️ How many targets may be selected at once within a domain.
/// 🌱️ serde's derives are carried ALONGSIDE the hand-written `ToValue`/`FromValue` twin below —
/// the transitional state the serde-fanout playbook prescribes ("add alongside, do not blind-swap").
/// Attributes are restored verbatim from 67fb4216b2 so the serde wire shape stays byte-identical to
/// the twin. Drop them once every consumer in `🔗️causal`/`📡️wire`/`⚔️conflict` moves to `ToValue`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase")]
#[serde(rename_all = "camelCase")]
pub enum SelectionMode {
    Single,
    Multiple,
}

impl semio_framework_value::retirement::RetireOwned for SelectionMode {
    fn retirement(self)->Box<dyn semio_framework_value::retirement::RetirementCursor>{semio_framework_value::retirement::leaf(())}
    fn retirement_birth_bytes(&self)->Option<usize>{Some(semio_framework_value::retirement::leaf_birth_bytes::<()>())}
    fn controlled_retirement_supported()->bool{true}
}

/// 🎯️ How a surface gathers targets for one `interactionSelect` dispatch.
#[derive(Clone, Copy, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum SelectionMethod {
    Pick,
    Rectangle,
    Lasso,
}

/// 🌱️ Hand-written, not derived — same DAG reason as `SelectionMode` above.
impl semio_framework_value::ToValue for SelectionMethod {
    fn to_value(&self) -> semio_framework_value::DslValue {
        semio_framework_value::DslValue::String(match self { SelectionMethod::Pick => "pick", SelectionMethod::Rectangle => "rectangle", SelectionMethod::Lasso => "lasso" }.to_string())
    }
}
impl semio_framework_value::FromValue for SelectionMethod {
    fn from_value(value: semio_framework_value::DslValue) -> Result<Self, semio_framework_value::ValueError> {
        match value {
            semio_framework_value::DslValue::String(s) => match s.as_str() {
                "pick" => Ok(SelectionMethod::Pick),
                "rectangle" => Ok(SelectionMethod::Rectangle),
                "lasso" => Ok(SelectionMethod::Lasso),
                other => Err(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("unknown SelectionMethod variant `{other}`"))),
            },
            other => Err(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("expected a string, found {other:?}"))),
        }
    }
}

/// 🧮️ Set algebra applied when merging new targets into the current selection — see `next_selection`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum MergeMode {
    Replace,
    Additive,
    Subtractive,
    Invertive,
    Range,
}

semio_framework_value::artifact_retire_leaf!(MergeMode);
impl MergeMode {
    /// 🎯️ The ONE wire spelling of a merge mode — the `MergeMode` enum of
    /// `🧰️framework/🔨️modules/🕹️interaction/🧬️schema/🔣️.json`, pinned by
    /// `🕹️interaction/🧫️fixtures/🎯️merge-modes.json` and its two twins. Every producer
    /// (renderer hosts, the wgpu world, plugin emits) and every consumer (`parse_merge_mode`,
    /// `merge_world_selection_ids`, `merge_string_ids`) goes through this pair, so no layer can
    /// invent a private spelling of the same set algebra again — ticket
    /// 26/09/09/PROCEDURAL-3D-END-TO-END, `📓️selection-merge-vocabulary-2026-09-12.md`: a second
    /// `add`/`remove`/`toggle` vocabulary made every modifier-click on a domain-bound world scene
    /// fault with `interactionSelect: unknown merge '…'`.
    // 🚫️async: E1 — called from the sync `ToValue`/`FromValue` impl bodies below, and from `json!`
    // argument position in `const`-shaped emit sites; see R9.
    pub fn wire_label(self) -> &'static str {
        match self {
            MergeMode::Replace => "replace",
            MergeMode::Additive => "additive",
            MergeMode::Subtractive => "subtractive",
            MergeMode::Invertive => "invertive",
            MergeMode::Range => "range",
        }
    }

    /// 🎯️ Inverse of [`MergeMode::wire_label`] — `None` for every word outside the schema enum, so a
    /// caller decides whether an unknown merge is a fault (the domain path) or a skipped optimistic
    /// preview (the local wgpu world).
    // 🚫️async: E1 — paired with `wire_label`, called from the sync `FromValue` impl below; see R9.
    pub fn from_wire_label(raw: &str) -> Option<Self> {
        match raw {
            "replace" => Some(MergeMode::Replace),
            "additive" => Some(MergeMode::Additive),
            "subtractive" => Some(MergeMode::Subtractive),
            "invertive" => Some(MergeMode::Invertive),
            "range" => Some(MergeMode::Range),
            _ => None,
        }
    }
}

/// 🌱️ Hand-written, not derived — same DAG reason as `SelectionMode` above.
impl semio_framework_value::ToValue for MergeMode {
    fn to_value(&self) -> semio_framework_value::DslValue {
        semio_framework_value::DslValue::String(self.wire_label().to_string())
    }
}
impl semio_framework_value::FromValue for MergeMode {
    fn from_value(value: semio_framework_value::DslValue) -> Result<Self, semio_framework_value::ValueError> {
        match value {
            semio_framework_value::DslValue::String(s) => MergeMode::from_wire_label(&s).ok_or_else(|| semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("unknown MergeMode variant `{s}`"))),
            other => Err(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("expected a string, found {other:?}"))),
        }
    }
}

// 🚫️async: E1 — called from a plain `Default` impl body, which is sync.
fn default_pointer_channels() -> Vec<String> {
    vec!["pointer".to_string()]
}

//#region 🔖️Runtime
/// 🎯️ One addressed target: a granularity id plus the target's own id (u32 domain ids are stringified
/// at the app boundary before reaching this module).await.
#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
#[derive(semio_framework_value::RetireOwned)]
pub struct InteractionTarget {
    pub granularity: String,
    pub id: String,
}

/// 🌱️ Hand-written, not derived — same DAG reason as `SelectionMode` above.
impl semio_framework_value::ToValue for InteractionTarget {
    fn to_value(&self) -> semio_framework_value::DslValue {
        semio_framework_value::DslValue::object(vec![("granularity".to_string(), semio_framework_value::ToValue::to_value(&self.granularity)), ("id".to_string(), semio_framework_value::ToValue::to_value(&self.id))])
    }
}
impl semio_framework_value::FromValue for InteractionTarget {
    fn from_value(value: semio_framework_value::DslValue) -> Result<Self, semio_framework_value::ValueError> {
        let semio_framework_value::DslValue::Object(fields) = value else {
            return Err(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("expected an object for InteractionTarget, found {value:?}")));
        };
        let mut granularity = None;
        let mut id = None;
        for (key, entry) in fields {
            match key.as_str() {
                "granularity" => granularity = Some(<String as semio_framework_value::FromValue>::from_value(entry).map_err(|e| e.under("granularity"))?),
                "id" => id = Some(<String as semio_framework_value::FromValue>::from_value(entry).map_err(|e| e.under("id"))?),
                _ => {}
            }
        }
        Ok(InteractionTarget {
            granularity: granularity.ok_or_else(|| semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "InteractionTarget missing granularity"))?,
            id: id.ok_or_else(|| semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "InteractionTarget missing id"))?,
        })
    }
}

/// 🖱️ One domain's current selection: the active granularity, the selected ids, and the anchor id
/// range selection pivots from.
/// 🌱️ Carries serde's derives ALONGSIDE the hand-written `ToValue`/`FromValue` below, which is the
/// transitional state the serde-fanout playbook prescribes ("add alongside — do not blind-swap").
/// Its consumers in `⚔️conflict`, `📡️wire/🏠️local-interaction` and `🔗️causal` still serialize it
/// through serde; the three fields are `String`/`Vec<String>`/`Option<String>`, so the derives add no
/// bound fan-out. Drop them once those call sites move to `ToValue`.
#[derive(Clone, Debug, Default, PartialEq, Eq, serde::Serialize, serde::Deserialize, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase")]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct DomainSelection {
    #[value(default, skip_serializing_if = "Option::is_none")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub anchor_id: Option<String>,
    pub granularity: String,
    pub ids: Vec<String>,
}

semio_framework_value::artifact_retire_struct!(DomainSelection {granularity,ids,anchor_id});

/// 🐁️ One domain's current hover on one channel: the transitive closure (root first) when
/// `HoverSpec::transitive`, otherwise just the raw hovered ids.
#[derive(Clone, Debug, Default, PartialEq, Eq, serde::Serialize, serde::Deserialize, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase")]
#[serde(rename_all = "camelCase")]
pub struct DomainHover {
    pub channel: String,
    pub ids: Vec<String>,
}
semio_framework_value::artifact_retire_struct!(DomainHover {channel,ids});

/// 🗺️ Own persisted-local selection (`Interaction` history lane).await + ephemeral-local hover, keyed by
/// domain id — the framework-owned counterpart to what every per-app config used to hand-roll.
#[derive(Clone, Debug, Default, PartialEq, serde::Serialize, serde::Deserialize, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase")]
#[serde(rename_all = "camelCase")]
pub struct InteractionState {
    pub active_granularity: BTreeMap<String, String>,
    pub active_mode: BTreeMap<String, SelectionMode>,
    pub hover: BTreeMap<String, DomainHover>,
    pub selection: BTreeMap<String, DomainSelection>,
}
semio_framework_value::artifact_retire_struct!(InteractionState {selection,hover,active_mode,active_granularity});

//#endregion 🔖️Runtime

//#region 🔖️Topology
/// 🌳️ One node of a domain's topology: its own granularity and its parent id (`None` = a root).await.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TopologyNode {
    pub id: String,
    pub granularity: String,
    pub parent: Option<String>,
}

/// 🌱️ Hand-written, not derived — same DAG reason as `SelectionMode` above.
impl semio_framework_value::ToValue for TopologyNode {
    fn to_value(&self) -> semio_framework_value::DslValue {
        let mut entries = vec![("id".to_string(), semio_framework_value::ToValue::to_value(&self.id)), ("granularity".to_string(), semio_framework_value::ToValue::to_value(&self.granularity))];
        if self.parent.is_some() {
            entries.push(("parent".to_string(), semio_framework_value::ToValue::to_value(&self.parent)));
        }
        semio_framework_value::DslValue::object(entries)
    }
}
impl semio_framework_value::FromValue for TopologyNode {
    fn from_value(value: semio_framework_value::DslValue) -> Result<Self, semio_framework_value::ValueError> {
        let semio_framework_value::DslValue::Object(fields) = value else {
            return Err(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("expected an object for TopologyNode, found {value:?}")));
        };
        let mut id = None;
        let mut granularity = None;
        let mut parent = None;
        for (key, entry) in fields {
            match key.as_str() {
                "id" => id = Some(<String as semio_framework_value::FromValue>::from_value(entry).map_err(|e| e.under("id"))?),
                "granularity" => granularity = Some(<String as semio_framework_value::FromValue>::from_value(entry).map_err(|e| e.under("granularity"))?),
                "parent" => parent = <Option<String> as semio_framework_value::FromValue>::from_value(entry).map_err(|e| e.under("parent"))?,
                _ => {}
            }
        }
        Ok(TopologyNode {
            id: id.ok_or_else(|| semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "TopologyNode missing id"))?,
            granularity: granularity.ok_or_else(|| semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "TopologyNode missing granularity"))?,
            parent,
        })
    }
}

/// 🌲️ One domain's topology, pre-order: `ordered`'s sequence IS the range-selection order, and every
/// node's descendants form a contiguous run immediately following it.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct DomainTopology {
    pub ordered: Vec<TopologyNode>,
}

/// 🌱️ Hand-written, not derived — same DAG reason as `SelectionMode` above.
impl semio_framework_value::ToValue for DomainTopology {
    fn to_value(&self) -> semio_framework_value::DslValue {
        semio_framework_value::DslValue::object(vec![("ordered".to_string(), semio_framework_value::ToValue::to_value(&self.ordered))])
    }
}
impl semio_framework_value::FromValue for DomainTopology {
    fn from_value(value: semio_framework_value::DslValue) -> Result<Self, semio_framework_value::ValueError> {
        let semio_framework_value::DslValue::Object(fields) = value else {
            return Err(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("expected an object for DomainTopology, found {value:?}")));
        };
        let mut ordered = Vec::new();
        for (key, entry) in fields {
            if key == "ordered" {
                ordered = <Vec<TopologyNode> as semio_framework_value::FromValue>::from_value(entry).map_err(|e| e.under("ordered"))?;
            }
        }
        Ok(DomainTopology { ordered })
    }
}

impl DomainTopology {
    /// 🔎️ The pre-order index of `id`, or `None` when absent.
    pub async fn index_of(&self, id: &str) -> Option<usize> {
        self.ordered.iter().position(|node| node.id == id)
    }

    /// ✅️ Whether `id` is a known node in this topology.
    pub async fn contains(&self, id: &str) -> bool {
        self.index_of(id).await.is_some()
    }

    /// 🗂️ One membership set over this whole topology, so a caller checking MANY ids pays the `n` walk
    /// once instead of once per id. [`validate_state`] prunes every selected and every hovered id against
    /// the same topology, and the flagship puzzle3d document carries ~900 nodes: at one linear
    /// [`Self::contains`] per id a marquee selection was quadratic, and even a single pick walked the
    /// whole forest twice (26/09/02/PUZZLE-3D-END-TO-END wave B46).
    pub async fn membership(&self) -> std::collections::BTreeSet<&str> {
        self.ordered.iter().map(|node| node.id.as_str()).collect()
    }

    async fn children_by_parent(&self) -> HashMap<String, Vec<String>> {
        let mut children: HashMap<String, Vec<String>> = HashMap::new();
        for node in &self.ordered {
            if let Some(parent) = &node.parent {
                children.entry(parent.clone()).or_default().push(node.id.clone());
            }
        }
        children
    }

    /// 🌳️ `root_id` plus every descendant, pre-order (root first) — empty when `root_id` is absent.
    pub async fn descendant_closure(&self, root_id: &str) -> Vec<String> {
        if !self.contains(root_id).await {
            return Vec::new();
        }
        expand_interaction_ids(self, &[InteractionTarget { granularity: String::new(), id: root_id.to_string() }], true).await
    }

    /// 🪜️ `id`'s ancestor chain, nearest parent first, root last.
    pub async fn ancestors(&self, id: &str) -> Vec<String> {
        let mut out = Vec::new();
        let parents: HashMap<&str, Option<&str>> = self.ordered.iter().map(|node| (node.id.as_str(), node.parent.as_deref())).collect();
        let mut seen = HashSet::from([id]);
        let mut current = parents.get(id).copied().flatten();
        while let Some(parent_id) = current {
            if !seen.insert(parent_id) { break; }
            current = parents.get(parent_id).copied().flatten();
            out.push(parent_id.to_string());
        }
        out
    }
}

async fn expand_interaction_ids(topo: &DomainTopology, targets: &[InteractionTarget], transitive: bool) -> Vec<String> {
    let children = if transitive { topo.children_by_parent().await } else { HashMap::new() };
    let members: HashSet<&str> = if transitive { topo.ordered.iter().map(|node| node.id.as_str()).collect() } else { HashSet::new() };
    let mut seen = HashSet::new();
    let mut pending: Vec<String> = targets.iter().rev().map(|target| target.id.clone()).collect();
    let mut out = Vec::new();
    while let Some(id) = pending.pop() {
        if !seen.insert(id.clone()) { continue; }
        if members.contains(id.as_str()) {
            if let Some(descendants) = children.get(&id) { pending.extend(descendants.iter().rev().cloned()); }
        }
        out.push(id);
    }
    out
}

/// 🗺️ Every domain's topology for one app instance, keyed by domain id — `ArtifactApp::interaction_topology`
/// returns this (wave 3).
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct InteractionTopology {
    pub domains: BTreeMap<String, DomainTopology>,
}

/// 🌱️ Hand-written, not derived — same DAG reason as `SelectionMode` above.
impl semio_framework_value::ToValue for InteractionTopology {
    fn to_value(&self) -> semio_framework_value::DslValue {
        semio_framework_value::DslValue::object(vec![("domains".to_string(), semio_framework_value::ToValue::to_value(&self.domains))])
    }
}
impl semio_framework_value::FromValue for InteractionTopology {
    fn from_value(value: semio_framework_value::DslValue) -> Result<Self, semio_framework_value::ValueError> {
        let semio_framework_value::DslValue::Object(fields) = value else {
            return Err(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("expected an object for InteractionTopology, found {value:?}")));
        };
        let mut domains = BTreeMap::new();
        for (key, entry) in fields {
            if key == "domains" {
                domains = <BTreeMap<String, DomainTopology> as semio_framework_value::FromValue>::from_value(entry).map_err(|e| e.under("domains"))?;
            }
        }
        Ok(InteractionTopology { domains })
    }
}
//#endregion 🔖️Topology

//#region 🔖️SelectionMachine
/// 🖱️ One `next_selection` call's input: the batch of targets (a single pick or a marquee gather),
/// the merge mode to apply, and the currently active selection mode.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SelectionInput {
    pub targets: Vec<InteractionTarget>,
    pub merge: MergeMode,
    pub mode: SelectionMode,
}

/// 🌱️ Hand-written, not derived — same DAG reason as `SelectionMode` above.
impl semio_framework_value::ToValue for SelectionInput {
    fn to_value(&self) -> semio_framework_value::DslValue {
        semio_framework_value::DslValue::object(vec![
            ("targets".to_string(), semio_framework_value::ToValue::to_value(&self.targets)),
            ("merge".to_string(), semio_framework_value::ToValue::to_value(&self.merge)),
            ("mode".to_string(), semio_framework_value::ToValue::to_value(&self.mode)),
        ])
    }
}
impl semio_framework_value::FromValue for SelectionInput {
    fn from_value(value: semio_framework_value::DslValue) -> Result<Self, semio_framework_value::ValueError> {
        let semio_framework_value::DslValue::Object(fields) = value else {
            return Err(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("expected an object for SelectionInput, found {value:?}")));
        };
        let mut targets = None;
        let mut merge = None;
        let mut mode = None;
        for (key, entry) in fields {
            match key.as_str() {
                "targets" => targets = Some(<Vec<InteractionTarget> as semio_framework_value::FromValue>::from_value(entry).map_err(|e| e.under("targets"))?),
                "merge" => merge = Some(<MergeMode as semio_framework_value::FromValue>::from_value(entry).map_err(|e| e.under("merge"))?),
                "mode" => mode = Some(<SelectionMode as semio_framework_value::FromValue>::from_value(entry).map_err(|e| e.under("mode"))?),
                _ => {}
            }
        }
        Ok(SelectionInput {
            targets: targets.ok_or_else(|| semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "SelectionInput missing targets"))?,
            merge: merge.ok_or_else(|| semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "SelectionInput missing merge"))?,
            mode: mode.ok_or_else(|| semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "SelectionInput missing mode"))?,
        })
    }
}

/// 🖱️ Computes the next `DomainSelection` for one domain — the generalization of Tree's
/// `getTreeNextSelectionState` (`🖱️ui/🧱️elements/🪵️Tree/🟦️.tsx:946-968`), preserving its
/// exact single/range/toggle semantics while adding batch targets, `Additive`/`Subtractive` as
/// distinct merges, and transitive descendant-closure expansion.
///
/// - `Single` mode ignores `merge` entirely and clamps to the batch's last target (mirrors Tree
///   returning `{selectedIds:[targetId]}` unconditionally in single mode).await.
/// - `Range` replaces the selection with the topology-order slice between the anchor (falling back to
///   `current.anchor_id`, then `current.ids.last()`, then the target itself — mirrors Tree's
///   `fallbackAnchorId`) and the batch's last target, ascending index order; the anchor does not move.
/// - `Replace`/`Additive`/`Subtractive`/`Invertive` apply ordinary set algebra over the batch's targets
///   (each expanded to its descendant closure first when `spec.transitive`), and update the anchor to
///   the batch's last target.
///
/// Empty `input.targets` is a no-op (returns `current` unchanged).
pub async fn next_selection(spec: &SelectionSpec, current: &DomainSelection, topo: &DomainTopology, input: &SelectionInput) -> DomainSelection {
    let Some(last_target) = input.targets.last() else {
        return current.clone();
    };
    let granularity = last_target.granularity.clone();
    let last_target_id = last_target.id.clone();

    if input.mode == SelectionMode::Single {
        return DomainSelection { granularity, ids: vec![last_target_id.clone()], anchor_id: Some(last_target_id) };
    }

    if input.merge == MergeMode::Range {
        let fallback_anchor = current.anchor_id.clone().or_else(|| current.ids.last().cloned()).unwrap_or_else(|| last_target_id.clone());
        if let (Some(anchor_index), Some(target_index)) = (topo.index_of(&fallback_anchor).await, topo.index_of(&last_target_id).await) {
            let (start, end) = if anchor_index <= target_index { (anchor_index, target_index) } else { (target_index, anchor_index) };
            let ids = topo.ordered[start..=end].iter().map(|node| node.id.clone()).collect();
            return DomainSelection { granularity, ids, anchor_id: Some(fallback_anchor) };
        }
        return DomainSelection { granularity, ids: vec![last_target_id.clone()], anchor_id: Some(last_target_id) };
    }

    let expanded = expand_interaction_ids(topo, &input.targets, spec.transitive).await;
    let incoming: HashSet<&str> = expanded.iter().map(String::as_str).collect();
    let existing: HashSet<&str> = current.ids.iter().map(String::as_str).collect();

    let mut ids = match input.merge {
        MergeMode::Replace => dedup_preserving_order(expanded).await,
        MergeMode::Additive => {
            let mut ids = current.ids.clone();
            ids.extend(expanded.iter().filter(|id| !existing.contains(id.as_str())).cloned());
            ids
        }
        MergeMode::Subtractive => current.ids.iter().filter(|id| !incoming.contains(id.as_str())).cloned().collect(),
        MergeMode::Invertive => {
            let mut ids: Vec<String> = current.ids.iter().filter(|id| !incoming.contains(id.as_str())).cloned().collect();
            ids.extend(expanded.iter().filter(|id| !existing.contains(id.as_str())).cloned());
            ids
        }
        MergeMode::Range => unreachable!("Range handled above"),
    };
    ids = dedup_preserving_order(ids).await;
    DomainSelection { granularity, ids, anchor_id: Some(last_target_id) }
}

async fn dedup_preserving_order(ids: Vec<String>) -> Vec<String> {
    let mut out: Vec<String> = Vec::with_capacity(ids.len());
    let mut seen = HashSet::new();
    for id in ids {
        if seen.insert(id.clone()) {
            out.push(id);
        }
    }
    out
}
//#endregion 🔖️SelectionMachine

//#region 🔖️HoverMachine
/// 🐁️ One `next_hover` call's input: the channel and the batch of hovered targets (empty = clear).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct HoverInput {
    pub channel: String,
    pub targets: Vec<InteractionTarget>,
}

/// 🌱️ Hand-written, not derived — same DAG reason as `SelectionMode` above.
impl semio_framework_value::ToValue for HoverInput {
    fn to_value(&self) -> semio_framework_value::DslValue {
        semio_framework_value::DslValue::object(vec![("channel".to_string(), semio_framework_value::ToValue::to_value(&self.channel)), ("targets".to_string(), semio_framework_value::ToValue::to_value(&self.targets))])
    }
}
impl semio_framework_value::FromValue for HoverInput {
    fn from_value(value: semio_framework_value::DslValue) -> Result<Self, semio_framework_value::ValueError> {
        let semio_framework_value::DslValue::Object(fields) = value else {
            return Err(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("expected an object for HoverInput, found {value:?}")));
        };
        let mut channel = None;
        let mut targets = None;
        for (key, entry) in fields {
            match key.as_str() {
                "channel" => channel = Some(<String as semio_framework_value::FromValue>::from_value(entry).map_err(|e| e.under("channel"))?),
                "targets" => targets = Some(<Vec<InteractionTarget> as semio_framework_value::FromValue>::from_value(entry).map_err(|e| e.under("targets"))?),
                _ => {}
            }
        }
        Ok(HoverInput {
            channel: channel.ok_or_else(|| semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "HoverInput missing channel"))?,
            targets: targets.ok_or_else(|| semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "HoverInput missing targets"))?,
        })
    }
}

/// 🐁️ Computes the next `DomainHover` for one channel: always REPLACES the channel's id list (hover
/// has no merge algebra). When `spec.transitive`, each target expands to its descendant closure with
/// the hovered root first; multiple targets concatenate in input order, deduplicated. Disabled specs
/// and empty target batches both clear the channel.
pub async fn next_hover(spec: &HoverSpec, topo: &DomainTopology, input: &HoverInput) -> DomainHover {
    if !spec.enabled || input.targets.is_empty() {
        return DomainHover { channel: input.channel.clone(), ids: Vec::new() };
    }
    let ids = expand_interaction_ids(topo, &input.targets, spec.transitive).await;
    DomainHover { channel: input.channel.clone(), ids }
}
//#endregion 🔖️HoverMachine

//#region 🔖️Validation
/// 🪞️ The label/icon-free projection of a domain's `InteractionDefinition` that `validate_state`
/// needs — `semio-framework`'s `InteractionDefinition::outline()` builds one of these per call since
/// this crate cannot name `InteractionDefinition` itself (see this region's header comment).
#[derive(Clone, Debug, PartialEq)]
pub struct InteractionOutline {
    pub id: String,
    /// 🪜️ Non-empty; the first entry is the domain's default granularity.
    pub granularity_ids: Vec<String>,
    pub selection: SelectionSpec,
}

/// 🌱️ Hand-written, not derived — same DAG reason as `SelectionMode` above.
impl semio_framework_value::ToValue for InteractionOutline {
    fn to_value(&self) -> semio_framework_value::DslValue {
        semio_framework_value::DslValue::object(vec![
            ("id".to_string(), semio_framework_value::ToValue::to_value(&self.id)),
            ("granularityIds".to_string(), semio_framework_value::ToValue::to_value(&self.granularity_ids)),
            ("selection".to_string(), semio_framework_value::ToValue::to_value(&self.selection)),
        ])
    }
}
impl semio_framework_value::FromValue for InteractionOutline {
    fn from_value(value: semio_framework_value::DslValue) -> Result<Self, semio_framework_value::ValueError> {
        let semio_framework_value::DslValue::Object(fields) = value else {
            return Err(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("expected an object for InteractionOutline, found {value:?}")));
        };
        let mut id = None;
        let mut granularity_ids = None;
        let mut selection = None;
        for (key, entry) in fields {
            match key.as_str() {
                "id" => id = Some(<String as semio_framework_value::FromValue>::from_value(entry).map_err(|e| e.under("id"))?),
                "granularityIds" => granularity_ids = Some(<Vec<String> as semio_framework_value::FromValue>::from_value(entry).map_err(|e| e.under("granularityIds"))?),
                "selection" => selection = Some(<SelectionSpec as semio_framework_value::FromValue>::from_value(entry).map_err(|e| e.under("selection"))?),
                _ => {}
            }
        }
        Ok(InteractionOutline {
            id: id.ok_or_else(|| semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "InteractionOutline missing id"))?,
            granularity_ids: granularity_ids.ok_or_else(|| semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "InteractionOutline missing granularityIds"))?,
            selection: selection.ok_or_else(|| semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "InteractionOutline missing selection"))?,
        })
    }
}

/// 🧹️ Re-derives a consistent `InteractionState` from declared `defs` + current `topo`: drops any
/// domain absent from `defs` (renamed/removed interaction declaration), prunes selection/hover ids no
/// longer present in that domain's topology (deleted document nodes — called after every artifact
/// dispatch), resets `active_granularity`/`active_mode` to a declared value (falling back to the
/// domain's default, its first declared entry) when the stored one is no longer declared, and clamps
/// `Single`-mode selections down to their first id (mirrors `normalizeTreeSelectedIds`'s external-update
/// normalization, not `next_selection`'s recency-preferring clamp).await.
pub async fn validate_state(defs: &[InteractionOutline], topo: &InteractionTopology, state: &InteractionState) -> InteractionState {
    let mut result = InteractionState::default();

    for def in defs {
        let domain_topo = topo.domains.get(&def.id);
        let declared_granularities: Vec<&str> = def.granularity_ids.iter().map(String::as_str).collect();
        let default_granularity = def.granularity_ids.first().cloned().unwrap_or_default();
        let default_mode = def.selection.modes.first().copied().unwrap_or(SelectionMode::Single);

        let mode = state.active_mode.get(&def.id).copied().filter(|mode| def.selection.modes.contains(mode)).unwrap_or(default_mode);
        result.active_mode.insert(def.id.clone(), mode);

        let granularity = state.active_granularity.get(&def.id).cloned().filter(|granularity| declared_granularities.contains(&granularity.as_str())).unwrap_or_else(|| default_granularity.clone());
        result.active_granularity.insert(def.id.clone(), granularity);

        // 🗂️ ONE membership walk per domain, reused by both the selection and the hover prune below — see
        // `DomainTopology::membership`. A `None` topology means "no membership information, keep every id".
        let members = match domain_topo {
            Some(topo) => Some(topo.membership().await),
            None => None,
        };
        let keeps = |id: &String| members.as_ref().is_none_or(|members| members.contains(id.as_str()));
        if let Some(selection) = state.selection.get(&def.id) {
            let selection_granularity = if declared_granularities.contains(&selection.granularity.as_str()) { selection.granularity.clone() } else { default_granularity.clone() };
            let mut ids: Vec<String> = selection.ids.iter().filter(|id| keeps(id)).cloned().collect();
            if mode == SelectionMode::Single && ids.len() > 1 {
                ids.truncate(1);
            }
            let anchor_id = selection.anchor_id.clone().filter(|anchor| ids.contains(anchor));
            result.selection.insert(def.id.clone(), DomainSelection { granularity: selection_granularity, ids, anchor_id });
        }

        if let Some(hover) = state.hover.get(&def.id) {
            let ids: Vec<String> = hover.ids.iter().filter(|id| keeps(id)).cloned().collect();
            result.hover.insert(def.id.clone(), DomainHover { channel: hover.channel.clone(), ids });
        }
    }

    result
}
//#endregion 🔖️Validation

//#region 🔖️InteractionStorePack
//#endregion 🔖️InteractionStorePack

#[cfg(test)]
#[path = "../🧪️tests/🔬️interaction/🦀️.rs"]
mod interaction_tests;
//#endregion 🔖️Interaction

#[cfg(test)]
#[path = "../🧪️tests/🔬️interaction-wire/🦀️.rs"]
mod interaction_wire_tests;

#[cfg(test)]
#[path = "../🧪️tests/🖱️typed-field-order/🦀️.rs"]
mod typed_field_order_tests;
