meta:
  id: stdio_semio_animation_diff
  endian: le
doc: |
  Real binary diff frame for `stdio.semio.animation`: `format` (1 byte) + `presence` (1 byte,
  bit0=`timelines` — the only collection this facet has) then, when `presence` bit0 is set, one
  opaque `payload` tail carrying the SAME `enc_indexed_triple`-produced text this facet's own
  `print_diff` already emits (`[removed];[modified];[added]`, recipe §1.4's index-keyed collection-
  triple shape). See the sibling `📡️.protocol.semio` for the conformance-tested version.
seq:
  - id: format
    type: u1
  - id: presence
    type: u1
  - id: payload
    size-eos: true
