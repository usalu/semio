@capability-json-rfc8259-mutate
@oracle-json-rfc8259-mutate
@comparison-ordered-json-v1
@mutations-json-rfc8259-any
Feature: Apply every typed RFC 8259 JSON mutation to a real-world document
  The input is shared://🔣️hexagonal-cut-concrete-forest-left.model.json, copied verbatim (`cp`,
  committed here as-is) from the real 424 KB CAD model already committed at
  ✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🖼️assets/🎮️play/
  🔣️hexagonal-cut-concrete-forest-left.model.json — 8,979 nodes, 71 vertices, 126 edges, 57 wires,
  57 faces, deeply nested objects and arrays, and 146 real exponent-notation floats (machine-epsilon
  boundary geometry like `4.44089209850063e-16`) that exercise this subset's own arbitrary-precision
  number lexeme against `serde_json`'s IEEE-754 `f64` parse. Every scenario copies the fixture into
  the case work directory before touching it; the committed file is never written to.

  Two RFC 8259 conformance points this case's comparison deliberately narrows rather than silently
  normalizes, both real and both documented in `../../🏅️standards/🔖️rfc8259/🪆️subsets/✳️any/
  🧪️oracle/🔣️component.json`'s oracle rationale: object member order is unordered per §4 — this
  subset's own codec preserves insertion order, `serde_json`'s default (non-`preserve_order`) `Map`
  re-sorts every object alphabetically on parse/serialize, so the `ordered-json-v1` core profile
  (array order significant, key order never) is used rather than a bespoke one; and number
  comparison is by PARSED VALUE, not lexeme — `serde_json` without `arbitrary_precision` keeps
  integers exact within `u64`/`i64` and rounds anything else through `f64`, while this subset's own
  codec preserves the source lexeme verbatim (§6 permits arbitrary precision). The real fixture never
  exceeds 3 integer digits before any decimal point, so that precision boundary costs nothing here —
  a genuine information-loss risk for a future fixture with 19+ digit integers, recorded rather than
  exercised.

  @id-mutate
  @level-exhaustive
  @mode-differential
  Scenario Outline: Apply <id> to the real document
    Given the real input document shared://🔣️hexagonal-cut-concrete-forest-left.model.json
    When the <id> mutation is applied with its parameters
      """
      {"kind": "<id>", "params": <params>}
      """
    Then the oracle and the subject agree on the semantic projection
    Examples:
      | id                    | params                                                                                                                                    |
      | no-mutation           | {}                                                                                                                                        |
      | set-snapshot          | {"value": {"a": 1, "b": [true, null, "Ünïcödé, mit Sonderzeichen"], "nested": {"c": "value"}}}                                            |
      | set-member            | {"path": ["models", 0, "model"], "key": "revision", "value": 99}                                                                         |
      | remove-member         | {"path": ["models", 0, "model", "objects", 0], "key": "typology"}                                                                        |
      | insert-array-element  | {"path": ["models", 0, "model", "geometry", "vertices"], "index": 0, "value": {"id": "mutation-test-vertex", "position": [0, 0, 0]}}     |
      | remove-array-element  | {"path": ["models", 0, "model", "geometry", "vertices"], "index": 10}                                                                    |
      | set-scalar            | {"path": ["models", 0, "model", "geometry", "vertices", 0, "position", 0], "value": 999.25}                                              |

  @id-inverse
  @level-exhaustive
  @mode-property
  Scenario Outline: Undoing <id> restores the real document
    Given the real input document shared://🔣️hexagonal-cut-concrete-forest-left.model.json
    When the <id> mutation is applied with its parameters
      """
      {"kind": "<id>", "params": <params>}
      """
    And the inverse mutation is applied to that result
    Then the oracle and the subject agree on the semantic projection of the original document
    Examples:
      | id                    | params                                                                                                                                    |
      | no-mutation           | {}                                                                                                                                        |
      | set-snapshot          | {"value": {"a": 1, "b": [true, null, "Ünïcödé, mit Sonderzeichen"], "nested": {"c": "value"}}}                                            |
      | set-member            | {"path": ["models", 0, "model"], "key": "revision", "value": 99}                                                                         |
      | remove-member         | {"path": ["models", 0, "model", "objects", 0], "key": "typology"}                                                                        |
      | insert-array-element  | {"path": ["models", 0, "model", "geometry", "vertices"], "index": 0, "value": {"id": "mutation-test-vertex", "position": [0, 0, 0]}}     |
      | remove-array-element  | {"path": ["models", 0, "model", "geometry", "vertices"], "index": 10}                                                                    |
      | set-scalar            | {"path": ["models", 0, "model", "geometry", "vertices", 0, "position", 0], "value": 999.25}                                              |

  @id-identity-round-trip
  @level-long
  @mode-round-trip
  Scenario: Decode and re-encode the real document without passing bytes through
    Given the real input document shared://🔣️hexagonal-cut-concrete-forest-left.model.json
    When the document is decoded to the typed snapshot and re-encoded from it alone
    Then the oracle and the subject agree on the semantic projection
    And the re-encoded bytes are not bit-identical to the input
