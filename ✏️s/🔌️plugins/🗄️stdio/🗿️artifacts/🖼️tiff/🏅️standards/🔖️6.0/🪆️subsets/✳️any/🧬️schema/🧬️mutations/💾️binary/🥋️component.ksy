meta:
  id: stdio_tiff_mutation
  endian: le
doc: |
  protocol::OpBinary raw JSON encoding of TiffMutation (`#[serde(tag = "mutation")]`) — no
  `.semio` envelope header (contrast with ../../📸️snapshot/💾️binary/, whose payload IS
  wrapped). No length prefix: the whole op body IS the JSON document.
seq:
  - id: json_bytes
    type: u1
    repeat: eos
    doc: UTF-8 JSON object bytes; the `"mutation"` tag selects one of 8 real variants (see
      sibling ../📝️text/📖️component.grammar.semio for the field-level shape).
