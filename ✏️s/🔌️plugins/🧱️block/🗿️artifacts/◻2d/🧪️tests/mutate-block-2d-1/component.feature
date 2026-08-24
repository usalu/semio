@capability-block-2d-1-mutate
@no-oracle-block-2d-mutation-semantics
@comparison-ordered-json-v1
@mutations-block-2d-1-any
Feature: Replay every typed Block 2d 1 mutation against its committed specification vector
  `s.block.2d@1/*` is a semio-NATIVE document, carried as `.dsl.semio`/`.pack.semio`. No third party reads
  those, and none is authoritative over `Block2dMutation`, so this case rests on the recorded
  `block-2d-mutation-semantics` no-oracle decision
  (`../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️oracle/🔣️component.json`) and its two named substitutes: the
  committed specification vectors, and the metamorphic laws below.

  A block document is not a drawing — it is the DEFINITION of one node kind, which is what makes this
  vocabulary unlike every puzzle one. Six kinds edit the identity record alone (`rename-node-kind` changes
  the machine name, `change-node-kind-label` the human one, and they are deliberately separate verbs), one
  replaces the 2d presentation wholesale, and the rest maintain the four catalogues a kind publishes:
  handle kinds, placed handles, compatibility rules and attributes — plus authorship and the editor camera,
  which are document metadata and are mutated by their own approved verbs rather than smuggled into a
  generic patch.

  Because nearly every kind has its OWN top-level field, the footprint law is unusually sharp here: an edit
  to `handleKinds` that also disturbed `handles`, or a `rename-node-kind` that reached into `presentation`,
  fails the inverse scenario rather than passing on a projection that happened to look right.

  Every scenario replays one committed `(before, mutation, diff, outcome, after)` quintet — the same
  bytes the production crate's own fixture tests beside each leaf assert against — end to end through
  the test platform. The vector each row names is written out in full in the row itself, so the
  provenance of every input is readable here and pinned by digest at plan time.

  @id-mutate
  @level-exhaustive
  @mode-conformance
  Scenario Outline: The committed <id> vector declares its own kind and moves the document
    Given the committed specification vector for the <id> kind
      """
      {
        "kind": "<id>",
        "before": "asset://🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/<vector>/📸️snapshot/⬅️before/🔣️component.json",
        "mutation": "asset://🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/<vector>/🦠️mutation/🔣️component.json",
        "diff": "asset://🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/<vector>/🔺️diff/🔣️component.json",
        "outcome": "asset://🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/<vector>/🎯️outcome/🔣️component.json",
        "after": "asset://🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/<vector>/📸️snapshot/➡️after/🔣️component.json"
      }
      """
    Then the committed mutation payload declares the <id> kind
    And the after-snapshot differs from the before-snapshot, or the committed outcome declares the vector a no-op
    Examples:
      | id                                   | vector                                                                       |
      | rename-node-kind                     | ✏️rename-node-kind/🧪️tests/renames-node-kind-to-gate                         |
      | change-node-kind-label               | 🏷️change-node-kind-label/🧪️tests/relabels-node-kind                          |
      | change-node-kind-variant             | 🔀️change-node-kind-variant/🧪️tests/switches-variant-to-b                     |
      | change-node-kind-description         | 📃️change-node-kind-description/🧪️tests/rewrites-node-kind-description        |
      | change-node-kind-icon                | 🖼️change-node-kind-icon/🧪️tests/repoints-node-kind-icon                      |
      | change-node-kind-unit                | 📐️change-node-kind-unit/🧪️tests/switches-unit-to-metre                       |
      | update-presentation                  | 🖌️update-presentation/🧪️tests/circle-to-rectangle                            |
      | create-handle-kind                   | 🌱️create-handle-kind/🧪️tests/appends-ground-handle-kind                      |
      | delete-handle-kind                   | 🗑️delete-handle-kind/🧪️tests/removes-power-handle-kind                       |
      | rename-handle-kind                   | ✒️rename-handle-kind/🧪️tests/renames-power-to-mains                          |
      | change-handle-kind-label             | 🔖️change-handle-kind-label/🧪️tests/relabels-power-handle-kind                |
      | change-handle-kind-color             | 🎨️change-handle-kind-color/🧪️tests/recolors-power-handle-kind                |
      | change-handle-kind-default-wire-kind | 🔌️change-handle-kind-default-wire-kind/🧪️tests/swaps-power-default-wire-kind |
      | create-handle                        | 🌿️create-handle/🧪️tests/appends-out-handle                                   |
      | delete-handle                        | ❌️delete-handle/🧪️tests/removes-in-handle                                    |
      | move-handle                          | 📍️move-handle/🧪️tests/swings-in-handle-along-the-rim                         |
      | change-handle-handle-kind            | 🧷️change-handle-handle-kind/🧪️tests/rekinds-in-handle-as-power               |
      | add-compatibility-rule               | ➕️add-compatibility-rule/🧪️tests/allows-signal-to-power                      |
      | remove-compatibility-rule            | ➖️remove-compatibility-rule/🧪️tests/revokes-signal-to-signal                 |
      | add-attribute                        | 🧩️add-attribute/🧪️tests/adds-pressure-attribute                              |
      | remove-attribute                     | 🚫️remove-attribute/🧪️tests/drops-material-attribute                          |
      | add-author                           | 👤️add-author/🧪️tests/credits-bo                                              |
      | remove-author                        | 🚷️remove-author/🧪️tests/uncredits-ada                                        |
      | move-camera2d                        | 🎥️move-camera2d/🧪️tests/pans-camera                                          |
      | scale-camera2d                       | 🔍️scale-camera2d/🧪️tests/zooms-camera-in                                     |
      | change-meta-description              | 💬️change-meta-description/🧪️tests/rewrites-session-notes                     |

  @id-inverse
  @level-exhaustive
  @mode-property
  Scenario Outline: The committed <id> vector changes only what its diff declares
    Given the committed specification vector for the <id> kind
      """
      {
        "kind": "<id>",
        "before": "asset://🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/<vector>/📸️snapshot/⬅️before/🔣️component.json",
        "mutation": "asset://🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/<vector>/🦠️mutation/🔣️component.json",
        "diff": "asset://🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/<vector>/🔺️diff/🔣️component.json",
        "outcome": "asset://🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/<vector>/🎯️outcome/🔣️component.json",
        "after": "asset://🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/<vector>/📸️snapshot/➡️after/🔣️component.json"
      }
      """
    Then every field where the after-snapshot differs from the before-snapshot is declared by the committed diff
    And every field the committed diff declares actually differs
    Examples:
      | id                                   | vector                                                                       |
      | rename-node-kind                     | ✏️rename-node-kind/🧪️tests/renames-node-kind-to-gate                         |
      | change-node-kind-label               | 🏷️change-node-kind-label/🧪️tests/relabels-node-kind                          |
      | change-node-kind-variant             | 🔀️change-node-kind-variant/🧪️tests/switches-variant-to-b                     |
      | change-node-kind-description         | 📃️change-node-kind-description/🧪️tests/rewrites-node-kind-description        |
      | change-node-kind-icon                | 🖼️change-node-kind-icon/🧪️tests/repoints-node-kind-icon                      |
      | change-node-kind-unit                | 📐️change-node-kind-unit/🧪️tests/switches-unit-to-metre                       |
      | update-presentation                  | 🖌️update-presentation/🧪️tests/circle-to-rectangle                            |
      | create-handle-kind                   | 🌱️create-handle-kind/🧪️tests/appends-ground-handle-kind                      |
      | delete-handle-kind                   | 🗑️delete-handle-kind/🧪️tests/removes-power-handle-kind                       |
      | rename-handle-kind                   | ✒️rename-handle-kind/🧪️tests/renames-power-to-mains                          |
      | change-handle-kind-label             | 🔖️change-handle-kind-label/🧪️tests/relabels-power-handle-kind                |
      | change-handle-kind-color             | 🎨️change-handle-kind-color/🧪️tests/recolors-power-handle-kind                |
      | change-handle-kind-default-wire-kind | 🔌️change-handle-kind-default-wire-kind/🧪️tests/swaps-power-default-wire-kind |
      | create-handle                        | 🌿️create-handle/🧪️tests/appends-out-handle                                   |
      | delete-handle                        | ❌️delete-handle/🧪️tests/removes-in-handle                                    |
      | move-handle                          | 📍️move-handle/🧪️tests/swings-in-handle-along-the-rim                         |
      | change-handle-handle-kind            | 🧷️change-handle-handle-kind/🧪️tests/rekinds-in-handle-as-power               |
      | add-compatibility-rule               | ➕️add-compatibility-rule/🧪️tests/allows-signal-to-power                      |
      | remove-compatibility-rule            | ➖️remove-compatibility-rule/🧪️tests/revokes-signal-to-signal                 |
      | add-attribute                        | 🧩️add-attribute/🧪️tests/adds-pressure-attribute                              |
      | remove-attribute                     | 🚫️remove-attribute/🧪️tests/drops-material-attribute                          |
      | add-author                           | 👤️add-author/🧪️tests/credits-bo                                              |
      | remove-author                        | 🚷️remove-author/🧪️tests/uncredits-ada                                        |
      | move-camera2d                        | 🎥️move-camera2d/🧪️tests/pans-camera                                          |
      | scale-camera2d                       | 🔍️scale-camera2d/🧪️tests/zooms-camera-in                                     |
      | change-meta-description              | 💬️change-meta-description/🧪️tests/rewrites-session-notes                     |

  @id-identity-round-trip
  @level-long
  @mode-round-trip
  Scenario: Decode and re-encode the fully populated node-kind definition
    Given the committed before-snapshot asset://🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✏️rename-node-kind/🧪️tests/renames-node-kind-to-gate/📸️snapshot/⬅️before/🔣️component.json
    When it is parsed by the platform's own dependency-free JSON reader, re-serialized and parsed again
    Then the document is unchanged and the re-serialized bytes are not the committed bytes
