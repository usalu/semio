meta:
  id: wav_mutations
  imports:
    - vlq_base128_le
doc: |
  A binary mutation op: format octet, variant tag (varint; 1 setFmt, 2 setData, 3 setOtherChunks, 4 patchData, 5 setPadBytes), then the variant's wire value.
seq:
  - id: format
    type: u1
  - id: tag
    type: vlq_base128_le
  - id: payload
    size-eos: true
