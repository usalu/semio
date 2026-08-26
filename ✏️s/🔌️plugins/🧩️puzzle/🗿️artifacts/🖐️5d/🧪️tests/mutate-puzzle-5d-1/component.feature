@capability-puzzle-5d-1-mutate
@oracle-puzzle-5d-python-independent
@comparison-ordered-json-v1
@mutations-puzzle-5d-1-any
Feature: Apply every typed puzzle5d assembly mutation twice — once in Rust, once in Python — and require the same answer
  This case is a CROSS-LANGUAGE DIFFERENTIAL. The reference is `🐍️component.py` in this directory: a
  second implementation of the `s.puzzle.5d` assembly document and its twenty-eight typed mutations,
  written in Python from `🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🔣️component.json`, from
  rules 1, 2, 4 and 7 of
  `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/📓️derivation-rules.md`, and
  from the twenty-eight committed quintets. It imports nothing from this repository's Rust.

  Why a second implementation rather than a third-party library, and why the previous answer was
  wrong. This case used to argue that its five-dimensional shape — every element placed in TWO spaces
  at once — is this subset's own specification and so not a fact an external library could confirm or
  refute. `mutate-fem2d-1`, `mutate-fem3d-1` and `mutate-gismap-1` refuted that in this same wave by
  taking Python second implementations over this same carrier. Two placements are not an obstacle;
  they are something a second implementation must model, and the reference models both, down to the
  grip that carries a 2d angle and a 3d position simultaneously. A third-party library was
  nonetheless declined and the reason is concrete: STEP AP214, glTF, USD and IFC each place an element
  in ONE space, none of them carries a part placed in a diagram and in a model at the same time with
  a joint addressed as `"<partId>:<gripId>"`, and none of them reads this carrier.

  ✅️ ALL TWENTY-EIGHT KINDS ARE ADJUDICATED AND NONE IS REFUSED — and this subset is the one that
  settles a question its siblings leave open. `mutate-puzzle-2d-1` and `mutate-puzzle-3d-1` each
  commit exactly one `replace-<container>-<port>` vector (`replace-node-handle`,
  `replace-object-vortex`) and each declares it `mutation.no-op` with an unchanged after-snapshot,
  which leaves three readings open: the verb is unimplemented, or it refuses an attached port, or it
  refuses an unadmitted kind. Here the corresponding `replace-part-grip` vector really does rekind
  `grip-1` — on a grip a fastener IS attached to — and the document moves. That narrows the siblings'
  three readings to one: the verb is implemented and attachment does not block it. Their
  `null-catalogs-is-noop` counterpart is settled here too: this subset commits a vector showing that
  `replace-kind-catalogs` with a NULL argument is accepted and does NOTHING, which is what makes the
  siblings' missing catalogue inverse a gap in the VOCABULARY rather than in an implementation.

  📌️ A FINDING MADE WHILE THE REFERENCE WAS BEING WRITTEN. Like both siblings,
  `🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔣️component.json` is not a mutation schema at
  all: it declares the SNAPSHOT's members — the pre-migration whole-snapshot-shaped generic schema
  that `s.architect.program`'s own mutation schema records itself as superseding. It was never
  replaced here, so the verbs and their argument lists had to be read off the committed payloads.

  📌️ TWO CEILINGS ON WHAT THIS COMPARISON ESTABLISHES, stated rather than implied. First, the
  SUBJECT half does not run this subset's codec: `🦀️component.rs` beside this file links no plugin
  crate and replays the committed vectors, so today the comparison establishes that an independent
  implementation of the specification computes the committed after-snapshots — a real check of the
  vectors, and the class of check that found `mutate-jack-1`'s wrong vector — but not yet our codec
  against a second producer. A `puzzle5d_mutation_report_json` bridge beside the mutation enum closes
  it; it was not added here for two reasons, and neither is that
  the verb is hard: it is PRODUCTION code in a crate this test-side pass deliberately does not
  touch, and it could not be verified end to end today anyway.
  `parity` was not measured for any case in this pass: the single-case probe
  `parity exhaustive --owner 🗒️note --case mutate-note-1` was killed at the runner's OWN 900 s
  per-case budget while still COMPILING the generated subject host — the runner's message names the
  cause, shared cargo target-dir lock contention from a concurrent session — and then threw
  `spawnSync cargo ETIMEDOUT` out of `runProbe` with no summary line at all.
  `📓️w14-final-audit.md` §5.3 measured the underlying blocker one day earlier (`unresolved import
  component::component_persistent_local` in `semio-framework-plugin`, which sits in every generated
  host's dependency graph); this pass did NOT re-verify whether that is still the state, and says so
  rather than repeating it as fact. This subset's own plugin crate compiles clean at
  `cargo check --lib`. Second, this case reads no real-world
  artifact: all 140 of its fixtures are handcrafted specification vectors.

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
