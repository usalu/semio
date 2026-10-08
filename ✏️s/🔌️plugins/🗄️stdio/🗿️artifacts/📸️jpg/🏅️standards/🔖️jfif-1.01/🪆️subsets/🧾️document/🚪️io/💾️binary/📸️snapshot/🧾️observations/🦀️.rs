//! 🧾️ Native JPEG frame, entropy table, and restart observations.

//#region FrameScanModel
/// 🧩 One SOF0 frame component descriptor: id, H/V sampling factors, and which of the (up to 4)
/// DQT tables it dequantizes against. Id-keyed within `JpgFrameHeader.components`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct JpgFrameComponent {
    pub id: u8,
    pub h_sampling: u8,
    pub v_sampling: u8,
    pub quant_table_id: u8,
}

/// 🖼️ Baseline (SOF0) frame header — sample precision, dimensions, and the per-component
/// sampling/quant-table layout the entropy-coded scan follows.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct JpgFrameHeader {
    pub precision: u8,
    pub width: u16,
    pub height: u16,
    pub components: Vec<JpgFrameComponent>,
}

/// 🎯 One SOS scan component: which DC/AC Huffman table (of up to 4 each) it decodes with.
/// Transient decode/encode state — not persisted on `JpgSnapshot` (the persisted per-component
/// table binding is `JpgFrameComponent.quant_table_id` plus `JpgSnapshot.huffman_tables`).
#[derive(Clone, Copy, Debug, PartialEq, Eq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all="camelCase")]
pub struct JpgScanComponent {
    pub id: u8,
    pub dc_table_id: u8,
    pub ac_table_id: u8,
}
//#endregion FrameScanModel

//#region QuantHuffmanTables
/// 📊️ One `DQT` table (id-keyed within `JpgSnapshot.quant_tables`). `values` is retained in the
/// EXACT zigzag scan order the DQT segment stores on disk (T.81 Annex B §B.2.4.1) — never
/// reindexed to natural/row-major order, so a decoded table round-trips byte-for-byte.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct JpgQuantTable {
    pub id: u8,
    /// 🔢️ DQT `Pq` nibble: `0` = 8-bit values, `1` = 16-bit values.
    pub precision: u8,
    pub values: [u16; 64],
}

/// 🌳️ `DHT` table class — DC (differential prediction) or AC (run-length coefficients).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, value_derive::ToValue, value_derive::FromValue, Default)]
#[value(rename_all = "camelCase")]
pub enum JpgHuffmanClass {
    #[default]
    Dc,
    Ac,
}

impl JpgHuffmanClass {
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn from_u8(v: u8) -> Result<Self, String> {
        match v {
            0 => Ok(JpgHuffmanClass::Dc),
            1 => Ok(JpgHuffmanClass::Ac),
            _ => Err(format!("jpg: unsupported huffman class {v}")),
        }
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn to_u8(self) -> u8 {
        match self {
            JpgHuffmanClass::Dc => 0,
            JpgHuffmanClass::Ac => 1,
        }
    }
}

/// 🌳️ One `DHT` table, keyed by `(class, id)` within `JpgSnapshot.huffman_tables` (DC id=0 and
/// AC id=0 are DIFFERENT tables — the compound key is load-bearing). `bits`/`values` are the raw
/// canonical-code counts-per-length and symbol-value bytes exactly as the DHT segment stores them.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct JpgHuffmanTable {
    pub id: u8,
    pub class: JpgHuffmanClass,
    pub bits: [u8; 16],
    #[value(default, with = "pack::value::bytes", serialize_controlled_with = "pack::value::bytes::to_value_controlled")]
    pub values: Vec<u8>,
}
//#endregion QuantHuffmanTables


/// 🧾️ Detailed physical observations retained separately from logical image content.
#[derive(Clone,Debug,PartialEq)]
pub struct JpgNativeObservations {
    pub frame:JpgFrameHeader,
    pub sof_marker:u8,
    pub arithmetic:bool,
    pub arithmetic_conditioning:Vec<JpgArithmeticConditioning>,
    pub scan_components:Vec<JpgScanComponent>,
    pub scan_parameters:[u8;3],
    pub quant_tables:Vec<JpgQuantTable>,
    pub huffman_tables:Vec<JpgHuffmanTable>,
    pub restart_interval:Option<u16>,
}
impl JpgNativeObservations {
    /// 🧮️ Projects native observations into immutable first-party conformance facts.
    pub fn baseline_facts(&self)->crate::standards::v_jfif_1_01::subsets::baseline::schema::conformance::JpgBaselineFacts {
        use crate::standards::v_jfif_1_01::subsets::baseline::schema::conformance::{JpgBaselineFacts,JpgSamplingFact};
        JpgBaselineFacts{has_frame:true,baseline_sequential:self.sof_marker==0xc0,sample_precision:self.frame.precision,arithmetic_conditioning:self.arithmetic,dc_table_count:self.huffman_tables.iter().filter(|table|table.class==JpgHuffmanClass::Dc).map(|table|table.id).collect::<std::collections::HashSet<_>>().len(),ac_table_count:self.huffman_tables.iter().filter(|table|table.class==JpgHuffmanClass::Ac).map(|table|table.id).collect::<std::collections::HashSet<_>>().len(),components:self.frame.components.iter().map(|component|JpgSamplingFact{id:component.id,horizontal:component.h_sampling,vertical:component.v_sampling}).collect()}
    }
}
/// 📦️ One physical admission with independent logical and native observation owners.
pub struct JpgDecodedDocument {pub snapshot:crate::JpgSnapshot,pub observations:JpgNativeObservations}

/// 🎛️ Exact DAC selector and conditioning value in native encounter order.
#[derive(Clone,Copy,Debug,PartialEq,Eq,value_derive::ToValue,value_derive::FromValue)]
#[value(rename_all="camelCase")]
pub struct JpgArithmeticConditioning{pub selector:u8,pub value:u8}
