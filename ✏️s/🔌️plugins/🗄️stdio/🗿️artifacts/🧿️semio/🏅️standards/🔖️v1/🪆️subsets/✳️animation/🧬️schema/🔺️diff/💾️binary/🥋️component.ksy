meta:
  id: stdio_semio_animation_diff
  endian: le
doc: |
  stdio.semio.animation.diff binary layout — see 📡️component.protocol.semio for the field grammar.
seq:
  - id: magic
    contents: "stdio.semio.animation.diff"
  - id: body
    size-eos: true
    doc: hex-encoded JSON envelope body (see the artifact's own ArtifactPack/DiffCodec impl)
