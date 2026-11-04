meta:
  id: stdio_wav_snapshot
  endian: le
doc: |
  Shared `.semio` binary envelope (store::semio_format::wrap_binary) around the artifact pack of `stdio.wav`;
  the pack's records are declared by the sibling 📡️.protocol.semio.
seq:
  - id: magic
    contents: [0x89, 0x53, 0x45, 0x4d, 0x0d, 0x0a, 0x1a, 0x0a]
  - id: token_len
    type: u4
  - id: token
    type: str
    size: token_len
    encoding: UTF-8
  - id: payload
    size-eos: true
    doc: The artifact pack bytes.
