meta:
  id: stdio_json_snapshot
  endian: le
doc: |
  `pack` (binary) form of a `stdio.json` snapshot: the `semio_format` envelope (magic/header/
  footer, see the sibling `📡️component.protocol.semio`) wrapping a `payload` segment whose bytes
  are the RFC8259 JSON document in COMPACT (no extraneous whitespace) form -- i.e. the exact same
  grammar as `../📝️text/📖️component.grammar.semio`'s `json-text` production, UTF-8 encoded. Not an
  opaque blob: the payload's own internal structure is defined there, not here -- this leaf's
  concern is only the envelope framing this codec adds around it.
seq:
  - id: payload
    size-eos: true
    doc: UTF-8 RFC8259 JSON text, see the text-facet grammar for its structure.
