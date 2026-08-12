meta:
  id: semio_value_diff
  endian: le
doc: |
  REAL binary form of a `stdio.semio.value` diff, upgraded from the old `print_diff().into_bytes()`
  text-as-binary shortcut: `format u8` + `presence u8` (bit0=`root` present, bit1=`nodes`
  present) fixed header, then 0-2 recursive LEB128-varint-encoded `SemioValueDiff`/nodes-diff
  payloads back-to-back as one opaque trailing `payload` chain -- real Rust-side recursion
  (`enc_value_diff_bin`/`enc_nodes_diff_bin`), opaque only at THIS description layer (`Prim::Ref`
  self-recursion isn't describable at the protocol-dialect level, see the sibling
  `📡️component.protocol.semio`'s own doc comment). See `../📝️text/📖️component.grammar.semio` for
  the equivalent tag-prefixed text shape this payload encodes.
seq:
  - id: format
    type: u1
  - id: presence
    type: u1
    doc: "bit0 = root present, bit1 = nodes present"
  - id: payload
    size-eos: true
    doc: 0-2 back-to-back LEB128-varint-framed SemioValueDiff/nodes-diff payloads, per presence.
