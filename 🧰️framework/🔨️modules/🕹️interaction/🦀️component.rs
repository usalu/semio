//! 🕹️ First-class hover/selection mechanism: declarative `InteractionDefinition` types (mirroring
//! the manifest's action/utility/tool/command family) plus the pure, render-independent
//! hover/selection state machine (`next_selection`/`next_hover`/`validate_state`) every app's
//! runtime interception delegates to instead of hand-rolling `set-selection`/`set-hover` commands.
//! Handcrafted TS parity lives in `🟦️component.ts`; schema leaves live in `🧬️schema/`.

use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};
use ui_wgpu::wgpu::LocalizedLabel;

use crate::IconName;

//#region 🔖️Definition
/// 🕹️ One interaction domain an app declares (e.g. "graph", "mesh", "ast", "world"): the target
/// universe/hierarchy shared by both its hover and selection sub-specs. `AppDefinition.interactions`
/// holds these; `WindowKindDefinition.interactions` references them via `InteractionRef`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "typegen", derive(ts_rs::TS))]
#[serde(rename_all = "camelCase")]
pub struct InteractionDefinition {
    pub id: String,
    /// 🗣️ Manifest-level, locale×terminology-checked — see `LocalizedLabel` (follow-up: no ts-rs mirror yet).
    #[cfg_attr(feature = "typegen", ts(type = "unknown"))]
    pub label: LocalizedLabel,
    /// 🪜️ Non-empty; the first entry is the domain's default granularity.
    pub granularities: Vec<GranularityDefinition>,
    pub hierarchy: HierarchyProvider,
    pub hover: HoverSpec,
    pub selection: SelectionSpec,
}

/// 🔬️ One selectable/hoverable level of detail within a domain (e.g. mesh's object/face/edge/vertex).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "typegen", derive(ts_rs::TS))]
#[serde(rename_all = "camelCase")]
pub struct GranularityDefinition {
    pub id: String,
    /// 🗣️ Manifest-level, locale×terminology-checked — see `LocalizedLabel` (follow-up: no ts-rs mirror yet).
    #[cfg_attr(feature = "typegen", ts(type = "unknown"))]
    pub label: LocalizedLabel,
    pub icon_id: IconName,
}

/// 🌳️ Where a domain's target ids come from, and thus what `DomainTopology` (if any) is available for
/// range selection and transitive hover/select closures.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "typegen", derive(ts_rs::TS))]
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

/// 🐁️ One domain's hover behavior.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "typegen", derive(ts_rs::TS))]
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
    fn default() -> Self {
        Self { enabled: true, transitive: false, channels: default_pointer_channels(), broadcast: true }
    }
}

/// 🖱️ One domain's selection behavior.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "typegen", derive(ts_rs::TS))]
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

/// 🔢️ How many targets may be selected at once within a domain.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "typegen", derive(ts_rs::TS))]
#[serde(rename_all = "camelCase")]
pub enum SelectionMode {
    Single,
    Multiple,
}

/// 🎯️ How a surface gathers targets for one `interactionSelect` dispatch.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "typegen", derive(ts_rs::TS))]
#[serde(rename_all = "camelCase")]
pub enum SelectionMethod {
    Pick,
    Rectangle,
    Lasso,
}

/// 🧮️ Set algebra applied when merging new targets into the current selection — see `next_selection`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "typegen", derive(ts_rs::TS))]
#[serde(rename_all = "camelCase")]
pub enum MergeMode {
    Replace,
    Additive,
    Subtractive,
    Invertive,
    Range,
}

fn default_true() -> bool {
    true
}

fn default_pointer_channels() -> Vec<String> {
    vec!["pointer".to_string()]
}

/// 📇️ A validated reference into an app's `AppDefinition.interactions` registry — mirrors
/// `ActionRef`/`UtilityRef` exactly.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[cfg_attr(feature = "typegen", derive(ts_rs::TS))]
#[serde(transparent)]
pub struct InteractionRef(String);

