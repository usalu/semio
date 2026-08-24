@capability-puzzle-3d-1-mutate
@no-oracle-puzzle-3d-mutation-semantics
@comparison-ordered-json-v1
@mutations-puzzle-3d-1-any
Feature: Replay every typed Puzzle 3d 1 mutation against its committed specification vector
  `s.puzzle.3d@1/*` is a semio-NATIVE document, carried as `.dsl.semio`/`.pack.semio`/`.op.semio`/
  `.spr.semio`. No third party reads those, and none is authoritative over `Puzzle3dMutation`, so this case
  rests on the recorded `puzzle-3d-mutation-semantics` no-oracle decision
  (`../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️oracle/🔣️component.json`) and its two named substitutes: the
  committed specification vectors, and the metamorphic laws below.

  This is the widest vocabulary in the puzzle plugin — thirty-five kinds over FOUR independent top-level
  collections. `objects` hold vortices; `attractions` name their endpoints as `object:vortex` pairs, so an
  attraction refers into a collection it does not contain and `remove-object-vortex` can strand one;
  `targetVolumes` are placed solids with their own move/rotate/scale/hidden/locked family; and `references`
  are image planes with a source, a width and their own lifecycle. Four collections mean four different
  footprints, which is what makes the footprint law informative here where the shallow `💠️lowpoly` and
  `🌀️procedural` documents give it almost nothing to say.

  One row deserves naming rather than hiding: `replace-object-vortex`'s ONLY committed vector is
  `rekind-vortex-1-is-noop`, whose own `🎯️outcome` records a `mutation.no-op` warning. For a vector the
  fixture itself declares a no-op, the scenario asserts the OPPOSITE of observability: nothing moved, and
  the diff declares nothing. That the kind has no vector which actually replaces a vortex is a real gap in
  this subset's production fixtures, visible here rather than papered over.

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
      | id                            | vector                                                               |
      | create-object                 | 🌱create-object/🧪️tests/appends-object-c                              |
      | delete-object                 | 🗑delete-object/🧪️tests/removes-object-a-and-severs-attraction        |
      | move-object                   | 📍move-object/🧪️tests/moves-object-a                                  |
      | rotate-object                 | 🔃rotate-object/🧪️tests/half-turn-about-z                             |
      | scale-object                  | 📏scale-object/🧪️tests/uniform-to-per-axis                            |
      | change-object-mesh            | 🧱change-object-mesh/🧪️tests/repoints-object-a-mesh                   |
      | edit-object-label             | 🖋️edit-object-label/🧪️tests/relabels-object-a                        |
      | change-object-kind            | 🏗change-object-kind/🧪️tests/reassigns-object-a-kind                  |
      | change-object-anchor          | ⚓change-object-anchor/🧪️tests/fixed-to-derived                       |
      | change-object-hidden          | 👁change-object-hidden/🧪️tests/hides-object-a                         |
      | change-object-locked          | 🔒change-object-locked/🧪️tests/locks-object-a                         |
      | add-object-vortex             | ➕add-object-vortex/🧪️tests/appends-vortex-3-to-object-b              |
      | remove-object-vortex          | ➖remove-object-vortex/🧪️tests/removes-vortex-2-and-severs-attraction |
      | replace-object-vortex         | 🔌replace-object-vortex/🧪️tests/rekind-vortex-1-is-noop               |
      | connect-vortices              | 🔗connect-vortices/🧪️tests/adds-second-attraction                     |
      | disconnect-vortices           | ✂️disconnect-vortices/🧪️tests/removes-attraction-1                   |
      | replace-attraction-geometry   | 🧮replace-attraction-geometry/🧪️tests/repositions-attraction-1        |
      | create-target-volume          | 🌍create-target-volume/🧪️tests/appends-volume-2                       |
      | delete-target-volume          | 🪦delete-target-volume/🧪️tests/removes-volume-1                       |
      | move-target-volume            | 🚀move-target-volume/🧪️tests/lifts-volume-1                           |
      | rotate-target-volume          | 🌀rotate-target-volume/🧪️tests/half-turn-about-z                      |
      | scale-target-volume           | 📐scale-target-volume/🧪️tests/per-axis-to-uniform                     |
      | change-target-volume-hidden   | 🙈change-target-volume-hidden/🧪️tests/hides-volume-1                  |
      | change-target-volume-locked   | 🔐change-target-volume-locked/🧪️tests/locks-volume-1                  |
      | create-reference              | 🖼create-reference/🧪️tests/appends-reference-2                        |
      | delete-reference              | 🚮delete-reference/🧪️tests/removes-reference-1                        |
      | move-reference                | 🎯move-reference/🧪️tests/slides-reference-1                           |
      | resize-reference              | 📎resize-reference/🧪️tests/widens-reference-1                         |
      | replace-reference-source      | 🖇replace-reference-source/🧪️tests/repoints-reference-1-source        |
      | change-reference-hidden       | 👀change-reference-hidden/🧪️tests/hides-reference-1                   |
      | change-reference-locked       | 🗝change-reference-locked/🧪️tests/locks-reference-1                   |
      | change-domain                 | 🌐change-domain/🧪️tests/architecture-to-engineering                   |
      | connect-kind-compatibility    | 🤝connect-kind-compatibility/🧪️tests/adds-vortex-kind-pair            |
      | disconnect-kind-compatibility | 💔disconnect-kind-compatibility/🧪️tests/removes-vortex-kind-pair      |
      | replace-kind-catalogs         | 📚replace-kind-catalogs/🧪️tests/installs-vortex-kind-catalog          |

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
      | id                            | vector                                                               |
      | create-object                 | 🌱create-object/🧪️tests/appends-object-c                              |
      | delete-object                 | 🗑delete-object/🧪️tests/removes-object-a-and-severs-attraction        |
      | move-object                   | 📍move-object/🧪️tests/moves-object-a                                  |
      | rotate-object                 | 🔃rotate-object/🧪️tests/half-turn-about-z                             |
      | scale-object                  | 📏scale-object/🧪️tests/uniform-to-per-axis                            |
      | change-object-mesh            | 🧱change-object-mesh/🧪️tests/repoints-object-a-mesh                   |
      | edit-object-label             | 🖋️edit-object-label/🧪️tests/relabels-object-a                        |
      | change-object-kind            | 🏗change-object-kind/🧪️tests/reassigns-object-a-kind                  |
      | change-object-anchor          | ⚓change-object-anchor/🧪️tests/fixed-to-derived                       |
      | change-object-hidden          | 👁change-object-hidden/🧪️tests/hides-object-a                         |
      | change-object-locked          | 🔒change-object-locked/🧪️tests/locks-object-a                         |
      | add-object-vortex             | ➕add-object-vortex/🧪️tests/appends-vortex-3-to-object-b              |
      | remove-object-vortex          | ➖remove-object-vortex/🧪️tests/removes-vortex-2-and-severs-attraction |
      | replace-object-vortex         | 🔌replace-object-vortex/🧪️tests/rekind-vortex-1-is-noop               |
      | connect-vortices              | 🔗connect-vortices/🧪️tests/adds-second-attraction                     |
      | disconnect-vortices           | ✂️disconnect-vortices/🧪️tests/removes-attraction-1                   |
      | replace-attraction-geometry   | 🧮replace-attraction-geometry/🧪️tests/repositions-attraction-1        |
      | create-target-volume          | 🌍create-target-volume/🧪️tests/appends-volume-2                       |
      | delete-target-volume          | 🪦delete-target-volume/🧪️tests/removes-volume-1                       |
      | move-target-volume            | 🚀move-target-volume/🧪️tests/lifts-volume-1                           |
      | rotate-target-volume          | 🌀rotate-target-volume/🧪️tests/half-turn-about-z                      |
      | scale-target-volume           | 📐scale-target-volume/🧪️tests/per-axis-to-uniform                     |
      | change-target-volume-hidden   | 🙈change-target-volume-hidden/🧪️tests/hides-volume-1                  |
      | change-target-volume-locked   | 🔐change-target-volume-locked/🧪️tests/locks-volume-1                  |
      | create-reference              | 🖼create-reference/🧪️tests/appends-reference-2                        |
      | delete-reference              | 🚮delete-reference/🧪️tests/removes-reference-1                        |
      | move-reference                | 🎯move-reference/🧪️tests/slides-reference-1                           |
      | resize-reference              | 📎resize-reference/🧪️tests/widens-reference-1                         |
      | replace-reference-source      | 🖇replace-reference-source/🧪️tests/repoints-reference-1-source        |
      | change-reference-hidden       | 👀change-reference-hidden/🧪️tests/hides-reference-1                   |
      | change-reference-locked       | 🗝change-reference-locked/🧪️tests/locks-reference-1                   |
      | change-domain                 | 🌐change-domain/🧪️tests/architecture-to-engineering                   |
      | connect-kind-compatibility    | 🤝connect-kind-compatibility/🧪️tests/adds-vortex-kind-pair            |
      | disconnect-kind-compatibility | 💔disconnect-kind-compatibility/🧪️tests/removes-vortex-kind-pair      |
      | replace-kind-catalogs         | 📚replace-kind-catalogs/🧪️tests/installs-vortex-kind-catalog          |

  @id-identity-round-trip
  @level-long
  @mode-round-trip
  Scenario: Decode and re-encode the four-collection puzzle scene
    Given the committed before-snapshot asset://🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌱create-object/🧪️tests/appends-object-c/📸️snapshot/⬅️before/🔣️component.json
    When it is parsed by the platform's own dependency-free JSON reader, re-serialized and parsed again
    Then the document is unchanged and the re-serialized bytes are not the committed bytes
