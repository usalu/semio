meta:
  id: semio_value_snapshot
  endian: le
doc: |
  `pack` (binary) form of a `stdio.semio.value` snapshot: the `semio_format` envelope (magic/
  header/footer, described once at the framework level, not re-inlined here) wraps a `payload`
  segment whose bytes are this facet's OWN real recursive tag-prefixed text encoding of the WHOLE
  `SemioValueSnapshot` (`[hex(schema),<value>,[<node>,...]]`), UTF-8 encoded -- text-native like
  the sibling `json` artifact's own snapshot facet, NOT hex-of-JSON, NOT `serde_json`. Not an
  opaque blob: the payload's own internal structure (the `SemioValue` tagged-union recursion) is
  defined at `../📝️text/📖️component.grammar.semio`, not here.
seq:
  - id: payload
    size-eos: true
    doc: UTF-8 tag-prefixed SemioValueSnapshot text, see the text-facet grammar for its structure.
