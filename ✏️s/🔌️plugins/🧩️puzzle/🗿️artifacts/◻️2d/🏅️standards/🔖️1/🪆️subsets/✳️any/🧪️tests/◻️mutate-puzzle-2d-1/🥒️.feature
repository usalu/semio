@capability-puzzle-2d-1-mutate
@oracle-puzzle-2d-python-independent
@comparison-ordered-json-v1
@mutations-puzzle-2d-1-any
Feature: Apply every typed puzzle2d board mutation twice — once in Rust, once in Python — and require the same answer
  This case is a CROSS-LANGUAGE DIFFERENTIAL. The reference is `🐍️component.py` in this directory: a
  second implementation of the `s.puzzle.2d` board document and its thirty-six typed mutations,
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
  green. The guard now runs after the loop, and `🔌️rekinds` states the
  rule on real data: an UNCONNECTED door of the shipped Nakagin tower's second tambour, re-kinded to a
  kind the relation admits, really replaces the addressed handle. Second,
  `inverse-replace-kind-catalogs`: `🗑️clears` shows that a NULL
  `newCatalogs` argument is accepted and REMOVES the member, so undoing an install is expressible in
  this closed vocabulary after all. The reference refuses nothing today.

  📌️ WHAT THE THREE TABLES BELOW MEASURE. The `mutate`/`inverse` tables carry ONE vector per kind and
  every one of them is derived from a shipped example: twenty-five from the First-Storey-Tambour
  subgraph of `📚️examples/🏗️nakagin-capsule-tower` (twelve real capsule/tambour/base nodes with their
  real UUID ids, kinds, coordinates, id-codes and door-kind handles, ten real edges, and the tower's
  own fourteen-row kind-compatibility relation), and `replace-node-geometry` from
  `📚️examples/🌲️concrete-forest`'s seed node. The `spec-vector` table carries everything else: the
  original synthetic alpha-board vectors, which are kept rather than replaced because they exercise a
  two-node board no real example offers; twenty-nine REFUSAL vectors, each committing
  `🔺️diff/🚫️.absent` under contract D6 rather than an invented empty patch; and the two vectors that
  pin the warning-level branches — a duplicate edge id is a `mutation.no-op`, not a rejection, and a
  null catalogue argument clears rather than refuses. The three parametric selection transforms
  (`drag-`, `rotate-`, `scale-selection`) run on one synthetic selection board carrying a locked node
  and a locked target region, and add four rows each: a target set mixing nodes and target regions,
  a PARTIAL vector whose missing and locked members degrade to `mutation.partial` while the survivors
  move, a refusal whose every target is absent, and identity parameters that are a `no-op`. Ten
  `<kind>-invariant` refusals state the schema-first rule on the same board: a payload that breaks a hard
  bound of its own leaf schema (an empty or repeated target set, a zero or negative factor or scale, a
  negative radius, a zero-width node, a zero-radius or negative-scale handle, a template rim parameter
  off the outline) is refused as a Fatal `mutation.invariant` by both implementations.

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
        "before": "shared://🧬️mutations/<vector>/📸️snapshot/⬅️before/🔣️.json",
        "mutation": "shared://🧬️mutations/<vector>/🦠️mutation/🔣️.json",
        "diff": "shared://🧬️mutations/<vector>/🔺️diff/🔣️.json",
        "outcome": "shared://🧬️mutations/<vector>/🎯️outcome/🔣️.json",
        "after": "shared://🧬️mutations/<vector>/📸️snapshot/➡️after/🔣️.json"
      }
      """
    Then the committed mutation payload declares the <id> kind
    And the after-snapshot differs from the before-snapshot, or the committed outcome declares the vector a no-op
    Examples:
      | id                            | vector                                                                           |
      | create-node                   | 🌱create-node/🌱️appends                            |
      | delete-node                   | 🗑️delete-node/🚫️deletes             |
      | move-node                     | 📍move-node/📍️moves                            |
      | replace-node-geometry         | 🧊replace-node-geometry/🔳️squares                |
      | change-node-kind              | 🏗️change-node-kind/🏗️rekinds                  |
      | edit-node-text                | ✏️edit-node-text/✏️recodes                             |
      | change-node-icon              | 🎨change-node-icon/🎨️swaps-a-capsule-icon                                 |
      | scale-node                    | 📏scale-node/📏️scales                           |
      | change-node-visible           | 👁️change-node-visible/🙈️hides-a-capsule                                  |
      | change-node-locked            | 🔒change-node-locked/🔒️locks                     |
      | change-node-root              | 🌟change-node-root/🌳️promotes                            |
      | change-node-anchor            | ⚓change-node-anchor/⚓️derives          |
      | add-node-handle               | ➕add-node-handle/➕️adds                 |
      | remove-node-handle            | ➖remove-node-handle/🚫️removes |
      | replace-node-handle           | 🔌replace-node-handle/🔌️rekinds               |
      | connect-handles               | 🪢️connect-handles/🪢️rewires          |
      | disconnect-handles            | ✂️disconnect-handles/✂️severs    |
      | replace-edge-geometry         | 🧮replace-edge-geometry/🧮️reposes                     |
      | change-edge-kind              | 🏷️change-edge-kind/🏷️kinds                 |
      | change-edge-tips              | 🖇️change-edge-tips/🖇️tips                            |
      | change-edge-visible           | 👀change-edge-visible/🙈️hides                         |
      | change-edge-locked            | 🔐change-edge-locked/🔒️locks                     |
      | change-manifest-id            | 🆔change-manifest-id/📦️repoints         |
      | connect-kind-compatibility    | 🤝connect-kind-compatibility/🤝️admits   |
      | disconnect-kind-compatibility | 💔disconnect-kind-compatibility/💔️withdraws  |
      | replace-kind-catalogs         | 📚replace-kind-catalogs/📇️installs2               |
      | create-target-region          | 🌍create-target-region/🌍️paints                         |
      | delete-target-region          | 🪦delete-target-region/🪦️removes-region-1                                 |
      | move-target-region            | 🚀move-target-region/🚀️slides-region-1                                    |
      | resize-target-region          | 📐resize-target-region/📐️widens-region-1                                  |
      | edit-target-region-label      | 🖋️edit-target-region-label/🖋️renames                            |
      | change-target-region-hidden   | 🙈change-target-region-hidden/🙈️hides                            |
      | change-target-region-locked   | 🔏change-target-region-locked/🔏️locks                            |
      | drag-selection                | ✋️drag-selection/✋️drags-two-nodes                                        |
      | rotate-selection              | 🔄️rotate-selection/🔄️turns-two-nodes                             |
      | scale-selection               | 🔍️scale-selection/🔍️doubles-two-nodes                             |

  @id-inverse
  @level-exhaustive
  @mode-differential
  Scenario Outline: The committed <id> vector changes only what its diff declares
    Given the committed specification vector for the <id> kind
      """
      {
        "kind": "<id>",
        "before": "shared://🧬️mutations/<vector>/📸️snapshot/⬅️before/🔣️.json",
        "mutation": "shared://🧬️mutations/<vector>/🦠️mutation/🔣️.json",
        "diff": "shared://🧬️mutations/<vector>/🔺️diff/🔣️.json",
        "outcome": "shared://🧬️mutations/<vector>/🎯️outcome/🔣️.json",
        "after": "shared://🧬️mutations/<vector>/📸️snapshot/➡️after/🔣️.json"
      }
      """
    Then every field where the after-snapshot differs from the before-snapshot is declared by the committed diff
    And every field the committed diff declares actually differs
    Examples:
      | id                            | vector                                                                           |
      | create-node                   | 🌱create-node/🌱️appends                            |
      | delete-node                   | 🗑️delete-node/🚫️deletes             |
      | move-node                     | 📍move-node/📍️moves                            |
      | replace-node-geometry         | 🧊replace-node-geometry/🔳️squares                |
      | change-node-kind              | 🏗️change-node-kind/🏗️rekinds                  |
      | edit-node-text                | ✏️edit-node-text/✏️recodes                             |
      | change-node-icon              | 🎨change-node-icon/🎨️swaps-a-capsule-icon                                 |
      | scale-node                    | 📏scale-node/📏️scales                           |
      | change-node-visible           | 👁️change-node-visible/🙈️hides-a-capsule                                  |
      | change-node-locked            | 🔒change-node-locked/🔒️locks                     |
      | change-node-root              | 🌟change-node-root/🌳️promotes                            |
      | change-node-anchor            | ⚓change-node-anchor/⚓️derives          |
      | add-node-handle               | ➕add-node-handle/➕️adds                 |
      | remove-node-handle            | ➖remove-node-handle/🚫️removes |
      | replace-node-handle           | 🔌replace-node-handle/🔌️rekinds               |
      | connect-handles               | 🪢️connect-handles/🪢️rewires          |
      | disconnect-handles            | ✂️disconnect-handles/✂️severs    |
      | replace-edge-geometry         | 🧮replace-edge-geometry/🧮️reposes                     |
      | change-edge-kind              | 🏷️change-edge-kind/🏷️kinds                 |
      | change-edge-tips              | 🖇️change-edge-tips/🖇️tips                            |
      | change-edge-visible           | 👀change-edge-visible/🙈️hides                         |
      | change-edge-locked            | 🔐change-edge-locked/🔒️locks                     |
      | change-manifest-id            | 🆔change-manifest-id/📦️repoints         |
      | connect-kind-compatibility    | 🤝connect-kind-compatibility/🤝️admits   |
      | disconnect-kind-compatibility | 💔disconnect-kind-compatibility/💔️withdraws  |
      | replace-kind-catalogs         | 📚replace-kind-catalogs/📇️installs2               |
      | create-target-region          | 🌍create-target-region/🌍️paints                         |
      | delete-target-region          | 🪦delete-target-region/🪦️removes-region-1                                 |
      | move-target-region            | 🚀move-target-region/🚀️slides-region-1                                    |
      | resize-target-region          | 📐resize-target-region/📐️widens-region-1                                  |
      | edit-target-region-label      | 🖋️edit-target-region-label/🖋️renames                            |
      | change-target-region-hidden   | 🙈change-target-region-hidden/🙈️hides                            |
      | change-target-region-locked   | 🔏change-target-region-locked/🔏️locks                            |
      | drag-selection                | ✋️drag-selection/✋️drags-two-nodes                                        |
      | rotate-selection              | 🔄️rotate-selection/🔄️turns-two-nodes                             |
      | scale-selection               | 🔍️scale-selection/🔍️doubles-two-nodes                             |

  @id-spec-vector
  @level-exhaustive
  @mode-differential
  Scenario Outline: Replay the committed <id> specification vector through both implementations
    Given the committed specification vector for the <kind> kind
      """
      {
        "kind": "<kind>",
        "verdict": "<verdict>",
        "before": "shared://🧬️mutations/<vector>/📸️snapshot/⬅️before/🔣️.json",
        "mutation": "shared://🧬️mutations/<vector>/🦠️mutation/🔣️.json",
        "diff": "shared://🧬️mutations/<vector>/🔺️diff/<diff>",
        "outcome": "shared://🧬️mutations/<vector>/🎯️outcome/🔣️.json",
        "after": "shared://🧬️mutations/<vector>/📸️snapshot/➡️after/🔣️.json"
      }
      """
    Then each implementation gives the committed <verdict> answer in role, and the two agree
    Examples:
      | id                                    | kind                          | verdict | vector                                                                                      | diff      |
      | create-node-alpha                     | create-node                   | applied | 🌱create-node/🌱️appends-node-c                                                       | 🔣️.json   |
      | create-node-refused                   | create-node                   | refused | 🌱create-node/🚫️rejects                         | 🚫️.absent |
      | delete-node-alpha                     | delete-node                   | applied | 🗑️delete-node/🚫️removes                                      | 🔣️.json   |
      | delete-node-refused                   | delete-node                   | refused | 🗑️delete-node/🚫️rejects                     | 🚫️.absent |
      | move-node-alpha                       | move-node                     | applied | 📍move-node/📍️moves-node-a                                                           | 🔣️.json   |
      | move-node-refused                     | move-node                     | refused | 📍move-node/🚫️rejects                          | 🚫️.absent |
      | replace-node-geometry-alpha           | replace-node-geometry         | applied | 🧊replace-node-geometry/🔳️circle                                        | 🔣️.json   |
      | replace-node-geometry-refused         | replace-node-geometry         | refused | 🧊replace-node-geometry/🚫️rejects           | 🚫️.absent |
      | change-node-kind-alpha                | change-node-kind              | applied | 🏗️change-node-kind/🏷️reassigns                                          | 🔣️.json   |
      | change-node-kind-refused              | change-node-kind              | refused | 🏗️change-node-kind/🚫️rejects               | 🚫️.absent |
      | edit-node-text-alpha                  | edit-node-text                | applied | ✏️edit-node-text/✏️retitles-node-a                                                  | 🔣️.json   |
      | edit-node-text-refused                | edit-node-text                | refused | ✏️edit-node-text/🚫️rejects                  | 🚫️.absent |
      | change-node-icon-alpha                | change-node-icon              | applied | 🎨change-node-icon/🎨️swaps-node-a-icon                                               | 🔣️.json   |
      | change-node-icon-refused              | change-node-icon              | refused | 🎨change-node-icon/🚫️rejects                | 🚫️.absent |
      | scale-node-alpha                      | scale-node                    | applied | 📏scale-node/📏️doubles-node-a                                                        | 🔣️.json   |
      | scale-node-refused                    | scale-node                    | refused | 📏scale-node/🚫️rejects                        | 🚫️.absent |
      | change-node-visible-alpha             | change-node-visible           | applied | 👁️change-node-visible/🙈️hides-node-a                                                | 🔣️.json   |
      | change-node-visible-refused           | change-node-visible           | refused | 👁️change-node-visible/🚫️rejects               | 🚫️.absent |
      | change-node-locked-alpha              | change-node-locked            | applied | 🔒change-node-locked/🔒️locks-node-a                                                  | 🔣️.json   |
      | change-node-locked-refused            | change-node-locked            | refused | 🔒change-node-locked/🚫️rejects                | 🚫️.absent |
      | change-node-root-alpha                | change-node-root              | applied | 🌟change-node-root/🌳️promotes-node-a-to-root                                         | 🔣️.json   |
      | change-node-root-refused              | change-node-root              | refused | 🌟change-node-root/🚫️rejects                  | 🚫️.absent |
      | change-node-anchor-alpha              | change-node-anchor            | applied | ⚓change-node-anchor/⚓️fixed-to-derived                                              | 🔣️.json   |
      | change-node-anchor-refused            | change-node-anchor            | refused | ⚓change-node-anchor/🚫️rejects              | 🚫️.absent |
      | add-node-handle-alpha                 | add-node-handle               | applied | ➕add-node-handle/➕️appends-handle-3-to-node-b                                       | 🔣️.json   |
      | add-node-handle-refused               | add-node-handle               | refused | ➕add-node-handle/🚫️rejects          | 🚫️.absent |
      | remove-node-handle-alpha              | remove-node-handle            | applied | ➖remove-node-handle/🚫️removes2                              | 🔣️.json   |
      | remove-node-handle-refused            | remove-node-handle            | refused | ➖remove-node-handle/🚫️rejects                 | 🚫️.absent |
      | replace-node-handle-refused           | replace-node-handle           | refused | 🔌replace-node-handle/🚫️rejects               | 🚫️.absent |
      | connect-handles-alpha                 | connect-handles               | applied | 🪢️connect-handles/🪢️adds-second-edge                                                | 🔣️.json   |
      | connect-handles-duplicate             | connect-handles               | noop    | 🪢️connect-handles/⏸️keeps                           | 🔣️.json   |
      | disconnect-handles-alpha              | disconnect-handles            | applied | ✂️disconnect-handles/🚫️removes-edge-1                                               | 🔣️.json   |
      | disconnect-handles-refused            | disconnect-handles            | refused | ✂️disconnect-handles/🚫️rejects                | 🚫️.absent |
      | replace-edge-geometry-alpha           | replace-edge-geometry         | applied | 🧮replace-edge-geometry/📍️repositions-edge-1                                         | 🔣️.json   |
      | replace-edge-geometry-refused         | replace-edge-geometry         | refused | 🧮replace-edge-geometry/🚫️rejects              | 🚫️.absent |
      | change-edge-kind-alpha                | change-edge-kind              | applied | 🏷️change-edge-kind/🏷️rekinds-edge-1                                                 | 🔣️.json   |
      | change-edge-kind-refused              | change-edge-kind              | refused | 🏷️change-edge-kind/🚫️rejects                   | 🚫️.absent |
      | change-edge-tips-alpha                | change-edge-tips              | applied | 🖇️change-edge-tips/🔀️swaps-edge-1-tips                                              | 🔣️.json   |
      | change-edge-tips-refused              | change-edge-tips              | refused | 🖇️change-edge-tips/🚫️rejects                   | 🚫️.absent |
      | change-edge-visible-alpha             | change-edge-visible           | applied | 👀change-edge-visible/🙈️hides-edge-1                                                 | 🔣️.json   |
      | change-edge-visible-refused           | change-edge-visible           | refused | 👀change-edge-visible/🚫️rejects                  | 🚫️.absent |
      | change-edge-locked-alpha              | change-edge-locked            | applied | 🔐change-edge-locked/🔒️locks-edge-1                                                  | 🔣️.json   |
      | change-edge-locked-refused            | change-edge-locked            | refused | 🔐change-edge-locked/🚫️rejects                  | 🚫️.absent |
      | change-manifest-id-alpha              | change-manifest-id            | applied | 🆔change-manifest-id/📦️repoints-manifest                                             | 🔣️.json   |
      | connect-kind-compatibility-alpha      | connect-kind-compatibility    | applied | 🤝connect-kind-compatibility/🤝️adds                                 | 🔣️.json   |
      | disconnect-kind-compatibility-alpha   | disconnect-kind-compatibility | applied | 💔disconnect-kind-compatibility/🚫️removes                           | 🔣️.json   |
      | disconnect-kind-compatibility-refused | disconnect-kind-compatibility | refused | 💔disconnect-kind-compatibility/🚫️rejects | 🚫️.absent |
      | replace-kind-catalogs-alpha           | replace-kind-catalogs         | applied | 📚replace-kind-catalogs/📇️installs                               | 🔣️.json   |
      | replace-kind-catalogs-cleared         | replace-kind-catalogs         | applied | 📚replace-kind-catalogs/🗑️clears                        | 🔣️.json   |
      | create-target-region-alpha            | create-target-region          | applied | 🌍create-target-region/🌍️appends-region-2                                            | 🔣️.json   |
      | create-target-region-refused          | create-target-region          | refused | 🌍create-target-region/🚫️rejects                 | 🚫️.absent |
      | delete-target-region-refused          | delete-target-region          | refused | 🪦delete-target-region/🚫️rejects              | 🚫️.absent |
      | move-target-region-refused            | move-target-region            | refused | 🚀move-target-region/🚫️rejects                  | 🚫️.absent |
      | resize-target-region-refused          | resize-target-region          | refused | 📐resize-target-region/🚫️rejects              | 🚫️.absent |
      | edit-target-region-label-refused      | edit-target-region-label      | refused | 🖋️edit-target-region-label/🚫️rejects         | 🚫️.absent |
      | change-target-region-hidden-refused   | change-target-region-hidden   | refused | 🙈change-target-region-hidden/🚫️rejects         | 🚫️.absent |
      | change-target-region-locked-refused   | change-target-region-locked   | refused | 🔏change-target-region-locked/🚫️rejects        | 🚫️.absent |
      | drag-selection-mixed                  | drag-selection                | applied | ✋️drag-selection/🎯️drags-node-and-region                                            | 🔣️.json   |
      | drag-selection-partial                | drag-selection                | applied | ✋️drag-selection/⚠️skips-locked-ghost                                            | 🔣️.json   |
      | drag-selection-refused                | drag-selection                | refused | ✋️drag-selection/🚫️rejects-ghosts                                             | 🚫️.absent |
      | drag-selection-unchanged              | drag-selection                | noop    | ✋️drag-selection/⏸️keeps-a-zero-offset                                               | 🔣️.json   |
      | rotate-selection-mixed                | rotate-selection              | applied | 🔄️rotate-selection/🎯️skips-the-region                                         | 🔣️.json   |
      | rotate-selection-partial              | rotate-selection              | applied | 🔄️rotate-selection/⚠️skips-locked-ghost                                          | 🔣️.json   |
      | rotate-selection-refused              | rotate-selection              | refused | 🔄️rotate-selection/🚫️rejects-ghosts                                           | 🚫️.absent |
      | rotate-selection-unchanged            | rotate-selection              | noop    | 🔄️rotate-selection/⏸️keeps-a-zero-angle                                              | 🔣️.json   |
      | scale-selection-mixed                 | scale-selection               | applied | 🔍️scale-selection/🎯️halves-node-region                                           | 🔣️.json   |
      | scale-selection-partial               | scale-selection               | applied | 🔍️scale-selection/⚠️skips-locked-ghost                                           | 🔣️.json   |
      | scale-selection-refused               | scale-selection               | refused | 🔍️scale-selection/🚫️rejects-ghosts                                            | 🚫️.absent |
      | scale-selection-unchanged             | scale-selection               | noop    | 🔍️scale-selection/⏸️keeps-a-unit-factor                                              | 🔣️.json   |
      | drag-selection-invariant              | drag-selection                | refused | ✋️drag-selection/🧱️no-targets                                                        | 🚫️.absent |
      | rotate-selection-invariant            | rotate-selection              | refused | 🔄️rotate-selection/🧱️repeated-targets                                                | 🚫️.absent |
      | scale-selection-invariant             | scale-selection               | refused | 🔍️scale-selection/🧱️zero-factor                                                      | 🚫️.absent |
      | scale-selection-invariant-negative    | scale-selection               | refused | 🔍️scale-selection/⛔️negative-factor                                                  | 🚫️.absent |
      | scale-node-invariant                  | scale-node                    | refused | 📏scale-node/🧱️zero-scale                                                             | 🚫️.absent |
      | replace-node-geometry-invariant       | replace-node-geometry         | refused | 🧊replace-node-geometry/🧱️negative-radius                                             | 🚫️.absent |
      | create-node-invariant                 | create-node                   | refused | 🌱create-node/🧱️zero-width-node                                                       | 🚫️.absent |
      | add-node-handle-invariant             | add-node-handle               | refused | ➕add-node-handle/🧱️zero-radius-handle                                                | 🚫️.absent |
      | replace-node-handle-invariant         | replace-node-handle           | refused | 🔌replace-node-handle/🧱️negative-scale                                                | 🚫️.absent |
      | replace-kind-catalogs-invariant       | replace-kind-catalogs         | refused | 📚replace-kind-catalogs/🧱️off-rim-template                                            | 🚫️.absent |

  @id-identity-round-trip
  @level-long
  @mode-round-trip
  Scenario: Decode and re-encode the real First-Storey-Tambour subgraph of the Nakagin Capsule Tower
    Given the committed before-snapshot shared://🧬️mutations/🌱create-node/🌱️appends/📸️snapshot/⬅️before/🔣️.json
    When it is parsed by the platform's own dependency-free JSON reader, re-serialized and parsed again
    Then the document is unchanged and the re-serialized bytes are not the committed bytes
