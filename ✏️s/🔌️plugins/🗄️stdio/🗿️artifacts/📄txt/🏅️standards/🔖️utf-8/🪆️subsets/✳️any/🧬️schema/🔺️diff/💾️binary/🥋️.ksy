meta:
  id: stdio_txt_diff
  endian: le
doc: |
  stdio.txt diff, binary op transport (OpBinary::encode_op): serde_json bytes of the TxtDiff
  struct -- the wire body is real JSON text, not an opaque payload.
seq:
  - id: json_body
    type: str
    size-eos:  true
    encoding: UTF-8
    doc: JSON-encoded TxtDiff (trailingNewline?/lineEnding?/lines?), see snapshot/text grammar
      for the field shapes this mirrors.
