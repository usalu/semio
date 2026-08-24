@capability-puzzle-5d-1-mutate
@no-oracle-puzzle-5d-mutation-semantics
@comparison-ordered-json-v1
@mutations-puzzle-5d-1-any
Feature: Replay every typed Puzzle 5d 1 mutation against its committed specification vector
  `s.puzzle.5d@1/*` is a semio-NATIVE document, carried as `.dsl.semio`/`.pack.semio`/`.op.semio`/
  `.spr.semio`. No third party reads those, and none is authoritative over `Puzzle5dMutation`, so this case
  rests on the recorded `puzzle-5d-mutation-semantics` no-oracle decision
  (`../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️oracle/🔣️component.json`) and its two named substitutes: the
  committed specification vectors, and the metamorphic laws below.

  The five dimensions are not five collections — they are one PART carrying two facets at once. A part has
  a 2d placement and a 3d placement simultaneously, which is why this vocabulary has a `move-part2d` AND a
  `move-part3d`, a `change-part2d-icon` beside a `change-part3d-mesh`, and why a create/delete pair has to
  bring both facets in and out together. Grips play the role handles play in the 2d subset, fasteners the
  role edges play, and — unlike either sibling — `kindCompatibility` is a TOP-LEVEL field here rather than
  a member of `meta`, so the compatibility kinds have a footprint of their own.

  One row deserves naming rather than hiding: `replace-kind-catalogs`'s ONLY committed vector is
  `null-catalogs-is-noop`, whose own `🎯️outcome` records a `mutation.no-op` warning. For a vector the
  fixture itself declares a no-op, the scenario asserts the OPPOSITE of observability: nothing moved, and
  the diff declares nothing. That the kind has no vector which actually replaces the catalogs is a real gap
  in this subset's production fixtures, visible here rather than papered over.

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
      | id                            | vector                                                       |
      | create-part                   | 🌱create-part/🧪️tests/appends-part-c                          |
      | delete-part                   | 🗑delete-part/🧪️tests/removes-part-a-and-severs-fastener      |
      | move-part2d                   | 📍move-part2d/🧪️tests/moves-part-a                            |
      | replace-part2d-geometry       | 🧊replace-part2d-geometry/🧪️tests/circle-to-rectangle         |
      | edit-part2d-text              | ✏️edit-part2d-text/🧪️tests/retitles-part-a                   |
      | change-part2d-icon            | 🎨change-part2d-icon/🧪️tests/swaps-icon                       |
      | change-part2d-hidden          | 🙈change-part2d-hidden/🧪️tests/hides-part-a                   |
      | change-part2d-locked          | 🔒change-part2d-locked/🧪️tests/locks-part-a                   |
      | move-part3d                   | 🚀move-part3d/🧪️tests/moves-part-a-in-world                   |
      | rotate-part3d                 | 🔃rotate-part3d/🧪️tests/half-turn-about-z                     |
      | scale-part3d                  | 📏scale-part3d/🧪️tests/uniform-double                         |
      | change-part3d-mesh            | 🧱change-part3d-mesh/🧪️tests/repoints-mesh                    |
      | edit-part3d-label             | 🖋️edit-part3d-label/🧪️tests/relabels-part-a                  |
      | change-part-kind              | 🏗change-part-kind/🧪️tests/reassigns-kind                     |
      | change-part-anchor            | ⚓change-part-anchor/🧪️tests/fixed-to-derived                 |
      | add-part-grip                 | ➕add-part-grip/🧪️tests/appends-grip-3                        |
      | remove-part-grip              | ➖remove-part-grip/🧪️tests/removes-grip-1-and-severs-fastener |
      | replace-part-grip             | 🔌replace-part-grip/🧪️tests/rekinds-grip-1                    |
      | connect-grips                 | 🔗connect-grips/🧪️tests/adds-second-fastener                  |
      | disconnect-grips              | ✂️disconnect-grips/🧪️tests/removes-fast-1                    |
      | replace-fastener-geometry     | 🧮replace-fastener-geometry/🧪️tests/repositions-fast-1        |
      | change-fastener-kind          | 🎯change-fastener-kind/🧪️tests/rekinds-fast-1                 |
      | rename-puzzle5d               | 🏷rename-puzzle5d/🧪️tests/relabels-document                   |
      | change-domain                 | 🌐change-domain/🧪️tests/architecture-to-engineering           |
      | change-description            | 📝change-description/🧪️tests/rewrites-description             |
      | connect-kind-compatibility    | 🤝connect-kind-compatibility/🧪️tests/adds-grip-pair           |
      | disconnect-kind-compatibility | 💔disconnect-kind-compatibility/🧪️tests/removes-grip-pair     |
      | replace-kind-catalogs         | 📚replace-kind-catalogs/🧪️tests/null-catalogs-is-noop         |

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
      | id                            | vector                                                       |
      | create-part                   | 🌱create-part/🧪️tests/appends-part-c                          |
      | delete-part                   | 🗑delete-part/🧪️tests/removes-part-a-and-severs-fastener      |
      | move-part2d                   | 📍move-part2d/🧪️tests/moves-part-a                            |
      | replace-part2d-geometry       | 🧊replace-part2d-geometry/🧪️tests/circle-to-rectangle         |
      | edit-part2d-text              | ✏️edit-part2d-text/🧪️tests/retitles-part-a                   |
      | change-part2d-icon            | 🎨change-part2d-icon/🧪️tests/swaps-icon                       |
      | change-part2d-hidden          | 🙈change-part2d-hidden/🧪️tests/hides-part-a                   |
      | change-part2d-locked          | 🔒change-part2d-locked/🧪️tests/locks-part-a                   |
      | move-part3d                   | 🚀move-part3d/🧪️tests/moves-part-a-in-world                   |
      | rotate-part3d                 | 🔃rotate-part3d/🧪️tests/half-turn-about-z                     |
      | scale-part3d                  | 📏scale-part3d/🧪️tests/uniform-double                         |
      | change-part3d-mesh            | 🧱change-part3d-mesh/🧪️tests/repoints-mesh                    |
      | edit-part3d-label             | 🖋️edit-part3d-label/🧪️tests/relabels-part-a                  |
      | change-part-kind              | 🏗change-part-kind/🧪️tests/reassigns-kind                     |
      | change-part-anchor            | ⚓change-part-anchor/🧪️tests/fixed-to-derived                 |
      | add-part-grip                 | ➕add-part-grip/🧪️tests/appends-grip-3                        |
      | remove-part-grip              | ➖remove-part-grip/🧪️tests/removes-grip-1-and-severs-fastener |
      | replace-part-grip             | 🔌replace-part-grip/🧪️tests/rekinds-grip-1                    |
      | connect-grips                 | 🔗connect-grips/🧪️tests/adds-second-fastener                  |
      | disconnect-grips              | ✂️disconnect-grips/🧪️tests/removes-fast-1                    |
      | replace-fastener-geometry     | 🧮replace-fastener-geometry/🧪️tests/repositions-fast-1        |
      | change-fastener-kind          | 🎯change-fastener-kind/🧪️tests/rekinds-fast-1                 |
      | rename-puzzle5d               | 🏷rename-puzzle5d/🧪️tests/relabels-document                   |
      | change-domain                 | 🌐change-domain/🧪️tests/architecture-to-engineering           |
      | change-description            | 📝change-description/🧪️tests/rewrites-description             |
      | connect-kind-compatibility    | 🤝connect-kind-compatibility/🧪️tests/adds-grip-pair           |
      | disconnect-kind-compatibility | 💔disconnect-kind-compatibility/🧪️tests/removes-grip-pair     |
      | replace-kind-catalogs         | 📚replace-kind-catalogs/🧪️tests/null-catalogs-is-noop         |

  @id-identity-round-trip
  @level-long
  @mode-round-trip
  Scenario: Decode and re-encode the two-part, one-fastener puzzle assembly
    Given the committed before-snapshot asset://🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌱create-part/🧪️tests/appends-part-c/📸️snapshot/⬅️before/🔣️component.json
    When it is parsed by the platform's own dependency-free JSON reader, re-serialized and parsed again
    Then the document is unchanged and the re-serialized bytes are not the committed bytes
