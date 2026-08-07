//! ⚡️ Block 5D artifact — the operation enum, its `Operation` law and the store aliases
//! (constitutional: op).


//#region 📖️SemioGrammar
/// 📖️ Normative handcrafted text grammar for this facet (`dialect grammar`).
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️component.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️component.grammar.semio");
//#endregion 📖️SemioGrammar


use crate::artifacts::block5d::diff::{block5d_index_of, Block5dDiff};
use crate::artifacts::block5d::{Block5dDefinition, Block5dGripKind, Block5dGripTemplate, Block5dPart2d, Block5dPart3d};
use crate::{BlockAttribute, BlockAuthor, BlockCamera2d, BlockCamera3d, BlockCompatibilityRule, BlockKindIdentity, BlockMeta, BlockRepresentation};
use protocol::Operation;
use serde::{Deserialize, Serialize};

// #region 🔖️Operation
/// 🧮️ Block-5d operation: id-keyed table edits plus scalar part_kind/part_2d/part_3d/camera2d/
/// camera3d/meta, each with a true inverse computed from the pre-operation projection, and a
/// whole-document replace for example loads.
// 🧯️ `large_enum_variant`: `SetDocument`'s whole-document payload makes it far larger than the other
// scalar/id-keyed variants, but boxing it would require the `#[derive(dsl::DslEnum)]` field-shape
// machinery to see through `Box<T>`, which is unverified — same accepted tradeoff as gis's
// `Gis2dConfigOperation`/💡️reasoning's `ReplaceDocument`.
#[allow(clippy::large_enum_variant)]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslEnum)]
#[serde(tag = "operation", rename_all = "camelCase")]
pub enum Block5dOperation {
    #[dsl(key = "setPartKind")]
    SetPartKind {
        #[dsl(block)]
        part_kind: BlockKindIdentity,
    },
    #[dsl(key = "setPart2d")]
    SetPart2d {
        #[dsl(block)]
        part_2d: Block5dPart2d,
    },
    #[dsl(key = "setPart3d")]
    SetPart3d {
        #[dsl(block)]
        part_3d: Block5dPart3d,
    },
    #[dsl(key = "setRepresentation")]
    SetRepresentation {
        index: usize,
        #[dsl(block)]
        representation: BlockRepresentation,
    },
    #[dsl(key = "removeRepresentation")]
    RemoveRepresentation { id: String },
    #[dsl(key = "setGripKind")]
    SetGripKind {
        index: usize,
        #[dsl(block)]
        grip_kind: Block5dGripKind,
    },
    #[dsl(key = "removeGripKind")]
    RemoveGripKind { id: String },
    #[dsl(key = "setGrip")]
    SetGrip {
        index: usize,
        #[dsl(block)]
        grip: Block5dGripTemplate,
    },
    #[dsl(key = "removeGrip")]
    RemoveGrip { id: String },
    #[dsl(key = "setCompatibilityRule")]
    SetCompatibilityRule {
        index: usize,
        #[dsl(block)]
        rule: BlockCompatibilityRule,
    },
    #[dsl(key = "removeCompatibilityRule")]
    RemoveCompatibilityRule { id: String },
    #[dsl(key = "setAttribute")]
    SetAttribute {
        index: usize,
        #[dsl(block)]
        attribute: BlockAttribute,
    },
    #[dsl(key = "removeAttribute")]
    RemoveAttribute { key: String },
    #[dsl(key = "setAuthors")]
    SetAuthors { authors: Vec<BlockAuthor> },
    #[dsl(key = "setCamera2d")]
    SetCamera2d {
        #[dsl(block)]
        camera2d: BlockCamera2d,
    },
    #[dsl(key = "setCamera3d")]
    SetCamera3d {
        #[dsl(block)]
        camera3d: BlockCamera3d,
    },
    #[dsl(key = "setMeta")]
    SetMeta {
        #[dsl(block)]
        meta: BlockMeta,
    },
    #[dsl(key = "setDocument")]
    SetDocument {
        #[dsl(block)]
        document: Block5dDefinition,
    },
}
//#region 🔖️HandcraftedOpCodecs
/// ⚡️ P6 handcrafted OpText/OpBinary (derive no longer emits these traits).
impl protocol::OpText for Block5dOperation {
    fn parse_op(line: &str) -> Result<Self, store::TextError> {
        let variants = <Self as dsl::DslVariants>::variants();
        for (keyword, spec_fn) in &variants {
            let probe = format!("{} ", keyword);
            if line == keyword.as_str() || line.starts_with(&probe) {
                let record = dsl::parse(
                    line,
                    &spec_fn(),
                    &dsl::ParseOptions { limits: dsl::Limits::default(), mode: dsl::SourceMode::Inline },
                )?;
                return <Self as dsl::DslVariants>::from_named_record(keyword, &record);
            }
        }
        Err(dsl::__rt::field_error(format!("unknown operation line '{line}'")))
    }
    fn print_op(&self) -> String {
        let (keyword, record) = <Self as dsl::DslVariants>::to_named_record(self);
        let variants = <Self as dsl::DslVariants>::variants();
        let spec_fn = variants.iter().find(|(k, _)| k == &keyword).map(|(_, s)| *s).expect("variant spec must exist for its own keyword");
        dsl::print(&record, &spec_fn(), dsl::JoinMode::Inline)
    }
}

