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
  from the committed quintets. It imports nothing from this repository's Rust.

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

  ✅️ THE TWO REFUSALS THIS CASE USED TO ARGUE ARE BOTH SETTLED, by evidence rather than by fiat.
  First, `replace-node-handle`. Its only vector used to be `⏸️rekind-handle-1-is-noop`, which supplied
  a genuinely different handle and yet declared `mutation.no-op`, leaving three readings open at once:
  an unimplemented verb, a refusal of an edge-attached handle, or a refusal of a kind the
  `kindCompatibility` relation does not admit. The subject half turned out to hold a real defect —
  `🔌replace-node-handle/🔺️diff/🦀️.rs` ran its no-op guard BEFORE the replacement loop, so the verb
  could never move anything — and the reference's refusal is what kept that defect visible instead of
  green. The guard now runs after the loop, and `🔌️rekinds-an-unconnected-tambour-door` states the
  rule on real data: an UNCONNECTED door of the shipped Nakagin tower's second tambour, re-kinded to a
  kind the relation admits, really replaces the addressed handle. Second,
  `inverse-replace-kind-catalogs`: `🗑️clears-the-installed-handle-catalog` shows that a NULL
  `newCatalogs` argument is accepted and REMOVES the member, so undoing an install is expressible in
  this closed vocabulary after all. The reference refuses nothing today.

  📌️ WHAT THE THREE TABLES BELOW MEASURE. The `mutate`/`inverse` tables carry ONE vector per kind and
  every one of them is derived from a shipped example: twenty-five from the First-Storey-Tambour
  subgraph of `📚️examples/🏗️nakagin-capsule-tower` (twelve real capsule/tambour/base nodes with their
  real UUID ids, kinds, coordinates, id-codes and door-kind handles, ten real edges, and the tower's
  own fourteen-row kind-compatibility relation), and `replace-node-geometry` from
  `📚️examples/🌲️concrete-forest`'s seed node. The `spec-vector` table carries everything else: the
  original synthetic alpha-board vectors, which are kept rather than replaced because they exercise a
  two-node board no real example offers; twenty-two REFUSAL vectors, each committing
  `🔺️diff/🚫️.absent` under contract D6 rather than an invented empty patch; and the two vectors that
  pin the warning-level branches — a duplicate edge id is a `mutation.no-op`, not a rejection, and a
  null catalogue argument clears rather than refuses.

  📌️ TWO CEILINGS ON WHAT THIS COMPARISON ESTABLISHES, stated rather than implied. First, the
  SUBJECT half does not run this subset's codec: `🦀️component.rs` beside this file links no plugin
  crate and replays the committed vectors, so today the comparison establishes that an independent
  implementation of the specification computes the committed after-snapshots — a real check of the
  vectors, and the class of check that found `🦅️mutate-jack-1`'s wrong vector — but not yet our codec
  against a second producer. A `puzzle2d_mutation_report_json` bridge beside the mutation enum closes
  it; it is PRODUCTION code in a crate this test-side pass deliberately does not touch. The per-leaf
  `🧪️tests/<vector>/🦀️.rs` files DO link the crate and do assert the diff builders, the inverses and
  the refusal codes against exactly these committed leaves, so the gap is narrower than it reads.
  Second, the alpha-board vectors remain handcrafted specification vectors; only the real-world half
  of the corpus is example-derived.

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
      | id                            | vector                                                                           |
      | create-node                   | 🌱create-node/🧪️tests/🌱️appends-a-capsule-to-the-tower                            |
      | delete-node                   | 🗑️delete-node/🧪️tests/🚫️deletes-the-tambour-and-severs-its-ten-edges             |
      | move-node                     | 📍move-node/🧪️tests/📍️moves-a-capsule-across-the-shaft                            |
      | replace-node-geometry         | 🧊replace-node-geometry/🧪️tests/🔳️squares-the-concrete-forest-seed                |
      | change-node-kind              | 🏗️change-node-kind/🧪️tests/🏗️rekinds-a-capsule-j-as-a-capsule-l                  |
      | edit-node-text                | ✏️edit-node-text/🧪️tests/✏️recodes-a-capsule-id-code                             |
      | change-node-icon              | 🎨change-node-icon/🧪️tests/🎨️swaps-a-capsule-icon                                 |
      | scale-node                    | 📏scale-node/🧪️tests/📏️scales-a-capsule-by-three-halves                           |
      | change-node-visible           | 👁️change-node-visible/🧪️tests/🙈️hides-a-capsule                                  |
      | change-node-locked            | 🔒change-node-locked/🧪️tests/🔒️locks-the-first-storey-tambour                     |
      | change-node-root              | 🌟change-node-root/🧪️tests/🌳️promotes-the-base-to-root                            |
      | change-node-anchor            | ⚓change-node-anchor/🧪️tests/⚓️derives-a-capsule-pose-from-its-door-edge          |
      | add-node-handle               | ➕add-node-handle/🧪️tests/➕️adds-a-third-slot-door-to-the-tambour                 |
      | remove-node-handle            | ➖remove-node-handle/🧪️tests/🚫️removes-a-tambour-door-and-severs-its-capsule-edge |
      | replace-node-handle           | 🔌replace-node-handle/🧪️tests/🔌️rekinds-an-unconnected-tambour-door               |
      | connect-handles               | 🪢️connect-handles/🧪️tests/🪢️rewires-the-capsule-the-subgraph-left-loose          |
      | disconnect-handles            | ✂️disconnect-handles/🧪️tests/✂️severs-a-capsule-from-the-first-storey-tambour    |
      | replace-edge-geometry         | 🧮replace-edge-geometry/🧪️tests/🧮️reposes-a-capsule-door-edge                     |
      | change-edge-kind              | 🏷️change-edge-kind/🧪️tests/🏷️kinds-a-capsule-door-edge-as-a-link                 |
      | change-edge-tips              | 🖇️change-edge-tips/🧪️tests/🖇️tips-a-capsule-door-edge                            |
      | change-edge-visible           | 👀change-edge-visible/🧪️tests/🙈️hides-a-capsule-door-edge                         |
      | change-edge-locked            | 🔐change-edge-locked/🧪️tests/🔒️locks-the-base-to-tambour-edge                     |
      | change-manifest-id            | 🆔change-manifest-id/🧪️tests/📦️repoints-the-tower-at-its-example-manifest         |
      | connect-kind-compatibility    | 🤝connect-kind-compatibility/🧪️tests/🤝️admits-the-reverse-tambour-circular-pair   |
      | disconnect-kind-compatibility | 💔disconnect-kind-compatibility/🧪️tests/💔️withdraws-the-tambour-rectangular-pair  |
      | replace-kind-catalogs         | 📚replace-kind-catalogs/🧪️tests/📇️installs-the-tower-handle-catalog               |

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
      | id                            | vector                                                                           |
      | create-node                   | 🌱create-node/🧪️tests/🌱️appends-a-capsule-to-the-tower                            |
      | delete-node                   | 🗑️delete-node/🧪️tests/🚫️deletes-the-tambour-and-severs-its-ten-edges             |
      | move-node                     | 📍move-node/🧪️tests/📍️moves-a-capsule-across-the-shaft                            |
      | replace-node-geometry         | 🧊replace-node-geometry/🧪️tests/🔳️squares-the-concrete-forest-seed                |
      | change-node-kind              | 🏗️change-node-kind/🧪️tests/🏗️rekinds-a-capsule-j-as-a-capsule-l                  |
      | edit-node-text                | ✏️edit-node-text/🧪️tests/✏️recodes-a-capsule-id-code                             |
      | change-node-icon              | 🎨change-node-icon/🧪️tests/🎨️swaps-a-capsule-icon                                 |
      | scale-node                    | 📏scale-node/🧪️tests/📏️scales-a-capsule-by-three-halves                           |
      | change-node-visible           | 👁️change-node-visible/🧪️tests/🙈️hides-a-capsule                                  |
      | change-node-locked            | 🔒change-node-locked/🧪️tests/🔒️locks-the-first-storey-tambour                     |
      | change-node-root              | 🌟change-node-root/🧪️tests/🌳️promotes-the-base-to-root                            |
      | change-node-anchor            | ⚓change-node-anchor/🧪️tests/⚓️derives-a-capsule-pose-from-its-door-edge          |
      | add-node-handle               | ➕add-node-handle/🧪️tests/➕️adds-a-third-slot-door-to-the-tambour                 |
      | remove-node-handle            | ➖remove-node-handle/🧪️tests/🚫️removes-a-tambour-door-and-severs-its-capsule-edge |
      | replace-node-handle           | 🔌replace-node-handle/🧪️tests/🔌️rekinds-an-unconnected-tambour-door               |
      | connect-handles               | 🪢️connect-handles/🧪️tests/🪢️rewires-the-capsule-the-subgraph-left-loose          |
      | disconnect-handles            | ✂️disconnect-handles/🧪️tests/✂️severs-a-capsule-from-the-first-storey-tambour    |
      | replace-edge-geometry         | 🧮replace-edge-geometry/🧪️tests/🧮️reposes-a-capsule-door-edge                     |
      | change-edge-kind              | 🏷️change-edge-kind/🧪️tests/🏷️kinds-a-capsule-door-edge-as-a-link                 |
      | change-edge-tips              | 🖇️change-edge-tips/🧪️tests/🖇️tips-a-capsule-door-edge                            |
      | change-edge-visible           | 👀change-edge-visible/🧪️tests/🙈️hides-a-capsule-door-edge                         |
      | change-edge-locked            | 🔐change-edge-locked/🧪️tests/🔒️locks-the-base-to-tambour-edge                     |
      | change-manifest-id            | 🆔change-manifest-id/🧪️tests/📦️repoints-the-tower-at-its-example-manifest         |
      | connect-kind-compatibility    | 🤝connect-kind-compatibility/🧪️tests/🤝️admits-the-reverse-tambour-circular-pair   |
      | disconnect-kind-compatibility | 💔disconnect-kind-compatibility/🧪️tests/💔️withdraws-the-tambour-rectangular-pair  |
      | replace-kind-catalogs         | 📚replace-kind-catalogs/🧪️tests/📇️installs-the-tower-handle-catalog               |

  @id-spec-vector
  @level-exhaustive
  @mode-differential
  Scenario Outline: Replay the committed <id> specification vector through both implementations
    Given the committed specification vector for the <kind> kind
      """
      {
        "kind": "<kind>",
        "verdict": "<verdict>",
        "before": "asset://🧬️schema/🧬️mutations/<vector>/📸️snapshot/⬅️before/🔣️.json",
        "mutation": "asset://🧬️schema/🧬️mutations/<vector>/🦠️mutation/🔣️.json",
        "diff": "asset://🧬️schema/🧬️mutations/<vector>/🔺️diff/<diff>",
        "outcome": "asset://🧬️schema/🧬️mutations/<vector>/🎯️outcome/🔣️.json",
        "after": "asset://🧬️schema/🧬️mutations/<vector>/📸️snapshot/➡️after/🔣️.json"
      }
      """
    Then each implementation gives the committed <verdict> answer in role, and the two agree
    Examples:
      | id                                    | kind                          | verdict | vector                                                                                      | diff      |
      | create-node-alpha                     | create-node                   | applied | 🌱create-node/🧪️tests/🌱️appends-node-c                                                       | 🔣️.json   |
      | create-node-refused                   | create-node                   | refused | 🌱create-node/🧪️tests/🚫️rejects-a-capsule-id-the-tower-already-holds                         | 🚫️.absent |
      | delete-node-alpha                     | delete-node                   | applied | 🗑️delete-node/🧪️tests/🚫️removes-node-a-and-severs-edge                                      | 🔣️.json   |
      | delete-node-refused                   | delete-node                   | refused | 🗑️delete-node/🧪️tests/🚫️rejects-deleting-a-capsule-the-board-never-held                     | 🚫️.absent |
      | move-node-alpha                       | move-node                     | applied | 📍move-node/🧪️tests/📍️moves-node-a                                                           | 🔣️.json   |
      | move-node-refused                     | move-node                     | refused | 📍move-node/🧪️tests/🚫️rejects-moving-a-capsule-the-board-never-held                          | 🚫️.absent |
      | replace-node-geometry-alpha           | replace-node-geometry         | applied | 🧊replace-node-geometry/🧪️tests/🔳️circle-to-rectangle                                        | 🔣️.json   |
      | replace-node-geometry-refused         | replace-node-geometry         | refused | 🧊replace-node-geometry/🧪️tests/🚫️rejects-reshaping-a-capsule-the-board-never-held           | 🚫️.absent |
      | change-node-kind-alpha                | change-node-kind              | applied | 🏗️change-node-kind/🧪️tests/🏷️reassigns-node-a-kind                                          | 🔣️.json   |
      | change-node-kind-refused              | change-node-kind              | refused | 🏗️change-node-kind/🧪️tests/🚫️rejects-rekinding-a-capsule-the-board-never-held               | 🚫️.absent |
      | edit-node-text-alpha                  | edit-node-text                | applied | ✏️edit-node-text/🧪️tests/✏️retitles-node-a                                                  | 🔣️.json   |
      | edit-node-text-refused                | edit-node-text                | refused | ✏️edit-node-text/🧪️tests/🚫️rejects-recoding-a-capsule-the-board-never-held                  | 🚫️.absent |
      | change-node-icon-alpha                | change-node-icon              | applied | 🎨change-node-icon/🧪️tests/🎨️swaps-node-a-icon                                               | 🔣️.json   |
      | change-node-icon-refused              | change-node-icon              | refused | 🎨change-node-icon/🧪️tests/🚫️rejects-reiconing-a-capsule-the-board-never-held                | 🚫️.absent |
      | scale-node-alpha                      | scale-node                    | applied | 📏scale-node/🧪️tests/📏️doubles-node-a                                                        | 🔣️.json   |
      | scale-node-refused                    | scale-node                    | refused | 📏scale-node/🧪️tests/🚫️rejects-scaling-a-capsule-the-board-never-held                        | 🚫️.absent |
      | change-node-visible-alpha             | change-node-visible           | applied | 👁️change-node-visible/🧪️tests/🙈️hides-node-a                                                | 🔣️.json   |
      | change-node-visible-refused           | change-node-visible           | refused | 👁️change-node-visible/🧪️tests/🚫️rejects-hiding-a-capsule-the-board-never-held               | 🚫️.absent |
      | change-node-locked-alpha              | change-node-locked            | applied | 🔒change-node-locked/🧪️tests/🔒️locks-node-a                                                  | 🔣️.json   |
      | change-node-locked-refused            | change-node-locked            | refused | 🔒change-node-locked/🧪️tests/🚫️rejects-locking-a-capsule-the-board-never-held                | 🚫️.absent |
      | change-node-root-alpha                | change-node-root              | applied | 🌟change-node-root/🧪️tests/🌳️promotes-node-a-to-root                                         | 🔣️.json   |
      | change-node-root-refused              | change-node-root              | refused | 🌟change-node-root/🧪️tests/🚫️rejects-rooting-a-capsule-the-board-never-held                  | 🚫️.absent |
      | change-node-anchor-alpha              | change-node-anchor            | applied | ⚓change-node-anchor/🧪️tests/⚓️fixed-to-derived                                              | 🔣️.json   |
      | change-node-anchor-refused            | change-node-anchor            | refused | ⚓change-node-anchor/🧪️tests/🚫️rejects-anchoring-a-capsule-the-board-never-held              | 🚫️.absent |
      | add-node-handle-alpha                 | add-node-handle               | applied | ➕add-node-handle/🧪️tests/➕️appends-handle-3-to-node-b                                       | 🔣️.json   |
      | add-node-handle-refused               | add-node-handle               | refused | ➕add-node-handle/🧪️tests/🚫️rejects-adding-a-door-to-a-capsule-the-board-never-held          | 🚫️.absent |
      | remove-node-handle-alpha              | remove-node-handle            | applied | ➖remove-node-handle/🧪️tests/🚫️removes-handle-2-and-severs-edge                              | 🔣️.json   |
      | remove-node-handle-refused            | remove-node-handle            | refused | ➖remove-node-handle/🧪️tests/🚫️rejects-removing-a-door-the-tambour-never-had                 | 🚫️.absent |
      | replace-node-handle-refused           | replace-node-handle           | refused | 🔌replace-node-handle/🧪️tests/🚫️rejects-replacing-a-door-the-tambour-never-had               | 🚫️.absent |
      | connect-handles-alpha                 | connect-handles               | applied | 🪢️connect-handles/🧪️tests/🪢️adds-second-edge                                                | 🔣️.json   |
      | connect-handles-duplicate             | connect-handles               | noop    | 🪢️connect-handles/🧪️tests/⏸️keeps-an-edge-the-tower-already-holds                           | 🔣️.json   |
      | disconnect-handles-alpha              | disconnect-handles            | applied | ✂️disconnect-handles/🧪️tests/🚫️removes-edge-1                                               | 🔣️.json   |
      | disconnect-handles-refused            | disconnect-handles            | refused | ✂️disconnect-handles/🧪️tests/🚫️rejects-severing-an-edge-the-board-never-held                | 🚫️.absent |
      | replace-edge-geometry-alpha           | replace-edge-geometry         | applied | 🧮replace-edge-geometry/🧪️tests/📍️repositions-edge-1                                         | 🔣️.json   |
      | replace-edge-geometry-refused         | replace-edge-geometry         | refused | 🧮replace-edge-geometry/🧪️tests/🚫️rejects-reposing-an-edge-the-board-never-held              | 🚫️.absent |
      | change-edge-kind-alpha                | change-edge-kind              | applied | 🏷️change-edge-kind/🧪️tests/🏷️rekinds-edge-1                                                 | 🔣️.json   |
      | change-edge-kind-refused              | change-edge-kind              | refused | 🏷️change-edge-kind/🧪️tests/🚫️rejects-kinding-an-edge-the-board-never-held                   | 🚫️.absent |
      | change-edge-tips-alpha                | change-edge-tips              | applied | 🖇️change-edge-tips/🧪️tests/🔀️swaps-edge-1-tips                                              | 🔣️.json   |
      | change-edge-tips-refused              | change-edge-tips              | refused | 🖇️change-edge-tips/🧪️tests/🚫️rejects-tipping-an-edge-the-board-never-held                   | 🚫️.absent |
      | change-edge-visible-alpha             | change-edge-visible           | applied | 👀change-edge-visible/🧪️tests/🙈️hides-edge-1                                                 | 🔣️.json   |
      | change-edge-visible-refused           | change-edge-visible           | refused | 👀change-edge-visible/🧪️tests/🚫️rejects-hiding-an-edge-the-board-never-held                  | 🚫️.absent |
      | change-edge-locked-alpha              | change-edge-locked            | applied | 🔐change-edge-locked/🧪️tests/🔒️locks-edge-1                                                  | 🔣️.json   |
      | change-edge-locked-refused            | change-edge-locked            | refused | 🔐change-edge-locked/🧪️tests/🚫️rejects-locking-an-edge-the-board-never-held                  | 🚫️.absent |
      | change-manifest-id-alpha              | change-manifest-id            | applied | 🆔change-manifest-id/🧪️tests/📦️repoints-manifest                                             | 🔣️.json   |
      | connect-kind-compatibility-alpha      | connect-kind-compatibility    | applied | 🤝connect-kind-compatibility/🧪️tests/🤝️adds-handle-kind-pair                                 | 🔣️.json   |
      | disconnect-kind-compatibility-alpha   | disconnect-kind-compatibility | applied | 💔disconnect-kind-compatibility/🧪️tests/🚫️removes-handle-kind-pair                           | 🔣️.json   |
      | disconnect-kind-compatibility-refused | disconnect-kind-compatibility | refused | 💔disconnect-kind-compatibility/🧪️tests/🚫️rejects-withdrawing-a-pair-the-relation-never-held | 🚫️.absent |
      | replace-kind-catalogs-alpha           | replace-kind-catalogs         | applied | 📚replace-kind-catalogs/🧪️tests/📇️installs-handle-kind-catalog                               | 🔣️.json   |
      | replace-kind-catalogs-cleared         | replace-kind-catalogs         | applied | 📚replace-kind-catalogs/🧪️tests/🗑️clears-the-installed-handle-catalog                        | 🔣️.json   |

  @id-identity-round-trip
  @level-long
  @mode-round-trip
  Scenario: Decode and re-encode the real First-Storey-Tambour subgraph of the Nakagin Capsule Tower
    Given the committed before-snapshot asset://🧬️schema/🧬️mutations/🌱create-node/🧪️tests/🌱️appends-a-capsule-to-the-tower/📸️snapshot/⬅️before/🔣️.json
    When it is parsed by the platform's own dependency-free JSON reader, re-serialized and parsed again
    Then the document is unchanged and the re-serialized bytes are not the committed bytes
