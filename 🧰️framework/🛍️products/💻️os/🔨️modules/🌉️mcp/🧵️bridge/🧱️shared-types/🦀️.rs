//! 🧱️ Shared bridge types: shell kind, flags, approval decisions and withdrawals, refusals and the instance reference.

use semio_framework_os_mcp_foundation::errors::{GatewayError, GatewayErrorCode};
use semio_framework_value::{FromValue, ToValue};
use serde::{Deserialize, Serialize};
use super::wire;

//#region 🔖️SharedTypes

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, ToValue, FromValue)]
#[serde(rename_all = "kebab-case")]
#[value(rename_all = "kebab-case")]
pub enum ShellKind {
    React,
    WgpuWeb,
    WgpuNative,
}

impl ShellKind {
    pub fn to_tag(self) -> u8 {
        match self {
            ShellKind::React => 0,
            ShellKind::WgpuWeb => 1,
            ShellKind::WgpuNative => 2,
        }
    }
    pub(super) fn from_tag(tag: u8) -> Result<Self, GatewayError> {
        match tag {
            0 => Ok(ShellKind::React),
            1 => Ok(ShellKind::WgpuWeb),
            2 => Ok(ShellKind::WgpuNative),
            other => Err(GatewayError::new(GatewayErrorCode::InputInvalid, format!("bridge frame: unknown ShellKind tag {other}"))),
        }
    }
}

/// 🚩️ `RelayAppCommands|SharedBackbone|Elicit` — a bitmask on the wire (`to_bits`/`from_bits`), a
/// small struct in Rust/JSON so fixtures and call sites read as named booleans, not magic numbers.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, ToValue, FromValue)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub struct BridgeFlags {
    pub relay_app_commands: bool,
    pub shared_backbone: bool,
    pub elicit: bool,
}

impl BridgeFlags {
    pub const NONE: Self = Self { relay_app_commands: false, shared_backbone: false, elicit: false };

    pub(super) fn to_bits(self) -> u8 {
        (self.relay_app_commands as u8) | ((self.shared_backbone as u8) << 1) | ((self.elicit as u8) << 2)
    }
    pub(super) fn from_bits(bits: u8) -> Self {
        Self { relay_app_commands: bits & 0b001 != 0, shared_backbone: bits & 0b010 != 0, elicit: bits & 0b100 != 0 }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, ToValue, FromValue)]
#[serde(rename_all = "snake_case")]
#[value(rename_all = "snake_case")]
pub enum ApprovalDecision {
    Deny,
    Once,
    Session,
}

impl ApprovalDecision {
    pub fn to_tag(self) -> u8 {
        match self {
            ApprovalDecision::Deny => 0,
            ApprovalDecision::Once => 1,
            ApprovalDecision::Session => 2,
        }
    }
    pub(super) fn from_tag(tag: u8) -> Result<Self, GatewayError> {
        match tag {
            0 => Ok(ApprovalDecision::Deny),
            1 => Ok(ApprovalDecision::Once),
            2 => Ok(ApprovalDecision::Session),
            other => Err(GatewayError::new(GatewayErrorCode::InputInvalid, format!("bridge frame: unknown ApprovalDecision tag {other}"))),
        }
    }
}

/// 🪦️ Why the gateway took an approval request back before any human decided it. The shell retires
/// the affordance and says why; the gateway no longer waits for a decision on it.
/// - `Cancelled`: the agent's call that parked the request was cancelled.
/// - `TimedOut`: nobody decided within the request's own countdown.
/// - `Superseded`: the request moved to a newer shell connection, which now carries it.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, ToValue, FromValue)]
#[serde(rename_all = "snake_case")]
#[value(rename_all = "snake_case")]
pub enum ApprovalWithdrawal {
    Cancelled,
    TimedOut,
    Superseded,
}

impl ApprovalWithdrawal {
    pub fn to_tag(self) -> u8 {
        match self {
            ApprovalWithdrawal::Cancelled => 0,
            ApprovalWithdrawal::TimedOut => 1,
            ApprovalWithdrawal::Superseded => 2,
        }
    }
    pub(super) fn from_tag(tag: u8) -> Result<Self, GatewayError> {
        match tag {
            0 => Ok(ApprovalWithdrawal::Cancelled),
            1 => Ok(ApprovalWithdrawal::TimedOut),
            2 => Ok(ApprovalWithdrawal::Superseded),
            other => Err(GatewayError::new(GatewayErrorCode::InputInvalid, format!("bridge frame: unknown ApprovalWithdrawal tag {other}"))),
        }
    }
}

/// 🚫️ Why a gateway refused a shell's connection instead of welcoming it. A shell treats a refusal as
/// a state of that offer — one localized notice, no redial of the same offer — never as a fault.
/// - `Version`: the shell's `Hello.bridge_version` is not the gateway's [`BRIDGE_VERSION`].
/// - `Capacity`: the gateway already serves as many connections as it admits.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, ToValue, FromValue)]
#[serde(rename_all = "snake_case")]
#[value(rename_all = "snake_case")]
pub enum BridgeRefusal {
    Version,
    Capacity,
}

impl BridgeRefusal {
    pub fn to_tag(self) -> u8 {
        match self {
            BridgeRefusal::Version => 0,
            BridgeRefusal::Capacity => 1,
        }
    }
    pub(super) fn from_tag(tag: u8) -> Result<Self, GatewayError> {
        match tag {
            0 => Ok(BridgeRefusal::Version),
            1 => Ok(BridgeRefusal::Capacity),
            other => Err(GatewayError::new(GatewayErrorCode::InputInvalid, format!("bridge frame: unknown BridgeRefusal tag {other}"))),
        }
    }
}

/// 📇️ One entry of `Instances{entries}` — `BridgeInstanceRef{plugin_id, app_id, instance_id,
/// artifact_ref, window_ids}` verbatim from `📋️master.md` §2.2.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, ToValue, FromValue)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub struct BridgeInstanceRef {
    pub plugin_id: String,
    pub app_id: String,
    pub instance_id: String,
    pub artifact_ref: String,
    pub window_ids: Vec<String>,
}

impl BridgeInstanceRef {
    pub fn encode(&self, buf: &mut Vec<u8>) {
        wire::write_string(buf, &self.plugin_id);
        wire::write_string(buf, &self.app_id);
        wire::write_string(buf, &self.instance_id);
        wire::write_string(buf, &self.artifact_ref);
        wire::write_string_vec(buf, &self.window_ids);
    }
    pub fn decode(reader: &mut wire::Reader<'_>) -> Result<Self, GatewayError> {
        Ok(Self { plugin_id: reader.read_string()?, app_id: reader.read_string()?, instance_id: reader.read_string()?, artifact_ref: reader.read_string()?, window_ids: reader.read_string_vec()? })
    }
}

//#endregion 🔖️SharedTypes
