meta:
  id: stdio_txt_mutation
  endian: le
doc: |
  stdio.txt mutation, binary op transport: direct-leaf payload bytes selected by a one-byte tag.
seq:
  - id: json_body
    type: str
    size-eos:  true
    encoding: UTF-8
