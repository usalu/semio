# Design Notes

## Preserved Source Rationale

<a id="source-notes-33"></a>

### 🦀️.rs:33

🗂️ Admits one entry at its SORTED position instead of requiring the caller to arrive in order.

⚖️ A `UiFixedMap`'s wire form is a JSON object, and object key order carries no meaning: the
wgpu bridge hands the guest's document to `JSON.stringify`, and every ECMAScript engine emits
array-index-like keys (`"10".."32"`) first in numeric order and the remaining string keys
(`"01".."09"`) after. Decoding through [`Self::try_push`] therefore refused the whole
app-static catalogue document on the wgpu renderer while React's reader — which sorts the
object's keys itself — read the same bytes. This is that reader's Rust half. One displaced
payload is carried through the tail rather than copied, so admission allocates nothing beyond
the list's own page grant. Ticket 26/09/09/PROCEDURAL-3D-END-TO-END.

<a id="source-notes-62"></a>

### 🦀️.rs:62

🩺️ Names WHY [`Self::try_insert`] refused, for a decoder that can only report a string.

⚖️ The two refusals are different defects with different owners and a bare "requires at most N
entries" hid one behind the other: a map at capacity is a producer publishing too many keys,
while a map refused below capacity is the aggregate `ui_value_headroom` page grant running out
mid-document — a whole-document budget question. The wgpu bridge surfaces this text verbatim in
`renderDocument result parse failed`, which was the only evidence available for two generation3d
examples whose preview document the shell refused (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