impl protocol::OpBinary for Block5dOperation {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        dsl::variants_binary::encode_op(self)
    }
    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        dsl::variants_binary::decode_op(bytes)
    }
}
//#endregion 🔖️HandcraftedOpCodecs




fn block5d_operation_diff(operation: &Block5dOperation) -> Block5dDiff {
    let mut diff = Block5dDiff::default();
    match operation {
        Block5dOperation::SetPartKind { part_kind } => diff.part_kind = Some(part_kind.clone()),
        Block5dOperation::SetPart2d { part_2d } => diff.part_2d = Some(part_2d.clone()),
        Block5dOperation::SetPart3d { part_3d } => diff.part_3d = Some(part_3d.clone()),
        Block5dOperation::SetRepresentation { index, representation } => diff.representations.set.push((*index, representation.clone())),
        Block5dOperation::RemoveRepresentation { id } => diff.representations.removed.push(id.clone()),
        Block5dOperation::SetGripKind { index, grip_kind } => diff.grip_kinds.set.push((*index, grip_kind.clone())),
        Block5dOperation::RemoveGripKind { id } => diff.grip_kinds.removed.push(id.clone()),
        Block5dOperation::SetGrip { index, grip } => diff.grips.set.push((*index, grip.clone())),
        Block5dOperation::RemoveGrip { id } => diff.grips.removed.push(id.clone()),
        Block5dOperation::SetCompatibilityRule { index, rule } => diff.compatibility.set.push((*index, rule.clone())),
        Block5dOperation::RemoveCompatibilityRule { id } => diff.compatibility.removed.push(id.clone()),
        Block5dOperation::SetAttribute { index, attribute } => diff.attributes.set.push((*index, attribute.clone())),
        Block5dOperation::RemoveAttribute { key } => diff.attributes.removed.push(key.clone()),
        Block5dOperation::SetAuthors { authors } => diff.authors = Some(authors.clone()),
        Block5dOperation::SetCamera2d { camera2d } => diff.camera2d = Some(camera2d.clone()),
        Block5dOperation::SetCamera3d { camera3d } => diff.camera3d = Some(camera3d.clone()),
        Block5dOperation::SetMeta { meta } => diff.meta = Some(meta.clone()),
        Block5dOperation::SetDocument { document } => diff.document = Some(document.clone()),
    }
    diff
}

impl Operation<Block5dDefinition> for Block5dOperation {
    type Diff = Block5dDiff;

    fn diff(&self, _projection: &Block5dDefinition) -> Block5dDiff {
        block5d_operation_diff(self)
    }