impl InteractionRef {
    pub fn new(id: impl Into<String>) -> Self {
        Self(id.into())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl From<&str> for InteractionRef {
    fn from(value: &str) -> Self {
        Self(value.to_string())
    }
}

impl From<String> for InteractionRef {
    fn from(value: String) -> Self {
        Self(value)
    }
}
//#endregion 🔖️Definition

//#region 🔖️Runtime
/// 🎯️ One addressed target: a granularity id plus the target's own id (u32 domain ids are stringified
/// at the app boundary before reaching this module).
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "typegen", derive(ts_rs::TS))]
#[serde(rename_all = "camelCase")]
pub struct InteractionTarget {
    pub granularity: String,
    pub id: String,
}

/// 🖱️ One domain's current selection: the active granularity, the selected ids, and the anchor id
/// range selection pivots from.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "typegen", derive(ts_rs::TS))]
#[serde(rename_all = "camelCase")]
pub struct DomainSelection {
    pub granularity: String,
    pub ids: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "typegen", ts(optional))]
    pub anchor_id: Option<String>,
}

/// 🐁️ One domain's current hover on one channel: the transitive closure (root first) when
/// `HoverSpec::transitive`, otherwise just the raw hovered ids.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "typegen", derive(ts_rs::TS))]
#[serde(rename_all = "camelCase")]
pub struct DomainHover {
    pub channel: String,
    pub ids: Vec<String>,
}

/// 🗺️ Own persisted-local selection (`Interaction` history lane) + ephemeral-local hover, keyed by
/// domain id — the framework-owned counterpart to what every per-app config used to hand-roll.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "typegen", derive(ts_rs::TS))]
#[serde(rename_all = "camelCase")]
pub struct InteractionState {
    pub selection: BTreeMap<String, DomainSelection>,
    pub hover: BTreeMap<String, DomainHover>,
    pub active_mode: BTreeMap<String, SelectionMode>,
    pub active_granularity: BTreeMap<String, String>,
}
//#endregion 🔖️Runtime

//#region 🔖️Topology
/// 🌳️ One node of a domain's topology: its own granularity and its parent id (`None` = a root).
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "typegen", derive(ts_rs::TS))]
#[serde(rename_all = "camelCase")]
pub struct TopologyNode {
    pub id: String,
    pub granularity: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "typegen", ts(optional))]
    pub parent: Option<String>,
}

/// 🌲️ One domain's topology, pre-order: `ordered`'s sequence IS the range-selection order, and every
/// node's descendants form a contiguous run immediately following it.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "typegen", derive(ts_rs::TS))]
#[serde(rename_all = "camelCase")]
pub struct DomainTopology {
    pub ordered: Vec<TopologyNode>,
}

impl DomainTopology {
    /// 🔎️ The pre-order index of `id`, or `None` when absent.
    pub fn index_of(&self, id: &str) -> Option<usize> {
        self.ordered.iter().position(|node| node.id == id)
    }

    /// ✅️ Whether `id` is a known node in this topology.
    pub fn contains(&self, id: &str) -> bool {
        self.index_of(id).is_some()
    }

    fn children_by_parent(&self) -> BTreeMap<String, Vec<String>> {
        let mut children: BTreeMap<String, Vec<String>> = BTreeMap::new();
        for node in &self.ordered {
            if let Some(parent) = &node.parent {
                children.entry(parent.clone()).or_default().push(node.id.clone());
            }
        }
        children
    }

    /// 🌳️ `root_id` plus every descendant, pre-order (root first) — empty when `root_id` is absent.
    pub fn descendant_closure(&self, root_id: &str) -> Vec<String> {
        if !self.contains(root_id) {
            return Vec::new();
        }
        let children = self.children_by_parent();
        let mut out = Vec::new();
        visit_descendants(root_id, &children, &mut out);
        out
    }

    /// 🪜️ `id`'s ancestor chain, nearest parent first, root last.
    pub fn ancestors(&self, id: &str) -> Vec<String> {
        let mut out = Vec::new();
        let mut current = self.ordered.iter().find(|node| node.id == id).and_then(|node| node.parent.clone());
        while let Some(parent_id) = current {
            current = self.ordered.iter().find(|node| node.id == parent_id).and_then(|node| node.parent.clone());
            out.push(parent_id);
        }
        out
    }
}

