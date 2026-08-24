@capability-playbook-1-mutate
@no-oracle-playbook-document-mutation-semantics
@comparison-ordered-json-v1
@mutations-playbook-1-any
Feature: Apply every typed playbook document mutation to its committed specification vectors
  `s.playbook.playbook` is a semio-NATIVE artifact: it is persisted as `.dsl.semio` text and
  `.pack.semio` binary through this subset's own codecs, and no third party reads or writes either.
  There is therefore no reference implementation to register as an oracle — recorded as the
  `playbook-document-mutation-semantics` no-oracle decision in
  `../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️oracle/🔣️component.json`, whose substitutes are the
  committed specification vectors and the inverse law. Because that decision is recorded, the runner
  dispatches NO oracle role for this case: every assertion below lives inside the subject handler,
  and a handler that merely ran the mutation and returned would report a pass having checked nothing.

  What distinguishes this subset from its close relative `📋️forms` — which shares the very same
  `PlaybookStep`/`PlaybookBlock` record types, aliased there as `FormStep`/`FormQuestion` — is the
  VERB SET, and the difference is not cosmetic. Playbook has nine kinds where forms has ten, and
  they are not a renaming of each other: playbook has no `change-step-description`, because its
  `update-step` patches a step's whole header (title AND description) in one mutation, and its
  `move-block` carries a `fromStepId` and a `toStepId` so a block crosses steps by construction,
  where forms' `move-block-to-step` is defined to also cover the stay-in-place case. Those two
  differences are what make this a separate vocabulary rather than a copy, and the committed vectors
  are chosen on them: `update-step`'s vector resends the header the step already carries, and
  `move-block`'s aims a block at a step that does not exist.

  ⚠️ EIGHT of the nine committed vectors pin a REJECTION or NO-OP branch rather than an effect, for
  the same structural reason forms records: `PlaybookSnapshot` persists a title and a composed
  `s.stdio.semio.flow` CHILD handle, the step flow lives behind it in a session-side working scene,
  and a successful mutation re-mints that handle content-addressed — so an effect vector's `➡️after`
  cannot be hand-authored. Those eight are named in the adapter's `GUARD_VECTORS` list and exempted
  from the observability law; `change-title`, the one kind that touches a persisted scalar, carries
  it unexempted. In exchange each `mutate-<kind>` scenario asserts the committed `🎯️outcome`'s
  declared status AND diagnostic code, which separates the four `mutation.target-missing`
  rejections from the four `mutation.no-op` warnings — a distinction an effect-only vector never
  makes.

  📌️ The `scene` column is the other half of each before-state, transcribed from that leaf's OWN
  `🧪️tests/<fixture>/🦀️component.rs::before()`; see the forms feature for the same note and the same
  fix (commit the scene beside the snapshot).

  Every scenario reads the committed vectors where the domain already keeps them, through
  `asset://`, and never writes to them.

  @id-mutate
  @level-exhaustive
  @mode-conformance
  Scenario Outline: Applying <id> to its committed before-snapshot yields the committed after-snapshot
    Given the committed before-snapshot asset://🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/<vector>/📸️snapshot/⬅️before/🔣️component.json
    And the committed mutation payload asset://🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/<vector>/🦠️mutation/🔣️component.json
    And the committed after-snapshot asset://🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/<vector>/📸️snapshot/➡️after/🔣️component.json
    And the committed outcome vector asset://🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/<vector>/🎯️outcome/🔣️component.json
    When <id> is applied through apply_playbook_mutation_outcome
      """
      {"kind": "<id>", "vector": "<vector>", "scene": <scene>}
      """
    Then the resulting snapshot is the committed after-snapshot and the raised diagnostics are the committed outcome's
    Examples:
      | id            | vector                                                                | scene                                                                                                                                              |
      | add-step      | ➕add-step/🧪️tests/no-ops-on-a-duplicate-step-id                       | [{"id":"s-review","title":"Review","blocks":[]}]                                                                                                   |
      | remove-step   | ➖remove-step/🧪️tests/rejects-removing-a-missing-step                  | []                                                                                                                                                 |
      | move-step     | ↔️move-step/🧪️tests/no-ops-when-the-step-is-already-at-that-index     | [{"id":"s-intro","title":"Intro","blocks":[]}]                                                                                                     |
      | add-block     | 🧱add-block/🧪️tests/rejects-adding-a-block-to-a-missing-step           | []                                                                                                                                                 |
      | remove-block  | 🗑️remove-block/🧪️tests/rejects-removing-a-block-missing-from-its-step | [{"id":"s-intro","title":"Intro","blocks":[]}]                                                                                                     |
      | move-block    | 🔀move-block/🧪️tests/rejects-moving-a-block-into-a-missing-step        | [{"id":"s-intro","title":"Intro","blocks":[{"id":"b-notes","label":"Notes","kind":"text"}]}]                                                       |
      | replace-block | 🔄replace-block/🧪️tests/no-ops-when-the-block-is-already-identical     | [{"id":"s-intro","title":"Intro","blocks":[{"id":"b-size","label":"Team size","kind":"number","required":true,"min":1,"max":80,"unit":"people"}]}] |
      | update-step   | 🩹update-step/🧪️tests/no-ops-when-the-header-is-already-current        | [{"id":"s-intro","title":"Intro","description":"Warm up the room.","blocks":[]}]                                                                   |
      | change-title  | ✏️change-title/🧪️tests/changes-the-playbook-title                     | []                                                                                                                                                 |

  @id-inverse
  @level-exhaustive
  @mode-property
  Scenario Outline: Undoing <id> restores the committed before-snapshot
    Given the committed before-snapshot asset://🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/<vector>/📸️snapshot/⬅️before/🔣️component.json
    And the committed mutation payload asset://🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/<vector>/🦠️mutation/🔣️component.json
    When <id> is applied and then its own computed inverse is applied through apply_playbook_mutation_outcome
      """
      {"kind": "<id>", "vector": "<vector>", "scene": <scene>}
      """
    Then the projection is the committed before-snapshot's again, field for field
    Examples:
      | id            | vector                                                                | scene                                                                                                                                              |
      | add-step      | ➕add-step/🧪️tests/no-ops-on-a-duplicate-step-id                       | [{"id":"s-review","title":"Review","blocks":[]}]                                                                                                   |
      | remove-step   | ➖remove-step/🧪️tests/rejects-removing-a-missing-step                  | []                                                                                                                                                 |
      | move-step     | ↔️move-step/🧪️tests/no-ops-when-the-step-is-already-at-that-index     | [{"id":"s-intro","title":"Intro","blocks":[]}]                                                                                                     |
      | add-block     | 🧱add-block/🧪️tests/rejects-adding-a-block-to-a-missing-step           | []                                                                                                                                                 |
      | remove-block  | 🗑️remove-block/🧪️tests/rejects-removing-a-block-missing-from-its-step | [{"id":"s-intro","title":"Intro","blocks":[]}]                                                                                                     |
      | move-block    | 🔀move-block/🧪️tests/rejects-moving-a-block-into-a-missing-step        | [{"id":"s-intro","title":"Intro","blocks":[{"id":"b-notes","label":"Notes","kind":"text"}]}]                                                       |
      | replace-block | 🔄replace-block/🧪️tests/no-ops-when-the-block-is-already-identical     | [{"id":"s-intro","title":"Intro","blocks":[{"id":"b-size","label":"Team size","kind":"number","required":true,"min":1,"max":80,"unit":"people"}]}] |
      | update-step   | 🩹update-step/🧪️tests/no-ops-when-the-header-is-already-current        | [{"id":"s-intro","title":"Intro","description":"Warm up the room.","blocks":[]}]                                                                   |
      | change-title  | ✏️change-title/🧪️tests/changes-the-playbook-title                     | []                                                                                                                                                 |

  @id-identity-round-trip
  @level-long
  @mode-round-trip
  Scenario: Parse the real committed example document and print it back without losing or copying anything
    Given the real committed artifact asset://🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🎬️demo/🖼️assets/🗣️example.dsl.semio
    When the artifact is parsed to a PlaybookSnapshot, printed back to `.playbook` DSL and parsed again
    Then both parses agree on the same document and the printed text reproduces the committed bytes exactly
