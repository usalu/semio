//! 📥️ Presence peer decoding: the peer reader and the peer, tool run and history edit decoders. See [`super`].

use super::*;

//#region 🔖️Presence
struct PresencePeerReader<'a> {
    pub(super) bytes: &'a [u8],
    pub(super) position: usize,
    pub(super) limits: PresencePeerWireLimitsV1,
}

impl<'a> PresencePeerReader<'a> {
    pub(super) fn malformed(&self, what: &'static str, detail: impl Into<String>) -> semio_framework_replication_error::ProtocolError {
        semio_framework_replication_error::ProtocolError::Malformed { what, offset: self.position as u64, detail: detail.into() }
    }

    pub(super) fn varint(&mut self, what: &'static str) -> Result<u64, semio_framework_replication_error::ProtocolError> {
        let start = self.position;
        let mut value = 0u64;
        for index in 0..10 {
            let byte = *self.bytes.get(self.position).ok_or_else(|| self.malformed(what, "truncated varint"))?;
            self.position += 1;
            if index == 9 && byte > 1 { return Err(self.malformed(what, "varint exceeds u64")); }
            value |= u64::from(byte & 0x7f) << (index * 7);
            if byte & 0x80 == 0 {
                if self.position - start > 1 && byte == 0 { return Err(self.malformed(what, "noncanonical varint")); }
                return Ok(value);
            }
        }
        Err(self.malformed(what, "overlong varint"))
    }

    pub(super) fn length(&mut self, maximum: usize, what: &'static str) -> Result<usize, semio_framework_replication_error::ProtocolError> {
        let value = self.varint(what)?;
        if value > maximum as u64 { return Err(semio_framework_replication_error::ProtocolError::LimitExceeded(what)); }
        let length = usize::try_from(value).map_err(|_| semio_framework_replication_error::ProtocolError::LimitExceeded(what))?;
        if length > self.bytes.len().saturating_sub(self.position) { return Err(self.malformed(what, "truncated field")); }
        Ok(length)
    }

    pub(super) fn count(&mut self, maximum: usize, what: &'static str) -> Result<usize, semio_framework_replication_error::ProtocolError> {
        let count = self.length(maximum, what)?;
        Ok(count)
    }

    pub(super) fn text(&mut self, what: &'static str) -> Result<String, semio_framework_replication_error::ProtocolError> {
        let length = self.length(self.limits.maximum_text_bytes, what)?;
        let end = self.position + length;
        let value = std::str::from_utf8(&self.bytes[self.position..end]).map_err(|_| self.malformed(what, "invalid utf8"))?.to_owned();
        self.position = end;
        Ok(value)
    }

    pub(super) fn blob(&mut self, what: &'static str) -> Result<Vec<u8>, semio_framework_replication_error::ProtocolError> {
        let length = self.length(self.limits.maximum_presence_pack_bytes, what)?;
        let end = self.position + length;
        let value = self.bytes[self.position..end].to_vec();
        self.position = end;
        Ok(value)
    }

    pub(super) fn byte(&mut self, what: &'static str) -> Result<u8, semio_framework_replication_error::ProtocolError> {
        let value = *self.bytes.get(self.position).ok_or_else(|| self.malformed(what, "truncated byte"))?;
        self.position += 1;
        Ok(value)
    }

    pub(super) fn principal_kind(&mut self) -> Result<PresencePrincipalKind, semio_framework_replication_error::ProtocolError> {
        let tag = self.byte("presence peer principal kind")?;
        PresencePrincipalKind::ALL.get(usize::from(tag)).copied().ok_or_else(|| self.malformed("presence peer principal kind", format!("unknown principal kind tag {tag}")))
    }

    pub(super) fn boolean(&mut self, what: &'static str) -> Result<bool, semio_framework_replication_error::ProtocolError> {
        match self.byte(what)? {
            0 => Ok(false),
            1 => Ok(true),
            _ => Err(self.malformed(what, "boolean must be zero or one")),
        }
    }

