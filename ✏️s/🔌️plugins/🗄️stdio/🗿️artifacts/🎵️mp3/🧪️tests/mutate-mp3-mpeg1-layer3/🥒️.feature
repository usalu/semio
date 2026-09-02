@capability-mp3-mpeg1-layer3-mutate
@oracle-id3-mpeg1-layer3-mutate
@comparison-semantic-mp3-mpeg1-layer3-v1
@mutations-mp3-mpeg1-layer3-any
Feature: Apply every typed MP3 mpeg1-layer3 mutation to a real encoded stream
  The input is shared://🔊️.mp3 — 193,275 bytes of genuinely encoded
  MPEG-1 Layer III audio: a 179-byte ID3v2.3.0 region and 462 real frames at 128 kbps / 44.1 kHz
  mono, with no ID3v1 trailer. Every scenario copies it into the case work directory before touching
  it; the committed file is never written to.

  Its provenance, because "real" is a claim and not an adjective. This repository's only real
  recorded media is `♻️mit-bestand/🎤️präsentation/📅️33.projektetage/🌐️public/🎥️bauen-mit-bestand.mp4`
  and `ffprobe` confirms it carries NO audio stream — the sibling 🔊️wav case established that
  already, and answered it by using real measured data from that same real camera-captured footage
  rather than shipping a synthetic tone. This fixture does the same thing at the one sample rate
  MPEG-1 admits: 12 s of the real video decoded to 8-bit grayscale at 25 fps and 42×42 gives
  25·42·42 = 44,100 real per-pixel light-intensity measurements per second, which IS 44.1 kHz, so
  nothing is resampled — the capture rate is the sample rate. Each luma byte is centred
  (`(byte-128)·256`) into a signed 16-bit sample, written as canonical mono PCM with Python's
  standard-library `wave` module, and encoded by `lame` — a real third-party MPEG-1 Layer III
  encoder, not this repository's code. The exact script is committed in this ticket's
  `mp3-fixture-derive/🐍️derive-real-mp3-fixture.py`.

  What that buys, concretely, over the artifact's own 1,725-byte committed demo example this case
  previously read. That file is four frame headers over digital silence, all 417 bytes; here
  128000/44100 is not an integer, so a real CBR encoder MUST alternate the padding slot to hold the
  average rate, and BOTH values genuinely occur — 20 frames of 417 bytes and 442 of 418. The
  `144·bitrate/rate + pad` frame-size formula is therefore exercised on both of its branches instead
  of only one. The tag is real too: LAME wrote TSSE (its own encoder signature), TIT2 and TPE1 in
  encoding `1` — UTF-16 with a byte-order mark, which is what a real-world writer emits and which
  the previous ISO-8859-1-throughout fixture never exercised — and TLEN.

  An `.mp3` file is two independent layers stacked in one byte stream and no crate is authoritative
  over both, so the oracle is a composition — the same shape 📼️avi (riff + a hand-written
  hdrl/strl/movi codec) and 💬️bcf (zip + quick-xml) already use. `id3` 1.17 (MIT) owns the ID3
  layer: `Tag::skip` independently locates where the ID3v2 region ends (the same boundary this
  subset's own `decode_mp3` must find for itself), `Tag::read_from2` parses it and `Tag::write_to`
  re-serializes it from the crate's own frame model alone. The MPEG frame layer is walked from
  ISO/IEC 11172-3 directly in this subset's oracle module — the 11-bit sync word, the
  version/layer/bitrate/sample-rate fields and the Layer I versus Layer II/III frame-size formulae —
  and never calls the subject's `find_frame_sync`/`parse_frame_header`. `id3::v1::Tag` is
  READ-ONLY, so `set-id3v1` writes the 128-byte trailer from the ID3v1 field layout directly (fixed
  offsets, fixed widths, zero-padded ISO-8859-1 — no writer freedom at all) and the reference reads
  it back: a genuine differential on the read side, honestly narrower on the write side.

  The two roles are bound by OPPOSITE byte laws on the identity round trip, and each asserts its
  own rather than one being contrived to match the other. `id3`'s writer chooses its own ID3v2
  padding and re-derives the region, so its output must NOT be bit-identical to the input, or
  nothing was parsed. This subset's own codec is deliberately byte-retaining instead — `encode_mp3`
  re-emits each frame's retained payload verbatim and recomputes the ID3v2 sizes from the frame
  data, and this fixture's tag is already canonical under that rule (its 169-byte body is exactly
  TSSE's 10+47 plus TIT2's 10+63 plus TPE1's 10+13 plus TLEN's 10+6, with no trailing padding —
  LAME wrote it tight), so its `codec_retention_law` says its output reproduces the input exactly.
  Demanding a byte difference from it would be a fabricated law; demanding byte equality from the
  reference would be a false one.

  ⚖️ `symphonia`, the obvious pure-Rust MP3 decoder, is MPL-2.0 and this repository has no owner
  ruling on that licence, so it is NOT linked. Nothing this vocabulary addresses needs decoded PCM,
  so the licence question never had to be answered to give this subset a real oracle.

  @id-mutate
  @level-exhaustive
  @mode-differential
  Scenario Outline: Apply <id> to the real stream
    Given the real input stream shared://🔊️.mp3
    When the <id> mutation is applied with its parameters
      """
      {"kind": "<id>", "params": <params>}
      """
    Then the oracle and the subject agree on the semantic projection
    Examples:
      | id           | params |
      | no-mutation  | {} |
      | set-snapshot | {"text": [{"id": "TALB", "text": "replaced wholesale"}], "take": 3, "v1": {"title": "snapshot", "artist": "semio", "album": "", "year": "2026", "comment": "", "genreId": 12}} |
      | set-id3v2    | {"text": [{"id": "TIT2", "text": "renamed by the oracle"}, {"id": "TPE1", "text": "semio"}]} |
      | set-frames   | {"take": 231} |
      | set-id3v1    | {"v1": {"title": "added trailer", "artist": "semio", "album": "", "year": "2026", "comment": "", "genreId": 12}} |

  @id-inverse
  @level-exhaustive
  @mode-property
  Scenario Outline: Undoing <id> restores the real stream
    Given the real input stream shared://🔊️.mp3
    When the <id> mutation is applied with its parameters
      """
      {"kind": "<id>", "params": <params>}
      """
    And the inverse mutation computed against the untouched original is applied to that result
    Then the restored stream's semantic projection equals the original's, asserted in role
    Examples:
      | id           | params |
      | no-mutation  | {} |
      | set-snapshot | {"text": [{"id": "TALB", "text": "replaced wholesale"}], "take": 3, "v1": {"title": "snapshot", "artist": "semio", "album": "", "year": "2026", "comment": "", "genreId": 12}} |
      | set-id3v2    | {"text": [{"id": "TIT2", "text": "renamed by the oracle"}, {"id": "TPE1", "text": "semio"}]} |
      | set-frames   | {"take": 231} |
      | set-id3v1    | {"v1": {"title": "added trailer", "artist": "semio", "album": "", "year": "2026", "comment": "", "genreId": 12}} |

  @id-identity-round-trip
  @level-long
  @mode-round-trip
  Scenario: Decode and re-encode the real stream
    Given the real input stream shared://🔊️.mp3
    When the stream is decoded to its three layers and re-encoded from them alone
    Then the semantic projection is unchanged, asserted in role
    And each role asserts the byte law its own encoder is actually bound by, asserted in role