fn visit_descendants(id: &str, children: &BTreeMap<String, Vec<String>>, out: &mut Vec<String>) {
    out.push(id.to_string());
    if let Some(kids) = children.get(id) {
        for kid in kids {
            visit_descendants(kid, children, out);
        }
    }
}

/// 🗺️ Every domain's topology for one app instance, keyed by domain id — `ArtifactApp::interaction_topology`
/// returns this (wave 3).
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "typegen", derive(ts_rs::TS))]
#[serde(rename_all = "camelCase")]
pub struct InteractionTopology {
    pub domains: BTreeMap<String, DomainTopology>,
}
//#endregion 🔖️Topology

//#region 🔖️SelectionMachine
/// 🖱️ One `next_selection` call's input: the batch of targets (a single pick or a marquee gather),
/// the merge mode to apply, and the currently active selection mode.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "typegen", derive(ts_rs::TS))]
#[serde(rename_all = "camelCase")]
pub struct SelectionInput {
    pub targets: Vec<InteractionTarget>,
    pub merge: MergeMode,
    pub mode: SelectionMode,
}

/// 🖱️ Computes the next `DomainSelection` for one domain — the generalization of Tree's
/// `getTreeNextSelectionState` (`🖱️ui/🧱️elements/🪵️Tree/🟦️component.tsx:946-968`), preserving its
/// exact single/range/toggle semantics while adding batch targets, `Additive`/`Subtractive` as
/// distinct merges, and transitive descendant-closure expansion.
///
/// - `Single` mode ignores `merge` entirely and clamps to the batch's last target (mirrors Tree
///   returning `{selectedIds:[targetId]}` unconditionally in single mode).
/// - `Range` replaces the selection with the topology-order slice between the anchor (falling back to
///   `current.anchor_id`, then `current.ids.last()`, then the target itself — mirrors Tree's
///   `fallbackAnchorId`) and the batch's last target, ascending index order; the anchor does not move.
/// - `Replace`/`Additive`/`Subtractive`/`Invertive` apply ordinary set algebra over the batch's targets
///   (each expanded to its descendant closure first when `spec.transitive`), and update the anchor to
///   the batch's last target.
///
/// Empty `input.targets` is a no-op (returns `current` unchanged).
pub fn next_selection(spec: &SelectionSpec, current: &DomainSelection, topo: &DomainTopology, input: &SelectionInput) -> DomainSelection {
    let Some(last_target) = input.targets.last() else {
        return current.clone();
    };
    let granularity = last_target.granularity.clone();
    let target_ids: Vec<String> = input.targets.iter().map(|target| target.id.clone()).collect();
    let last_target_id = last_target.id.clone();

    if input.mode == SelectionMode::Single {
        return DomainSelection { granularity, ids: vec![last_target_id.clone()], anchor_id: Some(last_target_id) };
    }

    if input.merge == MergeMode::Range {
        let fallback_anchor = current.anchor_id.clone().or_else(|| current.ids.last().cloned()).unwrap_or_else(|| last_target_id.clone());
        if let (Some(anchor_index), Some(target_index)) = (topo.index_of(&fallback_anchor), topo.index_of(&last_target_id)) {
            let (start, end) = if anchor_index <= target_index { (anchor_index, target_index) } else { (target_index, anchor_index) };
            let ids = topo.ordered[start..=end].iter().map(|node| node.id.clone()).collect();
            return DomainSelection { granularity, ids, anchor_id: Some(fallback_anchor) };
        }
        return DomainSelection { granularity, ids: vec![last_target_id.clone()], anchor_id: Some(last_target_id) };
    }

    let expanded: Vec<String> = target_ids
        .iter()
        .flat_map(|id| {
            if spec.transitive {
                let closure = topo.descendant_closure(id);
                if closure.is_empty() { vec![id.clone()] } else { closure }
            } else {
                vec![id.clone()]
            }
        })
        .collect();

    let mut ids = match input.merge {
        MergeMode::Replace => dedup_preserving_order(expanded),
        MergeMode::Additive => {
            let mut ids = current.ids.clone();
            for id in expanded {
                if !ids.contains(&id) {
                    ids.push(id);
                }
            }
            ids
        }
        MergeMode::Subtractive => current.ids.iter().filter(|id| !expanded.contains(id)).cloned().collect(),
        MergeMode::Invertive => {
            let mut ids = current.ids.clone();
            for id in expanded {
                match ids.iter().position(|existing| *existing == id) {
                    Some(index) => {
                        ids.remove(index);
                    }
                    None => ids.push(id),
                }
            }
            ids
        }
        MergeMode::Range => unreachable!("Range handled above"),
    };
    ids = dedup_preserving_order(ids);
    DomainSelection { granularity, ids, anchor_id: Some(last_target_id) }
}

