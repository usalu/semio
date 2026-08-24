@capability-mp3-mpeg1-layer3-mutate
@oracle-id3-mpeg1-layer3-mutate
@comparison-semantic-mp3-mpeg1-layer3-v1
@mutations-mp3-mpeg1-layer3-any
Feature: Apply every typed MP3 mpeg1-layer3 mutation to a real committed stream
  The input is asset://🏅️standards/🔖️mpeg1-layer3/🪆️subsets/✳️any/📚️examples/🎬️demo/🖼️assets/🎵️example.mp3,
  the artifact's own committed 1,725-byte stream: a real ID3v2.3.0 tag carrying TIT2 "semio fixture"
  and TPE1 "W0 handcraft", followed by four real MPEG1 Layer III frames (128 kbps, 44.1 kHz, joint
  stereo, 417 bytes each — 144·128000/44100 with no padding slot), and no ID3v1 trailer. Every
  scenario copies it into the case work directory before touching it; the committed file is never
  written to.

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
  padding, so the reference re-encodes this fixture's 57-byte tag region as 86 bytes: its output
  must NOT be bit-identical to the input, or nothing was parsed. This subset's own codec is
  deliberately byte-retaining instead — `encode_mp3` re-emits each frame's retained payload verbatim
  and recomputes the ID3v2 sizes from the frame data, and this fixture's tag is already canonical
  under that rule (its 47-byte body is exactly TIT2's 10+14 plus TPE1's 10+13, with no padding), so
  its `codec_retention_law` says its output reproduces the input exactly. Demanding a byte
  difference from it would be a fabricated law; demanding byte equality from the reference would be
  a false one.

  ⚖️ `symphonia`, the obvious pure-Rust MP3 decoder, is MPL-2.0 and this repository has no owner
  ruling on that licence, so it is NOT linked. Nothing this vocabulary addresses needs decoded PCM,
  so the licence question never had to be answered to give this subset a real oracle.

  @id-mutate
  @level-exhaustive
  @mode-differential
  Scenario Outline: Apply <id> to the real stream
    Given the real input stream asset://🏅️standards/🔖️mpeg1-layer3/🪆️subsets/✳️any/📚️examples/🎬️demo/🖼️assets/🎵️example.mp3
    When the <id> mutation is applied with its parameters
      """
      {"kind": "<id>", "params": <params>}
      """
    Then the oracle and the subject agree on the semantic projection
    Examples:
      | id           | params |
      | no-mutation  | {} |
      | set-snapshot | {"text": [{"id": "TALB", "text": "replaced wholesale"}], "take": 1, "v1": {"title": "snapshot", "artist": "semio", "album": "", "year": "2026", "comment": "", "genreId": 12}} |
      | set-id3v2    | {"text": [{"id": "TIT2", "text": "renamed by the oracle"}, {"id": "TPE1", "text": "W0 handcraft"}]} |
      | set-frames   | {"take": 2} |
      | set-id3v1    | {"v1": {"title": "added trailer", "artist": "semio", "album": "", "year": "2026", "comment": "", "genreId": 12}} |

  @id-inverse
  @level-exhaustive
  @mode-property
  Scenario Outline: Undoing <id> restores the real stream
    Given the real input stream asset://🏅️standards/🔖️mpeg1-layer3/🪆️subsets/✳️any/📚️examples/🎬️demo/🖼️assets/🎵️example.mp3
    When the <id> mutation is applied with its parameters
      """
      {"kind": "<id>", "params": <params>}
      """
    And the inverse mutation computed against the untouched original is applied to that result
    Then the restored stream's semantic projection equals the original's, asserted in role
    Examples:
      | id           | params |
      | no-mutation  | {} |
      | set-snapshot | {"text": [{"id": "TALB", "text": "replaced wholesale"}], "take": 1, "v1": {"title": "snapshot", "artist": "semio", "album": "", "year": "2026", "comment": "", "genreId": 12}} |
      | set-id3v2    | {"text": [{"id": "TIT2", "text": "renamed by the oracle"}, {"id": "TPE1", "text": "W0 handcraft"}]} |
      | set-frames   | {"take": 2} |
      | set-id3v1    | {"v1": {"title": "added trailer", "artist": "semio", "album": "", "year": "2026", "comment": "", "genreId": 12}} |

  @id-identity-round-trip
  @level-long
  @mode-round-trip
  Scenario: Decode and re-encode the real stream
    Given the real input stream asset://🏅️standards/🔖️mpeg1-layer3/🪆️subsets/✳️any/📚️examples/🎬️demo/🖼️assets/🎵️example.mp3
    When the stream is decoded to its three layers and re-encoded from them alone
    Then the semantic projection is unchanged, asserted in role
    And each role asserts the byte law its own encoder is actually bound by, asserted in role
