@capability-repo-event-payload-encoding
@oracle-ajv-repo-events
@comparison-ordered-json-v1
Feature: Every implementation encodes each event payload to the same JSON
  A payload crosses the wire between the Go CLI, the Rust CLI and the coordinator. Field order,
  omit-empty semantics and the difference between an absent field, an empty string and an explicit
  null are all part of the contract, so the golden encoding of every payload type is frozen in
  `🧫️fixtures/✉️payload-vectors.json` and both implementations must reproduce it byte for byte.
  Ajv independently judges the same encodings against `🧬️schema/🔣️.json`.

  @id-golden-encoding-per-payload
  @level-fundamental
  @mode-differential
  Scenario: Decoding then re-encoding a payload reproduces its golden JSON
    Given the payload vectors shared://✉️payload-vectors.json
    When the implementation decodes each input into its typed payload and re-encodes it
    Then every encoding equals the frozen golden JSON of that vector

  @id-omit-empty-and-explicit-null
  @level-quick
  @mode-conformance
  Scenario: An absent optional, an empty string and an explicit null stay distinguishable
    Given the payload vectors shared://✉️payload-vectors.json
    When the implementation re-encodes the vectors that exercise optional fields
    Then an unset optional is absent, an empty optional string is absent and a nulled pointer is absent

  @id-envelope-round-trip
  @level-fundamental
  @mode-differential
  Scenario: The envelope wraps a payload without reordering or re-escaping it
    Given a payload and a declared kind
    When the implementation builds the envelope
    Then the envelope encodes as kind, source, payload in that order
