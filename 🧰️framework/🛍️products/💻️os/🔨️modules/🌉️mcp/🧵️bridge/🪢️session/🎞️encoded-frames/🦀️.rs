//! 🎞️ Encoded frames of the bridge session: the encoded frame with its page leases and release, and the encoded lease. See [`super`].

use super::*;

//#region 🔖️BridgeHandle
pub(crate) struct BridgeEncodedFrame {
    pub(crate) pages: [Option<Box<[u8; BRIDGE_OUTBOX_PAGE_BYTES]>>; BRIDGE_OUTBOX_MAX_PAGES],
    pub len: usize,
    pub(crate) retirement: Option<(Arc<BridgeAsyncAuthority>, BridgeRetirementGrant)>,
}

impl BridgeEncodedFrame {
    pub(crate) fn empty(len: usize, retirement: Option<(Arc<BridgeAsyncAuthority>, BridgeRetirementGrant)>) -> Self {
        Self { pages: std::array::from_fn(|_| None), len, retirement }
    }

    pub fn encode(frame: &GatewayToShell, expected: usize) -> Self {
        let mut encoded = Self::empty(0, None);
        match frame {
            GatewayToShell::Welcome { bridge_version, connection, principal } => {
                encoded.write_u8(0);
                encoded.write_bytes(&bridge_version.to_le_bytes());
                encoded.write_field(connection.as_bytes());
                encoded.write_field(principal.as_bytes());
            }
            GatewayToShell::ShellCommand { seq, command } => {
                encoded.write_u8(1);
                encoded.write_bytes(&seq.to_le_bytes());
                encoded.write_field(command);
            }
            GatewayToShell::AppCommand { seq, instance_id, command } => {
                encoded.write_u8(2);
                encoded.write_bytes(&seq.to_le_bytes());
                encoded.write_field(instance_id.as_bytes());
                encoded.write_field(command);
            }
            GatewayToShell::ApprovalRequested { approval_id, summary } => {
                encoded.write_u8(3);
                encoded.write_field(approval_id.as_bytes());
                encoded.write_field(summary.as_bytes());
            }
            GatewayToShell::ApprovalResolved { approval_id, decision } => {
                encoded.write_u8(4);
                encoded.write_field(approval_id.as_bytes());
                encoded.write_u8(decision.to_tag());
            }
            GatewayToShell::AgentPresence { active, label, invocation_id } => {
                encoded.write_u8(5);
                encoded.write_u8(*active as u8);
                encoded.write_field(label.as_bytes());
                encoded.write_u8(invocation_id.is_some() as u8);
                if let Some(invocation_id) = invocation_id {
                    encoded.write_field(invocation_id.as_bytes());
                }
            }
            GatewayToShell::Pong => encoded.write_u8(6),
            GatewayToShell::Bye { reason } => {
                encoded.write_u8(7);
                encoded.write_field(reason.as_bytes());
            }
            GatewayToShell::AgentToolCall { invocation_id, tool_name, arguments } => {
                encoded.write_u8(8);
                encoded.write_field(invocation_id.as_bytes());
                encoded.write_field(tool_name.as_bytes());
                encoded.write_field(arguments.as_bytes());
            }
            GatewayToShell::AgentToolResult { invocation_id, tool_name, ok, summary } => {
                encoded.write_u8(9);
                encoded.write_field(invocation_id.as_bytes());
                encoded.write_field(tool_name.as_bytes());
                encoded.write_u8(*ok as u8);
                encoded.write_field(summary.as_bytes());
            }
            GatewayToShell::AgentReply { reply_id, in_reply_to, text, complete } => {
                encoded.write_u8(10);
                encoded.write_field(reply_id.as_bytes());
                encoded.write_u8(in_reply_to.is_some() as u8);
                if let Some(in_reply_to) = in_reply_to {
                    encoded.write_field(in_reply_to.as_bytes());
                }
                encoded.write_field(text.as_bytes());
                encoded.write_u8(*complete as u8);
            }
            GatewayToShell::ApprovalWithdrawn { approval_id, reason } => {
                encoded.write_u8(11);
                encoded.write_field(approval_id.as_bytes());
                encoded.write_u8(reason.to_tag());
            }
            GatewayToShell::Refused { reason, gateway_version } => {
                encoded.write_u8(12);
                encoded.write_u8(reason.to_tag());
                encoded.write_bytes(&gateway_version.to_le_bytes());
            }
        }
        assert_eq!(encoded.len, expected, "preflighted bridge frame length changed during encode");
        encoded
    }

