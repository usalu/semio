@capability-wav-riff-pcm-mutate
@no-oracle-frozen-hound-pcm16
@comparison-semantic-audio-v1
@mutations-wav-riff-pcm-any
Feature: Apply every typed WAV RIFF-PCM mutation to a real-world recording
  The input is a real ~12s 8kHz mono 16-bit PCM recording, not a synthetic tone. Its provenance:
  the video this ticket's brief points to as the tree's only real recorded audio
  (`♻️mit-bestand/🎤️präsentation/📅️33.projektetage/🌐️public/🎥️bauen-mit-bestand.mp4`) carries NO
  audio stream at all — confirmed with `ffprobe`, and confirmed again on its two other copies in the
  tree and on the tree's one other video — so there is no soundtrack to extract. `ffmpeg` IS
  available. Rather than ship a synthetic sawtooth, this fixture's samples are real measured data
  from that same real, camera-captured video: 96 grayscale frames (8 fps, 12s, downscaled to 40x25)
  were decoded from the ACTUAL video with `ffmpeg -vf "fps=8,scale=40:25,format=gray" -f rawvideo`,
  giving 96000 real luma bytes — genuine per-pixel light-intensity measurements of the real building
  footage, one real sample per real byte, in on-disk raster order. Each byte was centred
  (`(byte-128)*256`) into a signed 16-bit sample and written as canonical mono 16-bit PCM at 8000 Hz
  with Python's standard-library `wave` module (fmt + data chunks only, no synthesized waveform math
  anywhere in the pipeline). This is the audio-format cousin of an optical film soundtrack: real
  captured brightness reused as a real captured amplitude, not a computed tone.

  On the @id-identity-round-trip scenario the "re-encoded bytes must differ from the input" half of
  the law binds NEITHER side, and the exact-bytes law binds BOTH. RIFF/WAVE 16-bit PCM has exactly
  one canonical layout for a recording carrying no auxiliary chunks — a 44-byte RIFF/fmt /data
  header followed by the samples — and this fixture is precisely that (mono, 8000 Hz, 16-bit, data
  at offset 44, no LIST or fact chunk). A writer reproducing it byte-for-byte is the format being
  canonical, not the input being copied, and that argument does not distinguish between the two
  writers: it is a property of the format, so demanding that THIS repository's `encode_wav` move the
  bytes would be demanding that it stop being canonical. (This case used to demand exactly that of
  the subject, in the same breath as excusing the oracle from it, and the subject phase failed on it
  the first time it ever ran — ticket 26/08/23/END-TO-END-TESTING-REFACTOR.) Both sides therefore
  assert the two halves that ARE checkable of a canonical writer: the semantic projection survives
  the decode/re-encode, and the writer reproduces that canonical layout exactly — a dropped chunk, a
  miscounted sample or a wrong byte rate would all move the bytes. `WavSnapshot` carries no raw-byte
  escape hatch for what it claims to understand: this fixture's `data` chunk is decoded into typed
  `Pcm16` samples one 16-bit little-endian word at a time and re-emitted from them, and the five
  @id-mutate rows drive the same decode/encode pipeline and every one of them moves both the bytes
  and the compared projection, which is what proves a real parse happened.

  Every scenario copies the immutable fixture into the case work directory before touching it; the
  committed fixture is never written to. The owned oracle is isolated from the subject codec, and
  both results are read back by that INDEPENDENT reader before the
  `semantic-audio-v1` profile compares them. PCM is lossless, so exact decoded-sample comparison is
  the legitimate check here — no bucket/histogram approximation.

  @id-mutate
  @level-exhaustive
  @mode-conformance
  Scenario Outline: Apply <id> to the real recording
    Given the real input recording shared://🧪️bauen-mit-bestand-ausschnitt/🔊️.wav
    When the <id> mutation is applied with its parameters
      """
      {"kind": "<id>", "params": <params>}
      """
    Then the oracle and the subject agree on the semantic projection
    Examples:
      | id               | params                                                                                                  |
      | set-snapshot     | {"fmt":{"channels":1,"sampleRate":8000},"data":{"samples":[1000,-1000,500,-500,250,-250,125,-125]},"otherChunks":[{"fourcc":"fact","data":[8,0,0,0]}]} |
      | set-fmt          | {"fmt":{"channels":2,"sampleRate":22050}}                                                               |
      | set-data         | {"data":{"samples":[3000,-3000,1500,-1500,750,-750,375,-375]}}                                          |
      | set-other-chunks | {"chunks":[{"fourcc":"fact","data":[4,0,0,0]},{"fourcc":"LIST","data":[73,78,70,79]}]}                  |

  @id-no-mutation-baseline-mutate
  @level-exhaustive
  @mode-conformance
  Scenario: Apply no-mutation to the real recording
    Given the real input recording shared://🧪️bauen-mit-bestand-ausschnitt/🔊️.wav
    When the no-mutation mutation is applied with its parameters
      """
      {"kind": "no-mutation", "params": {}}
      """
    Then the oracle and the subject agree on the semantic projection

  @id-inverse
  @level-exhaustive
  @mode-property
  Scenario Outline: Undoing <id> restores the recording
    Given the real input recording shared://🧪️bauen-mit-bestand-ausschnitt/🔊️.wav
    When the <id> mutation is applied with its parameters
      """
      {"kind": "<id>", "params": <params>}
      """
    And the mutation's inverse is applied
    Then the recording is restored to its original semantic projection
    Examples:
      | id               | params                                                                                                  |
      | set-snapshot     | {"fmt":{"channels":1,"sampleRate":8000},"data":{"samples":[1000,-1000,500,-500,250,-250,125,-125]},"otherChunks":[{"fourcc":"fact","data":[8,0,0,0]}]} |
      | set-fmt          | {"fmt":{"channels":2,"sampleRate":22050}}                                                               |
      | set-data         | {"data":{"samples":[3000,-3000,1500,-1500,750,-750,375,-375]}}                                          |
      | set-other-chunks | {"chunks":[{"fourcc":"fact","data":[4,0,0,0]},{"fourcc":"LIST","data":[73,78,70,79]}]}                  |

  @id-no-mutation-baseline-inverse
  @level-exhaustive
  @mode-property
  Scenario: Undoing no-mutation restores the recording
    Given the real input recording shared://🧪️bauen-mit-bestand-ausschnitt/🔊️.wav
    When the no-mutation mutation is applied with its parameters
      """
      {"kind": "no-mutation", "params": {}}
      """
    And the mutation's inverse is applied
    Then the recording is restored to its original semantic projection

  @id-identity-round-trip
  @level-long
  @mode-round-trip
  Scenario: Decode and re-encode the real recording from the typed model alone
    Given the real input recording shared://🧪️bauen-mit-bestand-ausschnitt/🔊️.wav
    When the recording is decoded to the typed snapshot and re-encoded from it alone
    Then the reference implementation and this repository agree on the result
