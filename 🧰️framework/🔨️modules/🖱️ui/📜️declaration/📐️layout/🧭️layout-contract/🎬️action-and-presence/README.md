# Design Notes

## Preserved Source Rationale

<a id="source-notes-71"></a>

### 🦀️.rs:71

🚩️ The shared, compile-time-enforced state model every rendered UI element embeds as a
mandatory `presence` field: `state` × `status` × `hover` × `selected`. All combinations are
visually distinguishable except `state == Hidden`, which makes the rest irrelevant — see
[`UiPresence::visible`]. Defaults to fully inert (`Normal`/`Idle`/`false`/`false`) and is omitted
from the wire format entirely at default (see `UiPresence::is_default`).
👥️ One peer's mark on the element carrying this `UiPresence` — hover/selection dot plus
initials chip (contract-freeze §C7.6 of ticket `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️17/
SHARED-PRESENCE-SESSION-COLORS-AND-UNIVERSAL-ARTIFACT-CREATION`). `label` is the actor id
itself (no display name is carried this far down the stack — see `PeerPresence`'s own doc
comment in the plugin crate); a renderer that has the full roster may substitute a friendlier
name, but every renderer must always carry SOME text alongside color (never color alone).

<a id="source-notes-99"></a>

### 🦀️.rs:99

⌛️ The shared, compile-time-enforced state model every rendered UI element embeds as a
mandatory `presence` field: `state` × `status` × `hover` × `selected` × own `color` × peer
`marks`. All combinations are visually distinguishable except `state == Hidden`, which makes
the rest irrelevant — see [`UiPresence::visible`]. Defaults to fully inert
(`Normal`/`Idle`/`false`/`false`/`None`/`[]`) and is omitted from the wire format entirely at
default (see `UiPresence::is_default`). `color`/`peers` (ticket 26/08/17/SHARED-PRESENCE-
SESSION-COLORS-AND-UNIVERSAL-ARTIFACT-CREATION C7.6) make `UiPresence` `Clone`-only — no
longer `Copy` — since `peers: Vec<UiPeerMark>` owns heap data; `UiNode::presence()`/
`UiControlNode::presence()` therefore return `&UiPresence`, not a by-value copy.
