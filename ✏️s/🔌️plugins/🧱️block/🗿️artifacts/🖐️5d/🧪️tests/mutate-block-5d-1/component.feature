@capability-block-5d-1-mutate
@no-oracle-block-5d-mutation-semantics
@comparison-ordered-json-v1
@mutations-block-5d-1-any
Feature: Replay every typed Block 5d 1 mutation against its committed specification vector
  `s.block.5d@1/*` is a semio-NATIVE document, carried as `.dsl.semio`/`.pack.semio`. No third party reads
  those, and none is authoritative over `Block5dMutation`, so this case rests on the recorded
  `block-5d-mutation-semantics` no-oracle decision
  (`../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️oracle/🔣️component.json`) and its two named substitutes: the
  committed specification vectors, and the metamorphic laws below.

  This is the widest vocabulary in the block plugin — forty-one kinds — because a part kind carries BOTH
  facets at once. `update-part2d` and `update-part3d` are separate verbs on the same definition;
  `move-grip2d`, `move-grip3d` and `resize-grip3d` are three verbs where the 2d sibling has one; there are
  two cameras, each with its own move and scale; and on top of that sits a representation catalogue with
  mesh urls, LODs, descriptions, tags and per-representation attributes that the 2d sibling has no
  equivalent for at all.

  It is also the only subset in this scope whose diff RENAMES snapshot fields. The snapshot calls the two
  facets `2d` and `3d` — names no Rust identifier can carry — and the diff calls them `part2d` and
  `part3d`. The adapter's alias table states that mapping explicitly; without it the footprint law would
  report `update-part2d` as an undeclared change on every single run.

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
      | id                                 | vector                                                                     |
      | rename-part-kind                   | ✏️rename-part-kind/🧪️tests/renames-part-kind-to-pod                        |
      | change-part-kind-label             | 🏷️change-part-kind-label/🧪️tests/relabels-part-kind                        |
      | change-part-kind-variant           | 🔀️change-part-kind-variant/🧪️tests/switches-variant-to-b                   |
      | change-part-kind-description       | 📃️change-part-kind-description/🧪️tests/rewrites-part-kind-description      |
      | change-part-kind-icon              | 🖼️change-part-kind-icon/🧪️tests/repoints-part-kind-icon                    |
      | change-part-kind-unit              | 📐change-part-kind-unit/🧪️tests/switches-unit-to-centimeter                 |
      | update-part2d                      | 🖌️update-part-2d/🧪️tests/circle-to-rectangle                               |
      | update-part3d                      | 🧊update-part-3d/🧪️tests/reorients-and-rescales-part                        |
      | create-representation              | 🧱create-representation/🧪️tests/appends-frame-representation                |
      | delete-representation              | 🗑delete-representation/🧪️tests/removes-shell-representation                |
      | rename-representation              | ✒rename-representation/🧪️tests/renames-shell-to-hull                       |
      | change-representation-mesh-url     | 🌐change-representation-mesh-url/🧪️tests/repoints-shell-mesh-url            |
      | change-representation-lod          | 🏔change-representation-lod/🧪️tests/promotes-shell-to-lod2                  |
      | change-representation-description  | 📜change-representation-description/🧪️tests/rewrites-shell-description      |
      | add-representation-tag             | 🔖add-representation-tag/🧪️tests/tags-shell-as-structural                   |
      | remove-representation-tag          | 🚫remove-representation-tag/🧪️tests/untags-shell-printable                  |
      | add-representation-attribute       | 🧩add-representation-attribute/🧪️tests/adds-color-attribute-to-shell        |
      | remove-representation-attribute    | ➖remove-representation-attribute/🧪️tests/drops-finish-attribute-from-shell |
      | create-grip-kind                   | 🌱create-grip-kind/🧪️tests/appends-hook-grip-kind                           |
      | delete-grip-kind                   | ❌delete-grip-kind/🧪️tests/removes-plug-grip-kind                           |
      | rename-grip-kind                   | 🖋rename-grip-kind/🧪️tests/renames-plug-to-coupler                          |
      | change-grip-kind-label             | 🎫change-grip-kind-label/🧪️tests/relabels-plug-grip-kind                    |
      | change-grip-kind-color             | 🎨change-grip-kind-color/🧪️tests/recolors-plug-grip-kind                    |
      | change-grip-kind-default-rope-kind | 🪢change-grip-kind-default-rope-kind/🧪️tests/swaps-plug-default-rope-kind   |
      | create-grip                        | 🌿create-grip/🧪️tests/appends-south-grip                                    |
      | delete-grip                        | 🕳delete-grip/🧪️tests/removes-north-grip                                    |
      | move-grip2d                        | 📍move-grip-2d/🧪️tests/swings-north-grip-along-the-rim                      |
      | move-grip3d                        | 🧭move-grip-3d/🧪️tests/repositions-north-grip-in-world                      |
      | resize-grip3d                      | 📏resize-grip-3d/🧪️tests/widens-north-grip-radius                           |
      | change-grip-grip-kind              | 🧷change-grip-grip-kind/🧪️tests/rekinds-north-grip-as-socket                |
      | add-compatibility-rule             | ➕add-compatibility-rule/🧪️tests/allows-plug-to-socket                      |
      | remove-compatibility-rule          | ✂remove-compatibility-rule/🧪️tests/revokes-plug-to-plug                    |
      | add-attribute                      | 🔩add-attribute/🧪️tests/adds-weight-attribute                               |
      | remove-attribute                   | 🚷remove-attribute/🧪️tests/drops-material-attribute                         |
      | add-author                         | 👤add-author/🧪️tests/credits-bo                                             |
      | remove-author                      | 🙅remove-author/🧪️tests/uncredits-ada                                       |
      | move-camera2d                      | 🎥move-camera2d/🧪️tests/pans-2d-camera                                      |
      | scale-camera2d                     | 🔍scale-camera2d/🧪️tests/zooms-2d-camera-in                                 |
      | move-camera3d                      | 🎬move-camera3d/🧪️tests/orbits-3d-camera                                    |
      | scale-camera3d                     | 🔎scale-camera3d/🧪️tests/zooms-3d-camera-out                                |
      | change-meta-description            | 💬change-meta-description/🧪️tests/rewrites-session-notes                    |

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
      | id                                 | vector                                                                     |
      | rename-part-kind                   | ✏️rename-part-kind/🧪️tests/renames-part-kind-to-pod                        |
      | change-part-kind-label             | 🏷️change-part-kind-label/🧪️tests/relabels-part-kind                        |
      | change-part-kind-variant           | 🔀️change-part-kind-variant/🧪️tests/switches-variant-to-b                   |
      | change-part-kind-description       | 📃️change-part-kind-description/🧪️tests/rewrites-part-kind-description      |
      | change-part-kind-icon              | 🖼️change-part-kind-icon/🧪️tests/repoints-part-kind-icon                    |
      | change-part-kind-unit              | 📐change-part-kind-unit/🧪️tests/switches-unit-to-centimeter                 |
      | update-part2d                      | 🖌️update-part-2d/🧪️tests/circle-to-rectangle                               |
      | update-part3d                      | 🧊update-part-3d/🧪️tests/reorients-and-rescales-part                        |
      | create-representation              | 🧱create-representation/🧪️tests/appends-frame-representation                |
      | delete-representation              | 🗑delete-representation/🧪️tests/removes-shell-representation                |
      | rename-representation              | ✒rename-representation/🧪️tests/renames-shell-to-hull                       |
      | change-representation-mesh-url     | 🌐change-representation-mesh-url/🧪️tests/repoints-shell-mesh-url            |
      | change-representation-lod          | 🏔change-representation-lod/🧪️tests/promotes-shell-to-lod2                  |
      | change-representation-description  | 📜change-representation-description/🧪️tests/rewrites-shell-description      |
      | add-representation-tag             | 🔖add-representation-tag/🧪️tests/tags-shell-as-structural                   |
      | remove-representation-tag          | 🚫remove-representation-tag/🧪️tests/untags-shell-printable                  |
      | add-representation-attribute       | 🧩add-representation-attribute/🧪️tests/adds-color-attribute-to-shell        |
      | remove-representation-attribute    | ➖remove-representation-attribute/🧪️tests/drops-finish-attribute-from-shell |
      | create-grip-kind                   | 🌱create-grip-kind/🧪️tests/appends-hook-grip-kind                           |
      | delete-grip-kind                   | ❌delete-grip-kind/🧪️tests/removes-plug-grip-kind                           |
      | rename-grip-kind                   | 🖋rename-grip-kind/🧪️tests/renames-plug-to-coupler                          |
      | change-grip-kind-label             | 🎫change-grip-kind-label/🧪️tests/relabels-plug-grip-kind                    |
      | change-grip-kind-color             | 🎨change-grip-kind-color/🧪️tests/recolors-plug-grip-kind                    |
      | change-grip-kind-default-rope-kind | 🪢change-grip-kind-default-rope-kind/🧪️tests/swaps-plug-default-rope-kind   |
      | create-grip                        | 🌿create-grip/🧪️tests/appends-south-grip                                    |
      | delete-grip                        | 🕳delete-grip/🧪️tests/removes-north-grip                                    |
      | move-grip2d                        | 📍move-grip-2d/🧪️tests/swings-north-grip-along-the-rim                      |
      | move-grip3d                        | 🧭move-grip-3d/🧪️tests/repositions-north-grip-in-world                      |
      | resize-grip3d                      | 📏resize-grip-3d/🧪️tests/widens-north-grip-radius                           |
      | change-grip-grip-kind              | 🧷change-grip-grip-kind/🧪️tests/rekinds-north-grip-as-socket                |
      | add-compatibility-rule             | ➕add-compatibility-rule/🧪️tests/allows-plug-to-socket                      |
      | remove-compatibility-rule          | ✂remove-compatibility-rule/🧪️tests/revokes-plug-to-plug                    |
      | add-attribute                      | 🔩add-attribute/🧪️tests/adds-weight-attribute                               |
      | remove-attribute                   | 🚷remove-attribute/🧪️tests/drops-material-attribute                         |
      | add-author                         | 👤add-author/🧪️tests/credits-bo                                             |
      | remove-author                      | 🙅remove-author/🧪️tests/uncredits-ada                                       |
      | move-camera2d                      | 🎥move-camera2d/🧪️tests/pans-2d-camera                                      |
      | scale-camera2d                     | 🔍scale-camera2d/🧪️tests/zooms-2d-camera-in                                 |
      | move-camera3d                      | 🎬move-camera3d/🧪️tests/orbits-3d-camera                                    |
      | scale-camera3d                     | 🔎scale-camera3d/🧪️tests/zooms-3d-camera-out                                |
      | change-meta-description            | 💬change-meta-description/🧪️tests/rewrites-session-notes                    |

  @id-identity-round-trip
  @level-long
  @mode-round-trip
  Scenario: Decode and re-encode the fully populated part-kind definition with both facets
    Given the committed before-snapshot asset://🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✏️rename-part-kind/🧪️tests/renames-part-kind-to-pod/📸️snapshot/⬅️before/🔣️component.json
    When it is parsed by the platform's own dependency-free JSON reader, re-serialized and parsed again
    Then the document is unchanged and the re-serialized bytes are not the committed bytes
