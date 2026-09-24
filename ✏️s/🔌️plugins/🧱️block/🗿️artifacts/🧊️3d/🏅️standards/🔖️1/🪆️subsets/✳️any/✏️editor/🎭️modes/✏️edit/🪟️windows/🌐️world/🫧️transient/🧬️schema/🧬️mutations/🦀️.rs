//! 🫧️ Block3d world-window transient mutation aggregate.

use super::Block3dWorldWindowTransient;

#[path = "👁️set-brush-preview/🦀️.rs"]
mod set_brush_preview;
pub use set_brush_preview::SetBrushPreview;

#[derive(Clone, Debug, PartialEq, dsl::ToValue, dsl::FromValue, dsl::DslOps, dsl::Mutations)]
#[mutations(snapshot = Block3dWorldWindowTransient, diff = Block3dWorldWindowTransient, schema = "block.3dworldwindowtransient")]
pub enum Block3dWorldWindowTransientMutation {
    #[dsl(key = "set-brush-preview")]
    SetBrushPreview(SetBrushPreview),
}

impl protocol::OpText for Block3dWorldWindowTransientMutation {
    fn parse_op(line: &str) -> Result<Self, store::TextError> {
        let variants = <Self as dsl::DslVariants>::variants();
        for (keyword, spec_fn) in &variants {
            let probe = format!("{} ", keyword);
            if line == keyword.as_str() || line.starts_with(&probe) {
                let record = dsl::parse(line, &spec_fn(), &dsl::ParseOptions { limits: dsl::Limits::default(), mode: dsl::SourceMode::Inline })?;
                return <Self as dsl::DslVariants>::from_named_record(keyword, &record);
            }
        }
        Err(dsl::__rt::field_error(format!("unknown operation line '{line}'")))
    }

    fn print_op(&self) -> String {
        let (keyword, record) = <Self as dsl::DslVariants>::to_named_record(self);
        let variants = <Self as dsl::DslVariants>::variants();
        let spec_fn = variants.iter().find(|(key, _)| key == &keyword).map(|(_, spec)| *spec).expect("variant spec must exist for its own keyword");
        dsl::print(&record, &spec_fn(), dsl::JoinMode::Inline)
    }
}

impl protocol::OpBinary for Block3dWorldWindowTransientMutation {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        dsl::variants_binary::encode_op(self)
    }

    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        dsl::variants_binary::decode_op(bytes)
    }
}

//#region 🌉️TestBridge
/// 🌉️ The committed-vector report of this state lane for the language-neutral case adapter, which links only this
/// crate: production dispatch (`Mutation::diff(..).apply_to`) and the mutation's own inverse over `Block3dWorldWindowTransient`.
///
/// @see store::os_store::test_support::mutation_report_json
pub fn block3d_world_window_transient_mutation_report_json(base_json: &str, mutation_json: &str, after_json: &str) -> Result<String, String> {
    store::os_store::test_support::mutation_report_json::<Block3dWorldWindowTransient, Block3dWorldWindowTransientMutation>(base_json, mutation_json, after_json)
}
//#endregion 🌉️TestBridge
