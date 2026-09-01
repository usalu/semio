@capability-json-rfc8259-mutate
@oracle-json-rust-rfc8259-mutate
@comparison-ordered-json-v1
@mutations-json-rfc8259-any
Feature: Apply every typed RFC 8259 JSON mutation to a real-world document
  The input is shared://🔣️hexagonal-cut-concrete-forest-left.model.json, copied verbatim (`cp`,
  committed here as-is) from the real 424 KB CAD model already committed at
  ✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🖼️assets/🎮️play/
  🔣️hexagonal-cut-concrete-forest-left.model.json — 8,979 nodes, 71 vertices, 126 edges, 57 wires,
  57 faces, deeply nested objects and arrays, and 146 real exponent-notation floats (machine-epsilon
  boundary geometry like `4.44089209850063e-16`) that exercise this subset's own arbitrary-precision
  number lexeme against the reference codec's own number parse. Every scenario copies the fixture
  into the case work directory before touching it; the committed file is never written to.

  The reference here is `json` (json-rust) 0.12, deliberately NOT `serde_json` even though
  `serde_json` was already linked test-only in this crate: this subset's OWN production code
  (`../../🏅️standards/🔖️rfc8259/🪆️subsets/✳️base/🧬️schema/📸️snapshot/🦀️component.rs`) declares
  `impl From<serde_json::Value> for JsonValue` and the reverse — a real interop conversion path FROM
  the reference's own type INTO this subset's model, for callers elsewhere in the app that want a
  `serde_json::Value` view of a decoded document. A `serde_json` differential would therefore compare
  this implementation against something it already converts from, not independent evidence — and
  `serde_json` is, separately and correctly, a genuine production runtime dependency of this
  repository at large (workspace `serde_json = "1.0.149"`, reachable from several hundred production
  files for reasons unrelated to this case). `json` (json-rust) has neither relationship: no
  production file in this repository reaches it, and this subset's codec has no conversion path to or
  from its `JsonValue`/`Object`/`Number` types.

  Two RFC 8259 conformance points this case's comparison deliberately narrows rather than silently
  normalizes, both real and both documented in `../../🏅️standards/🔖️rfc8259/🪆️subsets/✳️base/
  🧪️oracle/🔣️.json`'s oracle rationale and in that oracle module's own doc comment: object
  member order is unordered per §4 — this subset's own codec preserves insertion order, while
  `json::object::Object` stores entries in a hash-ordered binary tree keyed by an FNV-1a hash of each
  key (not insertion order, and not alphabetical either — it does not preserve order at all), so the
  `ordered-json-v1` core profile (array order significant, key order never) is used rather than a
  bespoke one; and number comparison is by PARSED VALUE, not lexeme — `json::number::Number` is a
  `(sign, mantissa: u64, exponent: i16)` decimal pair, not this subset's own arbitrary-precision
  LEXEME (§6 permits arbitrary precision). The real fixture never exceeds 3 integer digits before any
  decimal point, so that precision boundary costs nothing here — a genuine information-loss risk for
  a future fixture with 19+ digit integers, recorded rather than exercised.

  Two REAL DEFECTS in that reference's number handling were found by this case's own inverse law and
  are worked around rather than hidden, each reproduced standalone against `json` 0.12 alone and each
  pinned by a unit test in the oracle module so a later release that fixes it makes the test fail.
  Neither is the precision boundary above: both move values that ARE exactly representable as `f64`.
  `impl From<f64> for JsonValue` rounds going INTO the decimal pair — the fixture's
  `2.7000102824824506` vertex coordinate dumps back as `…507`, and `-8.881784197001252e-16` as
  `…253e-16` (2 of 9 probed values moved) — and `as_f64()` rounds coming back OUT of it, recomputing
  `mantissa * 10^exponent` in floating point, so the fixture's `-1.3283902924697095e-17` surface
  normal reads out as `…097e-17`. The crate's own parser and `dump()` are exact on every probed
  value, so the oracle reaches the same `Number` through those halves instead. Before the workaround
  Whole-document cycling failed on exactly those two coordinates: carrying the 424 KB model back
  through the reference drifts one ULP per cycle. That failure was a true report
  about the reference library, not about this repository's codec and not about the fixture.

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
      | set-member            | {"path": ["models", 0, "model"], "key": "revision", "value": 99}                                                                         |
      | remove-member         | {"path": ["models", 0, "model", "objects", 0], "key": "typology"}                                                                        |
      | insert-array-element  | {"path": ["models", 0, "model", "geometry", "vertices"], "index": 0, "value": {"id": "mutation-test-vertex", "position": [0, 0, 0]}}     |
      | remove-array-element  | {"path": ["models", 0, "model", "geometry", "vertices"], "index": 10}                                                                    |
      | set-scalar            | {"path": ["models", 0, "model", "geometry", "vertices", 0, "position", 0], "value": 999.25}                                              |

  @id-inverse
  @level-exhaustive
  @mode-differential
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
