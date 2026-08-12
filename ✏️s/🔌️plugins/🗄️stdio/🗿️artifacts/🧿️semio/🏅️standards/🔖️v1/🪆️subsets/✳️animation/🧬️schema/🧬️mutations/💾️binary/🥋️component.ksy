meta:
  id: stdio_semio_animation_mutations
  endian: le
doc: |
  Real binary op frame for `stdio.semio.animation`: `format` (1 byte) + `tag` (1 byte, the
  `SemioAnimationMutation` variant ordinal — `🧬️mutations/🦀️component.rs`'s `OP_KEYWORDS`/
  `variant_ordinal`, 0-12), then one opaque `payload` tail carrying the variant's own
  `key=value,...` argument text (reuses the real, tested `print_op`/`parse_op` text codec). See the
  sibling `📡️component.protocol.semio` for the conformance-tested version.
seq:
  - id: format
    type: u1
  - id: tag
    type: u1
  - id: payload
    size-eos: true
