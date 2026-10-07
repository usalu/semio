//! 🧮️ First-party observations consumed by pure baseline rules.
#[derive(Clone,Debug,PartialEq,Eq,value_derive::ToValue,value_derive::FromValue)]
#[value(rename_all="camelCase")]
pub struct JpgSamplingFact { pub id:u8,pub horizontal:u8,pub vertical:u8 }
#[derive(Clone,Debug,Default,PartialEq,Eq,value_derive::ToValue,value_derive::FromValue)]
#[value(rename_all="camelCase")]
pub struct JpgBaselineFacts {
    pub has_frame:bool,
    pub baseline_sequential:bool,
    pub sample_precision:u8,
    pub arithmetic_conditioning:bool,
    pub dc_table_count:usize,
    pub ac_table_count:usize,
    pub components:Vec<JpgSamplingFact>,
}
