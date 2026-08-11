meta:
  id: stdio_semio_object_snapshot
  endian: le
doc: |
  `pack` (binary) form of a `stdio.semio.object` snapshot: the `semio_format` envelope wrapping a
  `payload` segment whose bytes are a COMPACT `serde_json` serialization of the whole
  `SemioObjectSnapshot` struct (`schema`/`root`/`objects`), UTF-8 encoded -- the exact same JSON
  shape as `../📝️text/📖️component.grammar.semio`'s `snapshot-json` production. Not an opaque blob:
  the payload's own internal structure is defined there, not here -- this leaf's concern is only
  the envelope framing this codec adds around it (same convention `json`'s own binary leaf uses).
seq:
  - id: payload
    size-eos: true
    doc: UTF-8 compact-JSON `SemioObjectSnapshot`, see the text-facet grammar for its structure.