fn dedup_preserving_order(ids: Vec<String>) -> Vec<String> {
    let mut out: Vec<String> = Vec::with_capacity(ids.len());
    for id in ids {
        if !out.contains(&id) {
            out.push(id);
        }
    }
    out
}
//#endregion 🔖️SelectionMachine

//#region 🔖️HoverMachine
/// 🐁️ One `next_hover` call's input: the channel and the batch of hovered targets (empty = clear).
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "typegen", derive(ts_rs::TS))]
#[serde(rename_all = "camelCase")]
pub struct HoverInput {
    pub channel: String,
    pub targets: Vec<InteractionTarget>,
}

/// 🐁️ Computes the next `DomainHover` for one channel: always REPLACES the channel's id list (hover
/// has no merge algebra). When `spec.transitive`, each target expands to its descendant closure with
/// the hovered root first; multiple targets concatenate in input order, deduplicated. Disabled specs
/// and empty target batches both clear the channel.
pub fn next_hover(spec: &HoverSpec, topo: &DomainTopology, input: &HoverInput) -> DomainHover {
    if !spec.enabled || input.targets.is_empty() {
        return DomainHover { channel: input.channel.clone(), ids: Vec::new() };
    }
    let mut ids: Vec<String> = Vec::new();
    for target in &input.targets {
        let expanded = if spec.transitive {
            let closure = topo.descendant_closure(&target.id);
            if closure.is_empty() { vec![target.id.clone()] } else { closure }
        } else {
            vec![target.id.clone()]
        };
        for id in expanded {
            if !ids.contains(&id) {
                ids.push(id);
            }
        }
    }
    DomainHover { channel: input.channel.clone(), ids }
}
//#endregion 🔖️HoverMachine

//#region 🔖️Validation
/// 🧹️ Re-derives a consistent `InteractionState` from declared `defs` + current `topo`: drops any
/// domain absent from `defs` (renamed/removed interaction declaration), prunes selection/hover ids no
/// longer present in that domain's topology (deleted document nodes — called after every artifact
/// dispatch), resets `active_granularity`/`active_mode` to a declared value (falling back to the
/// domain's default, its first declared entry) when the stored one is no longer declared, and clamps
/// `Single`-mode selections down to their first id (mirrors `normalizeTreeSelectedIds`'s external-update
/// normalization, not `next_selection`'s recency-preferring clamp).
pub fn validate_state(defs: &[InteractionDefinition], topo: &InteractionTopology, state: &InteractionState) -> InteractionState {
    let mut result = InteractionState::default();

    for def in defs {
        let domain_topo = topo.domains.get(&def.id);
        let declared_granularities: Vec<&str> = def.granularities.iter().map(|granularity| granularity.id.as_str()).collect();
        let default_granularity = def.granularities.first().map(|granularity| granularity.id.clone()).unwrap_or_default();
        let default_mode = def.selection.modes.first().copied().unwrap_or(SelectionMode::Single);

        let mode = state
            .active_mode
            .get(&def.id)
            .copied()
            .filter(|mode| def.selection.modes.contains(mode))
            .unwrap_or(default_mode);
        result.active_mode.insert(def.id.clone(), mode);

        let granularity = state
            .active_granularity
            .get(&def.id)
            .cloned()
            .filter(|granularity| declared_granularities.contains(&granularity.as_str()))
            .unwrap_or_else(|| default_granularity.clone());
        result.active_granularity.insert(def.id.clone(), granularity);

        if let Some(selection) = state.selection.get(&def.id) {
            let selection_granularity = if declared_granularities.contains(&selection.granularity.as_str()) {
                selection.granularity.clone()
            } else {
                default_granularity.clone()
            };
            let mut ids: Vec<String> = selection
                .ids
                .iter()
                .filter(|id| domain_topo.is_none_or(|topo| topo.contains(id)))
                .cloned()
                .collect();
            if mode == SelectionMode::Single && ids.len() > 1 {
                ids.truncate(1);
            }
            let anchor_id = selection.anchor_id.clone().filter(|anchor| ids.contains(anchor));
            result.selection.insert(def.id.clone(), DomainSelection { granularity: selection_granularity, ids, anchor_id });
        }

        if let Some(hover) = state.hover.get(&def.id) {
            let ids: Vec<String> = hover.ids.iter().filter(|id| domain_topo.is_none_or(|topo| topo.contains(id))).cloned().collect();
            result.hover.insert(def.id.clone(), DomainHover { channel: hover.channel.clone(), ids });
        }
    }

    result
}
//#endregion 🔖️Validation

