//! 🚧️ Rust projection of `HubRefusalV1` ([`🔣️.json`](🧬️schema/🔣️.json)): every hub answer that is not a
//! success carries a named code in `x-semio-refusal`. The hub's router applies [`refusal_code`] to every
//! response it sends, so no route can answer an untyped status; a bare refusal stays body-free, so a
//! refusal never discloses more than its status and code.

/// 🧬️ The draft-07 module this projection is checked against.
pub const HUB_REFUSAL_SCHEMA_JSON: &str = include_str!("🧬️schema/🔣️.json");

/// 🏷️ The `schema` of one `HubRefusalV1` record.
pub const HUB_REFUSAL_SCHEMA: &str = "semio.hub.refusal/v1";

/// 🪪️ The response header naming a hub refusal.
pub const HUB_REFUSAL_HEADER: &str = "x-semio-refusal";

/// 🗺️ `HubRefusalStatusCodesV1`: the code a refusal carries when its route names none.
pub const HUB_REFUSAL_STATUS_CODES: [(u16, &str); 23] = [
    (400, "malformed-request"),
    (401, "unauthenticated"),
    (403, "forbidden"),
    (404, "not-found"),
    (405, "method-not-allowed"),
    (406, "not-acceptable"),
    (408, "request-timeout"),
    (409, "conflict"),
    (410, "gone"),
    (412, "precondition-failed"),
    (413, "payload-too-large"),
    (414, "uri-too-long"),
    (415, "unsupported-media-type"),
    (416, "range-not-satisfiable"),
    (422, "unprocessable"),
    (426, "upgrade-required"),
    (428, "precondition-required"),
    (429, "rate-limited"),
    (431, "headers-too-large"),
    (500, "internal"),
    (501, "not-implemented"),
    (503, "unavailable"),
    (504, "timeout"),
];

/// 🏷️ The code a refusal with `status` carries when its route names none; `None` for a success.
pub fn refusal_code(status: u16) -> Option<&'static str> {
    if !(400..=599).contains(&status) {
        return None;
    }
    Some(HUB_REFUSAL_STATUS_CODES.iter().find(|(known, _)| *known == status).map_or("refused", |(_, code)| code))
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
