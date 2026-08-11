meta:
  id: semio_document_diff
  endian: be
doc: |
  Binary diff form = the text diff's UTF-8 bytes verbatim (protocol::DiffCodec::encode_diff wraps
  print_diff() directly; no separate binary framing).
seq:
  - id: text_diff_utf8
    size-eos: true
    doc: The exact bytes of print_diff()'s space-separated key=value output, UTF-8 encoded.
