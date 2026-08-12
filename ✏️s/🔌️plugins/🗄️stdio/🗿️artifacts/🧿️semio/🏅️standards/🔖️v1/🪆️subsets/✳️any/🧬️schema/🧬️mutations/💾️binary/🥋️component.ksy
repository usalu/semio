meta:
  id: stdio_semio_mutations
  endian: le
doc: |
  `SemioMutation` real binary frame, past the `semio_format` envelope: real `format`/`tag` bytes
  (`🧬️mutations/🦀️component.rs`'s `mutation_tag`), then one opaque `payload` tail — the wrapped
  subset's own real `OpBinary::encode_op()` bytes (or, for tag 1/`setSnapshot`, the wrapped
  snapshot's own real `ArtifactPack::encode_pack()` bytes).
seq:
  - id: format
    type: u1
  - id: tag
    type: u1
  - id: payload
    size-eos: true
