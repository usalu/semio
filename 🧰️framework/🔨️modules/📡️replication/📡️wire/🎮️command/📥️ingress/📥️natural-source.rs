//! 🧾 Revision-free wire records for the dedicated natural source command.

pub const NATURAL_SOURCE_COMMAND_KIND: u8 = 241;

#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize,
    semio_framework_value_derive::ToValue, semio_framework_value_derive::FromValue,
    semio_framework_value::NativeReceive, semio_framework_value::RetireOwned)]
#[serde(rename_all = "camelCase", rename_all_fields = "camelCase")]
#[value(crate = "::semio_framework_value", rename_all = "camelCase", rename_all_fields = "camelCase")]
pub enum ActorInstanceOpenPurpose {
    Ordinary,
    NaturalSource { declaration: NaturalSourceWireDeclaration, extent: u32 },
}

#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize,
    semio_framework_value_derive::ToValue, semio_framework_value_derive::FromValue,
    semio_framework_value::NativeReceive, semio_framework_value::RetireOwned)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
#[value(crate = "::semio_framework_value", rename_all = "camelCase", deny_unknown_fields)]
pub struct NaturalSourceWireDeclaration {
    pub format_kind: String,
    pub extension: String,
    pub media_type: String,
    pub binary: bool,
}

#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize,
    semio_framework_value_derive::ToValue, semio_framework_value_derive::FromValue,
    semio_framework_value::NativeReceive, semio_framework_value::RetireOwned)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
#[value(crate = "::semio_framework_value", rename_all = "camelCase", deny_unknown_fields)]
pub struct NaturalSourcePrepare {
    pub instance_id: u32,
    pub seq: u64,
    pub declaration: NaturalSourceWireDeclaration,
    pub extent: u32,
}

#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize,
    semio_framework_value_derive::ToValue, semio_framework_value_derive::FromValue,
    semio_framework_value::NativeReceive, semio_framework_value::RetireOwned)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
#[value(crate = "::semio_framework_value", rename_all = "camelCase", deny_unknown_fields)]
pub struct NaturalSourceWireLoan {
    pub operation_id: String,
    pub generation: String,
    pub instance_id: u32,
    pub declaration: NaturalSourceWireDeclaration,
    pub extent: u32,
}

#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize,
    semio_framework_value_derive::ToValue, semio_framework_value_derive::FromValue,
    semio_framework_value::NativeReceive, semio_framework_value::RetireOwned)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
#[value(crate = "::semio_framework_value", rename_all = "camelCase", deny_unknown_fields)]
pub struct NaturalSourceControlEvent {
    pub loan: NaturalSourceWireLoan,
    pub owner: u64,
    pub seq: u64,
    pub kind: u8,
    pub extent: u32,
    pub control: NaturalSourceWireControl,
}

#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize,
    semio_framework_value_derive::ToValue, semio_framework_value_derive::FromValue,
    semio_framework_value::NativeReceive, semio_framework_value::RetireOwned)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
#[value(crate = "::semio_framework_value", rename_all = "camelCase", deny_unknown_fields)]
pub struct NaturalSourceWireAck {
    pub loan: Option<NaturalSourceWireLoan>,
    pub owner: u64,
    pub seq: u64,
    pub kind: u8,
    pub extent: u32,
    pub phase: NaturalSourceWirePhase,
    pub page_index: Option<u32>,
    pub result: NaturalSourceWireResult,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize,
    semio_framework_value_derive::ToValue, semio_framework_value_derive::FromValue,
    semio_framework_value::NativeReceive, semio_framework_value::RetireOwned)]
#[serde(rename_all = "camelCase")]
#[value(crate = "::semio_framework_value", rename_all = "camelCase")]
pub enum NaturalSourceWireControl { Begin, Complete, Cancel }

#[derive(Clone, Copy, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize,
    semio_framework_value_derive::ToValue, semio_framework_value_derive::FromValue,
    semio_framework_value::NativeReceive, semio_framework_value::RetireOwned)]
#[serde(rename_all = "camelCase")]
#[value(crate = "::semio_framework_value", rename_all = "camelCase")]
pub enum NaturalSourceWirePhase { Prepare, Begin, Page, Complete, Cancel }

#[derive(Clone, Copy, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize,
    semio_framework_value_derive::ToValue, semio_framework_value_derive::FromValue,
    semio_framework_value::NativeReceive, semio_framework_value::RetireOwned)]
#[serde(rename_all = "camelCase")]
#[value(crate = "::semio_framework_value", rename_all = "camelCase")]
pub enum NaturalSourceWireResult { Retry, Pending, Accepted, Stale, Refused }
