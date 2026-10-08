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
  READ-ONLY and the vocabulary carries named ID3v1 metadata, so `set-id3v1` carries
  title, artist, album, year, comment, track and genre; independent IO writes fixed offsets,
  zero-padded ISO-8859-1 — no writer freedom at all), and the reference reads it back: a genuine
  differential on the read side, honestly narrower on the write side.

  Every Examples `params` cell is exactly the leaf's wire payload — its `payload_value()`, camelCase,
  no aggregate tag. The subject decodes it through the derive-generated `from_payload_value`; the
  oracle reads the same field names, decodes each ID3v2.3 text-frame body (encoding byte, then
  ISO-8859-1 or byte-order-marked UTF-16) and packs every MPEG frame header from its twelve typed
  fields per ISO/IEC 11172-3 §2.4.1.3 before re-walking the region. The frames a row carries are
  real frames of this stream, read out of the fixture by the ticket's `🧪️w2w-media-wire-rows.py`:
  `set-frames`'s three cross the first padding-slot change (frames 0 and 1 are 417 bytes, frame 2
  is 418), so the packed headers land on both branches of the frame-size formula.

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
      | id | params |
      | set-id3v2 | {"id3v2":{"frames":[{"id":"TIT2","content":{"kind":"text","values":["renamed by the oracle"]}},{"id":"TPE1","content":{"kind":"text","values":["semio"]}}]}} |
      | set-frames | {"frames":[{"header":{"mpegVersionId":3,"layer":1,"protectionBit":true,"bitrateIndex":9,"sampleRateIndex":0,"padding":false,"privateBit":false,"channelMode":3,"modeExtension":0,"copyright":false,"original":true,"emphasis":0},"payload":[0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,73,110,102,111,0,0,0,15,0,0,1,205,0,2,242,72,0,3,5,7,11,13,15,18,21,23,26,28,31,33,36,38,41,44,46,48,52,54,56,58,62,64,66,69,72,74,77,79,82,84,87,89,92,95,97,99,103,105,107,109,113,115,117,121,123,125,128,131,133,135,138,141,143,146,148,151,154,156,158,161,164,166,168,172,174,176,179,182,184,186,189,192,194,197,199,202,205,207,209,212,215,217,219,223,225,227,231,233,235,237,241,243,245,248,251,253,0,0,0,57,76,65,77,69,51,46,49,48,48,1,205,0,0,0,0,46,126,0,0,20,128,36,5,16,66,0,0,128,0,2,242,72,153,230,49,31,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0]},{"header":{"mpegVersionId":3,"layer":1,"protectionBit":true,"bitrateIndex":9,"sampleRateIndex":0,"padding":false,"privateBit":false,"channelMode":3,"modeExtension":0,"copyright":false,"original":true,"emphasis":0},"payload":[0,0,20,240,207,90,20,244,128,11,62,177,110,183,53,48,2,5,64,152,33,11,178,222,61,99,214,45,226,110,18,1,12,37,138,13,95,176,33,128,12,1,129,176,76,19,4,193,48,220,133,100,242,32,20,6,5,2,132,4,232,231,57,205,26,50,113,64,160,145,2,8,65,2,2,116,104,209,163,70,141,27,104,24,132,33,4,8,24,110,115,70,141,26,54,247,248,40,129,3,16,132,231,57,206,115,156,208,32,98,0,0,0,97,225,225,225,224,0,0,0,0,97,225,225,227,240,0,0,0,3,15,15,15,15,0,0,0,0,3,15,15,15,15,0,0,0,0,19,152,120,120,120,0,0,0,2,48,240,240,241,224,0,0,0,0,97,227,254,56,0,0,0,3,186,30,30,123,0,0,0,31,152,120,244,74,80,165,37,41,198,236,141,38,154,4,162,137,57,1,204,48,18,179,222,77,3,210,182,199,112,197,149,151,65,182,18,79,3,15,20,115,200,104,55,84,241,96,196,79,3,56,28,113,176,64,34,8,106,8,108,10,164,155,28,241,115,6,47,4,84,56,98,2,233,27,42,58,13,128,149,197,146,53,2,219,11,32,27,206,110,92,21,176,7,48,145,38,139,226,66,44,242,112,208,186,30,193,116,196,157,101,7,144,156,93,72,14,80,236,172,87,3,33,14,195,52,196,36,27,197,97,10,164,241,212,65,69,10,3,2,99,67,65,158,39,204,234,34,5,245,34,163,126,109,229,102,156,94,164,73,45,69,6,110,141,212,178,124,157,214,110,212,169,144,213,188,233,230,101,24,59,162,177,207,122,148,199,159,44,37,172,184,250,183,201,230,212,89,193,42,119,219,95,250,223,71,95,150,175,101,21,170,100,57,86,85,9,70,51,7,64,251,124,45,27,13,134,192,52,69,208,55]}]} |
      | set-id3v1 | {"id3v1":{"title":"added trailer","artist":"semio","album":"","year":"2026","comment":"","track":null,"genre":12}} |

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
      | id | params |
      | set-id3v2 | {"id3v2":{"frames":[{"id":"TIT2","content":{"kind":"text","values":["renamed by the oracle"]}},{"id":"TPE1","content":{"kind":"text","values":["semio"]}}]}} |
      | set-frames | {"frames":[{"header":{"mpegVersionId":3,"layer":1,"protectionBit":true,"bitrateIndex":9,"sampleRateIndex":0,"padding":false,"privateBit":false,"channelMode":3,"modeExtension":0,"copyright":false,"original":true,"emphasis":0},"payload":[0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,73,110,102,111,0,0,0,15,0,0,1,205,0,2,242,72,0,3,5,7,11,13,15,18,21,23,26,28,31,33,36,38,41,44,46,48,52,54,56,58,62,64,66,69,72,74,77,79,82,84,87,89,92,95,97,99,103,105,107,109,113,115,117,121,123,125,128,131,133,135,138,141,143,146,148,151,154,156,158,161,164,166,168,172,174,176,179,182,184,186,189,192,194,197,199,202,205,207,209,212,215,217,219,223,225,227,231,233,235,237,241,243,245,248,251,253,0,0,0,57,76,65,77,69,51,46,49,48,48,1,205,0,0,0,0,46,126,0,0,20,128,36,5,16,66,0,0,128,0,2,242,72,153,230,49,31,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0]},{"header":{"mpegVersionId":3,"layer":1,"protectionBit":true,"bitrateIndex":9,"sampleRateIndex":0,"padding":false,"privateBit":false,"channelMode":3,"modeExtension":0,"copyright":false,"original":true,"emphasis":0},"payload":[0,0,20,240,207,90,20,244,128,11,62,177,110,183,53,48,2,5,64,152,33,11,178,222,61,99,214,45,226,110,18,1,12,37,138,13,95,176,33,128,12,1,129,176,76,19,4,193,48,220,133,100,242,32,20,6,5,2,132,4,232,231,57,205,26,50,113,64,160,145,2,8,65,2,2,116,104,209,163,70,141,27,104,24,132,33,4,8,24,110,115,70,141,26,54,247,248,40,129,3,16,132,231,57,206,115,156,208,32,98,0,0,0,97,225,225,225,224,0,0,0,0,97,225,225,227,240,0,0,0,3,15,15,15,15,0,0,0,0,3,15,15,15,15,0,0,0,0,19,152,120,120,120,0,0,0,2,48,240,240,241,224,0,0,0,0,97,227,254,56,0,0,0,3,186,30,30,123,0,0,0,31,152,120,244,74,80,165,37,41,198,236,141,38,154,4,162,137,57,1,204,48,18,179,222,77,3,210,182,199,112,197,149,151,65,182,18,79,3,15,20,115,200,104,55,84,241,96,196,79,3,56,28,113,176,64,34,8,106,8,108,10,164,155,28,241,115,6,47,4,84,56,98,2,233,27,42,58,13,128,149,197,146,53,2,219,11,32,27,206,110,92,21,176,7,48,145,38,139,226,66,44,242,112,208,186,30,193,116,196,157,101,7,144,156,93,72,14,80,236,172,87,3,33,14,195,52,196,36,27,197,97,10,164,241,212,65,69,10,3,2,99,67,65,158,39,204,234,34,5,245,34,163,126,109,229,102,156,94,164,73,45,69,6,110,141,212,178,124,157,214,110,212,169,144,213,188,233,230,101,24,59,162,177,207,122,148,199,159,44,37,172,184,250,183,201,230,212,89,193,42,119,219,95,250,223,71,95,150,175,101,21,170,100,57,86,85,9,70,51,7,64,251,124,45,27,13,134,192,52,69,208,55]}]} |
      | set-id3v1 | {"id3v1":{"title":"added trailer","artist":"semio","album":"","year":"2026","comment":"","track":null,"genre":12}} |

  @id-identity-round-trip
  @level-long
  @mode-round-trip
  Scenario: Decode and re-encode the real stream
    Given the real input stream shared://🔊️.mp3
    When the stream is decoded to its three layers and re-encoded from them alone
    Then the semantic projection is unchanged, asserted in role
    And each role asserts the byte law its own encoder is actually bound by, asserted in role
