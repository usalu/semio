meta:
  id: stdio_txt_mutation
  endian: le
doc: |
  stdio.txt mutation, binary op transport (OpBinary::encode_op): serde_json bytes of the
  tagged TxtMutation enum -- see mutations/text grammar for the field shapes per variant.
seq:
  - id: json_body
    type: str
    size-eos:  true
    encoding: UTF-8
