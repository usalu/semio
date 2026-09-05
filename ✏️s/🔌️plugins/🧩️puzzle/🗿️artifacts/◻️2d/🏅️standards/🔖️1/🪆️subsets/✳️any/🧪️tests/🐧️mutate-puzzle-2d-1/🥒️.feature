@capability-puzzle-2d-1-mutate
@oracle-puzzle-2d-python-independent
@comparison-ordered-json-v1
@mutations-puzzle-2d-1-any
Feature: Apply every typed puzzle2d board mutation twice — once in Rust, once in Python — and require the same answer
  This case is a CROSS-LANGUAGE DIFFERENTIAL. The reference is `🐍️component.py` in this directory: a
  second implementation of the `s.puzzle.2d` board document and its twenty-six typed mutations,
  written in Python from `🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🔣️.json`, from
  rules 2, 4 and 7 of
  `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/📓️derivation-rules.md`, and
  from the twenty-six committed quintets. It imports nothing from this repository's Rust.

  Why a second implementation rather than a third-party library, and why the previous answer was
  wrong. This case used to argue that because handles live INSIDE nodes while the edges that join
  them are a sibling top-level collection, "that is this subset's own specification, not a fact an
  external diagram or graph library could confirm or refute". `mutate-fem2d-1` and `mutate-gismap-1`
  refuted that in this same wave by taking Python second implementations over this same carrier. A
  two-level connectivity is not an obstacle to a second implementation; it is something a second
  implementation must model — and the reference models BOTH cascades: deleting a node severs every
  edge attached to any of its handles, and removing a single handle severs the edges attached to that
  handle alone. A third-party library was nonetheless declined and the reason is concrete: GraphML,
  DOT and GEXF all join node to node, none of them can express an edge whose endpoints are ports
  OWNED BY a node, and none of them reads this carrier.

  🚧️ TWO REFUSALS THE REFERENCE ARGUES BY CLAUSE, and reports rather than works around. First,
  `replace-node-handle`. Its ONLY committed vector, `🌾️rekind-handle-1-is-noop`, supplies a genuinely
  different handle — `handle-1` moves from `handle-kind-a` to `handle-kind-c` — and yet its committed
  outcome declares `mutation.no-op` and its after-snapshot is identical to its before-snapshot. At
  least three rules produce exactly that and no committed document distinguishes them: the verb is
  unimplemented; it refuses a handle an edge is attached to, which `handle-1` is; or it refuses a
  handle kind the `kindCompatibility` relation does not admit, which `handle-kind-c` is.
  `📓️derivation-rules.md` rule 2 says `replace-<singular>-<member>` replaces the addressed record, so
  a second implementation written from the specification would move the document — and the reference
  declines to pick one of the three rules and call it agreement. ONE more vector, on an unconnected
  handle, decides it. Second, `inverse-replace-kind-catalogs`: the committed vector INSTALLS a
  catalogue where the before-snapshot carried none, so undoing it means REMOVING the member, and
  nothing committed says whether the verb accepts a null argument.

  📌️ A FINDING MADE WHILE THE REFERENCE WAS BEING WRITTEN.
  `🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔣️.json` is not a mutation schema at
  all: it is titled `Puzzle2dMutation` and declares `{schema, camera, nodes, edges, meta}` — a copy of
  the SNAPSHOT schema, the pre-migration whole-snapshot-shaped generic schema that
  `s.architect.program`'s own mutation schema records itself as superseding. It was never replaced
  here, so the verbs and their argument lists had to be read off the committed payloads instead. It is
  also where the one spelling inconsistency in this vocabulary would have been caught:
  `replace-edge-geometry` names the eight edge-geometry values with a `new` prefix and
  `connect-handles` names the same eight bare.

  📌️ TWO CEILINGS ON WHAT THIS COMPARISON ESTABLISHES, stated rather than implied. First, the
  SUBJECT half does not run this subset's codec: `🦀️component.rs` beside this file links no plugin
  crate and replays the committed vectors, so today the comparison establishes that an independent
  implementation of the specification computes the committed after-snapshots — a real check of the
  vectors, and the class of check that found `🦅️mutate-jack-1`'s wrong vector — but not yet our codec
  against a second producer. A `puzzle2d_mutation_report_json` bridge beside the mutation enum closes
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
  artifact: all 130 of its fixtures are handcrafted specification vectors.

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
      | id                            | vector                                                          |
      | create-node                   | 🌱create-node/🧪️tests/appends-node-c                             |
      | delete-node                   | 🗑️delete-node/🧪️tests/removes-node-a-and-severs-edge             |
      | move-node                     | 📍move-node/🧪️tests/moves-node-a                                 |
      | replace-node-geometry         | 🧊replace-node-geometry/🧪️tests/circle-to-rectangle              |
      | change-node-kind              | 🏗️change-node-kind/🧪️tests/reassigns-node-a-kind                 |
      | edit-node-text                | ✏️edit-node-text/🧪️tests/retitles-node-a                        |
      | change-node-icon              | 🎨change-node-icon/🧪️tests/swaps-node-a-icon                     |
      | scale-node                    | 📏scale-node/🧪️tests/doubles-node-a                              |
      | change-node-visible           | 👁️change-node-visible/🧪️tests/hides-node-a                       |
      | change-node-locked            | 🔒change-node-locked/🧪️tests/locks-node-a                        |
      | change-node-root              | 🌟change-node-root/🧪️tests/promotes-node-a-to-root               |
      | change-node-anchor            | ⚓change-node-anchor/🧪️tests/fixed-to-derived                    |
      | add-node-handle               | ➕add-node-handle/🧪️tests/appends-handle-3-to-node-b             |
      | remove-node-handle            | ➖remove-node-handle/🧪️tests/removes-handle-2-and-severs-edge    |
      | replace-node-handle           | 🔌replace-node-handle/🧪️tests/rekind-handle-1-is-noop            |
      | connect-handles               | 🪢️connect-handles/🧪️tests/adds-second-edge                       |
      | disconnect-handles            | ✂️disconnect-handles/🧪️tests/removes-edge-1                     |
      | replace-edge-geometry         | 🧮replace-edge-geometry/🧪️tests/repositions-edge-1               |
      | change-edge-kind              | 🏷️change-edge-kind/🧪️tests/rekinds-edge-1                        |
      | change-edge-tips              | 🖇️change-edge-tips/🧪️tests/swaps-edge-1-tips                     |
      | change-edge-visible           | 👀change-edge-visible/🧪️tests/hides-edge-1                       |
      | change-edge-locked            | 🔐change-edge-locked/🧪️tests/locks-edge-1                        |
      | change-manifest-id            | 🆔change-manifest-id/🧪️tests/repoints-manifest                   |
      | connect-kind-compatibility    | 🤝connect-kind-compatibility/🧪️tests/adds-handle-kind-pair       |
      | disconnect-kind-compatibility | 💔disconnect-kind-compatibility/🧪️tests/removes-handle-kind-pair |
      | replace-kind-catalogs         | 📚replace-kind-catalogs/🧪️tests/installs-handle-kind-catalog     |

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
      | id                            | vector                                                          |
      | create-node                   | 🌱create-node/🧪️tests/appends-node-c                             |
      | delete-node                   | 🗑️delete-node/🧪️tests/removes-node-a-and-severs-edge             |
      | move-node                     | 📍move-node/🧪️tests/moves-node-a                                 |
      | replace-node-geometry         | 🧊replace-node-geometry/🧪️tests/circle-to-rectangle              |
      | change-node-kind              | 🏗️change-node-kind/🧪️tests/reassigns-node-a-kind                 |
      | edit-node-text                | ✏️edit-node-text/🧪️tests/retitles-node-a                        |
      | change-node-icon              | 🎨change-node-icon/🧪️tests/swaps-node-a-icon                     |
      | scale-node                    | 📏scale-node/🧪️tests/doubles-node-a                              |
      | change-node-visible           | 👁️change-node-visible/🧪️tests/hides-node-a                       |
      | change-node-locked            | 🔒change-node-locked/🧪️tests/locks-node-a                        |
      | change-node-root              | 🌟change-node-root/🧪️tests/promotes-node-a-to-root               |
      | change-node-anchor            | ⚓change-node-anchor/🧪️tests/fixed-to-derived                    |
      | add-node-handle               | ➕add-node-handle/🧪️tests/appends-handle-3-to-node-b             |
      | remove-node-handle            | ➖remove-node-handle/🧪️tests/removes-handle-2-and-severs-edge    |
      | replace-node-handle           | 🔌replace-node-handle/🧪️tests/rekind-handle-1-is-noop            |
      | connect-handles               | 🪢️connect-handles/🧪️tests/adds-second-edge                       |
      | disconnect-handles            | ✂️disconnect-handles/🧪️tests/removes-edge-1                     |
      | replace-edge-geometry         | 🧮replace-edge-geometry/🧪️tests/repositions-edge-1               |
      | change-edge-kind              | 🏷️change-edge-kind/🧪️tests/rekinds-edge-1                        |
      | change-edge-tips              | 🖇️change-edge-tips/🧪️tests/swaps-edge-1-tips                     |
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
    Given the committed before-snapshot asset://🧬️schema/🧬️mutations/🌱create-node/🧪️tests/🍊️appends-node-c/📸️snapshot/⬅️before/🔣️.json
    When it is parsed by the platform's own dependency-free JSON reader, re-serialized and parsed again
    Then the document is unchanged and the re-serialized bytes are not the committed bytes
