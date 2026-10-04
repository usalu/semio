@capability-playbook-1-mutate
@oracle-playbook-python-independent
@comparison-ordered-json-v1
@mutations-playbook-1-any
Feature: Apply every typed playbook parent mutation twice — once in Rust, once in Python — and require the same answer
  This case is a CROSS-LANGUAGE DIFFERENTIAL. The reference is `🐍️.py` in this directory: a second implementation of the
  `s.playbook.playbook` parent document and its one parent-lane mutation, `change-title`, written in Python from
  `🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🔣️.json` and the committed vector. It imports nothing from this
  repository's Rust.

  📌️ WHAT THIS CASE COVERS. The playbook snapshot carries `schema`, `id`, `version`, a title and ONE composed `flow` child
  handle; its steps and blocks are content of that child and are edited only on the child's own lane (design §20.15 of ticket
  26/09/30/NON-DESTRUCTIVE-HISTORY-EDITING), so `change-title` is the whole parent vocabulary. The child-leaf builders the
  step/block verbs use are pinned by the language-neutral vectors `🗿️artifacts/📖️playbook/🧫️fixtures/🧫️child-leaves/🔣️.json`,
  which another independent Python implementation writes and the Rust law folds and undoes.

  🚧️ ONE SCENARIO IS REFUSED BY CLAUSE: `identity-round-trip`. The committed grammar
  `🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/📝️text/📖️.grammar.semio` describes a DIFFERENT DOCUMENT (the generic
  `family-scene` canvas grammar) while the committed artifact carries HEX-ENCODED scalars and a `[hex,hex]` child-handle pair the
  grammar never mentions, so a second implementation cannot read it.

  @id-mutate
  @level-exhaustive
  @mode-differential
  Scenario Outline: Applying <id> to its committed before-snapshot yields the committed after-snapshot
    Given the committed before-snapshot shared://🧬️mutations/<vector>/📸️snapshot/⬅️before/🔣️.json
    And the committed mutation payload shared://🧬️mutations/<vector>/🦠️mutation/🔣️.json
    And the committed after-snapshot shared://🧬️mutations/<vector>/📸️snapshot/➡️after/🔣️.json
    And the committed outcome vector shared://🧬️mutations/<vector>/🎯️outcome/🔣️.json
    When <id> is applied through apply_playbook_mutation_outcome
      """
      {"kind": "<id>", "vector": "<vector>"}
      """
    Then the resulting snapshot is the committed after-snapshot and the raised diagnostics are the committed outcome's
    Examples:
      | id           | vector                  |
      | change-title | ✏️change-title/🧪️changes |

  @id-inverse
  @level-exhaustive
  @mode-differential
  Scenario Outline: Undoing <id> restores the committed before-snapshot
    Given the committed before-snapshot shared://🧬️mutations/<vector>/📸️snapshot/⬅️before/🔣️.json
    And the committed mutation payload shared://🧬️mutations/<vector>/🦠️mutation/🔣️.json
    When <id> is applied and then its own computed inverse is applied through apply_playbook_mutation_outcome
      """
      {"kind": "<id>", "vector": "<vector>"}
      """
    Then the projection is the committed before-snapshot's again, field for field
    Examples:
      | id           | vector                  |
      | change-title | ✏️change-title/🧪️changes |

  @id-identity-round-trip
  @level-long
  @mode-round-trip
  Scenario: Parse the real committed example document and print it back without losing or copying anything
    Given the real committed artifact asset://🎬️demo/🗣️.dsl.semio
    When the artifact is parsed to a PlaybookSnapshot, printed back to `.playbook` DSL and parsed again
    Then both parses agree on the same document and the printed text reproduces the committed bytes exactly
