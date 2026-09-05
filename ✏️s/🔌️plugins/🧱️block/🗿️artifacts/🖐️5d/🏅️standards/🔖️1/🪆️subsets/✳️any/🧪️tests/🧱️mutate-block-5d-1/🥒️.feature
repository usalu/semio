@capability-block-5d-1-mutate
@oracle-block-5d-python-independent
@comparison-ordered-json-v1
@mutations-block-5d-1-any
Feature: Apply every typed block5d part-kind mutation twice — once in Rust, once in Python — and require the same answer
  This case is a CROSS-LANGUAGE DIFFERENTIAL. The reference is `🐍️.py` in this directory: a
  second implementation of the `s.block.5d` part-kind document and all forty-one typed mutations,
  written in Python from `🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🔣️.json`, from
  `…/🧬️schema/🧬️mutations/🔣️.json`, from rules 1, 2 and 7 of
  `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/📓️derivation-rules.md`, and
  from the forty-one committed quintets. It imports nothing from this repository's Rust.

  Why a second implementation rather than a third-party library, and why the previous answer was
  wrong. `s.block.5d@1/*` is a semio-NATIVE KIND DEFINITION carried as `.dsl.semio`/`.pack.semio`,
  and this case used to argue that because the diff renames the two facet members — the snapshot
  calls them `2d` and `3d`, names no Rust identifier can carry, and the diff calls them
  `part2d`/`part3d` — "that mapping IS this subset's specification, not a fact an external library
  could confirm or refute". The sibling `🧱️mutate-block-2d-1` — same plugin, same carrier, same
  kind-definition document shape — refuted that in this very wave by taking a Python second
  implementation. A renaming is not an obstacle to a second implementation; it is something a second
  implementation resolves through a declared alias table, and the reference declares one.

  What the document is, and what makes it the widest vocabulary in this plugin: one part kind
  carrying BOTH presentations at once — a 2d facet whose members change shape with the shape itself,
  and a 3d facet of orientation quaternion and scale — the mesh representations it offers at several
  levels of detail with their tags and attributes, the grip kinds it declares, and the grips placed
  on it in BOTH spaces at the same time, each grip carrying a polar 2d placement (`angle`,
  `radius2d`) and a 3d placement (`position`, `direction`, `radius3d`). Forty-one kinds, and two
  independent editor cameras.

  The one thing only the committed vectors state, and both implementations take it from there:
  `update-part-2d` REBUILDS the 2d facet from its six arguments in their declared order and DROPS
  every member whose argument is `null`. Its vector turns a circle with a radius and an icon kind
  into a rectangle with a width and a height and neither, so the facet loses two members and gains
  two — which is why the reference validates that facet by its shape discriminant rather than by a
  fixed member list, as it does for every other member.

  Unlike its `🧊️3d` sibling this subset holds its whole grip-kind vocabulary LOCALLY: no verb here
  reaches a composed child whose id is content-addressed by a function no specification states, so
  all forty-one kinds are adjudicated and none is refused.

  📌️ TWO CEILINGS ON WHAT THIS COMPARISON ESTABLISHES, stated rather than implied. First, the
  SUBJECT half does not run this subset's codec: `🦀️.rs` beside this file links no plugin
  crate and replays the committed vectors, so today the comparison establishes that an independent
  implementation of the specification computes the committed after-snapshots — a real check of the
  vectors, and the class of check that found `🦅️mutate-jack-1`'s wrong vector — but not yet our codec
  against a second producer. A `block5d_mutation_report_json` bridge beside the mutation enum, the
  one thing `🧱️mutate-block-2d-1` gained in this wave, is what closes that; it was not added here
  because `semio-s-plugin-block` does not compile today (1,522 errors from a peer session's in-flight
  async refactor), so it could not be verified. Second, this case reads no real-world artifact: all
  205 of its fixtures are handcrafted specification vectors.

  The committed specification vectors were KEPT, not replaced, and the reference asserts more against
  them than the subject half can: it applies each verb, requires the committed after-snapshot member
  by member, requires that the verb moved exactly ONE of the thirteen members, applies its OWN
  computed inverse and requires the committed before-snapshot back — the full inverse law, where the
  subject half asserts only the weaker footprint precondition.

  @id-mutate
  @level-exhaustive
  @mode-differential
  Scenario Outline: The committed <id> vector declares its own kind and moves the document
    Given the committed specification vector for the <id> kind
      """
      {
        "kind": "<id>",
        "before": "asset://🧬️schema/🧬️mutations/<vector>/📸️snapshot/⬅️before/🔣️.json",
        "mutation": "asset://🧬️schema/🧬️mutations/<vector>/🦠️mutation/🔣️.json",
        "diff": "asset://🧬️schema/🧬️mutations/<vector>/🔺️diff/🔣️.json",
        "outcome": "asset://🧬️schema/🧬️mutations/<vector>/🎯️outcome/🔣️.json",
        "after": "asset://🧬️schema/🧬️mutations/<vector>/📸️snapshot/➡️after/🔣️.json"
      }
      """
    Then the committed mutation payload declares the <id> kind
    And the after-snapshot differs from the before-snapshot, or the committed outcome declares the vector a no-op
    Examples:
      | id                                 | vector                                                                     |
      | rename-part-kind                   | ✏️rename-part-kind/🧪️tests/✏️renames-part-kind-to-pod                        |
      | change-part-kind-label             | 🏷️change-part-kind-label/🧪️tests/🔤️relabels-part-kind                        |
      | change-part-kind-variant           | 🔀️change-part-kind-variant/🧪️tests/🔀️switches-variant-to-b                   |
      | change-part-kind-description       | 📃️change-part-kind-description/🧪️tests/💬️rewrites-part-kind-description      |
      | change-part-kind-icon              | 🖼️change-part-kind-icon/🧪️tests/🖼️repoints-part-kind-icon                    |
      | change-part-kind-unit              | 📐change-part-kind-unit/🧪️tests/📏️switches-unit-to-centimeter                 |
      | update-part-2d                      | 🖌️update-part-2d/🧪️tests/🔲️circle-to-rectangle                               |
      | update-part-3d                      | 🧊update-part-3d/🧪️tests/🔄️reorients-and-rescales-part                        |
      | create-representation              | 🧱create-representation/🧪️tests/🏗️appends-frame-representation                |
      | delete-representation              | 🗑️delete-representation/🧪️tests/🚫️removes-shell-representation                |
      | rename-representation              | ✒️rename-representation/🧪️tests/✏️renames-shell-to-hull                       |
      | change-representation-mesh-url     | 🌐change-representation-mesh-url/🧪️tests/🐚️repoints-shell-mesh-url            |
      | change-representation-lod          | 🏔️change-representation-lod/🧪️tests/🐚️promotes-shell-to-lod2                  |
      | change-representation-description  | 📜change-representation-description/🧪️tests/🐚️rewrites-shell-description      |
      | add-representation-tag             | 🔖add-representation-tag/🧪️tests/🔖️tags-shell-as-structural                   |
      | remove-representation-tag          | 🚫remove-representation-tag/🧪️tests/🚫️untags-shell-printable                  |
      | add-representation-attribute       | 🧩add-representation-attribute/🧪️tests/🐚️adds-color-attribute-to-shell        |
      | remove-representation-attribute    | ➖remove-representation-attribute/🧪️tests/🐚️drops-finish-attribute-from-shell |
      | create-grip-kind                   | 🌱create-grip-kind/🧪️tests/🪝️appends-hook-grip-kind                           |
      | delete-grip-kind                   | ❌delete-grip-kind/🧪️tests/🚫️removes-plug-grip-kind                           |
      | rename-grip-kind                   | 🖋️rename-grip-kind/🧪️tests/✏️renames-plug-to-coupler                          |
      | change-grip-kind-label             | 🎫change-grip-kind-label/🧪️tests/🔤️relabels-plug-grip-kind                    |
      | change-grip-kind-color             | 🎨change-grip-kind-color/🧪️tests/⚫️recolors-plug-grip-kind                    |
      | change-grip-kind-default-rope-kind | 🪢change-grip-kind-default-rope-kind/🧪️tests/🪢️swaps-plug-default-rope-kind   |
      | create-grip                        | 🌿create-grip/🧪️tests/⬇️appends-south-grip                                    |
      | delete-grip                        | 🕳️delete-grip/🧪️tests/🚫️removes-north-grip                                    |
      | move-grip-2d                        | 📍move-grip-2d/🧪️tests/🔄️swings-north-grip-along-the-rim                      |
      | move-grip-3d                        | 🧭move-grip-3d/🧪️tests/🗺️repositions-north-grip-in-world                      |
      | resize-grip-3d                      | 📏resize-grip-3d/🧪️tests/📏️widens-north-grip-radius                           |
      | change-grip-grip-kind              | 🧷change-grip-grip-kind/🧪️tests/🔌️rekinds-north-grip-as-socket                |
      | add-compatibility-rule             | ➕add-compatibility-rule/🧪️tests/🔗️allows-plug-to-socket                      |
      | remove-compatibility-rule          | ✂️remove-compatibility-rule/🧪️tests/✂️revokes-plug-to-plug                    |
      | add-attribute                      | 🔩add-attribute/🧪️tests/⚖️adds-weight-attribute                               |
      | remove-attribute                   | 🚷remove-attribute/🧪️tests/➖️drops-material-attribute                         |
      | add-author                         | 👤add-author/🧪️tests/✏️credits-bo                                             |
      | remove-author                      | 🙅remove-author/🧪️tests/✏️uncredits-ada                                       |
      | move-camera2d                      | 🎥move-camera2d/🧪️tests/↔️pans-2d-camera                                      |
      | scale-camera2d                     | 🔍scale-camera2d/🧪️tests/🔭️zooms-2d-camera-in                                 |
      | move-camera3d                      | 🎬move-camera3d/🧪️tests/🪐️orbits-3d-camera                                    |
      | scale-camera3d                     | 🔎scale-camera3d/🧪️tests/🔭️zooms-3d-camera-out                                |
      | change-meta-description            | 💬change-meta-description/🧪️tests/📝️rewrites-session-notes                    |

  @id-inverse
  @level-exhaustive
  @mode-differential
  Scenario Outline: The committed <id> vector changes only what its diff declares
    Given the committed specification vector for the <id> kind
      """
      {
        "kind": "<id>",
        "before": "asset://🧬️schema/🧬️mutations/<vector>/📸️snapshot/⬅️before/🔣️.json",
        "mutation": "asset://🧬️schema/🧬️mutations/<vector>/🦠️mutation/🔣️.json",
        "diff": "asset://🧬️schema/🧬️mutations/<vector>/🔺️diff/🔣️.json",
        "outcome": "asset://🧬️schema/🧬️mutations/<vector>/🎯️outcome/🔣️.json",
        "after": "asset://🧬️schema/🧬️mutations/<vector>/📸️snapshot/➡️after/🔣️.json"
      }
      """
    Then every field where the after-snapshot differs from the before-snapshot is declared by the committed diff
    And every field the committed diff declares actually differs
    Examples:
      | id                                 | vector                                                                     |
      | rename-part-kind                   | ✏️rename-part-kind/🧪️tests/✏️renames-part-kind-to-pod                        |
      | change-part-kind-label             | 🏷️change-part-kind-label/🧪️tests/🔤️relabels-part-kind                        |
      | change-part-kind-variant           | 🔀️change-part-kind-variant/🧪️tests/🔀️switches-variant-to-b                   |
      | change-part-kind-description       | 📃️change-part-kind-description/🧪️tests/💬️rewrites-part-kind-description      |
      | change-part-kind-icon              | 🖼️change-part-kind-icon/🧪️tests/🖼️repoints-part-kind-icon                    |
      | change-part-kind-unit              | 📐change-part-kind-unit/🧪️tests/📏️switches-unit-to-centimeter                 |
      | update-part-2d                      | 🖌️update-part-2d/🧪️tests/🔲️circle-to-rectangle                               |
      | update-part-3d                      | 🧊update-part-3d/🧪️tests/🔄️reorients-and-rescales-part                        |
      | create-representation              | 🧱create-representation/🧪️tests/🏗️appends-frame-representation                |
      | delete-representation              | 🗑️delete-representation/🧪️tests/🚫️removes-shell-representation                |
      | rename-representation              | ✒️rename-representation/🧪️tests/✏️renames-shell-to-hull                       |
      | change-representation-mesh-url     | 🌐change-representation-mesh-url/🧪️tests/🐚️repoints-shell-mesh-url            |
      | change-representation-lod          | 🏔️change-representation-lod/🧪️tests/🐚️promotes-shell-to-lod2                  |
      | change-representation-description  | 📜change-representation-description/🧪️tests/🐚️rewrites-shell-description      |
      | add-representation-tag             | 🔖add-representation-tag/🧪️tests/🔖️tags-shell-as-structural                   |
      | remove-representation-tag          | 🚫remove-representation-tag/🧪️tests/🚫️untags-shell-printable                  |
      | add-representation-attribute       | 🧩add-representation-attribute/🧪️tests/🐚️adds-color-attribute-to-shell        |
      | remove-representation-attribute    | ➖remove-representation-attribute/🧪️tests/🐚️drops-finish-attribute-from-shell |
      | create-grip-kind                   | 🌱create-grip-kind/🧪️tests/🪝️appends-hook-grip-kind                           |
      | delete-grip-kind                   | ❌delete-grip-kind/🧪️tests/🚫️removes-plug-grip-kind                           |
      | rename-grip-kind                   | 🖋️rename-grip-kind/🧪️tests/✏️renames-plug-to-coupler                          |
      | change-grip-kind-label             | 🎫change-grip-kind-label/🧪️tests/🔤️relabels-plug-grip-kind                    |
      | change-grip-kind-color             | 🎨change-grip-kind-color/🧪️tests/⚫️recolors-plug-grip-kind                    |
      | change-grip-kind-default-rope-kind | 🪢change-grip-kind-default-rope-kind/🧪️tests/🪢️swaps-plug-default-rope-kind   |
      | create-grip                        | 🌿create-grip/🧪️tests/⬇️appends-south-grip                                    |
      | delete-grip                        | 🕳️delete-grip/🧪️tests/🚫️removes-north-grip                                    |
      | move-grip-2d                        | 📍move-grip-2d/🧪️tests/🔄️swings-north-grip-along-the-rim                      |
      | move-grip-3d                        | 🧭move-grip-3d/🧪️tests/🗺️repositions-north-grip-in-world                      |
      | resize-grip-3d                      | 📏resize-grip-3d/🧪️tests/📏️widens-north-grip-radius                           |
      | change-grip-grip-kind              | 🧷change-grip-grip-kind/🧪️tests/🔌️rekinds-north-grip-as-socket                |
      | add-compatibility-rule             | ➕add-compatibility-rule/🧪️tests/🔗️allows-plug-to-socket                      |
      | remove-compatibility-rule          | ✂️remove-compatibility-rule/🧪️tests/✂️revokes-plug-to-plug                    |
      | add-attribute                      | 🔩add-attribute/🧪️tests/⚖️adds-weight-attribute                               |
      | remove-attribute                   | 🚷remove-attribute/🧪️tests/➖️drops-material-attribute                         |
      | add-author                         | 👤add-author/🧪️tests/✏️credits-bo                                             |
      | remove-author                      | 🙅remove-author/🧪️tests/✏️uncredits-ada                                       |
      | move-camera2d                      | 🎥move-camera2d/🧪️tests/↔️pans-2d-camera                                      |
      | scale-camera2d                     | 🔍scale-camera2d/🧪️tests/🔭️zooms-2d-camera-in                                 |
      | move-camera3d                      | 🎬move-camera3d/🧪️tests/🪐️orbits-3d-camera                                    |
      | scale-camera3d                     | 🔎scale-camera3d/🧪️tests/🔭️zooms-3d-camera-out                                |
      | change-meta-description            | 💬change-meta-description/🧪️tests/📝️rewrites-session-notes                    |

  @id-identity-round-trip
  @level-long
  @mode-round-trip
  Scenario: Decode and re-encode the fully populated part-kind definition with both facets
    Given the committed before-snapshot asset://🧬️schema/🧬️mutations/✏️rename-part-kind/🧪️tests/✏️renames-part-kind-to-pod/📸️snapshot/⬅️before/🔣️.json
    When it is parsed by the platform's own dependency-free JSON reader, re-serialized and parsed again
    Then the document is unchanged and the re-serialized bytes are not the committed bytes
