@capability-rewrite-1-mutate
@no-oracle-rewrite-1-graph-rewrite-rule-mutation-semantics
@comparison-ordered-json-v1
@mutations-rewrite-1-any
Feature: Apply every typed rewrite-rule mutation to its committed specification vectors
  `s.trinity.rewrite` is a semio-NATIVE graph-rewrite rule: a before-fixture graph, a match pattern,
  a rewrite body, a parameter-binding map and a rule-layout map. Nothing third-party reads
  `.rewrite.dsl.semio`, so there is no reference implementation to register (recorded as the
  `rewrite-1-graph-rewrite-rule-mutation-semantics` no-oracle decision in
  `../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️oracle/🔣️component.json`).

  What distinguishes this vocabulary is that it deliberately does not look inside the thing it
  edits. The three authored bodies are opaque JSON STRINGS on the wire — a whole `trinity.graph`
  document, a whole pattern with its where-clause, a whole create/delete/set/merge body with its
  parameter declarations — and each is replaced atomically by ONE `edit` verb. There is no
  per-clause vocabulary, no `add-set-clause`, no `change-where`; the rule editor rewrites a body and
  the mutation carries it. Beside them sit the two KEY-ADDRESSED maps, `parameterBindings` and
  `ruleLayout`, and those get the `change`/`remove` pair a keyed map supports. Seven kinds, two
  disjoint shapes, and nothing else: whole-document replace was banned outright and `resetRule`
  routes through `Effect::LoadDocument`, so this catalog carries no `set-snapshot`.

  The `moves` column names, per kind, the single projection member that kind is allowed to touch,
  and the subject handler asserts both halves — the named member moved AND every other member is
  byte-identical. That is what holds the two shapes apart: an `edit-lhs` that also touched
  `parameterBindings`, or a `remove-rule-layout-point` that reached into `ruleLayout`'s sibling map,
  fails on the neighbour rather than passing on its own target. The two `remove` kinds carry the
  most weight in the inverse scenarios, because their undo has to restore a value the removal
  payload never carried — the key alone is what travels.

  Because this case records a no-oracle decision the runner executes NO oracle role, so every
  assertion lives inside the subject handler, which compares the applied snapshot against the
  committed after-snapshot and the reported diagnostics against the committed
  `🎯️outcome/🔣️component.json`. A handler that merely ran the mutation and returned would report a
  pass having checked nothing.

  @id-mutate
  @level-exhaustive
  @mode-conformance
  Scenario Outline: Apply <id> to its committed before-snapshot fixture
    Given the committed before-snapshot, mutation and outcome fixtures for the <id> kind
    When <id> is applied through apply_rewrite_mutation_reporting
      """
      {"kind": "<id>", "moves": "<moves>"}
      """
    Then the resulting snapshot matches the committed after-snapshot, only <moves> moved, and the reported diagnostics match the committed outcome
    Examples:
      | id                        | moves              |
      | edit-before-fixture       | beforeFixtureJson  |
      | edit-lhs                  | lhsJson            |
      | edit-rhs                  | rhsJson            |
      | change-parameter-binding  | parameterBindings  |
      | remove-parameter-binding  | parameterBindings  |
      | change-rule-layout-point  | ruleLayout         |
      | remove-rule-layout-point  | ruleLayout         |

  @id-inverse
  @level-exhaustive
  @mode-property
  Scenario Outline: Undoing <id> restores the committed before-snapshot fixture
    Given the committed before-snapshot and mutation fixtures for the <id> kind
    When <id> is applied and then its own computed inverse steps are applied
      """
      {"kind": "<id>", "moves": "<moves>"}
      """
    Then the rule equals the committed before-snapshot again, body for body and key for key
    Examples:
      | id                        | moves              |
      | edit-before-fixture       | beforeFixtureJson  |
      | edit-lhs                  | lhsJson            |
      | edit-rhs                  | rhsJson            |
      | change-parameter-binding  | parameterBindings  |
      | remove-parameter-binding  | parameterBindings  |
      | change-rule-layout-point  | ruleLayout         |
      | remove-rule-layout-point  | ruleLayout         |

  @id-identity-round-trip
  @level-long
  @mode-round-trip
  Scenario: Read the real committed Nakagin rule through its own DSL carrier and print it back
    Given the real committed text artifact asset://🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🎬️demo/🖼️assets/🗣️example.dsl.semio
    When the artifact is parsed, printed back to `.rewrite.dsl.semio` and parsed again
    Then every decoding agrees on the same rule — a two-piece Nakagin ground-floor before-fixture, a neighbour pattern with a where-clause, one `label` binding and two layout points — and the printed text reproduces the committed file byte for byte
