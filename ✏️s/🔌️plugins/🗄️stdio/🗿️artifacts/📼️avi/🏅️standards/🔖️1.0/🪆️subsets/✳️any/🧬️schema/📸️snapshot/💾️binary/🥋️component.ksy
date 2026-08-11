meta:
  id: stdio_avi_snapshot
  endian: le
doc: |
  Shared `.semio` binary envelope (store::semio_format::wrap_binary, big-endian magic per that
  codec) wrapping a `stdio.avi` payload: the REAL little-endian RIFF/AVI 1.0 file bytes
  `crate::artifacts::avi::standards::v1_0::engine::{decode_avi,encode_avi}` produce/consume.
seq:
  - id: envelope_magic
    endian: be
    contents: [0x89, 0x53, 0x45, 0x4d, 0x0d, 0x0a, 0x1a, 0x0a]
  - id: token_len
    type: u4
  - id: token
    type: str
    size: token_len
    encoding: UTF-8
    doc: "stdio.avi.pack v1"
  - id: riff
    type: riff_chunk
    doc: The real RIFF('AVI ' hdrl movi idx1) container.
types:
  riff_chunk:
    seq:
      - id: fourcc
        type: str
        size: 4
        encoding: ASCII
      - id: chunk_size
        type: u4
      - id: body
        size: chunk_size
      - id: pad
        size: chunk_size % 2
  main_avi_header:
    doc: "MainAVIHeader — 14 DWORDs, 56 bytes."
    seq:
      - id: micro_sec_per_frame
        type: u4
      - id: max_bytes_per_sec
        type: u4
      - id: padding_granularity
        type: u4
      - id: flags
        type: u4
      - id: total_frames
        type: u4
      - id: initial_frames
        type: u4
      - id: streams
        type: u4
      - id: suggested_buffer_size
        type: u4
      - id: width
        type: u4
      - id: height
        type: u4
      - id: reserved
        type: u4
        repeat: expr
        repeat-expr: 4
  avi_stream_header:
    doc: "AVIStreamHeader — 64 bytes; rcFrame is 4 LONGs (not SHORTs)."
    seq:
      - id: fcc_type
        type: str
        size: 4
        encoding: ASCII
      - id: fcc_handler
        type: str
        size: 4
        encoding: ASCII
      - id: flags
        type: u4
      - id: priority
        type: u2
      - id: language
        type: u2
      - id: initial_frames
        type: u4
      - id: scale
        type: u4
      - id: rate
        type: u4
      - id: start
        type: u4
      - id: length
        type: u4
      - id: suggested_buffer_size
        type: u4
      - id: quality
        type: s4
      - id: sample_size
        type: u4
      - id: rc_frame_left
        type: s4
      - id: rc_frame_top
        type: s4
      - id: rc_frame_right
        type: s4
      - id: rc_frame_bottom
        type: s4
  bitmap_info_header:
    doc: "BITMAPINFOHEADER — 40 bytes."
    seq:
      - id: size
        type: u4
      - id: width
        type: s4
      - id: height
        type: s4
      - id: planes
        type: u2
      - id: bit_count
        type: u2
      - id: compression
        type: str
        size: 4
        encoding: ASCII
      - id: size_image
        type: u4
      - id: x_pels_per_meter
        type: s4
      - id: y_pels_per_meter
        type: s4
      - id: colors_used
        type: u4
      - id: colors_important
        type: u4
