//! 🕹️ Presence interaction: the interaction and domain records with their value conversions, the vector and domain codecs and the interaction assembler. See [`super`].

use super::*;

//#region 🔖️PresenceInteraction
/// 🖱️ One peer's interaction roster for one app instance, mirrored onto `PresencePeer.interaction`
/// (bit 5) on the heartbeat — typed (not app-opaque `presence_pack`) so the Shell renders every peer's
/// selection/hover generically. Only explicit ids broadcast; receivers expand transitive closures via
/// their own topology.
#[derive(Clone, Debug, Default, PartialEq, semio_framework_value::RetireOwned)]
pub struct PresenceInteraction {
    pub app_id: String,
    pub domains: Vec<PresenceDomain>,
}

/// 🌱️ Hand-written, not derived — same DAG reason as `SelectionMode` above. This crate's own
/// hand-rolled binary codec (`🔖️PresenceInteractionCodec` below) is the real wire path; this impl
/// exists for capability parity with the type's former `Serialize`/`Deserialize`.
impl semio_framework_value::ToValue for PresenceInteraction {
    fn to_value(&self) -> semio_framework_value::DslValue {
        semio_framework_value::DslValue::object(vec![("appId".to_string(), semio_framework_value::ToValue::to_value(&self.app_id)), ("domains".to_string(), semio_framework_value::ToValue::to_value(&self.domains))])
    }
}

impl semio_framework_value::FromValue for PresenceInteraction {
    fn from_value(value: semio_framework_value::DslValue) -> Result<Self, semio_framework_value::ValueError> {
        let semio_framework_value::DslValue::Object(fields) = value else {
            return Err(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("expected an object for PresenceInteraction, found {value:?}")));
        };
        let mut out = PresenceInteraction::default();
        for (key, entry) in fields {
            match key.as_str() {
                "appId" => out.app_id = <String as semio_framework_value::FromValue>::from_value(entry).map_err(|e| e.under("appId"))?,
                "domains" => out.domains = <Vec<PresenceDomain> as semio_framework_value::FromValue>::from_value(entry).map_err(|e| e.under("domains"))?,
                _ => {}
            }
        }
        Ok(out)
    }
}

/// 📡️ One domain's broadcast slice of `PresenceInteraction` — the peer-facing mirror of a domain's
/// `DomainSelection`/`DomainHover`, flattened to raw explicit ids (no transitive expansion on the wire).
#[derive(Clone, Debug, Default, PartialEq, semio_framework_value::RetireOwned)]
pub struct PresenceDomain {
    pub domain: String,
    pub granularity: String,
    pub selected: Vec<String>,
    pub hovered: Vec<String>,
}

/// 🧰️ Hand-written, not derived — same DAG reason as `SelectionMode` above.
impl semio_framework_value::ToValue for PresenceDomain {
    fn to_value(&self) -> semio_framework_value::DslValue {
        semio_framework_value::DslValue::object(vec![
            ("domain".to_string(), semio_framework_value::ToValue::to_value(&self.domain)),
            ("granularity".to_string(), semio_framework_value::ToValue::to_value(&self.granularity)),
            ("selected".to_string(), semio_framework_value::ToValue::to_value(&self.selected)),
            ("hovered".to_string(), semio_framework_value::ToValue::to_value(&self.hovered)),
        ])
    }
}

impl semio_framework_value::FromValue for PresenceDomain {
    fn from_value(value: semio_framework_value::DslValue) -> Result<Self, semio_framework_value::ValueError> {
        let semio_framework_value::DslValue::Object(fields) = value else {
            return Err(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("expected an object for PresenceDomain, found {value:?}")));
        };
        let mut out = PresenceDomain::default();
        for (key, entry) in fields {
            match key.as_str() {
                "domain" => out.domain = <String as semio_framework_value::FromValue>::from_value(entry).map_err(|e| e.under("domain"))?,
                "granularity" => out.granularity = <String as semio_framework_value::FromValue>::from_value(entry).map_err(|e| e.under("granularity"))?,
                "selected" => out.selected = <Vec<String> as semio_framework_value::FromValue>::from_value(entry).map_err(|e| e.under("selected"))?,
                "hovered" => out.hovered = <Vec<String> as semio_framework_value::FromValue>::from_value(entry).map_err(|e| e.under("hovered"))?,
                _ => {}
            }
        }
        Ok(out)
    }
}

//#region 🔖️PresenceInteractionCodec
// 🎯️ Binary codec for the two structs above — what `encode_presence_peer`/`decode_presence_peer`
// (in the `🔖️Presence` region) call for bit 5. Self-delimiting throughout (varint counts, exactly
// `write_vec_bytes`/`write_vec_envelope`'s own convention in `🔖️Combinators` up top), so no outer
// length prefix is needed around the whole payload.
async fn write_vec_str(out: &mut Vec<u8>, values: &[String]) {
    semio_framework_replication_wire_core::write_varint_u64(out, values.len() as u64);
    for value in values {
        semio_framework_replication_wire_core::write_str(out, value);
    }
}

async fn read_vec_str(bytes: &[u8], pos: &mut usize) -> Result<Vec<String>, semio_framework_replication_error::ProtocolError> {
    let count = semio_framework_replication_wire_core::read_varint_u64(bytes, pos)?;
    let mut out = Vec::with_capacity(count as usize);
    for _ in 0..count {
        out.push(semio_framework_replication_wire_core::read_str(bytes, pos)?);
    }
    Ok(out)
}

