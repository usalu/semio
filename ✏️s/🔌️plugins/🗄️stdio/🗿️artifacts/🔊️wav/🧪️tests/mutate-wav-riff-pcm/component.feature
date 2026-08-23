@capability-wav-riff-pcm-mutate
@oracle-hound-wav-riff-pcm-mutate
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

  Every scenario copies the immutable fixture into the case work directory before touching it; the
  committed fixture is never written to. The reference implementation (`hound`) is used only by the
  test oracle, and both results are read back by an INDEPENDENT reader before the
  `semantic-audio-v1` profile compares them. PCM is lossless, so exact decoded-sample comparison is
  the legitimate check here — no bucket/histogram approximation.

  @id-mutate
  @level-exhaustive
  @mode-differential
  Scenario Outline: Apply <id> to the real recording
    Given the real input recording shared://🔊️bauen-mit-bestand-ausschnitt.wav
    When the <id> mutation is applied with its parameters
      """
      {"kind": "<id>", "params": <params>}
      """
    Then the oracle and the subject agree on the semantic projection
    Examples:
      | id               | params                                                                                                  |
      | no-mutation      | {}                                                                                                       |
      | set-snapshot     | {"fmt":{"channels":1,"sampleRate":8000},"data":{"samples":[1000,-1000,500,-500,250,-250,125,-125]},"otherChunks":[{"fourcc":"fact","data":[8,0,0,0]}]} |
      | set-fmt          | {"fmt":{"channels":2,"sampleRate":22050}}                                                               |
      | set-data         | {"data":{"samples":[3000,-3000,1500,-1500,750,-750,375,-375]}}                                          |
      | set-other-chunks | {"chunks":[{"fourcc":"fact","data":[4,0,0,0]},{"fourcc":"LIST","data":[73,78,70,79]}]}                  |

  @id-inverse
  @level-exhaustive
  @mode-property
  Scenario Outline: Undoing <id> restores the recording
    Given the real input recording shared://🔊️bauen-mit-bestand-ausschnitt.wav
    When the <id> mutation is applied with its parameters
      """
      {"kind": "<id>", "params": <params>}
      """
    And the mutation's inverse is applied
    Then the recording is restored to its original semantic projection
    Examples:
      | id               | params                                                                                                  |
      | no-mutation      | {}                                                                                                       |
      | set-snapshot     | {"fmt":{"channels":1,"sampleRate":8000},"data":{"samples":[1000,-1000,500,-500,250,-250,125,-125]},"otherChunks":[{"fourcc":"fact","data":[8,0,0,0]}]} |
      | set-fmt          | {"fmt":{"channels":2,"sampleRate":22050}}                                                               |
      | set-data         | {"data":{"samples":[3000,-3000,1500,-1500,750,-750,375,-375]}}                                          |
      | set-other-chunks | {"chunks":[{"fourcc":"fact","data":[4,0,0,0]},{"fourcc":"LIST","data":[73,78,70,79]}]}                  |

  @id-identity-round-trip
  @level-long
  @mode-round-trip
  Scenario: Decode and re-encode the real recording without passing bytes through
    Given the real input recording shared://🔊️bauen-mit-bestand-ausschnitt.wav
    When the recording is decoded to the typed snapshot and re-encoded from it alone
    Then the reference implementation and this repository agree on the result
