meta:
  id: stdio_mp4_diff
  endian: be
doc: |
  Shared tagged-record binary protocol for the sparse logical Mp4Diff.
seq:
  - id: structured_record
    size-eos: true
    doc: RecordSpec field tags, presence bitmap, and recursively typed logical field values.
