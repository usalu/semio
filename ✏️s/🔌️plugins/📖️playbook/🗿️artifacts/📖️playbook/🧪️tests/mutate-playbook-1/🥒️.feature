@capability-playbook-1-mutate
@oracle-playbook-python-independent
@comparison-ordered-json-v1
@mutations-playbook-1-any
Feature: Apply every typed playbook document mutation twice — once in Rust, once in Python — and require the same answer
  This case is a CROSS-LANGUAGE DIFFERENTIAL. The reference is `🐍️component.py` in this directory: a
  second implementation of the `s.playbook.playbook` document and all nine typed mutations, written in
  Python from `🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🔣️component.json`, from rules 1, 2
  and 3 of `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/📓️derivation-rules.md`,
  and from the nine committed vectors together with the `scene` array each scenario carries in
  its own doc string. It imports nothing from this repository's Rust.

  Why a second implementation rather than a third-party library, and why the previous answer was
  wrong. This case used to say that because this artifact is persisted through this subset's own
  codecs and no third party reads them, there is no reference to register. The fifteen `📕️norm` and
  nineteen `🧿️semio` references refuted that in this same wave over this same carrier. A third-party
  library was nonetheless declined and the reason is concrete: no form or checklist format models a programme whose content is a CHILD
  ARTIFACT addressed by content, and none of them reads this carrier.

  📌️ WHAT THIS CASE'S EVIDENCE ACTUALLY COVERS, stated plainly rather than left to be inferred from a
  green row. Eight of the nine committed vectors leave the snapshot BYTE-IDENTICAL,
  because Eight of the nine kinds address steps and blocks that live in the CHILD
  SCENE and not in this document — the snapshot carries only `schema`, `id`, `version`, a title and
  two composed child handles. What those vectors really pin is a DIAGNOSTIC, and the reference DERIVES
  it from the scene the doc string carries — status, code and path — rather than reading it off the
  committed outcome, which is the only way this comparison says anything at all. So this case's
  evidence is ONE applied mutation and Eight diagnostics: no committed vector in it
  exercises a add/remove/move/replace that SUCCEEDS. That is a real gap in the case's fixtures, and the reference
  states it rather than passing over it.

  📌️ A CROSS-CASE DIVERGENCE THE REFERENCE SURFACED, which neither case could see alone.
  `s.forms.form` is the same shape with the same verbs, and the two subsets answer the same
  situation differently: a duplicate step id is an APPLIED `mutation.no-op` here (`add-step`) and a
  REJECTED `mutation.duplicate-id` there (`create-step`); a block added to a step that does not exist
  is `mutation.target-missing` here (`add-block`) and `mutation.invariant` there (`create-block`). Neither divergence is stated anywhere. Both are visible only
  because one reference was written against both surfaces, and both are reported rather than absorbed
  into a per-case table.

  📌️ A SIBLING NOTE, because the count of second implementations must not be overstated. This
  reference and `mutate-forms-1`'s are ONE implementation instantiated twice, differing in the verb
  names, in the diagnostic each situation raises and in the handle members. That the two
  instantiations DISAGREE on two situations is the finding above.

  🚧️ ONE OF THE NINETEEN SCENARIOS IS REFUSED BY CLAUSE: `identity-round-trip`. The committed
  grammar `🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/📝️text/📖️component.grammar.semio`
  describes a DIFFERENT DOCUMENT — it is the generic `family-scene` canvas grammar,
  `doc-body = schema-line layers-block` with shape/path/text layers and `id`/`x`/`y`/`fill`/`stroke`/
  `opacity` fields — while the committed artifact carries no `layers` block at all and instead five
  HEX-ENCODED scalars and two `[hex,hex]` child-handle pairs the grammar never mentions. Nothing
  committed says the values are hex, that a pair is `(childId, target)`, or how the second element's
  `<artifactId>!<kind>@<standard>/<subset>` spelling is split. Four more subsets — `📋️forms`,
  `📏️layout`, `🖍️draw` and `🖨️raster` — carry that same canvas grammar over four equally unrelated
  documents, differing from this one only in their `grammar`, `extension` and `artifact-mark` lines.

  @id-mutate
  @level-exhaustive
  @mode-differential
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
  @mode-differential
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
