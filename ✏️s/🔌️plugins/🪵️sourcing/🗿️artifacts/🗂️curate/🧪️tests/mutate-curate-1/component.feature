@capability-curate-1-mutate
@no-oracle-curate-1-curation-selection-mutation-semantics
@comparison-ordered-json-v1
@mutations-curate-1-any
Feature: Apply every typed curation mutation to its committed specification vectors
  `sourcing.curate` is a semio-NATIVE curation document: a composed `s.stdio.semio.kit` catalogue
  child, a stock table and the curation itself. Nothing third-party reads `.curate.dsl.semio`, so
  there is no reference implementation to register (recorded as the
  `curate-1-curation-selection-mutation-semantics` no-oracle decision in
  `../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️oracle/🔣️component.json`).

  What distinguishes this vocabulary is how SMALL it is and why. Three kinds, over exactly one
  collection. `stock` is not in the vocabulary at all: it is a bulk-populated reference catalogue
  seeded from hot-installed `sourcing.module` contributions and replaced wholesale through
  `ArtifactStore::reset`, the same non-history path whole-document replace uses — so there is no
  `create-stock-item` and no `set-snapshot` here, and the catalog says three, not five. Within
  `curated` the schema closes the vocabulary just as tightly: a `CuratedItem` is an `objectId` and a
  `count` and nothing else, so there is no rename and no nested add/remove pair. `create`, `delete`
  and `change` over one id-keyed collection is the whole surface the schema supports.

  All three committed vectors under
  `../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/<slug>/🧪️tests/` run against the SAME
  two-entry curation — 12 glulam beams then 4 CLT panels — which is what lets each kind be held to a
  positional claim as well as a membership one. The `effect` column names that claim: `append`
  requires the new entry to land AFTER both members already present, `detach` requires the survivor
  to keep its index, and `retune` requires the list length and every position to be untouched while
  one count moves. An implementation that rebuilt or re-sorted the list would satisfy a membership
  comparison and fail all three.

  Because this case records a no-oracle decision the runner executes NO oracle role, so every
  assertion lives inside the subject handler, which compares the applied snapshot against the
  committed after-snapshot, checks the positional claim, and checks the reported diagnostics against
  the committed outcome. A handler that merely ran the mutation and returned would report a pass
  having checked nothing.

  @id-mutate
  @level-exhaustive
  @mode-conformance
  Scenario Outline: Apply <id> to its committed before-snapshot fixture
    Given the committed before-snapshot, mutation and outcome fixtures for the <id> kind
    When <id> is applied through apply_sourcing_mutation_reporting
      """
      {"kind": "<id>", "effect": "<effect>"}
      """
    Then the resulting snapshot matches the committed after-snapshot, the curation shows the <effect> claim, and the reported diagnostics match the committed outcome
    Examples:
      | id                        | effect |
      | create-curated-item       | append |
      | delete-curated-item       | detach |
      | change-curated-item-count | retune |

  @id-inverse
  @level-exhaustive
  @mode-property
  Scenario Outline: Undoing <id> restores the committed before-snapshot fixture
    Given the committed before-snapshot and mutation fixtures for the <id> kind
    When <id> is applied and then its own computed inverse steps are applied
      """
      {"kind": "<id>", "effect": "<effect>"}
      """
    Then the curation equals the committed before-snapshot again, entry for entry and position for position
    Examples:
      | id                        | effect |
      | create-curated-item       | append |
      | delete-curated-item       | detach |
      | change-curated-item-count | retune |

  @id-identity-round-trip
  @level-long
  @mode-round-trip
  Scenario: Read the real committed curation through its own DSL carrier and print it back
    Given the real committed text artifact asset://🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🎬️demo/🖼️assets/🗣️example.dsl.semio
    When the artifact is parsed, printed back to `.curate.dsl.semio` and parsed again
    Then every decoding agrees on the same document — ten stock entries against an empty curation — and the printed text reproduces the committed file byte for byte