    pub(super) fn number(&mut self, what: &'static str) -> Result<f64, semio_framework_replication_error::ProtocolError> {
        let end = self.position.checked_add(8).ok_or_else(|| self.malformed(what, "length overflow"))?;
        let slice = self.bytes.get(self.position..end).ok_or_else(|| self.malformed(what, "truncated f64"))?;
        let value = f64::from_le_bytes(slice.try_into().map_err(|_| self.malformed(what, "invalid f64"))?);
        if !value.is_finite() { return Err(self.malformed(what, "non-finite f64")); }
        self.position = end;
        Ok(value)
    }

    fn triple(&mut self, what: &'static str) -> Result<[f64; 3], semio_framework_replication_error::ProtocolError> {
        Ok([self.number(what)?, self.number(what)?, self.number(what)?])
    }

    pub(super) fn strings(&mut self, what: &'static str) -> Result<Vec<String>, semio_framework_replication_error::ProtocolError> {
        let count = self.count(self.limits.maximum_domain_ids, what)?;
        let mut values = Vec::with_capacity(count);
        for _ in 0..count { values.push(self.text(what)?); }
        Ok(values)
    }

    pub(super) fn interaction(&mut self) -> Result<PresenceInteraction, semio_framework_replication_error::ProtocolError> {
        let app_id = self.text("presence interaction app id")?;
        let count = self.count(self.limits.maximum_interaction_domains, "presence interaction domains")?;
        let mut domains = Vec::with_capacity(count);
        for _ in 0..count {
            domains.push(PresenceDomain {
                domain: self.text("presence interaction domain")?,
                granularity: self.text("presence interaction granularity")?,
                selected: self.strings("presence interaction selected")?,
                hovered: self.strings("presence interaction hovered")?,
            });
        }
        Ok(PresenceInteraction { app_id, domains })
    }

    fn view_kind(&mut self) -> Result<PresenceViewKind, semio_framework_replication_error::ProtocolError> {
        match self.byte("presence view kind")? {
            0 => Ok(PresenceViewKind::Canvas { x: self.number("presence canvas x")?, y: self.number("presence canvas y")?, zoom: self.number("presence canvas zoom")? }),
            1 => Ok(PresenceViewKind::Orbit { position: self.triple("presence orbit position")?, target: self.triple("presence orbit target")?, up: self.triple("presence orbit up")?, fov: self.number("presence orbit fov")? }),
            2 => Ok(PresenceViewKind::Geo { lng: self.number("presence geo longitude")?, lat: self.number("presence geo latitude")?, zoom: self.number("presence geo zoom")?, bearing: self.number("presence geo bearing")?, pitch: self.number("presence geo pitch")? }),
            tag => Err(self.malformed("presence view kind", format!("unknown tag {tag:#x}"))),
        }
    }

    pub(super) fn views(&mut self) -> Result<Vec<PresenceWindowView>, semio_framework_replication_error::ProtocolError> {
        let count = self.count(self.limits.maximum_views, "presence views")?;
        let mut views = Vec::with_capacity(count);
        for _ in 0..count {
            let window_id = self.text("presence view window id")?;
            let space = self.text("presence view space")?;
            let kind = self.view_kind()?;
            let size = [self.number("presence view width")?, self.number("presence view height")?];
            let pointer = if self.boolean("presence view pointer")? { Some(self.triple("presence view pointer")?) } else { None };
            let ray_origin = if self.boolean("presence view ray origin")? { Some(self.triple("presence view ray origin")?) } else { None };
            views.push(PresenceWindowView { window_id, space, kind, size, pointer, ray_origin });
        }
        Ok(views)
    }

    pub(super) fn optional_text(&mut self, what: &'static str) -> Result<Option<String>, semio_framework_replication_error::ProtocolError> {
        if self.boolean(what)? { Ok(Some(self.text(what)?)) } else { Ok(None) }
    }

    fn tool_run_units(&mut self, what: &'static str) -> Result<u64, semio_framework_replication_error::ProtocolError> {
        let value = self.varint(what)?;
        if value > self.limits.maximum_tool_run_units { return Err(semio_framework_replication_error::ProtocolError::LimitExceeded(what)); }
        Ok(value)
    }