    fn write_u8(&mut self, value: u8) {
        self.write_bytes(&[value]);
    }

    fn write_field(&mut self, value: &[u8]) {
        self.write_bytes(&(value.len() as u32).to_le_bytes());
        self.write_bytes(value);
    }

    fn write_bytes(&mut self, mut value: &[u8]) {
        while !value.is_empty() {
            let page_index = self.len / BRIDGE_OUTBOX_PAGE_BYTES;
            let page_offset = self.len % BRIDGE_OUTBOX_PAGE_BYTES;
            if self.pages[page_index].is_none() {
                self.pages[page_index] = Some(Box::new([0; BRIDGE_OUTBOX_PAGE_BYTES]));
            }
            let bytes = value.len().min(BRIDGE_OUTBOX_PAGE_BYTES - page_offset);
            self.pages[page_index].as_mut().expect("bridge encode page disappeared")[page_offset..page_offset + bytes].copy_from_slice(&value[..bytes]);
            self.len += bytes;
            value = &value[bytes..];
        }
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn copy_into(&self, offset: usize, output: &mut [u8]) -> usize {
        if offset >= self.len {
            return 0;
        }
        let mut source = offset;
        let mut written = 0;
        while written < output.len() && source < self.len {
            let page_index = source / BRIDGE_OUTBOX_PAGE_BYTES;
            let page_offset = source % BRIDGE_OUTBOX_PAGE_BYTES;
            let bytes = (output.len() - written).min(BRIDGE_OUTBOX_PAGE_BYTES - page_offset).min(self.len - source);
            output[written..written + bytes].copy_from_slice(&self.pages[page_index].as_ref().expect("bridge encoded page disappeared")[page_offset..page_offset + bytes]);
            written += bytes;
            source += bytes;
        }
        written
    }

    #[cfg(test)]
    pub(crate) fn page_count(&self) -> usize {
        self.pages.iter().filter(|page| page.is_some()).count()
    }

    #[cfg(test)]
    pub(crate) fn decode(&self) -> GatewayToShell {
        let mut bytes = Vec::new();
        bytes.try_reserve_exact(self.len).expect("bridge test decode reservation failed");
        for offset in (0..self.len).step_by(BRIDGE_OUTBOX_PAGE_BYTES) {
            let bytes_in_page = BRIDGE_OUTBOX_PAGE_BYTES.min(self.len - offset);
            let start = bytes.len();
            bytes.resize(start + bytes_in_page, 0);
            assert_eq!(self.copy_into(offset, &mut bytes[start..]), bytes_in_page);
        }
        GatewayToShell::decode(&bytes).expect("bridge fixed-page encode did not decode")
    }
}

impl Drop for BridgeEncodedFrame {
    fn drop(&mut self) {
        let Some((authority, grant)) = self.retirement.take() else { return };
        let mut pages = std::array::from_fn(|_| None);
        for (target, source) in pages.iter_mut().zip(self.pages.iter_mut()) {
            *target = source.take();
        }
        authority.publish_retirement(grant, pages);
    }
}

#[derive(Clone)]
pub struct BridgeEncodedLease {
    pub generation: u64,
    pub(crate) encoded: Arc<BridgeEncodedFrame>,
}

impl BridgeEncodedLease {
    pub fn len(&self) -> usize {
        self.encoded.len()
    }

    pub fn copy_into(&self, offset: usize, output: &mut [u8]) -> usize {
        self.encoded.copy_into(offset, output)
    }
}
//#endregion 🔖️BridgeHandle
