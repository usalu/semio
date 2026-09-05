meta:
  id: stdio_semio_flow_mutations
  endian: le
doc: |
  Real binary `SemioFlowMutation` op frame: `format` (1 byte) + `tag` (1 byte, the variant
  ordinal — see `🧬️mutations/🦀️.rs`'s `OP_KEYWORDS`), then an opaque `payload` tail (the
  variant's own `key=value ...` argument text, UTF-8) — see the sibling
  `📡️.protocol.semio`'s comment.
seq:
  - id: format
    type: u1
  - id: tag
    type: u1
  - id: payload
    size-eos: true
