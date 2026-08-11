meta:
  id: stdio_semio_brep_snapshot
  endian: le
doc: |
  `pack` (binary) form of a `stdio.semio.brep` snapshot: the `semio_format` envelope
  (magic/header/footer, see the sibling 📡️component.protocol.semio) wrapping a `payload`
  segment whose bytes are the JSON-pack encoding of a `SemioBrepSnapshot` -- see
  ../../🔣️component.json for that payload's own field-level structure. Not an opaque blob: the
  payload's structure is defined there, not here -- this leaf's concern is only the envelope
  framing this codec adds around it.
seq:
  - id: payload
    size-eos: true
    doc: JSON-pack bytes of a SemioBrepSnapshot, see ../../🔣️component.json for its structure.
