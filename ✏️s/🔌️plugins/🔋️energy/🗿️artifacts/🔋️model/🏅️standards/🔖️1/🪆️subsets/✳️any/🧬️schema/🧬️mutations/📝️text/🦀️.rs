//! 📝️ Energy-model mutation text framing — the handcrafted `OpText`/`OpBinary` pair over the
//! `dsl::DslVariants` binding `#[derive(dsl::DslEnum)]` emits for `EnergyModelMutation`. P6: the
//! derive no longer emits these traits, so they are written once here for the whole aggregate and
//! never per kind (identical shape to `📸️remodel`'s own facet).

//#region 📖️SemioGrammar
/// 📖️ Normative handcrafted text grammar for this mutation facet.
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️.grammar.semio");
//#endregion 📖️SemioGrammar

pub use crate::mutations::EnergyModelMutation;

//#region 🔖️HandcraftedOpCodecs
impl protocol::OpText for EnergyModelMutation {
    fn parse_op(line: &str) -> Result<Self, store::TextError> {
        dsl::variants_text::parse_op(line)
    }
    fn print_op(&self) -> String {
        dsl::variants_text::print_op(self)
    }
}

impl protocol::OpBinary for EnergyModelMutation {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        dsl::variants_binary::encode_tagged_op(include_str!("../💾️binary/📡️.protocol.semio"), self)
    }
    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        dsl::variants_binary::decode_tagged_op(include_str!("../💾️binary/📡️.protocol.semio"), bytes)
    }
}
//#endregion 🔖️HandcraftedOpCodecs
