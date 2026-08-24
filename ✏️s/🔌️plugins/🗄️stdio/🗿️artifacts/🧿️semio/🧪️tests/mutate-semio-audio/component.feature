@capability-semio-v1-audio-mutate
@no-oracle-semio-audio-mutation-semantics
@comparison-ordered-json-v1
@mutations-semio-v1-audio
Feature: Apply every typed semio AUDIO mutation to the decoded real tone artifact
  `s.stdio.semio.audio` is a semio-NATIVE format: no third party in any ecosystem reads or writes
  `.dsl.semio`/`.pack.semio`, so there is no reference implementation to register as an oracle
  (recorded as the `semio-audio-mutation-semantics` no-oracle decision in
  `../../🏅️standards/🔖️v1/🪆️subsets/✳️audio/🧪️oracle/🔣️component.json`, which also records why
  `hound` — already registered here as the wav oracle — was surveyed and rejected rather than
  simply absent: it models no `LIST INFO` chunk, so three of these ten kinds would have had nothing
  to compare against). The input is not synthetic. Every one of the ten kinds is applied to the
  snapshot this standard's own committed real artifact decodes to,
  `asset://🏅️standards/🔖️v1/🪆️subsets/✳️any/📚️examples/🎵️tone/🖼️assets/🗣️example.dsl.semio` — a
  44.1 kHz stereo `f32` tone carrying a `title` tag — so the vocabulary is measured against a real
  document of this format rather than a fixture invented for the test. Each kind's committed
  `(before, mutation, after)` specification vector lives in this case's own `🧫️fixtures/` and is
  declared as a `local://` URI, so BOTH roles read the same committed bytes: the `oracle` role
  reads the vector literally (no recomputation, no reimplementation of mutation semantics) and the
  `subject` role decodes it into real `SemioAudioSnapshot`/`SemioAudioMutation` values and runs the
  production entry point `apply_semio_audio_mutation`. The `ordered-json-v1` profile compares the
  two structurally.

  @id-mutate
  @level-exhaustive
  @mode-conformance
  Scenario Outline: Apply <id> to the decoded real tone snapshot
    Given the committed specification vector local://🦠️<id>.json for the <id> kind
    When <id> is applied to its before-snapshot through apply_semio_audio_mutation
    Then the resulting snapshot matches the vector's after-snapshot
    Examples:
      | id                  |
      | no-mutation         |
      | set-snapshot        |
      | set-sample-rate     |
      | set-format          |
      | insert-channel      |
      | remove-channel      |
      | set-channel-samples |
      | insert-tag          |
      | remove-tag          |
      | set-tag-value       |

  @id-inverse
  @level-exhaustive
  @mode-property
  Scenario Outline: Undoing <id> restores the decoded real tone snapshot
    Given the committed specification vector local://🦠️<id>.json for the <id> kind
    When <id> is applied to its before-snapshot through apply_semio_audio_mutation
    And the mutation's own computed inverse is applied through apply_semio_audio_mutation
    Then the snapshot matches the vector's before-snapshot again
    Examples:
      | id                  |
      | no-mutation         |
      | set-snapshot        |
      | set-sample-rate     |
      | set-format          |
      | insert-channel      |
      | remove-channel      |
      | set-channel-samples |
      | insert-tag          |
      | remove-tag          |
      | set-tag-value       |

  @id-identity-round-trip
  @level-long
  @mode-round-trip
  Scenario: Decode and re-encode the real tone artifact without passing bytes through
    Given the real committed artifact asset://🏅️standards/🔖️v1/🪆️subsets/✳️any/📚️examples/🎵️tone/🖼️assets/🗣️example.dsl.semio
    And the committed specification vector local://🦠️no-mutation.json whose before-snapshot is that artifact decoded
    When the artifact is parsed into a SemioAudioSnapshot, printed back to DSL text and parsed again
    Then the twice-decoded snapshot equals the committed before-snapshot
