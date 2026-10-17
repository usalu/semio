//! 🔗 Puzzle5d mutation — `ConnectGrips`: creates a fastener between two full grip ids
//! (`part_id:grip_id`), full initial connection-parameterization payload included.
use crate::standards::v1::subsets::any::schema::diff::Puzzle5dDiff;
use crate::Puzzle5dSnapshot;

//#region 🔖️Mutation
/// 🔗 `connect-grips` payload — fastener `id`, both endpoint full grip ids, and the full initial
/// connection-parameter payload.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner=semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[dsl(keyword = "connect-grips")]
pub struct ConnectGrips {
    pub id: String,
    pub source: String,
    pub target: String,
    pub fastener_kind: Option<String>,
    pub gap: f64,
    pub shift: f64,
    pub rise: f64,
    pub rotation: f64,
    pub turn: f64,
    pub tilt: f64,
    pub x: f64,
    pub y: f64,
    pub index: Option<usize>,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
#[allow(clippy::too_many_arguments)]
pub fn connect_grips<Op>(id: String, source: String, target: String, fastener_kind: Option<String>, gap: f64, shift: f64, rise: f64, rotation: f64, turn: f64, tilt: f64, x: f64, y: f64, index: Option<usize>) -> Op
where
    Op: From<crate::standards::v1::subsets::any::schema::mutations::connect_grips::ConnectGrips>,
{
    Op::from(ConnectGrips { id, source, target, fastener_kind, gap, shift, rise, rotation, turn, tilt, x, y, index })
}

impl<Op> protocol::MutationKind<Puzzle5dSnapshot, Op> for ConnectGrips
where
    Op: protocol::Mutation<Puzzle5dSnapshot, Diff = Puzzle5dDiff> + From<ConnectGrips> + From<crate::standards::v1::subsets::any::schema::mutations::disconnect_grips::DisconnectGrips>,
{
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "connect", entity: "grips", kind: "connect-grips", record: "ConnectedGrips" };

    fn diff(&self, base: &Puzzle5dSnapshot) -> protocol::MutationOutcome<Puzzle5dDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &Puzzle5dSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok({
        super::inverse::inverse(self, base)?
    
    })
}
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Connect \"{}\" to \"{}\"", self.source, self.target), &format!("\"{}\" mit \"{}\" verbinden", self.source, self.target))
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
//#endregion 🔖️Mutation
