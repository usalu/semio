meta:
  id: stdio_semio_animation_snapshot
  endian: le
doc: |
  stdio.semio.animation.snapshot binary layout — see 📡️component.protocol.semio for the field grammar.
seq:
  - id: magic
    contents: "stdio.semio.animation.snapshot"
  - id: body
    size-eos: true
    doc: hex-encoded JSON envelope body (see the artifact's own ArtifactPack/DiffCodec impl)
