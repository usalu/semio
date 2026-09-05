@capability-puzzle-3d-1-mutate
@oracle-puzzle-3d-python-independent
@comparison-ordered-json-v1
@mutations-puzzle-3d-1-any
Feature: Apply every typed puzzle3d scene mutation twice — once in Rust, once in Python — and require the same answer
  This case is a CROSS-LANGUAGE DIFFERENTIAL. The reference is `🐍️component.py` in this directory: a
  second implementation of the `s.puzzle.3d` scene document and its thirty-five typed mutations,
  written in Python from `🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🔣️.json`, from
  rules 2, 4 and 7 of
  `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/📓️derivation-rules.md`, and
  from the thirty-five committed quintets. It imports nothing from this repository's Rust.

  Why a second implementation rather than a third-party library, and why the previous answer was
  wrong. This case used to argue that its two-level connectivity — objects owning VORTICES, an
  attraction joining two of them by a `"<objectId>:<vortexId>"` address — is this subset's own
  specification and so not a fact an external library could confirm or refute. `mutate-fem3d-1` and
  `🗺️mutate-gisterrain-1` refuted that in this same wave by taking Python second implementations over
  this same carrier. The two-level connectivity is not an obstacle; it is something a second
  implementation must model — and the reference models BOTH cascades: deleting an object severs every
  attraction naming any of its vortices, and removing a single vortex severs the attractions
  addressed to that port alone. A third-party library was nonetheless declined and the reason is
  concrete: glTF, USD and IFC all model a scene graph of transforms, none of them can express a joint
  whose endpoints are named ports owned by two nodes, and none of them reads this carrier.

  What the document is: a domain, a metadata block holding a kind-compatibility relation and an
  optional kind catalogue, the objects with their own vortices and placements, the attractions
  between ports, the target volumes and the image references. Thirty-five kinds, and one union worth
  naming: `scale` is per-axis on an object and uniform on a target volume, and each verb writes the
  other shape over the one the before-snapshot holds.

  🚧️ THREE REFUSALS THE REFERENCE ARGUES BY CLAUSE, and reports rather than works around. First,
  `replace-object-vortex` in both roles. Its ONLY committed vector, `⏸️rekind-vortex-1-is-noop`,
  supplies a genuinely different vortex — `vortex-1` moves from `vortex-kind-a` to `vortex-kind-c` —
  and yet its committed outcome declares `mutation.no-op` and its after-snapshot is identical to its
  before-snapshot. At least three rules produce exactly that and no committed document distinguishes
  them: the verb is unimplemented; it refuses a vortex an attraction is addressed to, which
  `vortex-1` is; or it refuses a vortex kind the `kindCompatibility` relation does not admit, which
  `vortex-kind-c` is. `📓️derivation-rules.md` rule 2 says `replace-<singular>-<member>` replaces the
  addressed record, so a second implementation written from the specification would move the
  document. ONE more vector, on an unattracted vortex, decides it. Second,
  `inverse-replace-kind-catalogs`: the committed vector INSTALLS a catalogue where the before-snapshot
  carried none, so undoing it means REMOVING the member, and nothing committed says whether the verb
  accepts a null argument. The sibling `◻️mutate-puzzle-2d-1` reports both gaps identically.

  📌️ A FINDING MADE WHILE THE REFERENCE WAS BEING WRITTEN.
  `🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔣️.json` is not a mutation schema at
  all: it is titled `Puzzle3dMutation` and declares the SNAPSHOT's members — the pre-migration
  whole-snapshot-shaped generic schema that `s.architect.program`'s own mutation schema records itself
  as superseding. It was never replaced here, so the verbs and their argument lists had to be read off
  the committed payloads instead. It is also where the one spelling inconsistency in this vocabulary
  would have been caught: `replace-attraction-geometry` names the eight attraction-geometry values
  with a `new` prefix and `connect-vortices` names the same eight bare.

  📌️ TWO CEILINGS ON WHAT THIS COMPARISON ESTABLISHES, stated rather than implied. First, the
  SUBJECT half does not run this subset's codec: `🦀️component.rs` beside this file links no plugin
  crate and replays the committed vectors, so today the comparison establishes that an independent
  implementation of the specification computes the committed after-snapshots — a real check of the
  vectors, and the class of check that found `🦅️mutate-jack-1`'s wrong vector — but not yet our codec
  against a second producer. A `puzzle3d_mutation_report_json` bridge beside the mutation enum closes
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
  artifact: all 175 of its fixtures are handcrafted specification vectors.

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
      | id                            | vector                                                               |
      | create-object                 | 🌱create-object/🧪️tests/🌱️appends-object-c                              |
      | delete-object                 | 🗑️delete-object/🧪️tests/🚫️removes-object-a-and-severs-attraction        |
      | move-object                   | 📍move-object/🧪️tests/📍️moves-object-a                                  |
      | rotate-object                 | 🔃rotate-object/🧪️tests/🔄️half-turn-about-z                             |
      | scale-object                  | 📏scale-object/🧪️tests/📐️uniform-to-per-axis                            |
      | change-object-mesh            | 🧱change-object-mesh/🧪️tests/🕸️repoints-object-a-mesh                   |
      | edit-object-label             | 🖋️edit-object-label/🧪️tests/🔤️relabels-object-a                        |
      | change-object-kind            | 🏗️change-object-kind/🧪️tests/🏷️reassigns-object-a-kind                  |
      | change-object-anchor          | ⚓change-object-anchor/🧪️tests/⚓️fixed-to-derived                       |
      | change-object-hidden          | 👁️change-object-hidden/🧪️tests/🙈️hides-object-a                         |
      | change-object-locked          | 🔒change-object-locked/🧪️tests/🔒️locks-object-a                         |
      | add-object-vortex             | ➕add-object-vortex/🧪️tests/🌀️appends-vortex-3-to-object-b              |
      | remove-object-vortex          | ➖remove-object-vortex/🧪️tests/🚫️removes-vortex-2-and-severs-attraction |
      | replace-object-vortex         | 🔌replace-object-vortex/🧪️tests/⏸️rekind-vortex-1-is-noop               |
      | connect-vortices              | 🪢️connect-vortices/🧪️tests/🧲️adds-second-attraction                     |
      | disconnect-vortices           | ✂️disconnect-vortices/🧪️tests/🚫️removes-attraction-1                   |
      | replace-attraction-geometry   | 🧮replace-attraction-geometry/🧪️tests/📍️repositions-attraction-1        |
      | create-target-volume          | 🌍create-target-volume/🧪️tests/🧊️appends-volume-2                       |
      | delete-target-volume          | 🪦delete-target-volume/🧪️tests/🚫️removes-volume-1                       |
      | move-target-volume            | 🚀move-target-volume/🧪️tests/⬆️lifts-volume-1                           |
      | rotate-target-volume          | 🌀rotate-target-volume/🧪️tests/🔄️half-turn-about-z                      |
      | scale-target-volume           | 📐scale-target-volume/🧪️tests/📏️per-axis-to-uniform                     |
      | change-target-volume-hidden   | 🙈change-target-volume-hidden/🧪️tests/🙈️hides-volume-1                  |
      | change-target-volume-locked   | 🔐change-target-volume-locked/🧪️tests/🔒️locks-volume-1                  |
      | create-reference              | 🖼️create-reference/🧪️tests/🖼️appends-reference-2                        |
      | delete-reference              | 🚮delete-reference/🧪️tests/🚫️removes-reference-1                        |
      | move-reference                | 🎯move-reference/🧪️tests/↔️slides-reference-1                           |
      | resize-reference              | 📎resize-reference/🧪️tests/↔️widens-reference-1                         |
      | replace-reference-source      | 🖇️replace-reference-source/🧪️tests/🖇️repoints-reference-1-source        |
      | change-reference-hidden       | 👀change-reference-hidden/🧪️tests/🙈️hides-reference-1                   |
      | change-reference-locked       | 🗝️change-reference-locked/🧪️tests/🔒️locks-reference-1                   |
      | change-domain                 | 🌐change-domain/🧪️tests/⚙️architecture-to-engineering                   |
      | connect-kind-compatibility    | 🤝connect-kind-compatibility/🧪️tests/🤝️adds-vortex-kind-pair            |
      | disconnect-kind-compatibility | 💔disconnect-kind-compatibility/🧪️tests/🚫️removes-vortex-kind-pair      |
      | replace-kind-catalogs         | 📚replace-kind-catalogs/🧪️tests/📇️installs-vortex-kind-catalog          |

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
      | id                            | vector                                                               |
      | create-object                 | 🌱create-object/🧪️tests/🌱️appends-object-c                              |
      | delete-object                 | 🗑️delete-object/🧪️tests/🚫️removes-object-a-and-severs-attraction        |
      | move-object                   | 📍move-object/🧪️tests/📍️moves-object-a                                  |
      | rotate-object                 | 🔃rotate-object/🧪️tests/🔄️half-turn-about-z                             |
      | scale-object                  | 📏scale-object/🧪️tests/📐️uniform-to-per-axis                            |
      | change-object-mesh            | 🧱change-object-mesh/🧪️tests/🕸️repoints-object-a-mesh                   |
      | edit-object-label             | 🖋️edit-object-label/🧪️tests/🔤️relabels-object-a                        |
      | change-object-kind            | 🏗️change-object-kind/🧪️tests/🏷️reassigns-object-a-kind                  |
      | change-object-anchor          | ⚓change-object-anchor/🧪️tests/⚓️fixed-to-derived                       |
      | change-object-hidden          | 👁️change-object-hidden/🧪️tests/🙈️hides-object-a                         |
      | change-object-locked          | 🔒change-object-locked/🧪️tests/🔒️locks-object-a                         |
      | add-object-vortex             | ➕add-object-vortex/🧪️tests/🌀️appends-vortex-3-to-object-b              |
      | remove-object-vortex          | ➖remove-object-vortex/🧪️tests/🚫️removes-vortex-2-and-severs-attraction |
      | replace-object-vortex         | 🔌replace-object-vortex/🧪️tests/⏸️rekind-vortex-1-is-noop               |
      | connect-vortices              | 🪢️connect-vortices/🧪️tests/🧲️adds-second-attraction                     |
      | disconnect-vortices           | ✂️disconnect-vortices/🧪️tests/🚫️removes-attraction-1                   |
      | replace-attraction-geometry   | 🧮replace-attraction-geometry/🧪️tests/📍️repositions-attraction-1        |
      | create-target-volume          | 🌍create-target-volume/🧪️tests/🧊️appends-volume-2                       |
      | delete-target-volume          | 🪦delete-target-volume/🧪️tests/🚫️removes-volume-1                       |
      | move-target-volume            | 🚀move-target-volume/🧪️tests/⬆️lifts-volume-1                           |
      | rotate-target-volume          | 🌀rotate-target-volume/🧪️tests/🔄️half-turn-about-z                      |
      | scale-target-volume           | 📐scale-target-volume/🧪️tests/📏️per-axis-to-uniform                     |
      | change-target-volume-hidden   | 🙈change-target-volume-hidden/🧪️tests/🙈️hides-volume-1                  |
      | change-target-volume-locked   | 🔐change-target-volume-locked/🧪️tests/🔒️locks-volume-1                  |
      | create-reference              | 🖼️create-reference/🧪️tests/🖼️appends-reference-2                        |
      | delete-reference              | 🚮delete-reference/🧪️tests/🚫️removes-reference-1                        |
      | move-reference                | 🎯move-reference/🧪️tests/↔️slides-reference-1                           |
      | resize-reference              | 📎resize-reference/🧪️tests/↔️widens-reference-1                         |
      | replace-reference-source      | 🖇️replace-reference-source/🧪️tests/🖇️repoints-reference-1-source        |
      | change-reference-hidden       | 👀change-reference-hidden/🧪️tests/🙈️hides-reference-1                   |
      | change-reference-locked       | 🗝️change-reference-locked/🧪️tests/🔒️locks-reference-1                   |
      | change-domain                 | 🌐change-domain/🧪️tests/⚙️architecture-to-engineering                   |
      | connect-kind-compatibility    | 🤝connect-kind-compatibility/🧪️tests/🤝️adds-vortex-kind-pair            |
      | disconnect-kind-compatibility | 💔disconnect-kind-compatibility/🧪️tests/🚫️removes-vortex-kind-pair      |
      | replace-kind-catalogs         | 📚replace-kind-catalogs/🧪️tests/📇️installs-vortex-kind-catalog          |

  @id-identity-round-trip
  @level-long
  @mode-round-trip
  Scenario: Decode and re-encode the four-collection puzzle scene
    Given the committed before-snapshot asset://🧬️schema/🧬️mutations/🌱create-object/🧪️tests/🌱️appends-object-c/📸️snapshot/⬅️before/🔣️.json
    When it is parsed by the platform's own dependency-free JSON reader, re-serialized and parsed again
    Then the document is unchanged and the re-serialized bytes are not the committed bytes
