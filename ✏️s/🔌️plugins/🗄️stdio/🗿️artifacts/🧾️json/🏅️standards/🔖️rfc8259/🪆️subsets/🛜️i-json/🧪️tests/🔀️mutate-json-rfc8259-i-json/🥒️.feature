@capability-json-rfc8259-i-json-mutate
@oracle-simplejson-json-rfc8259-i-json-mutate
@comparison-semantic-i-json-v1
@mutations-json-rfc8259-i-json
Feature: Apply every typed RFC 7493 I-JSON mutation to a real-world document
  The input is shared://🔣️.json, the real 424 KB CAD model
  this artifact already keeps in its own fixtures directory — 8,979 nodes, 71 vertices, 126 edges,
  57 wires, 57 faces, deeply nested objects and arrays, and 146 real exponent-notation floats at the
  machine-epsilon boundary. It was verified I-JSON-conforming before this case was written: valid
  UTF-8, zero duplicate object member names anywhere in the tree, zero integers outside ±(2^53−1),
  and an object at the top level — so it is a legitimate `s.stdio.json@rfc8259/i-json` document and
  not merely a legitimate RFC 8259 one. Every scenario copies it into the case work directory before
  touching it; the committed file is never written to.

  This case exercises the 🛜️i-json subset's OWN vocabulary, not the ✳️any subset's. Four of the ten
  kinds carry a clause of RFC 7493 that the ✳️any sibling has no way to express:

    - `set-top-level` — §2.1. Its payload is an object or an array by TYPE, so a scalar document root
      is unrepresentable rather than merely rejected afterwards. ✳️any's `set-scalar` with an empty
      path can spell one.
    - `set-safe-number` — §2.2. Writes a number over a number, and refuses an integer lexeme outside
      ±(2^53−1) instead of writing it. The scenario below drives it to exactly 9007199254740991, the
      boundary itself, over the real document's own `revision` integer.
    - `rename-member` — §2.3. One atomic, position-preserving step that refuses to create a duplicate
      name. ✳️any has no rename at all; doing it as remove-then-insert transits a state in which both
      names exist, which is precisely what the clause forbids.
    - `set-string` — §2.4. Writes a string over a string and refuses a Unicode noncharacter.

  The remaining four (`upsert-member`, `remove-member`, `insert-array-element`,
  `remove-array-element`) are INHERITED from ✳️any unchanged, because RFC 7493
  says nothing about arrays and nothing about member insertion or deletion beyond uniqueness. That is
  the honest finding for them and it is recorded here rather than dressed up as a difference.

  The reference runs in PYTHON: `simplejson`, registered by this subset's own 🔮️oracles contribution.
  RFC 7493 restricts the JSON value space, so the reference has to surface the three facts the
  profile turns on — every object's ORDERED member list including any duplicate names, the exact
  number LEXEME, and the decoded string. `simplejson`'s `object_pairs_hook` reports duplicates that a
  dict-producing parser would silently collapse (the oracle asserts the real fixture has none, which
  is what makes it an I-JSON document), and `parse_int`/`parse_float` hand back the raw digits so
  §2.2 is checked on the lexeme rather than through a lossy double. It re-serializes as well as
  parses, so all ten kinds are genuinely differential rather than merely read back.

  Two producer freedoms the `semantic-i-json-v1` profile deliberately allows: object member order
  (RFC 8259 §4 leaves it to the producer and RFC 7493 §2.3 only requires uniqueness, so this subset's
  insertion-order codec and the reference's own emission order never register as a difference), and
  number spelling (§6 permits arbitrary precision; this subset keeps the original lexeme while the
  reference re-emits a shortest-round-trip decimal, so numbers compare by value with a 1e-12
  tolerance). Array order IS normative and is never sorted.

  @id-mutate
  @level-exhaustive
  @mode-differential
  Scenario Outline: Apply <id> to the real document
    Given the real input document shared://🔣️.json
    When the <id> mutation is applied with its parameters
      """
      {"kind": "<id>", "params": <params>}
      """
    Then the oracle and the subject agree on the semantic projection
    Examples:
      | id                   | params                                                                                                                                              |
      | set-top-level        | {"root": {"kind": "object", "members": [{"key": "schema", "value": {"kind": "string", "value": "spatial.modelspace"}}, {"key": "revision", "value": {"kind": "number", "lexeme": "5"}}, {"key": "models", "value": {"kind": "array", "items": []}}]}} |
      | upsert-member        | {"path": [{"kind": "key", "value": "models"}, {"kind": "index", "value": 0}, {"kind": "key", "value": "model"}], "key": "revision", "value": {"kind": "number", "lexeme": "99"}} |
      | remove-member        | {"path": [{"kind": "key", "value": "models"}, {"kind": "index", "value": 0}, {"kind": "key", "value": "model"}, {"kind": "key", "value": "objects"}, {"kind": "index", "value": 0}], "key": "typology"} |
      | rename-member        | {"path": [{"kind": "key", "value": "models"}, {"kind": "index", "value": 0}, {"kind": "key", "value": "model"}, {"kind": "key", "value": "geometry"}], "from": "anchors", "to": "anchorPoints"} |
      | set-safe-number      | {"path": [{"kind": "key", "value": "models"}, {"kind": "index", "value": 0}, {"kind": "key", "value": "model"}, {"kind": "key", "value": "revision"}], "lexeme": "9007199254740991"} |
      | set-string           | {"path": [{"kind": "key", "value": "models"}, {"kind": "index", "value": 0}, {"kind": "key", "value": "id"}], "value": "hexagonal-cut-concrete-forest-left, RFC 7493 §2.4 clean: Ünïcödé mit Sonderzeichen"} |
      | insert-array-element | {"path": [{"kind": "key", "value": "models"}, {"kind": "index", "value": 0}, {"kind": "key", "value": "model"}, {"kind": "key", "value": "geometry"}, {"kind": "key", "value": "vertices"}], "index": 0, "value": {"kind": "object", "members": [{"key": "id", "value": {"kind": "string", "value": "i-json-mutation-test-vertex"}}, {"key": "position", "value": {"kind": "array", "items": [{"kind": "number", "lexeme": "0"}, {"kind": "number", "lexeme": "0"}, {"kind": "number", "lexeme": "0"}]}}]}} |
      | remove-array-element | {"path": [{"kind": "key", "value": "models"}, {"kind": "index", "value": 0}, {"kind": "key", "value": "model"}, {"kind": "key", "value": "geometry"}, {"kind": "key", "value": "vertices"}], "index": 10} |
  @id-inverse
  @level-exhaustive
  @mode-differential
  Scenario Outline: Undoing <id> restores the real document
    Given the real input document shared://🔣️.json
    When the <id> mutation is applied with its parameters
      """
      {"kind": "<id>", "params": <params>}
      """
    And the inverse mutation is applied to that result
    Then the oracle and the subject agree on the semantic projection
    Examples:
      | id                   | params                                                                                                                                              |
      | set-top-level        | {"root": {"kind": "object", "members": [{"key": "schema", "value": {"kind": "string", "value": "spatial.modelspace"}}, {"key": "revision", "value": {"kind": "number", "lexeme": "5"}}, {"key": "models", "value": {"kind": "array", "items": []}}]}} |
      | upsert-member        | {"path": [{"kind": "key", "value": "models"}, {"kind": "index", "value": 0}, {"kind": "key", "value": "model"}], "key": "revision", "value": {"kind": "number", "lexeme": "99"}} |
      | remove-member        | {"path": [{"kind": "key", "value": "models"}, {"kind": "index", "value": 0}, {"kind": "key", "value": "model"}, {"kind": "key", "value": "objects"}, {"kind": "index", "value": 0}], "key": "typology"} |
      | rename-member        | {"path": [{"kind": "key", "value": "models"}, {"kind": "index", "value": 0}, {"kind": "key", "value": "model"}, {"kind": "key", "value": "geometry"}], "from": "anchors", "to": "anchorPoints"} |
      | set-safe-number      | {"path": [{"kind": "key", "value": "models"}, {"kind": "index", "value": 0}, {"kind": "key", "value": "model"}, {"kind": "key", "value": "revision"}], "lexeme": "9007199254740991"} |
      | set-string           | {"path": [{"kind": "key", "value": "models"}, {"kind": "index", "value": 0}, {"kind": "key", "value": "id"}], "value": "hexagonal-cut-concrete-forest-left, RFC 7493 §2.4 clean: Ünïcödé mit Sonderzeichen"} |
      | insert-array-element | {"path": [{"kind": "key", "value": "models"}, {"kind": "index", "value": 0}, {"kind": "key", "value": "model"}, {"kind": "key", "value": "geometry"}, {"kind": "key", "value": "vertices"}], "index": 0, "value": {"kind": "object", "members": [{"key": "id", "value": {"kind": "string", "value": "i-json-mutation-test-vertex"}}, {"key": "position", "value": {"kind": "array", "items": [{"kind": "number", "lexeme": "0"}, {"kind": "number", "lexeme": "0"}, {"kind": "number", "lexeme": "0"}]}}]}} |
      | remove-array-element | {"path": [{"kind": "key", "value": "models"}, {"kind": "index", "value": 0}, {"kind": "key", "value": "model"}, {"kind": "key", "value": "geometry"}, {"kind": "key", "value": "vertices"}], "index": 10} |
  @id-i-json-conformance
  @level-quick
  @mode-conformance
  Scenario: The real document is an I-JSON document, not merely an RFC 8259 one
    Given the real input document shared://🔣️.json
    When the reference implementation checks every clause RFC 7493 adds to RFC 8259
    Then the top level is an object, no object repeats a member name, every integer fits ±(2^53−1) and no string carries a Unicode noncharacter

  @id-identity-round-trip
  @level-long
  @mode-round-trip
  Scenario: Decode and re-encode the real document without passing bytes through
    Given the real input document shared://🔣️.json
    When the document is fully parsed into the subset's own snapshot model and re-encoded from it alone
    Then the oracle and the subject agree on the semantic projection
    And the re-encoded bytes are not bit-identical to the input
