@capability-block-3d-1-mutate
@oracle-block-3d-python-independent
@comparison-ordered-json-v1
@mutations-block-3d-1-any
Feature: Apply every typed block3d object-kind mutation twice — once in Rust, once in Python — and require the same answer
  This case is a CROSS-LANGUAGE DIFFERENTIAL. The reference is `🐍️component.py` in this directory: a
  second implementation of the `s.block.3d` object-kind document and all thirty-seven typed
  mutations, written in Python from `🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🔣️component.json`,
  from `…/🧬️schema/🧬️mutations/🔣️component.json`, from rules 1, 2 and 7 of
  `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/📓️derivation-rules.md`, and
  from the thirty-seven committed quintets. It imports nothing from this repository's Rust.

  Why a second implementation rather than a third-party library, and why the previous answer was
  wrong. `s.block.3d@1/*` is a semio-NATIVE KIND DEFINITION carried as `.dsl.semio`/`.pack.semio`,
  and this case used to argue that because its vortex-kind vocabulary is SPLIT across a shared
  catalogue child and a local extras table, "that split IS this subset's specification, not a fact an
  external library could confirm or refute". The sibling `mutate-block-2d-1` — same plugin, same
  carrier, same kind-definition document shape — refuted that in this very wave by taking a Python
  second implementation. The split is not an obstacle to a second implementation; it is something a
  second implementation must model, and the reference models it, including the many-to-one diff arm
  that maps BOTH snapshot members onto the diff's single `vortexKinds` field.

  What the document is: one object kind's identity and unit, the mesh representations it offers at
  several levels of detail with their tags and attributes, the vortex kinds it exposes, the vortices
  placed on it in 3-space by position and direction, the compatibility relation between vortex kinds,
  its attribute table, its authors, its editor camera and its metadata. Thirty-seven kinds — six
  object-kind scalars, ten over the representations, six over the vortex kinds, six over the
  vortices, two each over compatibility, attributes and authors, two camera gestures and one metadata
  setter.

  🚧️ THREE KINDS THE REFERENCE REFUSES BY CLAUSE, and reports rather than works around.
  `create-vortex-kind`, `delete-vortex-kind` and `rename-vortex-kind` all rewrite `catalog`, which a
  committed snapshot carries as a COMPOSED CHILD HANDLE `{childId, target}`. Their committed
  after-snapshots carry a NEW `childId` — `catalog-69f2059178f5dfa4`, `catalog-9dc5de0f33c9568d`,
  `catalog-e76534bc13e6b5a6` — which is a content address of the child `s.stdio.semio@v1/kit`
  document after the vocabulary moved, and no document in this repository states the addressing
  function or the child's canonical encoding. The LOCAL `vortexKindExtra` half of those kinds is
  implemented; the catalogue half is not, and the reference declines to guess rather than hard-code
  the committed answer. `mutate-program-1` reports the identical blocker over `knowledge` and
  `benchmarks`, and `mutate-en1990-1`'s two red scenarios are the same finding again: publishing the
  child-addressing rule closes all of them, and no comparison profile moves.

  📌️ TWO CEILINGS ON WHAT THIS COMPARISON ESTABLISHES, stated rather than implied. First, the
  SUBJECT half does not run this subset's codec: `🦀️component.rs` beside this file links no plugin
  crate and replays the committed vectors, so today the comparison establishes that an independent
  implementation of the specification computes the committed after-snapshots — a real check of the
  vectors, and the class of check that found `mutate-jack-1`'s wrong vector — but not yet our codec
  against a second producer. A `block3d_mutation_report_json` bridge beside the mutation enum, the
  one thing `mutate-block-2d-1` gained in this wave, is what closes that; it was not added here
  because `semio-s-plugin-block` does not compile today (1,522 errors from a peer session's in-flight
  async refactor), so it could not be verified. Second, this case reads no real-world artifact: all
  185 of its fixtures are handcrafted specification vectors.

  The committed specification vectors were KEPT, not replaced, and the reference asserts more against
  them than the subject half can: it applies each verb, requires the committed after-snapshot member
  by member, applies its OWN computed inverse and requires the committed before-snapshot back — the
  full inverse law, where the subject half asserts only the weaker footprint precondition.

  @id-mutate
  @level-exhaustive
  @mode-differential
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
      | id                                    | vector                                                                       |
      | rename-object-kind                    | ✏️rename-object-kind/🧪️tests/renames-object-kind-to-pod                      |
      | change-object-kind-label              | 🏷️change-object-kind-label/🧪️tests/relabels-object-kind                      |
      | change-object-kind-variant            | 🔀️change-object-kind-variant/🧪️tests/switches-variant-to-b                   |
      | change-object-kind-description        | 📃️change-object-kind-description/🧪️tests/rewrites-object-kind-description    |
      | change-object-kind-icon               | 🖼️change-object-kind-icon/🧪️tests/repoints-object-kind-icon                  |
      | change-object-kind-unit               | 📐change-object-kind-unit/🧪️tests/switches-unit-to-centimeter                 |
      | create-representation                 | 🧱create-representation/🧪️tests/appends-frame-representation                  |
      | delete-representation                 | 🗑delete-representation/🧪️tests/removes-shell-representation                  |
      | rename-representation                 | ✒rename-representation/🧪️tests/renames-shell-to-hull                         |
      | change-representation-mesh-url        | 🌐change-representation-mesh-url/🧪️tests/repoints-shell-mesh-url              |
      | change-representation-lod             | 🏔change-representation-lod/🧪️tests/promotes-shell-to-lod2                    |
      | change-representation-description     | 📜change-representation-description/🧪️tests/rewrites-shell-description        |
      | add-representation-tag                | 🔖add-representation-tag/🧪️tests/tags-shell-as-structural                     |
      | remove-representation-tag             | 🚫remove-representation-tag/🧪️tests/untags-shell-printable                    |
      | add-representation-attribute          | 🧩add-representation-attribute/🧪️tests/adds-color-attribute-to-shell          |
      | remove-representation-attribute       | ➖remove-representation-attribute/🧪️tests/drops-finish-attribute-from-shell   |
      | create-vortex-kind                    | 🌱create-vortex-kind/🧪️tests/appends-vent-vortex-kind                         |
      | delete-vortex-kind                    | ❌delete-vortex-kind/🧪️tests/removes-hatch-vortex-kind                        |
      | rename-vortex-kind                    | 🖋rename-vortex-kind/🧪️tests/renames-door-to-portal                           |
      | change-vortex-kind-label              | 🎫change-vortex-kind-label/🧪️tests/relabels-door-vortex-kind                  |
      | change-vortex-kind-color              | 🎨change-vortex-kind-color/🧪️tests/recolors-door-vortex-kind                  |
      | change-vortex-kind-default-cable-kind | 🔌change-vortex-kind-default-cable-kind/🧪️tests/swaps-door-default-cable-kind |
      | create-vortex                         | 🌀create-vortex/🧪️tests/appends-rear-vortex                                   |
      | delete-vortex                         | 🕳delete-vortex/🧪️tests/removes-front-vortex                                  |
      | move-vortex                           | 📍move-vortex/🧪️tests/repositions-front-vortex                                |
      | resize-vortex                         | 📏resize-vortex/🧪️tests/widens-front-vortex                                   |
      | change-vortex-vortex-kind             | 🧷change-vortex-vortex-kind/🧪️tests/rekinds-front-vortex-as-hatch             |
      | change-vortex-label                   | 🪧change-vortex-label/🧪️tests/relabels-front-vortex                           |
      | add-compatibility-rule                | ➕add-compatibility-rule/🧪️tests/allows-door-to-hatch                         |
      | remove-compatibility-rule             | ✂remove-compatibility-rule/🧪️tests/revokes-door-to-door                      |
      | add-attribute                         | 🔩add-attribute/🧪️tests/adds-weight-attribute                                 |
      | remove-attribute                      | 🚷remove-attribute/🧪️tests/drops-material-attribute                           |
      | add-author                            | 👤add-author/🧪️tests/credits-bo                                               |
      | remove-author                         | 🙅remove-author/🧪️tests/uncredits-ada                                         |
      | move-camera3d                         | 🎥move-camera3d/🧪️tests/orbits-camera                                         |
      | scale-camera3d                        | 🔍scale-camera3d/🧪️tests/zooms-camera-out                                     |
      | change-meta-description               | 💬change-meta-description/🧪️tests/rewrites-session-notes                      |

  @id-inverse
  @level-exhaustive
  @mode-differential
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
      | id                                    | vector                                                                       |
      | rename-object-kind                    | ✏️rename-object-kind/🧪️tests/renames-object-kind-to-pod                      |
      | change-object-kind-label              | 🏷️change-object-kind-label/🧪️tests/relabels-object-kind                      |
      | change-object-kind-variant            | 🔀️change-object-kind-variant/🧪️tests/switches-variant-to-b                   |
      | change-object-kind-description        | 📃️change-object-kind-description/🧪️tests/rewrites-object-kind-description    |
      | change-object-kind-icon               | 🖼️change-object-kind-icon/🧪️tests/repoints-object-kind-icon                  |
      | change-object-kind-unit               | 📐change-object-kind-unit/🧪️tests/switches-unit-to-centimeter                 |
      | create-representation                 | 🧱create-representation/🧪️tests/appends-frame-representation                  |
      | delete-representation                 | 🗑delete-representation/🧪️tests/removes-shell-representation                  |
      | rename-representation                 | ✒rename-representation/🧪️tests/renames-shell-to-hull                         |
      | change-representation-mesh-url        | 🌐change-representation-mesh-url/🧪️tests/repoints-shell-mesh-url              |
      | change-representation-lod             | 🏔change-representation-lod/🧪️tests/promotes-shell-to-lod2                    |
      | change-representation-description     | 📜change-representation-description/🧪️tests/rewrites-shell-description        |
      | add-representation-tag                | 🔖add-representation-tag/🧪️tests/tags-shell-as-structural                     |
      | remove-representation-tag             | 🚫remove-representation-tag/🧪️tests/untags-shell-printable                    |
      | add-representation-attribute          | 🧩add-representation-attribute/🧪️tests/adds-color-attribute-to-shell          |
      | remove-representation-attribute       | ➖remove-representation-attribute/🧪️tests/drops-finish-attribute-from-shell   |
      | create-vortex-kind                    | 🌱create-vortex-kind/🧪️tests/appends-vent-vortex-kind                         |
      | delete-vortex-kind                    | ❌delete-vortex-kind/🧪️tests/removes-hatch-vortex-kind                        |
      | rename-vortex-kind                    | 🖋rename-vortex-kind/🧪️tests/renames-door-to-portal                           |
      | change-vortex-kind-label              | 🎫change-vortex-kind-label/🧪️tests/relabels-door-vortex-kind                  |
      | change-vortex-kind-color              | 🎨change-vortex-kind-color/🧪️tests/recolors-door-vortex-kind                  |
      | change-vortex-kind-default-cable-kind | 🔌change-vortex-kind-default-cable-kind/🧪️tests/swaps-door-default-cable-kind |
      | create-vortex                         | 🌀create-vortex/🧪️tests/appends-rear-vortex                                   |
      | delete-vortex                         | 🕳delete-vortex/🧪️tests/removes-front-vortex                                  |
      | move-vortex                           | 📍move-vortex/🧪️tests/repositions-front-vortex                                |
      | resize-vortex                         | 📏resize-vortex/🧪️tests/widens-front-vortex                                   |
      | change-vortex-vortex-kind             | 🧷change-vortex-vortex-kind/🧪️tests/rekinds-front-vortex-as-hatch             |
      | change-vortex-label                   | 🪧change-vortex-label/🧪️tests/relabels-front-vortex                           |
      | add-compatibility-rule                | ➕add-compatibility-rule/🧪️tests/allows-door-to-hatch                         |
      | remove-compatibility-rule             | ✂remove-compatibility-rule/🧪️tests/revokes-door-to-door                      |
      | add-attribute                         | 🔩add-attribute/🧪️tests/adds-weight-attribute                                 |
      | remove-attribute                      | 🚷remove-attribute/🧪️tests/drops-material-attribute                           |
      | add-author                            | 👤add-author/🧪️tests/credits-bo                                               |
      | remove-author                         | 🙅remove-author/🧪️tests/uncredits-ada                                         |
      | move-camera3d                         | 🎥move-camera3d/🧪️tests/orbits-camera                                         |
      | scale-camera3d                        | 🔍scale-camera3d/🧪️tests/zooms-camera-out                                     |
      | change-meta-description               | 💬change-meta-description/🧪️tests/rewrites-session-notes                      |

  @id-identity-round-trip
  @level-long
  @mode-round-trip
  Scenario: Decode and re-encode the object-kind definition with its catalogue child and local extras
    Given the committed before-snapshot asset://🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✏️rename-object-kind/🧪️tests/renames-object-kind-to-pod/📸️snapshot/⬅️before/🔣️component.json
    When it is parsed by the platform's own dependency-free JSON reader, re-serialized and parsed again
    Then the document is unchanged and the re-serialized bytes are not the committed bytes
