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
  production entry point `apply_semio_audio_mutation`.

  ⚖️ Because this case records a no-oracle decision, the runner executes NO oracle role: it resolves
  an oracle implementation from an `@oracle-` tag this feature deliberately does not carry, so the
  comparison profile never receives two sides to compare and the `oracle` handlers below are the
  written statement of the reference answer rather than a second running party. Every law this
  feature claims is therefore asserted INSIDE the subject handler, which fails with both documents
  printed. A handler that merely ran the mutation and returned would report a pass having checked
  nothing. Here that means the applied snapshot is checked against the vector's
  after-snapshot, the undone snapshot against its before-snapshot, and `identity-round-trip`
  additionally checks that the real committed tone artifact decodes to exactly the before-snapshot
  every vector starts from — so a mistake in the vectors surfaces as a red scenario rather than a
  quietly agreeable one.

  `identity-round-trip` measures BYTES here, not only meaning. The committed `🎵️tone` artifact is a
  226-byte `.dsl.semio` record written by this subset's own printer, so re-printing the snapshot it
  parses to must land on those same 226 bytes; `law::carrier_is_exact` says so and fails with the
  offset of the first byte that drifts. The must-differ tripwire the wave applies to third-party
  carriers would be backwards for a codec reading its own output. `✳️audio` exports no pack bridge,
  so the committed `🎒️example.pack.semio` twin is NOT read by this case and no claim is made about it
  — one carrier, measured, and the other named as unmeasured.

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