    pub(super) fn tool_run(&mut self) -> Result<PresenceToolRun, semio_framework_replication_error::ProtocolError> {
        let tool_id = self.text("presence tool run id")?;
        let tag = self.byte("presence tool run state")?;
        let state = *PresenceToolRunState::ALL.get(usize::from(tag)).ok_or_else(|| self.malformed("presence tool run state", format!("unknown tag {tag:#x}")))?;
        let stage = u16::try_from(self.varint("presence tool run stage")?).map_err(|_| semio_framework_replication_error::ProtocolError::LimitExceeded("presence tool run stage"))?;
        let completed = self.tool_run_units("presence tool run completed")?;
        let total = if self.boolean("presence tool run total")? { Some(self.tool_run_units("presence tool run total")?) } else { None };
        if total.is_some_and(|total| completed > total) { return Err(self.malformed("presence tool run completed", "completed exceeds total")); }
        Ok(PresenceToolRun { tool_id, state, stage, completed, total })
    }

    pub(super) fn history_edit(&mut self) -> Result<PresenceHistoryEdit, semio_framework_replication_error::ProtocolError> {
        let mutation_id = self.text("presence history edit mutation")?;
        let tag = self.byte("presence history edit stage")?;
        let stage = *PresenceHistoryEditStage::ALL.get(usize::from(tag)).ok_or_else(|| self.malformed("presence history edit stage", format!("unknown tag {tag:#x}")))?;
        let drafts = u32::try_from(self.varint("presence history edit drafts")?).map_err(|_| semio_framework_replication_error::ProtocolError::LimitExceeded("presence history edit drafts"))?;
        Ok(PresenceHistoryEdit { mutation_id, stage, drafts })
    }

    pub(super) fn ui(&mut self) -> Result<PresenceUi, semio_framework_replication_error::ProtocolError> {
        Ok(PresenceUi { hovered_path: self.optional_text("presence ui hovered path")?, focused_path: self.optional_text("presence ui focused path")?, pressed_path: self.optional_text("presence ui pressed path")? })
    }

    pub(super) fn typing(&mut self) -> Result<Vec<PresenceTyping>, semio_framework_replication_error::ProtocolError> {
        let count = self.count(self.limits.maximum_typing_runs, "presence typing runs")?;
        if count == 0 {
            return Err(self.malformed("presence typing runs", "an empty typing list is never flagged"));
        }
        let mut runs = Vec::with_capacity(count);
        for _ in 0..count {
            let (window_id, deleted, insert) = (self.text("presence typing window")?, self.text("presence typing deleted")?, self.text("presence typing insert")?);
            if deleted.len() > PRESENCE_TYPING_EXCERPT_BYTES || insert.len() > PRESENCE_TYPING_EXCERPT_BYTES {
                return Err(semio_framework_replication_error::ProtocolError::LimitExceeded("presence typing excerpt"));
            }
            runs.push(PresenceTyping { window_id, deleted, insert });
        }
        Ok(runs)
    }
}

/// 🎞️ Exact inverse of [`encode_presence_tool_run`] over a standalone body: the peer decoder's limits
/// (state tag, `u16` stage, exact-integer units, `completed ≤ total`) and no trailing bytes.
pub fn decode_presence_tool_run(bytes: &[u8]) -> Result<PresenceToolRun, semio_framework_replication_error::ProtocolError> {
    let limits = PRESENCE_PEER_WIRE_LIMITS_V1;
    if bytes.len() > limits.maximum_entry_bytes { return Err(semio_framework_replication_error::ProtocolError::LimitExceeded("presence tool run bytes")); }
    let mut reader = PresencePeerReader { bytes, position: 0, limits };
    let tool_run = reader.tool_run()?;
    if reader.position != bytes.len() { return Err(reader.malformed("presence tool run", "trailing bytes")); }
    Ok(tool_run)
}

