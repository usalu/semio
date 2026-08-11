meta:
  id: semio_document_mutation
  endian: be
doc: |
  Binary op form = the text op's UTF-8 bytes verbatim (protocol::OpBinary::encode_op wraps
  print_op() directly; no separate binary framing).
seq:
  - id: text_op_utf8
    size-eos: true
    doc: The exact bytes of print_op()'s `keyword arg=value ...` output, UTF-8 encoded.
