# Design Notes

## Preserved Source Rationale

<a id="source-notes-158"></a>

### 🦀️.rs:158

🗂️ Admits one entry at its SORTED position instead of requiring the caller to arrive in order.

⚖️ A `UiMap`'s wire form is a JSON object, and object key order carries no meaning: the wgpu
bridge hands the guest's document to `JSON.stringify`, and every ECMAScript engine emits
array-index-like keys first in numeric order and the remaining string keys after. Decoding
through [`Self::push`] therefore refused whole documents on the wgpu renderer while React's
reader — which sorts the object's keys itself — read the same bytes. One displaced page is
spliced into the arena-backed chain rather than buffered into a vector.