#[cfg(test)]
mod tests {
    use super::*;

    //#region 🔖️Fixtures
    /// 🌲️ root → {a → {a1, a2}, b → {b1}}, pre-order: root, a, a1, a2, b, b1.
    fn sample_topology() -> DomainTopology {
        let node = |id: &str, parent: Option<&str>| TopologyNode { id: id.into(), granularity: "node".into(), parent: parent.map(Into::into) };
        DomainTopology {
            ordered: vec![
                node("root", None),
                node("a", Some("root")),
                node("a1", Some("a")),
                node("a2", Some("a")),
                node("b", Some("root")),
                node("b1", Some("b")),
            ],
        }
    }

    fn target(id: &str) -> InteractionTarget {
        InteractionTarget { granularity: "node".into(), id: id.into() }
    }

    fn selection(ids: &[&str], anchor: Option<&str>) -> DomainSelection {
        DomainSelection { granularity: "node".into(), ids: ids.iter().map(|id| id.to_string()).collect(), anchor_id: anchor.map(Into::into) }
    }

    fn spec(transitive: bool, merges: &[MergeMode]) -> SelectionSpec {
        SelectionSpec {
            modes: vec![SelectionMode::Multiple, SelectionMode::Single],
            methods: vec![SelectionMethod::Pick],
            merges: merges.to_vec(),
            transitive,
            broadcast: true,
        }
    }

    fn multiple_input(ids: &[&str], merge: MergeMode) -> SelectionInput {
        SelectionInput { targets: ids.iter().map(|id| target(id)).collect(), merge, mode: SelectionMode::Multiple }
    }
    //#endregion 🔖️Fixtures

    //#region 🔖️MergeModes
    #[test]
    fn replace_sets_selection_to_batch_targets() {
        let current = selection(&["a1"], Some("a1"));
        let next = next_selection(&spec(false, &[MergeMode::Replace]), &current, &sample_topology(), &multiple_input(&["b", "b1"], MergeMode::Replace));
        assert_eq!(next.ids, vec!["b".to_string(), "b1".to_string()]);
        assert_eq!(next.anchor_id.as_deref(), Some("b1"));
    }

    #[test]
    fn additive_unions_batch_into_current_selection() {
        let current = selection(&["a1"], Some("a1"));
        let next = next_selection(&spec(false, &[MergeMode::Additive]), &current, &sample_topology(), &multiple_input(&["a2"], MergeMode::Additive));
        assert_eq!(next.ids, vec!["a1".to_string(), "a2".to_string()]);
        assert_eq!(next.anchor_id.as_deref(), Some("a2"));
    }

    #[test]
    fn subtractive_removes_batch_from_current_selection() {
        let current = selection(&["a1", "a2", "b1"], Some("b1"));
        let next = next_selection(&spec(false, &[MergeMode::Subtractive]), &current, &sample_topology(), &multiple_input(&["a2"], MergeMode::Subtractive));
        assert_eq!(next.ids, vec!["a1".to_string(), "b1".to_string()]);
        assert_eq!(next.anchor_id.as_deref(), Some("a2"), "anchor tracks the last acted-on target, even on removal");
    }