/// ✏️ Exact inverse of [`encode_presence_history_edit`] over a standalone body: the peer decoder's limits (stage tag,
/// `u32` drafts) and no trailing bytes.
pub fn decode_presence_history_edit(bytes: &[u8]) -> Result<PresenceHistoryEdit, semio_framework_replication_error::ProtocolError> {
    let limits = PRESENCE_PEER_WIRE_LIMITS_V1;
    if bytes.len() > limits.maximum_entry_bytes { return Err(semio_framework_replication_error::ProtocolError::LimitExceeded("presence history edit bytes")); }
    let mut reader = PresencePeerReader { bytes, position: 0, limits };
    let history_edit = reader.history_edit()?;
    if reader.position != bytes.len() { return Err(reader.malformed("presence history edit", "trailing bytes")); }
    Ok(history_edit)
}

/// 🎯️ Exact, allocation-bounded inverse of [`encode_presence_peer`]. Unknown flags,
/// noncanonical varints, non-finite view values, hostile collection counts, and trailing bytes are
/// rejected before a peer can cross a network authority boundary.
pub async fn decode_presence_peer(bytes: &[u8]) -> Result<PresencePeer, semio_framework_replication_error::ProtocolError> {
    let limits = PRESENCE_PEER_WIRE_LIMITS_V1;
    if bytes.len() > limits.maximum_entry_bytes { return Err(semio_framework_replication_error::ProtocolError::LimitExceeded("presence peer entry bytes")); }
    let mut reader = PresencePeerReader { bytes, position: 0, limits };
    let actor = reader.text("presence peer actor")?;
    let flags = reader.varint("presence peer flags")?;
    if flags >> 15 != 0 { return Err(reader.malformed("presence peer flags", format!("unknown flag bits set: {flags:#x}"))); }
    let connected_at = reader.varint("presence peer connected at")?;
    if connected_at > limits.maximum_connected_at_ms { return Err(semio_framework_replication_error::ProtocolError::LimitExceeded("presence peer connected at")); }
    let connected_at_ms = connected_at as i64;
    let label = if flags & (1 << 0) != 0 { Some(reader.text("presence peer label")?) } else { None };
    let presence_pack = if flags & (1 << 1) != 0 { Some(reader.blob("presence peer pack")?) } else { None };
    let user_id = if flags & (1 << 2) != 0 { Some(reader.text("presence peer user id")?) } else { None };
    let role = if flags & (1 << 3) != 0 { Some(reader.text("presence peer role")?) } else { None };
    let drag_ghost_json = if flags & (1 << 4) != 0 { Some(reader.text("presence peer drag ghost")?) } else { None };
    let interaction = if flags & (1 << 5) != 0 { Some(reader.interaction()?) } else { None };
    let color = if flags & (1 << 6) != 0 { Some(reader.byte("presence peer color")?) } else { None };
    let surface = if flags & (1 << 7) != 0 { Some(reader.text("presence peer surface")?) } else { None };
    let views = if flags & (1 << 8) != 0 { reader.views()? } else { Vec::new() };
    let ui = if flags & (1 << 9) != 0 { Some(reader.ui()?) } else { None };
    let tool_run = if flags & (1 << 10) != 0 { Some(reader.tool_run()?) } else { None };
    let principal_kind = if flags & (1 << 11) != 0 { Some(reader.principal_kind()?) } else { None };
    let active_tool = if flags & (1 << 12) != 0 { Some(reader.text("presence peer active tool")?) } else { None };
    let history_edit = if flags & (1 << 13) != 0 { Some(reader.history_edit()?) } else { None };
    let typing = if flags & (1 << 14) != 0 { reader.typing()? } else { Vec::new() };
    if reader.position != bytes.len() { return Err(reader.malformed("presence peer", "trailing bytes")); }
    Ok(PresencePeer { actor, connected_at_ms, label, presence_pack, user_id, role, drag_ghost_json, interaction, color, surface, views, ui, tool_run, principal_kind, active_tool, history_edit, typing })
}
//#endregion 🔖️Presence
