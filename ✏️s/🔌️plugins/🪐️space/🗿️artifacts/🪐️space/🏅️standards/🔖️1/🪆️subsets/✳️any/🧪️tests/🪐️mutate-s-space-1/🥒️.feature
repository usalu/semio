@capability-s-space-1-mutate
@oracle-s-space-1-python-independent
@comparison-ordered-json-v1
@mutations-s-space-1-any
Feature: Apply every typed s.space.space index mutation against an independent Python implementation

  `s.space.space` is a semio-NATIVE artifact — no third party reads `.sspace.dsl.semio` — generic
  table readers and content-addressed store crates were surveyed and DECLINED rather than merely
  absent. The second producer a differential comparison needs is therefore a second IMPLEMENTATION,
  and `🐍️component.py` beside this file is it: all four kinds of this vocabulary, written in Python
  from this subset's own committed `🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🔣️.json` and
  each mutation's own payload schema, and from
  `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-DIRECT-LEAF-OVERHAUL/📓️taxonomy.md`'s
  `create`/`delete`/`rename` verb entries and `📓️derivation-rules.md`'s per-id-keyed-collection
  recipe. It imports nothing from the Rust it judges and transliterates none of it — the diff
  granularity (table-level, not row-level) is read directly off the four committed `🔺️diff` vectors
  rather than assumed. The no-oracle decision this replaces (`s-space-index-mutation-semantics`) is
  narrowed to an empty `capabilities` list rather than deleted, because its own investigation remains
  the honest record of what was checked.

  Both implementations now read the SAME committed bytes: every `(before, mutation, after, diff,
  outcome)` path is a declared `asset://` fixture rather than an `include_str!`-only literal, so the
  plan pins its digest and a Python reference can resolve it.

  What distinguishes this subset is that it is an INDEX, not a document. Each row carries an
  artifact's metadata — `id`, `name`, `kind_id`, `schema`, a nested `dialect` block of
  `(artifactKind, standard, subset)`, and two clock pairs — and never that artifact's own bytes,
  which live in their own backbone document addressed by the same `id`. The DSL layout is therefore a
  `#[dsl(table)]` row grid with a BLOCK-typed column, which is why a flat record reader is not
  enough to round-trip it.

  Four verbs, and they are not four instances of one shape. `create-artifact` appends a whole row and
  inverts to a delete of the id it minted. `delete-artifact` removes one and inverts by re-inserting
  the captured row, so an inverse that rebuilt the row from the payload rather than from BASE loses
  every field the payload never carried. `rename-artifact` writes `name` alone and must leave
  `kind_id`, `schema` and the whole `dialect` block untouched. `touch-artifact` is the only verb that
  writes a CLOCK: it stamps `updated_at_ms` and `updated_by` together, so its inverse has to restore
  BOTH halves of the pair — restoring the timestamp and forgetting the editor is the exact failure
  its committed vector, `🗿️stamps-artifact-1-with-a-new-editor`, is built to expose.

  The identity round trip reads the artifact's own committed demo example, a space index whose
  `space-id` is `demo-space` and whose row table is declared with its full nine-column header and
  then left EMPTY — the one document shape where a codec that confuses "no rows" with "no table"
  produces something that still parses.

  Where the assertions live. Every `mutate-<id>`/`inverse-<id>` scenario below now dispatches BOTH an
  oracle role (the Python implementation, reached through this plugin's `oracleHostPackages` entry)
  and a subject role (this repository's own `s_space_mutation_report_json`), each independently
  asserting the forward/inverse laws in role through the shared law module
  `✏️s/🔌️plugins/🗄️stdio/🧪️oracle/⚖️law/🦀️component.rs` that the stdio subsets use, before the two are
  compared byte for byte. `identity-round-trip` stays Rust-subject-only, unaffected.

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
      | id              | dir                | fixture                              |
      | create-artifact | 🌱create-artifact  | appends-artifact-3-to-the-index      |
      | delete-artifact | 🗑️delete-artifact  | removes-artifact-2-from-the-index    |
      | rename-artifact | 🏷️rename-artifact  | renames-artifact-1                   |
      | touch-artifact  | 🕒touch-artifact   | stamps-artifact-1-with-a-new-editor  |

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
      | id              | dir                | fixture                              |
      | create-artifact | 🌱create-artifact  | appends-artifact-3-to-the-index      |
      | delete-artifact | 🗑️delete-artifact  | removes-artifact-2-from-the-index    |
      | rename-artifact | 🏷️rename-artifact  | renames-artifact-1                   |
      | touch-artifact  | 🕒touch-artifact   | stamps-artifact-1-with-a-new-editor  |

  @id-identity-round-trip
  @level-long
  @mode-round-trip
  Scenario: Parse the real committed space index document, print it back and cross it against its binary encoding
    Given the real committed document asset://📚️examples/🎬️demo/🖼️assets/🗣️.dsl.semio
    When the document is parsed, printed back to canonical DSL, parsed again, and separately encoded to a pack and decoded back
    Then every decoding agrees on one snapshot, and printing the canonical text a second time reproduces it byte for byte as ArtifactDsl's own fixpoint law requires
