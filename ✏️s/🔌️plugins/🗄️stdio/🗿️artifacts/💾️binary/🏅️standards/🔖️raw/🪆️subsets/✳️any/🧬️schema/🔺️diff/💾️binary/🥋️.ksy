meta:
  id: stdio_binary_diff
  endian: le
doc: |
  stdio.binary diff, binary op transport (OpBinary::encode_op): serde_json bytes of the
  BinaryDiff splice list -- see diff/text grammar for the field shapes.
seq:
  - id: json-body
    type: str
    size-eos:  true
    encoding: UTF-8
