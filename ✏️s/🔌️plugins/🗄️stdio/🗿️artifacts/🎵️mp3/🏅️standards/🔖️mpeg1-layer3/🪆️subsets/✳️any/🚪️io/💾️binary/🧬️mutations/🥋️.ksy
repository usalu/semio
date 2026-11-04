meta:
  id: mp3_mutations
  imports:
    - vlq_base128_le
doc: |
  A binary mutation op: format octet, variant tag (varint; 1 setId3v2, 2 setFrames, 3 setId3v1, 4 patchFrames, 5 setId3Frame, 6 removeId3Frame, see 📡️.protocol.semio), then the variant's wire value.
seq:
  - id: format
    type: u1
  - id: tag
    type: vlq_base128_le
  - id: payload
    size-eos: true
