@capability-s-home-1-mutate
@oracle-s-home-1-python-independent
@comparison-ordered-json-v1
@mutations-s-home-1-any
Feature: Apply the one typed s.space.home mutation against an independent Python implementation

  `s.space.home` is a semio-NATIVE artifact: nothing outside this repository reads `.shome.dsl.semio`
  or its `.pack.semio` twin — the candidate category is empty rather than merely unexplored. The
  second producer a differential comparison needs is therefore a second IMPLEMENTATION, and
  `🐍️component.py` beside this file is it: the one kind of this vocabulary, written in Python from
  this subset's own committed `🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🔣️.json` and
  mutation payload schema, and from
  `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-DIRECT-LEAF-OVERHAUL/📓️taxonomy.md`'s
  `change` verb entry. It imports nothing from the Rust it judges and transliterates none of it. The
  no-oracle decision this replaces (`s-home-mutation-semantics`) is narrowed to an empty
  `capabilities` list rather than deleted, because its own investigation remains the honest record of
  what was checked.

  Both implementations now read the SAME committed bytes: `(before, mutation, after, diff, outcome)`
  under
  `../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔢️change-catalog-generation/🧪️tests/bumps-the-catalog-generation-to-7/`
  is a declared `asset://` fixture rather than an `include_str!`-only literal, so the plan pins its
  digest and a Python reference can resolve it.

  What distinguishes this subset is how little of it is mutable. The launcher document persists two
  fields, `schema` and `catalog_generation`, of which only the second may ever be written by a
  mutation — so the whole vocabulary is one root-scalar setter with no id, no path and no collection
  to address. The counter is not authored content: it is the re-materialize trigger the studio list
  watches, bumped after a create, import or delete on the catalog port. Because the verb is a SETTER,
  its inverse cannot be derived structurally from the diff; it has to read the previous value out of
  BASE and re-pin it, and that is the single most likely thing for an implementation to get wrong.

  The committed specification vector is chosen against exactly that hazard. It pins the counter from
  3 to 7, not from 0 to 1, so an implementation that INCREMENTED rather than SET lands on 4 and the
  forward scenario fails; and the leaf's only guard is an equal-counter `mutation.no-op` warning, so
  the vector's declared message-free `applied` outcome is itself an assertion that 3 and 7 were
  compared rather than assumed. The committed diff pins that exactly one of `SHomeDiff`'s four
  optional fields is written: an artifact-lane counter pin that reached into `activePanelTab` or
  `locale` would be a config-lane leak and shows up as a diff divergence.

  The identity round trip reads the artifact's own committed demo example, a 40-byte document that is
  the smallest complete `.dsl.semio` envelope in this plugin — a preamble line plus one body line
  carrying `schema` and `gen`.

  Where the assertions live. `mutate-change-catalog-generation` and
  `inverse-change-catalog-generation` now dispatch BOTH an oracle role (the Python implementation,
  reached through this plugin's `oracleHostPackages` entry) and a subject role (this repository's own
  `s_home_mutation_report_json`), each independently asserting the forward/inverse laws in role
  through the shared law module `✏️s/🔌️plugins/🗄️stdio/🧪️oracle/⚖️law/🦀️component.rs` that the stdio
  subsets use, before the two are compared byte for byte. `identity-round-trip` stays
  Rust-subject-only, unaffected: this subset's committed snapshot text grammar is the repository-wide
  placeholder `payload = OCTET+`, whose header production declares `"schema" SP "stdio.json"` against
  an artifact whose own first line says otherwise, so a second reader has nothing to parse from.

  @id-mutate
  @level-exhaustive
  @mode-differential
  Scenario Outline: Apply <id> to its committed specification vector
    Given the committed before-snapshot asset://🧬️schema/🧬️mutations/<dir>/🧪️tests/<fixture>/📸️snapshot/⬅️before/🔣️.json
    And the committed mutation payload asset://🧬️schema/🧬️mutations/<dir>/🧪️tests/<fixture>/🦠️mutation/🔣️.json
    And the committed after-snapshot asset://🧬️schema/🧬️mutations/<dir>/🧪️tests/<fixture>/📸️snapshot/➡️after/🔣️.json
    And the committed outcome asset://🧬️schema/🧬️mutations/<dir>/🧪️tests/<fixture>/🎯️outcome/🔣️.json
    When both implementations apply the committed mutation to the committed before-snapshot
    Then each reaches the committed after-snapshot under the committed outcome status and the two agree
    Examples:
      | id                        | dir                            | fixture                          |
      | change-catalog-generation | 🔢change-catalog-generation    | bumps-the-catalog-generation-to-7 |

  @id-inverse
  @level-exhaustive
  @mode-differential
  Scenario Outline: Undoing <id> restores its committed before-snapshot
    Given the committed before-snapshot asset://🧬️schema/🧬️mutations/<dir>/🧪️tests/<fixture>/📸️snapshot/⬅️before/🔣️.json
    And the committed mutation payload asset://🧬️schema/🧬️mutations/<dir>/🧪️tests/<fixture>/🦠️mutation/🔣️.json
    And the committed after-snapshot asset://🧬️schema/🧬️mutations/<dir>/🧪️tests/<fixture>/📸️snapshot/➡️after/🔣️.json
    And the committed outcome asset://🧬️schema/🧬️mutations/<dir>/🧪️tests/<fixture>/🎯️outcome/🔣️.json
    When each implementation applies the committed mutation and then its OWN computed inverse
    Then both restore the before-snapshot and agree on the mutated and the restored document
    Examples:
      | id                        | dir                            | fixture                          |
      | change-catalog-generation | 🔢change-catalog-generation    | bumps-the-catalog-generation-to-7 |

  @id-identity-round-trip
  @level-long
  @mode-round-trip
  Scenario: Parse the real committed launcher document, print it back and cross it against its binary encoding
    Given the real committed document asset://📚️examples/🎬️demo/🖼️assets/🗣️.dsl.semio
    When the document is parsed, printed back to canonical DSL, parsed again, and separately encoded to a pack and decoded back
    Then every decoding agrees on one snapshot, and printing the canonical text a second time reproduces it byte for byte as ArtifactDsl's own fixpoint law requires