    fn backwards(&self, projection: &Block5dDefinition) -> Vec<Self> {
        match self {
            Block5dOperation::SetPartKind { .. } => vec![Block5dOperation::SetPartKind { part_kind: projection.part_kind.clone() }],
            Block5dOperation::SetPart2d { .. } => vec![Block5dOperation::SetPart2d { part_2d: projection.part_2d.clone() }],
            Block5dOperation::SetPart3d { .. } => vec![Block5dOperation::SetPart3d { part_3d: projection.part_3d.clone() }],
            Block5dOperation::SetRepresentation { representation, .. } => match block5d_index_of(&projection.representations, &representation.id) {
                Some(index) => vec![Block5dOperation::SetRepresentation { index, representation: projection.representations[index].clone() }],
                None => vec![Block5dOperation::RemoveRepresentation { id: representation.id.clone() }],
            },
            Block5dOperation::RemoveRepresentation { id } => {
                block5d_index_of(&projection.representations, id).map(|index| vec![Block5dOperation::SetRepresentation { index, representation: projection.representations[index].clone() }]).unwrap_or_default()
            }
            Block5dOperation::SetGripKind { grip_kind, .. } => match block5d_index_of(&projection.grip_kinds, &grip_kind.id) {
                Some(index) => vec![Block5dOperation::SetGripKind { index, grip_kind: projection.grip_kinds[index].clone() }],
                None => vec![Block5dOperation::RemoveGripKind { id: grip_kind.id.clone() }],
            },
            Block5dOperation::RemoveGripKind { id } => block5d_index_of(&projection.grip_kinds, id).map(|index| vec![Block5dOperation::SetGripKind { index, grip_kind: projection.grip_kinds[index].clone() }]).unwrap_or_default(),
            Block5dOperation::SetGrip { grip, .. } => match block5d_index_of(&projection.grips, &grip.id) {
                Some(index) => vec![Block5dOperation::SetGrip { index, grip: projection.grips[index].clone() }],
                None => vec![Block5dOperation::RemoveGrip { id: grip.id.clone() }],
            },
            Block5dOperation::RemoveGrip { id } => block5d_index_of(&projection.grips, id).map(|index| vec![Block5dOperation::SetGrip { index, grip: projection.grips[index].clone() }]).unwrap_or_default(),
            Block5dOperation::SetCompatibilityRule { rule, .. } => match block5d_index_of(&projection.compatibility, &rule.id) {
                Some(index) => vec![Block5dOperation::SetCompatibilityRule { index, rule: projection.compatibility[index].clone() }],
                None => vec![Block5dOperation::RemoveCompatibilityRule { id: rule.id.clone() }],
            },
            Block5dOperation::RemoveCompatibilityRule { id } => block5d_index_of(&projection.compatibility, id).map(|index| vec![Block5dOperation::SetCompatibilityRule { index, rule: projection.compatibility[index].clone() }]).unwrap_or_default(),
            Block5dOperation::SetAttribute { attribute, .. } => match block5d_index_of(&projection.attributes, &attribute.key) {
                Some(index) => vec![Block5dOperation::SetAttribute { index, attribute: projection.attributes[index].clone() }],
                None => vec![Block5dOperation::RemoveAttribute { key: attribute.key.clone() }],
            },
            Block5dOperation::RemoveAttribute { key } => block5d_index_of(&projection.attributes, key).map(|index| vec![Block5dOperation::SetAttribute { index, attribute: projection.attributes[index].clone() }]).unwrap_or_default(),
            Block5dOperation::SetAuthors { .. } => vec![Block5dOperation::SetAuthors { authors: projection.authors.clone() }],
            Block5dOperation::SetCamera2d { .. } => vec![Block5dOperation::SetCamera2d { camera2d: projection.camera2d.clone() }],
            Block5dOperation::SetCamera3d { .. } => vec![Block5dOperation::SetCamera3d { camera3d: projection.camera3d.clone() }],
            Block5dOperation::SetMeta { .. } => vec![Block5dOperation::SetMeta { meta: projection.meta.clone() }],
            Block5dOperation::SetDocument { .. } => vec![Block5dOperation::SetDocument { document: projection.clone() }],
        }
    }
}

pub type Block5dEnvelope = store::DocumentEnvelope<Block5dDefinition, Block5dOperation>;
pub type Block5dStore = store::DocumentStore<Block5dDefinition, Block5dOperation>;
// #endregion 🔖️Operation

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::block5d::engine::empty_block5d_definition;
    use protocol::OperationDiff;

    #[test]
    fn set_grip_then_remove_round_trips_through_true_inverse() {
        let mut projection = empty_block5d_definition();
        let set = Block5dOperation::SetGrip { index: 0, grip: Block5dGripTemplate { id: "g0".into(), grip_kind: "b-l".into(), angle: -0.1, radius_2d: 3.0, position: [4.05, 4.68, 3.0], direction: [0.0, 1.0, 0.0], radius_3d: 0.36 } };
        let inverse = set.backwards(&projection);
        projection = set.diff(&projection).apply(&projection);
        assert_eq!(projection.grips.len(), 1);
        assert_eq!(inverse, vec![Block5dOperation::RemoveGrip { id: "g0".into() }]);
        for operation in &inverse {
            projection = operation.diff(&projection).apply(&projection);
        }
        assert_eq!(projection, empty_block5d_definition());
    }
}
//#endregion 🧪️Tests
