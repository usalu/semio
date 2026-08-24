@capability-block-3d-1-mutate
@no-oracle-block-3d-mutation-semantics
@comparison-ordered-json-v1
@mutations-block-3d-1-any
Feature: Replay every typed Block 3d 1 mutation against its committed specification vector
  `s.block.3d@1/*` is a semio-NATIVE document, carried as `.dsl.semio`/`.pack.semio`. No third party reads
  those, and none is authoritative over `Block3dMutation`, so this case rests on the recorded
  `block-3d-mutation-semantics` no-oracle decision
  (`../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️oracle/🔣️component.json`) and its two named substitutes: the
  committed specification vectors, and the metamorphic laws below.

  This document defines an object kind, and what makes it unlike either block sibling is that its
  vortex-kind vocabulary lives in TWO places at once: a shared catalogue held as a CHILD DOCUMENT reference
  in `catalog`, and this document's own additions in `vortexKindExtra`. `create-vortex-kind` and
  `delete-vortex-kind` move both fields together, `rename-vortex-kind` moves only the catalogue child, and
  the label/colour/default-cable-kind kinds move only the local extras — three different footprints over
  one apparent concept.

  The diff does not mirror that split: it declares all of it through a single `vortexKinds` field. The
  adapter's alias table maps BOTH snapshot fields onto that one diff field, which is the many-to-one case
  no other subset in this scope needs. Without it, six of the thirty-seven kinds would report undeclared
  changes on every run.

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
