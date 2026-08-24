@capability-puzzle-2d-1-mutate
@no-oracle-puzzle-2d-mutation-semantics
@comparison-ordered-json-v1
@mutations-puzzle-2d-1-any
Feature: Replay every typed Puzzle 2d 1 mutation against its committed specification vector
  `s.puzzle.2d@1/*` is a semio-NATIVE document, carried as `.dsl.semio`/`.pack.semio`/`.op.semio`/
  `.spr.semio`. No third party reads those, and none is authoritative over `Puzzle2dMutation`, so this case
  rests on the recorded `puzzle-2d-mutation-semantics` no-oracle decision
  (`../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️oracle/🔣️component.json`) and its two named substitutes: the
  committed specification vectors, and the metamorphic laws below.

  What distinguishes this subset is that its connectivity is split across two levels. A node owns its
  HANDLES; an edge is a top-level record naming two of them. So `add-node-handle` writes into the `nodes`
  collection while `connect-handles` writes into `edges`, and `disconnect-handles` has to reach back the
  other way — three kinds, three different footprints, one relationship. The kind catalogs and the
  compatibility relation are filed in `meta` rather than beside the elements they govern, which is why
  `connect-kind-compatibility`, `disconnect-kind-compatibility` and `replace-kind-catalogs` all land on
  `meta` and on nothing else.

  One row deserves naming rather than hiding: `replace-node-handle`'s ONLY committed vector is
  `rekind-handle-1-is-noop`, whose own `🎯️outcome` records a `mutation.no-op` warning. This feature does
  not let that report a green the way a real mutation would — for a vector the fixture itself declares a
  no-op, the scenario asserts the OPPOSITE of observability: nothing moved, and the diff declares nothing.
  That the kind has no vector which actually replaces a handle is a real gap in this subset's production
  fixtures, visible here rather than papered over.

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
      | id                            | vector                                                          |
      | create-node                   | 🌱create-node/🧪️tests/appends-node-c                             |
      | delete-node                   | 🗑delete-node/🧪️tests/removes-node-a-and-severs-edge             |
      | move-node                     | 📍move-node/🧪️tests/moves-node-a                                 |
      | replace-node-geometry         | 🧊replace-node-geometry/🧪️tests/circle-to-rectangle              |
      | change-node-kind              | 🏗change-node-kind/🧪️tests/reassigns-node-a-kind                 |
      | edit-node-text                | ✏️edit-node-text/🧪️tests/retitles-node-a                        |
      | change-node-icon              | 🎨change-node-icon/🧪️tests/swaps-node-a-icon                     |
      | scale-node                    | 📏scale-node/🧪️tests/doubles-node-a                              |
      | change-node-visible           | 👁change-node-visible/🧪️tests/hides-node-a                       |
      | change-node-locked            | 🔒change-node-locked/🧪️tests/locks-node-a                        |
      | change-node-root              | 🌟change-node-root/🧪️tests/promotes-node-a-to-root               |
      | change-node-anchor            | ⚓change-node-anchor/🧪️tests/fixed-to-derived                    |
      | add-node-handle               | ➕add-node-handle/🧪️tests/appends-handle-3-to-node-b             |
      | remove-node-handle            | ➖remove-node-handle/🧪️tests/removes-handle-2-and-severs-edge    |
      | replace-node-handle           | 🔌replace-node-handle/🧪️tests/rekind-handle-1-is-noop            |
      | connect-handles               | 🔗connect-handles/🧪️tests/adds-second-edge                       |
      | disconnect-handles            | ✂️disconnect-handles/🧪️tests/removes-edge-1                     |
      | replace-edge-geometry         | 🧮replace-edge-geometry/🧪️tests/repositions-edge-1               |
      | change-edge-kind              | 🏷change-edge-kind/🧪️tests/rekinds-edge-1                        |
      | change-edge-tips              | 🖇change-edge-tips/🧪️tests/swaps-edge-1-tips                     |
      | change-edge-visible           | 👀change-edge-visible/🧪️tests/hides-edge-1                       |
      | change-edge-locked            | 🔐change-edge-locked/🧪️tests/locks-edge-1                        |
      | change-manifest-id            | 🆔change-manifest-id/🧪️tests/repoints-manifest                   |
      | connect-kind-compatibility    | 🤝connect-kind-compatibility/🧪️tests/adds-handle-kind-pair       |
      | disconnect-kind-compatibility | 💔disconnect-kind-compatibility/🧪️tests/removes-handle-kind-pair |
      | replace-kind-catalogs         | 📚replace-kind-catalogs/🧪️tests/installs-handle-kind-catalog     |

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
      | id                            | vector                                                          |
      | create-node                   | 🌱create-node/🧪️tests/appends-node-c                             |
      | delete-node                   | 🗑delete-node/🧪️tests/removes-node-a-and-severs-edge             |
      | move-node                     | 📍move-node/🧪️tests/moves-node-a                                 |
      | replace-node-geometry         | 🧊replace-node-geometry/🧪️tests/circle-to-rectangle              |
      | change-node-kind              | 🏗change-node-kind/🧪️tests/reassigns-node-a-kind                 |
      | edit-node-text                | ✏️edit-node-text/🧪️tests/retitles-node-a                        |
      | change-node-icon              | 🎨change-node-icon/🧪️tests/swaps-node-a-icon                     |
      | scale-node                    | 📏scale-node/🧪️tests/doubles-node-a                              |
      | change-node-visible           | 👁change-node-visible/🧪️tests/hides-node-a                       |
      | change-node-locked            | 🔒change-node-locked/🧪️tests/locks-node-a                        |
      | change-node-root              | 🌟change-node-root/🧪️tests/promotes-node-a-to-root               |
      | change-node-anchor            | ⚓change-node-anchor/🧪️tests/fixed-to-derived                    |
      | add-node-handle               | ➕add-node-handle/🧪️tests/appends-handle-3-to-node-b             |
      | remove-node-handle            | ➖remove-node-handle/🧪️tests/removes-handle-2-and-severs-edge    |
      | replace-node-handle           | 🔌replace-node-handle/🧪️tests/rekind-handle-1-is-noop            |
      | connect-handles               | 🔗connect-handles/🧪️tests/adds-second-edge                       |
      | disconnect-handles            | ✂️disconnect-handles/🧪️tests/removes-edge-1                     |
      | replace-edge-geometry         | 🧮replace-edge-geometry/🧪️tests/repositions-edge-1               |
      | change-edge-kind              | 🏷change-edge-kind/🧪️tests/rekinds-edge-1                        |
      | change-edge-tips              | 🖇change-edge-tips/🧪️tests/swaps-edge-1-tips                     |
      | change-edge-visible           | 👀change-edge-visible/🧪️tests/hides-edge-1                       |
      | change-edge-locked            | 🔐change-edge-locked/🧪️tests/locks-edge-1                        |
      | change-manifest-id            | 🆔change-manifest-id/🧪️tests/repoints-manifest                   |
      | connect-kind-compatibility    | 🤝connect-kind-compatibility/🧪️tests/adds-handle-kind-pair       |
      | disconnect-kind-compatibility | 💔disconnect-kind-compatibility/🧪️tests/removes-handle-kind-pair |
      | replace-kind-catalogs         | 📚replace-kind-catalogs/🧪️tests/installs-handle-kind-catalog     |

  @id-identity-round-trip
  @level-long
  @mode-round-trip
  Scenario: Decode and re-encode the two-node, one-edge puzzle drawing
    Given the committed before-snapshot asset://🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌱create-node/🧪️tests/appends-node-c/📸️snapshot/⬅️before/🔣️component.json
    When it is parsed by the platform's own dependency-free JSON reader, re-serialized and parsed again
    Then the document is unchanged and the re-serialized bytes are not the committed bytes
