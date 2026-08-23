@capability-wave-audio
@oracle-hound
@comparison-semantic-audio-v1
Feature: Create and retune a RIFF/PCM WAVE file
  The reference implementation writes the input waveform; this repository decodes that artifact and
  re-encodes it. Both results are read back by the INDEPENDENT reader before the `semantic-audio-v1`
  profile compares them, so a lossy decode or a malformed re-encode shows up as a real difference
  rather than as a producer agreeing with its own reading.

  Chunk padding, LIST/INFO metadata, chunk order and total byte length are writer choices and are
  canonicalized away. The format block and every decoded sample are normative.

  @id-mono-sawtooth-round-trips
  @level-quick
  @mode-round-trip
  Scenario: A mono 8 kHz sawtooth survives decode and re-encode
    Given the waveform
    """
    { "channels": 1, "sampleRate": 8000, "frames": 64 }
    """
    When the waveform is written and read back
    Then the format block and every sample are unchanged

  @id-stereo-round-trips
  @level-quick
  @mode-round-trip
  Scenario: A stereo waveform survives decode and re-encode
    Given the waveform
    """
    { "channels": 2, "sampleRate": 44100, "frames": 128 }
    """
    When the waveform is written and read back
    Then the format block and every sample are unchanged

  @id-retuned-sample-rate
  @level-quick
  @mode-differential
  Scenario: Changing the declared sample rate keeps every sample
    Given the waveform
    """
    { "channels": 1, "sampleRate": 8000, "frames": 64, "retuneTo": 16000 }
    """
    When the declared sample rate is changed without resampling
    Then the reference implementation and this repository agree on the result
