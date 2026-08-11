meta:
  id: stdio_semio_model_snapshot
  endian: le
doc: |
  `pack` (binary) form of a `stdio.semio.model` snapshot: the shared `semio_format` envelope
  (magic + token-length + token, see the sibling `📡️component.protocol.semio`) wrapping a
  `payload` segment whose bytes are the compact-JSON `SemioModelSnapshot` (schema/spatial/
  elements/relations -- see `../../📝️text/🔣️component.json` for that shape). Genuinely read to
  end of stream: no length field follows the token, so `payload`'s extent really is everything
  remaining in the file, not a lazy scaffold catch-all.
seq:
  - id: magic
    contents: [0x89, 0x53, 0x45, 0x4d, 0x0d, 0x0a, 0x1a, 0x0a]
  - id: token_len
    type: u4
  - id: token
    type: str
    size: token_len
    encoding: UTF-8
  - id: payload
    size-eos:  true
    doc: Compact-JSON SemioModelSnapshot, see the snapshot text-facet grammar for its structure.
