@capability-avi-1-0-mutate
@oracle-riff-avi-1-0-mutate
@comparison-semantic-avi-v1
@mutations-avi-1-0-any
Feature: Apply every typed AVI 1.0 mutation to a real-world video container

  No standalone Rust crate reads AND writes AVI credibly (investigated in full at
  `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️23/END-TO-END-TESTING-REFACTOR/📓️w7-avi-1-0-mutate-report.md`):
  `avirus` 0.2.5 never parses a typed header at all, and its "write" path copies the entire `hdrl`
  section byte-for-byte — the exact pass-through this platform forbids; `rff-format-avi` 0.1.0's own
  doc comment says its muxer is "still scaffolded"; `oxideav-avi` 0.0.9 has real AVI-specific depth
  but zero independent adoption (created four months before this ticket, 0 GitHub stars/forks,
  would pull in an equally unvalidated `oxideav-core`). AVI IS a RIFF container, so the oracle
  composes `riff` 2.0 — a mature, independent, heavily used (11.9M downloads) generic RIFF/LIST/
  chunk reader/writer, MIT — with a hand-written AVI 1.0 `hdrl`/`strl`/`movi`/`idx1` codec written
  fresh against the format's own public specification. The same composition shape `💬️bcf`'s oracle
  already established for `zip`+`quick-xml`, for the identical reason: no standalone crate for the
  specific format exists, so two independent, credible, more-generic layers are composed instead.

  The input is a real 3-second, 480x432 excerpt derived once from this repository's only real video
  (`♻️mit-bestand/🎤️präsentation/📅️33.projektetage/🌐️public/🎥️bauen-mit-bestand.mp4`, confirmed
  video-only by `ffprobe`), re-encoded to Motion-JPEG-in-AVI (the canonical AVI 1.0 video codec) and
  downscaled to keep the committed fixture small: `ffmpeg -y -i bauen-mit-bestand.mp4 -t 3 -vf
  "scale=480:-2" -r 15 -c:v mjpeg -q:v 6 -pix_fmt yuvj420p 📼️bauen-mit-bestand-mjpeg.avi` (the exact
  invocation, with the full source path, is recorded in this ticket's own
  `📓️w7-avi-1-0-mutate-report.md`).

  The result carries genuinely real structure this suite exercises directly rather than against
  stand-ins: a real `idx1` (45 entries, every one keyframe — MJPEG's frames are each independently
  decodable), a real top-level `LIST INFO` and a real top-level `JUNK` chunk (`set-idx1-present` and
  the `*-unknown-chunk` kinds act on these, not synthetic filler), and a real single `vids`/`MJPG`
  stream with 45 real chunks.

  Two honest findings this real fixture surfaces, both recorded in the oracle module's own doc
  comment rather than hidden by loosening the projection:

  - Its `strh` is 56 bytes, not 64 — `ffmpeg`'s own AVI-1.0 muxer writes the classic
    `AVISTREAMHEADER` with `rcFrame` simply omitted, a real, common, spec-legal producer behaviour.
    This oracle tolerates it (missing trailing bytes default to zero, the same tolerance every
    real-world AVI reader needs); production's `decode_avi` requires exactly 64 bytes and will
    reject this real file until fixed — a genuine pre-existing subject-side gap, not introduced
    here, in the same spirit as wave 7's TIFF/BMP findings.
  - The fixture's `strl` also carries a real `vprp` (video properties) chunk and a 4120-byte `JUNK`
    padding chunk, and its `hdrl` carries a further 260-byte `JUNK` — none of which this subset's
    schema (`AviSnapshot`) has a modelled slot for at all. Both the oracle and the subject silently
    drop that ~4.4 KB of real nested data on decode; a genuine schema-completeness gap, not a
    decode/encode bug in either implementation.

  A `movi` chunk's payload and an unknown chunk's payload project as size+digest, not raw bytes — a
  real chunk here runs into the tens of kilobytes, the same treatment the fleet brief's own raster/
  mp4 precedent gives large opaque binary payloads. Binary payloads that travel through mutation
  PARAMS (a freshly inserted chunk's bytes, `strf`'s `extra`) are lowercase hex, the same convention
  `💬️bcf`'s oracle and `AviSnapshot::parse_dsl`/`print_dsl` already use.

  Every scenario copies the real fixture into the case work directory before touching it; the
  committed file is never written to.

  @id-mutate
  @level-exhaustive
  @mode-differential
  Scenario Outline: Apply <id> to the real video container
    Given the real input document shared://📼️bauen-mit-bestand-mjpeg.avi
    When the <id> mutation is applied with its parameters
      """
      {"kind": "<id>", "params": <params>}
      """
    Then the oracle and the subject agree on the semantic projection
    Examples:
      | id                   | params                                                                                                                                                                                                                                                                                    |
      | no-mutation          | {}                                                                                                                                                                                                                                                                                        |
      | set-snapshot         | {"mainHeader": {"microSecPerFrame": 40000, "maxBytesPerSec": 1000, "paddingGranularity": 0, "flags": 16, "totalFrames": 0, "initialFrames": 0, "streams": 0, "suggestedBufferSize": 0, "width": 64, "height": 64, "reserved": [0, 0, 0, 0]}, "streams": [], "idx1Present": false, "unknownChunks": []} |
      | set-main-header      | {"mainHeader": {"microSecPerFrame": 66666, "maxBytesPerSec": 25000, "paddingGranularity": 0, "flags": 2320, "totalFrames": 45, "initialFrames": 0, "streams": 1, "suggestedBufferSize": 1048576, "width": 960, "height": 864, "reserved": [0, 0, 0, 0]}}                                |
      | set-idx1-present     | {"idx1Present": false}                                                                                                                                                                                                                                                                    |
      | insert-stream        | {"index": 1, "stream": {"strh": {"fccType": "vids", "fccHandler": "MJPG", "flags": 0, "priority": 0, "language": 0, "initialFrames": 0, "scale": 1, "rate": 15, "start": 0, "length": 0, "suggestedBufferSize": 0, "quality": -1, "sampleSize": 0, "rcFrameLeft": 0, "rcFrameTop": 0, "rcFrameRight": 0, "rcFrameBottom": 0}, "strf": {"format": "bitmapInfo", "size": 40, "width": 480, "height": 432, "planes": 1, "bitCount": 24, "compression": "MJPG", "sizeImage": 0, "xPelsPerMeter": 0, "yPelsPerMeter": 0, "colorsUsed": 0, "colorsImportant": 0}, "chunks": []}} |
      | remove-stream        | {"index": 0}                                                                                                                                                                                                                                                                              |
      | set-stream-header    | {"streamIndex": 0, "strh": {"fccType": "vids", "fccHandler": "MJPG", "flags": 0, "priority": 100, "language": 0, "initialFrames": 0, "scale": 1, "rate": 30, "start": 0, "length": 45, "suggestedBufferSize": 21828, "quality": -1, "sampleSize": 0, "rcFrameLeft": 0, "rcFrameTop": 0, "rcFrameRight": 480, "rcFrameBottom": 432}} |
      | set-stream-format    | {"streamIndex": 0, "strf": {"format": "raw", "data": "deadbeef"}}                                                                                                                                                                                                                        |
      | insert-chunk         | {"streamIndex": 0, "index": 1, "chunk": {"fourcc": "00dc", "data": "ffd8ffe0", "keyframe": false}}                                                                                                                                                                                       |
      | remove-chunk         | {"streamIndex": 0, "index": 0}                                                                                                                                                                                                                                                            |
      | set-chunk-keyframe   | {"streamIndex": 0, "index": 0, "keyframe": false}                                                                                                                                                                                                                                         |
      | add-unknown-chunk    | {"index": 2, "item": {"fourcc": "XTRA", "data": "cafef00d"}}                                                                                                                                                                                                                              |
      | remove-unknown-chunk | {"index": 1}                                                                                                                                                                                                                                                                              |

  @id-inverse
  @level-exhaustive
  @mode-property
  Scenario Outline: Undoing <id> restores the real video container
    Given the real input document shared://📼️bauen-mit-bestand-mjpeg.avi
    When the <id> mutation is applied and then undone
      """
      {"kind": "<id>", "params": <params>}
      """
    Then the oracle and the subject agree on the semantic projection
    Examples:
      | id                   | params                                                                                                                                                                                                                                                                                    |
      | no-mutation          | {}                                                                                                                                                                                                                                                                                        |
      | set-snapshot         | {"mainHeader": {"microSecPerFrame": 40000, "maxBytesPerSec": 1000, "paddingGranularity": 0, "flags": 16, "totalFrames": 0, "initialFrames": 0, "streams": 0, "suggestedBufferSize": 0, "width": 64, "height": 64, "reserved": [0, 0, 0, 0]}, "streams": [], "idx1Present": false, "unknownChunks": []} |
      | set-main-header      | {"mainHeader": {"microSecPerFrame": 66666, "maxBytesPerSec": 25000, "paddingGranularity": 0, "flags": 2320, "totalFrames": 45, "initialFrames": 0, "streams": 1, "suggestedBufferSize": 1048576, "width": 960, "height": 864, "reserved": [0, 0, 0, 0]}}                                |
      | set-idx1-present     | {"idx1Present": false}                                                                                                                                                                                                                                                                    |
      | insert-stream        | {"index": 1, "stream": {"strh": {"fccType": "vids", "fccHandler": "MJPG", "flags": 0, "priority": 0, "language": 0, "initialFrames": 0, "scale": 1, "rate": 15, "start": 0, "length": 0, "suggestedBufferSize": 0, "quality": -1, "sampleSize": 0, "rcFrameLeft": 0, "rcFrameTop": 0, "rcFrameRight": 0, "rcFrameBottom": 0}, "strf": {"format": "bitmapInfo", "size": 40, "width": 480, "height": 432, "planes": 1, "bitCount": 24, "compression": "MJPG", "sizeImage": 0, "xPelsPerMeter": 0, "yPelsPerMeter": 0, "colorsUsed": 0, "colorsImportant": 0}, "chunks": []}} |
      | remove-stream        | {"index": 0}                                                                                                                                                                                                                                                                              |
      | set-stream-header    | {"streamIndex": 0, "strh": {"fccType": "vids", "fccHandler": "MJPG", "flags": 0, "priority": 100, "language": 0, "initialFrames": 0, "scale": 1, "rate": 30, "start": 0, "length": 45, "suggestedBufferSize": 21828, "quality": -1, "sampleSize": 0, "rcFrameLeft": 0, "rcFrameTop": 0, "rcFrameRight": 480, "rcFrameBottom": 432}} |
      | set-stream-format    | {"streamIndex": 0, "strf": {"format": "raw", "data": "deadbeef"}}                                                                                                                                                                                                                        |
      | insert-chunk         | {"streamIndex": 0, "index": 1, "chunk": {"fourcc": "00dc", "data": "ffd8ffe0", "keyframe": false}}                                                                                                                                                                                       |
      | remove-chunk         | {"streamIndex": 0, "index": 0}                                                                                                                                                                                                                                                            |
      | set-chunk-keyframe   | {"streamIndex": 0, "index": 0, "keyframe": false}                                                                                                                                                                                                                                         |
      | add-unknown-chunk    | {"index": 2, "item": {"fourcc": "XTRA", "data": "cafef00d"}}                                                                                                                                                                                                                              |
      | remove-unknown-chunk | {"index": 1}                                                                                                                                                                                                                                                                              |

  @id-identity-round-trip
  @level-long
  @mode-round-trip
  Scenario: Decode and re-encode the real video container without passing bytes through
    Given the real input document shared://📼️bauen-mit-bestand-mjpeg.avi
    When the document is fully parsed into the subset's own snapshot model and re-encoded from it alone
    Then the oracle and the subject agree on the semantic projection
    And the re-encoded bytes are not bit-identical to the input
