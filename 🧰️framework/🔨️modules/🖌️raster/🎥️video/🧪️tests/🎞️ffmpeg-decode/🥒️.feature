@capability-avc-mp4-decode
@oracle-ffmpeg
@comparison-ordered-json-v1
Feature: Every stream the first-party H.264 encoder writes decodes, frame for frame, in FFmpeg
  The raster video tier writes H.264 by itself — every new picture an IDR of `I_PCM` macroblocks, every repeat a
  P picture of `P_Skip` macroblocks — and muxes it into a progressive MP4 of its own making. Neither half is checked
  against its own reading here: FFmpeg's `mov` demuxer and `h264` decoder (libavformat + libavcodec, the reference
  every media stack is measured against) read each stream, and `I_PCM` + `P_Skip` is lossless in the 4:2:0 domain,
  so the decoded planes must equal the BT.601 samples the encoder claims to have written byte for byte — no tolerance.

  The streams are the ones `shared://🔣️.json` pins for both twins: the TypeScript twin encodes them here and the Rust
  twin's law (`🧪️tests/🔬️unit/🦀️.rs`) pins its own bytes to the same `mp4Sha256`, so one decode covers both.

  @id-decodes-every-fixture-stream
  @level-fundamental
  @mode-differential
  Scenario: FFmpeg reads the stream facts and the exact planes the encoder claims
    Given every case of `shared://🔣️.json` encoded by the first-party encoder
    When `ffprobe` counts its frames and names its key frames, and `ffmpeg` decodes it to `yuv420p` rawvideo
    Then codec, profile, picture size, frame rate, frame count, key frames and every decoded frame's plane digest equal the encoder's own claim
