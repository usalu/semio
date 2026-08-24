@capability-semio-v1-animation-mutate
@no-oracle-semio-animation-mutation-semantics
@comparison-ordered-json-v1
@mutations-semio-v1-animation
Feature: Apply every typed semio ANIMATION mutation to the decoded real walk artifact
  `s.stdio.semio.animation` is a semio-NATIVE format: no third party in any ecosystem reads or
  writes `.dsl.semio`/`.pack.semio`, so there is no reference implementation to register as an
  oracle (recorded as the `semio-animation-mutation-semantics` no-oracle decision in
  `../../🏅️standards/🔖️v1/🪆️subsets/✳️animation/🧪️oracle/🔣️component.json`, which also records why
  the `gif` crate registered here was surveyed and rejected: it exposes per-frame delays and nothing
  else, so nine of these thirteen kinds would have had nothing to compare against). The input is not
  synthetic. Every one of the thirteen kinds is applied to the snapshot this standard's own
  committed real artifact decodes to,
  `asset://🏅️standards/🔖️v1/🪆️subsets/✳️any/📚️examples/🚶️walk/🖼️assets/🗣️example.dsl.semio` — one
  named timeline whose four channels between them exercise every target property (translation,
  rotation, weights, a custom property), every interpolation mode and every keyframe value shape —
  so the vocabulary is measured against a real document of this format rather than a fixture
  invented for the test. Each kind's committed `(before, mutation, after)` specification vector
  lives in this case's own `🧫️fixtures/` and is declared as a `local://` URI, so BOTH roles read the
  same committed bytes: the `oracle` role reads the vector literally (no recomputation, no
  reimplementation of mutation semantics) and the `subject` role decodes it into real
  `SemioAnimationSnapshot`/`SemioAnimationMutation` values and runs the production entry point
  `apply_semio_animation_mutation`. The `ordered-json-v1` profile compares the two structurally.

  @id-mutate
  @level-exhaustive
  @mode-conformance
  Scenario Outline: Apply <id> to the decoded real walk snapshot
    Given the committed specification vector local://🦠️<id>.json for the <id> kind
    When <id> is applied to its before-snapshot through apply_semio_animation_mutation
    Then the resulting snapshot matches the vector's after-snapshot
    Examples:
      | id                        |
      | no-mutation               |
      | set-snapshot              |
      | insert-timeline           |
      | remove-timeline           |
      | set-timeline-name         |
      | insert-channel            |
      | remove-channel            |
      | set-channel-target        |
      | set-channel-interpolation |
      | insert-keyframe           |
      | remove-keyframe           |
      | set-keyframe-time         |
      | set-keyframe-value        |

  @id-inverse
  @level-exhaustive
  @mode-property
  Scenario Outline: Undoing <id> restores the decoded real walk snapshot
    Given the committed specification vector local://🦠️<id>.json for the <id> kind
    When <id> is applied to its before-snapshot through apply_semio_animation_mutation
    And the mutation's own computed inverse is applied through apply_semio_animation_mutation
    Then the snapshot matches the vector's before-snapshot again
    Examples:
      | id                        |
      | no-mutation               |
      | set-snapshot              |
      | insert-timeline           |
      | remove-timeline           |
      | set-timeline-name         |
      | insert-channel            |
      | remove-channel            |
      | set-channel-target        |
      | set-channel-interpolation |
      | insert-keyframe           |
      | remove-keyframe           |
      | set-keyframe-time         |
      | set-keyframe-value        |

  @id-identity-round-trip
  @level-long
  @mode-round-trip
  Scenario: Decode and re-encode the real walk artifact without passing bytes through
    Given the real committed artifact asset://🏅️standards/🔖️v1/🪆️subsets/✳️any/📚️examples/🚶️walk/🖼️assets/🗣️example.dsl.semio
    And the committed specification vector local://🦠️no-mutation.json whose before-snapshot is that artifact decoded
    When the artifact is parsed into a SemioAnimationSnapshot, printed back to DSL text and parsed again
    Then the twice-decoded snapshot equals the committed before-snapshot