async fn encode_presence_domain(domain: &PresenceDomain, out: &mut Vec<u8>) {
    semio_framework_replication_wire_core::write_str(out, &domain.domain);
    semio_framework_replication_wire_core::write_str(out, &domain.granularity);
    write_vec_str(out, &domain.selected).await;
    write_vec_str(out, &domain.hovered).await;
}

async fn decode_presence_domain(bytes: &[u8], pos: &mut usize) -> Result<PresenceDomain, semio_framework_replication_error::ProtocolError> {
    Ok(PresenceDomain { domain: semio_framework_replication_wire_core::read_str(bytes, pos)?, granularity: semio_framework_replication_wire_core::read_str(bytes, pos)?, selected: read_vec_str(bytes, pos).await?, hovered: read_vec_str(bytes, pos).await? })
}

/// 🎯️ Encodes one `PresenceInteraction` — `pub` (ticket 26/08/17/SHARED-PRESENCE-SESSION-
/// COLORS-AND-UNIVERSAL-ARTIFACT-CREATION C7.4): guests never enable the kernel's `sync` feature, and
/// `VcsArtifactApp` (the plugin ABI's presence adoption path) must be able to call this directly.
pub async fn encode_presence_interaction(interaction: &PresenceInteraction, out: &mut Vec<u8>) {
    semio_framework_replication_wire_core::write_str(out, &interaction.app_id);
    semio_framework_replication_wire_core::write_varint_u64(out, interaction.domains.len() as u64);
    for domain in &interaction.domains {
        encode_presence_domain(domain, out).await;
    }
}

/// ↩️ Inverse of [`encode_presence_interaction`] — see its doc for why this is `pub`.
pub async fn decode_presence_interaction(bytes: &[u8], pos: &mut usize) -> Result<PresenceInteraction, semio_framework_replication_error::ProtocolError> {
    let app_id = semio_framework_replication_wire_core::read_str(bytes, pos)?;
    let count = semio_framework_replication_wire_core::read_varint_u64(bytes, pos)?;
    let mut domains = Vec::with_capacity(count as usize);
    for _ in 0..count {
        domains.push(decode_presence_domain(bytes, pos).await?);
    }
    Ok(PresenceInteraction { app_id, domains })
}
//#endregion 🔖️PresenceInteractionCodec

//#region 🔖️Assemble
/// 🏗️ Assembles `PresencePeer.interaction` from local `InteractionState` plus each domain's
/// declared hover/selection behavior — the ONE place this logic lives, so every app broadcasts
/// selection+hover with ZERO app-side code (call this wherever a `PresenceHeartbeat`'s `peer` is
/// built, right before `presence_to_bytes`/`encode_presence_peer`, at the same cadence cursor updates
/// already get throttled at — this fn is pure/stateless, so it invents no throttle of its own).
/// Forwards each domain's already-computed `ids` verbatim (never re-derives a closure itself) —
/// EXPLICIT ids only cross the wire; a receiver re-expands any transitive closure via its own
/// `DomainTopology`. Only the `"pointer"` hover channel ever broadcasts (any other channel, e.g. a
/// drag-only one, stays local); a domain whose `HoverSpec::broadcast`/`SelectionSpec::broadcast` is
/// `false` contributes nothing for that half. A domain that ends up with both halves empty is omitted
/// from `domains` entirely.
///
/// 🎯️ Moved here (ticket 26/08/17/SHARED-PRESENCE-SESSION-COLORS-AND-UNIVERSAL-ARTIFACT-CREATION
/// C7.4) from `store_sync` — guests never enable the kernel's `sync` feature, and `VcsArtifactApp`
/// (the plugin ABI's presence-adoption path, §C7.6) must be able to call this pure fn without pulling
/// in the whole actor layer.
pub async fn assemble_presence_interaction(app_id: &str, state: &InteractionState, hover_specs: &BTreeMap<String, HoverSpec>, selection_specs: &BTreeMap<String, SelectionSpec>) -> PresenceInteraction {
    let mut domain_ids: std::collections::BTreeSet<&String> = std::collections::BTreeSet::new();
    domain_ids.extend(state.selection.keys());
    domain_ids.extend(state.hover.keys());

    let mut domains = Vec::new();
    for domain_id in domain_ids {
        let selected = if selection_specs.get(domain_id).is_some_and(|spec| spec.broadcast) { state.selection.get(domain_id).map(|selection| selection.ids.clone()).unwrap_or_default() } else { Vec::new() };
        let hovered = if hover_specs.get(domain_id).is_some_and(|spec| spec.broadcast) { state.hover.get(domain_id).filter(|hover| hover.channel == "pointer").map(|hover| hover.ids.clone()).unwrap_or_default() } else { Vec::new() };
        if selected.is_empty() && hovered.is_empty() {
            continue;
        }
        let granularity = state.active_granularity.get(domain_id).cloned().unwrap_or_default();
        domains.push(PresenceDomain { domain: domain_id.clone(), granularity, selected, hovered });
    }

    PresenceInteraction { app_id: app_id.to_string(), domains }
}
//#endregion 🔖️Assemble
//#endregion 🔖️PresenceInteraction
