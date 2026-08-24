@capability-semio-v1-video-mutate
@no-oracle-semio-video-mutation-semantics
@comparison-ordered-json-v1
@mutations-semio-v1-video
Feature: Apply every typed semio VIDEO mutation to the decoded real clip artifact
  `s.stdio.semio.video` is a semio-NATIVE format: no third party in any ecosystem reads or writes
  `.dsl.semio`/`.pack.semio`, so there is no reference implementation to register as an oracle
  (recorded as the `semio-video-mutation-semantics` no-oracle decision in
  `../../🏅️standards/🔖️v1/🪆️subsets/✳️video/🧪️oracle/🔣️component.json`, which also records why the
  `mp4` and `riff` crates already registered here were surveyed and rejected: both round-trip
  CONTAINERS, and this snapshot is deliberately container-neutral and payload-opaque, so judging it
  through either would mean inventing fields the snapshot does not carry). The input is not
  synthetic. Every one of the nine kinds is applied to the snapshot this standard's own committed
  real artifact decodes to,
  `asset://🏅️standards/🔖️v1/🪆️subsets/✳️any/📚️examples/🎥️clip/🖼️assets/🗣️example.dsl.semio` — a
  1920x1080 30/1 h264 track of two samples beside a 48000/1000 aac track — so the vocabulary is
  measured against a real document of this format rather than a fixture invented for the test. Each
  kind's committed `(before, mutation, after)` specification vector lives in this case's own
  `🧫️fixtures/` and is declared as a `local://` URI, so BOTH roles read the same committed bytes:
  the `oracle` role reads the vector literally (no recomputation, no reimplementation of mutation
  semantics) and the `subject` role decodes it into real `SemioVideoSnapshot`/`SemioVideoMutation`
  values and runs the production entry point `apply_semio_video_mutation`.

  ⚖️ Because this case records a no-oracle decision, the runner executes NO oracle role: it resolves
  an oracle implementation from an `@oracle-` tag this feature deliberately does not carry, so the
  comparison profile never receives two sides to compare and the `oracle` handlers below are the
  written statement of the reference answer rather than a second running party. Every law this
  feature claims is therefore asserted INSIDE the subject handler, which fails with both documents
  printed. A handler that merely ran the mutation and returned would report a pass having checked
  nothing. Here that means the applied snapshot is checked against the vector's
  after-snapshot, the undone snapshot against its before-snapshot, and `identity-round-trip`
  additionally checks that the real committed clip artifact decodes to exactly the before-snapshot
  every vector starts from — so a mistake in the vectors surfaces as a red scenario rather than a
  quietly agreeable one.

  @id-mutate
  @level-exhaustive
  @mode-conformance
  Scenario Outline: Apply <id> to the decoded real clip snapshot
    Given the committed specification vector local://🦠️<id>.json for the <id> kind
    When <id> is applied to its before-snapshot through apply_semio_video_mutation
    Then the resulting snapshot matches the vector's after-snapshot
    Examples:
      | id               |
      | no-mutation      |
      | set-snapshot     |
      | insert-stream    |
      | remove-stream    |
      | set-stream-meta  |
      | insert-sample    |
      | remove-sample    |
      | set-sample-data  |
      | set-sample-flags |

  @id-inverse
  @level-exhaustive
  @mode-property
  Scenario Outline: Undoing <id> restores the decoded real clip snapshot
    Given the committed specification vector local://🦠️<id>.json for the <id> kind
    When <id> is applied to its before-snapshot through apply_semio_video_mutation
    And the mutation's own computed inverse is applied through apply_semio_video_mutation
    Then the snapshot matches the vector's before-snapshot again
    Examples:
      | id               |
      | no-mutation      |
      | set-snapshot     |
      | insert-stream    |
      | remove-stream    |
      | set-stream-meta  |
      | insert-sample    |
      | remove-sample    |
      | set-sample-data  |
      | set-sample-flags |

  @id-identity-round-trip
  @level-long
  @mode-round-trip
  Scenario: Decode and re-encode the real clip artifact without passing bytes through
    Given the real committed artifact asset://🏅️standards/🔖️v1/🪆️subsets/✳️any/📚️examples/🎥️clip/🖼️assets/🗣️example.dsl.semio
    And the committed specification vector local://🦠️no-mutation.json whose before-snapshot is that artifact decoded
    When the artifact is parsed into a SemioVideoSnapshot, printed back to DSL text and parsed again
    Then the twice-decoded snapshot equals the committed before-snapshot
