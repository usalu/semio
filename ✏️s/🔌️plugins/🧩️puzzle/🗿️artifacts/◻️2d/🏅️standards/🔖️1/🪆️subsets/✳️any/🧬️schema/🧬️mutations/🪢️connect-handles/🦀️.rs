//! 🔗 Puzzle2d mutation — `ConnectHandles`: creates a directed link between two handles, full
//! initial connection-parameterization payload included (rule 4: `connect-<nouns>{endpoints,
//! payload}`). A connection a drop records from proximity also states its precondition, the `tolerance` its two
//! handles lay within: replayed on a base where they no longer do, it still connects and reports
//! `mutation.precondition-drifted` (design
//! `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️30/NON-DESTRUCTIVE-HISTORY-EDITING/📋️design.md` §22.13).

use semio_framework_value::paged::PagedUtf8;

use crate::standards::v1::subsets::any::schema::diff::Puzzle2dDiff;
use crate::standards::v1::subsets::any::schema::mutations::Puzzle2dMutation;
use crate::Puzzle2dSnapshot;

//#region 🔖️Mutation
/// 🔗 `connect-handles` payload — edge `id`, both endpoint handle ids, and the full initial
/// connection-parameter payload (`edge_kind`/`gap`/`shift`/`rise`/`rotation`/`turn`/`tilt`/`x`/`y`/
/// `source_tip`/`target_tip`), proximity `tolerance` (`None`: no precondition), and final-state
/// insertion `index` (`None`: append; an index past the end clamps to append).
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::RetainedClone, semio_framework_value::RetireOwned)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[dsl(keyword = "connect-handles")]
pub struct ConnectHandles {
    pub id: PagedUtf8<{ usize::MAX }>,
    #[dsl(refs = "handle")]
    pub source: PagedUtf8<{ usize::MAX }>,
    #[dsl(refs = "handle")]
    pub target: PagedUtf8<{ usize::MAX }>,
    pub edge_kind: Option<PagedUtf8<{ usize::MAX }>>,
    pub gap: f64,
    pub shift: f64,
    pub rise: f64,
    pub rotation: f64,
    pub turn: f64,
    pub tilt: f64,
    pub x: f64,
    pub y: f64,
    pub source_tip: Option<PagedUtf8<{ usize::MAX }>>,
    pub target_tip: Option<PagedUtf8<{ usize::MAX }>>,
    #[cfg_attr(test, serde(default, skip_serializing_if = "Option::is_none"))]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub tolerance: Option<f64>,
    #[cfg_attr(test, serde(default, skip_serializing_if = "Option::is_none"))]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub index: Option<usize>,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
#[allow(clippy::too_many_arguments)]
pub fn connect_handles(
    id: PagedUtf8<{ usize::MAX }>,
    source: PagedUtf8<{ usize::MAX }>,
    target: PagedUtf8<{ usize::MAX }>,
    edge_kind: Option<PagedUtf8<{ usize::MAX }>>,
    gap: f64,
    shift: f64,
    rise: f64,
    rotation: f64,
    turn: f64,
    tilt: f64,
    x: f64,
    y: f64,
    source_tip: Option<PagedUtf8<{ usize::MAX }>>,
    target_tip: Option<PagedUtf8<{ usize::MAX }>>,
    index: Option<usize>,
) -> Puzzle2dMutation {
    Puzzle2dMutation::ConnectHandles(ConnectHandles { id, source, target, edge_kind, gap, shift, rise, rotation, turn, tilt, x, y, source_tip, target_tip, tolerance: None, index })
}

/// 🧲️ Builder — the connection a drop records from proximity: the default geometry and the `tolerance` its two
/// handles lay within when it was recorded.
pub fn connect_handles_in_proximity(id: PagedUtf8<{ usize::MAX }>, source: PagedUtf8<{ usize::MAX }>, target: PagedUtf8<{ usize::MAX }>, tolerance: f64) -> Puzzle2dMutation {
    Puzzle2dMutation::ConnectHandles(ConnectHandles { id, source, target, edge_kind: None, gap: 0.0, shift: 0.0, rise: 0.0, rotation: 0.0, turn: 0.0, tilt: 0.0, x: 0.0, y: 0.0, source_tip: None, target_tip: None, tolerance: Some(tolerance), index: None })
}

/// 🧩️ Captures a cold inverse edge with its exact original ordinal and optional flags.
pub(crate) fn restore_edge(edge: &crate::Puzzle2dEdge, index: usize) -> Vec<Puzzle2dMutation> {
    let visible = edge.visible.is_some().then(|| crate::standards::v1::subsets::any::schema::mutations::change_edge_visible::change_edge_visible(edge.id.clone(), edge.visible));
    let locked = edge.locked.is_some().then(|| crate::standards::v1::subsets::any::schema::mutations::change_edge_locked::change_edge_locked(edge.id.clone(), edge.locked));
    std::iter::once(connect_handles(edge.id.clone(), edge.source.clone(), edge.target.clone(), edge.edge_kind.clone(), edge.gap, edge.shift, edge.rise, edge.rotation, edge.turn, edge.tilt, edge.x, edge.y, edge.source_tip.clone(), edge.target_tip.clone(), Some(index))).chain(visible).chain(locked).collect()
}

impl protocol::MutationKind<Puzzle2dSnapshot, Puzzle2dMutation> for ConnectHandles {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "connect", entity: "handles", kind: "connect-handles", record: "ConnectedHandles" };

    fn diff(&self, base: &Puzzle2dSnapshot) -> protocol::MutationOutcome<Puzzle2dDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &Puzzle2dSnapshot) -> Result<Vec<Puzzle2dMutation>, semio_framework_value::ValueError> {
    Ok({
        super::inverse::inverse(self, base)?
    
    })
}
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Connect \"{}\" to \"{}\"", self.source, self.target), &format!("\"{}\" mit \"{}\" verbinden", self.source, self.target))
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.to_string_owner()]
    }
}
//#endregion 🔖️Mutation

#[cfg(test)]
#[path = "↩️inverse/📑️ordered-restoration/🧪️tests/🦀️.rs"]
mod ordered_restoration_tests;

#[path = "🎮️prepare/🦀️.rs"]
pub mod prepare;
