meta:
  id: stdio_binary_mutation
  endian: le
doc: |
  stdio.binary mutation, binary op transport (OpBinary::encode_op): serde_json bytes of the
  tagged BinaryMutation enum -- see mutations/text grammar for field shapes per variant.
seq:
  - id: json-body
    type: str
    size-eos:  true
    encoding: UTF-8
