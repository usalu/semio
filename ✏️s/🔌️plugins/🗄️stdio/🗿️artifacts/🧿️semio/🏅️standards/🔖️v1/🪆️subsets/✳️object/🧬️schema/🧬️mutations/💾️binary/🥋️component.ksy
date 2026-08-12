meta:
  id: semio_object_mutations
  endian: le
doc: |
  REAL binary form of a `stdio.semio.object` mutation, upgraded from the old
  `print_op().into_bytes()` text-as-binary shortcut: `format u8` + `tag u8` (the
  `SemioObjectMutation` variant ordinal, 0-8, same order as the text facet's keyword list) fixed
  header, then the variant's own recursive path/key/value/id payload as one opaque trailing
  `payload` chain -- real LEB128-varint-framed Rust-side encoding (`enc_semio_value_bin`/
  `enc_semio_path_bin`/`enc_semio_object_snapshot_bin`), opaque only at THIS description layer
  (`Prim::Ref` self-recursion isn't describable at the protocol-dialect level, see the sibling
  `📡️component.protocol.semio`'s own doc comment). See `../📝️text/📖️component.grammar.semio` for
  the equivalent tag-prefixed text shape this payload encodes.
seq:
  - id: format
    type: u1
  - id: tag
    type: u1
    doc: >
      0=no-mutation 1=set-snapshot 2=set-value 3=set-map-entry 4=remove-map-entry
      5=insert-list-item 6=remove-list-item 7=set-object 8=remove-object
  - id: payload
    size-eos: true
    doc: variant-specific LEB128-varint-framed path/key/value/id payload, per tag.