    #[test]
    fn invertive_toggles_each_batch_target_independently() {
        let current = selection(&["a1", "a2"], Some("a2"));
        let next = next_selection(
            &spec(false, &[MergeMode::Invertive]),
            &current,
            &sample_topology(),
            &multiple_input(&["a2", "b1"], MergeMode::Invertive),
        );
        assert_eq!(next.ids, vec!["a1".to_string(), "b1".to_string()], "a2 was present so it toggles off, b1 was absent so it toggles on");
    }
    //#endregion 🔖️MergeModes

    //#region 🔖️Range
    #[test]
    fn range_slices_topology_order_between_anchor_and_target() {
        let current = selection(&["a"], Some("a"));
        let next = next_selection(&spec(false, &[MergeMode::Range]), &current, &sample_topology(), &multiple_input(&["b1"], MergeMode::Range));
        assert_eq!(next.ids, vec!["a".to_string(), "a1".to_string(), "a2".to_string(), "b".to_string(), "b1".to_string()]);
        assert_eq!(next.anchor_id.as_deref(), Some("a"), "range never moves the anchor");
    }

    #[test]
    fn range_falls_back_to_last_selected_id_when_no_anchor_recorded() {
        let current = selection(&["a1", "a2"], None);
        let next = next_selection(&spec(false, &[MergeMode::Range]), &current, &sample_topology(), &multiple_input(&["b"], MergeMode::Range));
        assert_eq!(next.ids, vec!["a2".to_string(), "b".to_string()]);
        assert_eq!(next.anchor_id.as_deref(), Some("a2"));
    }

    #[test]
    fn range_handles_target_before_anchor_in_topology_order() {
        let current = selection(&["b"], Some("b"));
        let next = next_selection(&spec(false, &[MergeMode::Range]), &current, &sample_topology(), &multiple_input(&["a1"], MergeMode::Range));
        assert_eq!(next.ids, vec!["a1".to_string(), "a2".to_string(), "b".to_string()]);
    }
    //#endregion 🔖️Range

    //#region 🔖️SingleClamp
    #[test]
    fn single_mode_clamps_to_last_target_regardless_of_merge() {
        let current = selection(&["a1", "a2"], Some("a1"));
        let input = SelectionInput { targets: vec![target("b"), target("b1")], merge: MergeMode::Additive, mode: SelectionMode::Single };
        let next = next_selection(&spec(false, &[MergeMode::Additive]), &current, &sample_topology(), &input);
        assert_eq!(next.ids, vec!["b1".to_string()]);
        assert_eq!(next.anchor_id.as_deref(), Some("b1"));
    }
    //#endregion 🔖️SingleClamp

    //#region 🔖️Transitive
    #[test]
    fn transitive_select_expands_target_to_descendant_closure() {
        let current = DomainSelection::default();
        let next = next_selection(&spec(true, &[MergeMode::Replace]), &current, &sample_topology(), &multiple_input(&["a"], MergeMode::Replace));
        assert_eq!(next.ids, vec!["a".to_string(), "a1".to_string(), "a2".to_string()]);
    }

    #[test]
    fn transitive_hover_expands_with_root_first() {
        let hover_spec = HoverSpec { enabled: true, transitive: true, channels: default_pointer_channels(), broadcast: true };
        let input = HoverInput { channel: "pointer".into(), targets: vec![target("a")] };
        let hover = next_hover(&hover_spec, &sample_topology(), &input);
        assert_eq!(hover.ids, vec!["a".to_string(), "a1".to_string(), "a2".to_string()]);
        assert_eq!(hover.ids.first().map(String::as_str), Some("a"), "hovered root sorts first");
    }

    #[test]
    fn non_transitive_hover_replaces_with_raw_targets_only() {
        let hover_spec = HoverSpec { enabled: true, transitive: false, channels: default_pointer_channels(), broadcast: true };
        let input = HoverInput { channel: "pointer".into(), targets: vec![target("a")] };
        let hover = next_hover(&hover_spec, &sample_topology(), &input);
        assert_eq!(hover.ids, vec!["a".to_string()]);
    }

    #[test]
    fn empty_hover_targets_clears_the_channel() {
        let hover_spec = HoverSpec::default();
        let hover = next_hover(&hover_spec, &sample_topology(), &HoverInput { channel: "pointer".into(), targets: Vec::new() });
        assert!(hover.ids.is_empty());
    }
    //#endregion 🔖️Transitive

