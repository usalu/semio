meta:
  id: stdio_pdf_1_7_snapshot
  endian: le
doc: |
  Semio Pack payload for the logical PDF snapshot. Native PDF framing is materialized only by
  the native deserializer/serializer and is not retained here.
seq:
  - id: logical_snapshot
    size-eos: true