    //#region 🔖️ValidateState
    fn sample_definition() -> InteractionDefinition {
        InteractionDefinition {
            id: "graph".into(),
            label: LocalizedLabel::native("Graph", "Graph"),
            granularities: vec![
                GranularityDefinition { id: "node".into(), label: LocalizedLabel::native("Node", "Knoten"), icon_id: "circle".into() },
                GranularityDefinition { id: "edge".into(), label: LocalizedLabel::native("Edge", "Kante"), icon_id: "minus".into() },
            ],
            hierarchy: HierarchyProvider::Topology,
            hover: HoverSpec::default(),
            selection: spec(false, &[MergeMode::Replace, MergeMode::Additive, MergeMode::Subtractive, MergeMode::Invertive, MergeMode::Range]),
        }
    }

    #[test]
    fn validate_state_prunes_ids_absent_from_topology() {
        let def = sample_definition();
        let mut topo = InteractionTopology::default();
        topo.domains.insert("graph".into(), sample_topology());

        let mut state = InteractionState::default();
        state.selection.insert("graph".into(), selection(&["a1", "deleted-node", "b1"], Some("deleted-node")));
        state.hover.insert("graph".into(), DomainHover { channel: "pointer".into(), ids: vec!["a1".into(), "gone".into()] });
        state.active_mode.insert("graph".into(), SelectionMode::Multiple);
        state.active_granularity.insert("graph".into(), "node".into());

        let validated = validate_state(&[def], &topo, &state);
        let graph_selection = validated.selection.get("graph").expect("graph domain kept");
        assert_eq!(graph_selection.ids, vec!["a1".to_string(), "b1".to_string()], "deleted-node pruned");
        assert_eq!(graph_selection.anchor_id, None, "stale anchor pruned along with its id");
        assert_eq!(validated.hover.get("graph").unwrap().ids, vec!["a1".to_string()], "gone pruned");
    }

    #[test]
    fn validate_state_drops_undeclared_domains_and_granularities() {
        let def = sample_definition();
        let topo = InteractionTopology::default();

        let mut state = InteractionState::default();
        state.selection.insert("mesh".into(), selection(&["x"], None));
        state.active_granularity.insert("graph".into(), "face".into());

        let validated = validate_state(&[def], &topo, &state);
        assert!(validated.selection.get("mesh").is_none(), "undeclared domain dropped");
        assert_eq!(validated.active_granularity.get("graph").map(String::as_str), Some("node"), "undeclared granularity resets to the default");
    }

    #[test]
    fn validate_state_clamps_single_mode_selection_to_first_id() {
        let def = sample_definition();
        let mut topo = InteractionTopology::default();
        topo.domains.insert("graph".into(), sample_topology());

        let mut state = InteractionState::default();
        state.selection.insert("graph".into(), selection(&["a1", "a2", "b1"], None));
        state.active_mode.insert("graph".into(), SelectionMode::Single);

        let validated = validate_state(&[def], &topo, &state);
        assert_eq!(validated.selection.get("graph").unwrap().ids, vec!["a1".to_string()]);
    }
    //#endregion 🔖️ValidateState

    //#region 🔖️Serde
    #[test]
    fn interaction_definition_round_trips_through_json() {
        let def = sample_definition();
        let json = serde_json::to_string(&def).expect("serializes");
        assert!(json.contains("\"iconId\""), "{json}");
        assert!(json.contains("\"granularities\""), "{json}");
        let parsed: InteractionDefinition = serde_json::from_str(&json).expect("deserializes");
        assert_eq!(parsed, def);
    }

    #[test]
    fn hierarchy_provider_serializes_internally_tagged_variants() {
        let path_delimited = HierarchyProvider::PathDelimited { delimiter: "/".into() };
        let json = serde_json::to_string(&path_delimited).unwrap();
        assert_eq!(json, "{\"kind\":\"pathDelimited\",\"delimiter\":\"/\"}");
        assert_eq!(serde_json::from_str::<HierarchyProvider>(&json).unwrap(), path_delimited);

        let flat_json = serde_json::to_string(&HierarchyProvider::Flat).unwrap();
        assert_eq!(flat_json, "{\"kind\":\"flat\"}");
    }
    //#endregion 🔖️Serde
}
