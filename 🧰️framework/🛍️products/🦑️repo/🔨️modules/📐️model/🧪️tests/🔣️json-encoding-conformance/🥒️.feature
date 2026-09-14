@capability-repo.model.json-encoding
@oracle-ajv-model-schema
@comparison-ordered-json-v1
Feature: A domain document decodes and re-encodes to the same bytes in every implementation
  The model is what every repo module exchanges, so its wire shape is the contract: a member name,
  the `omitempty` rule that decides whether a member appears at all, the `null` a Go nil slice
  produces where an empty one produces `[]`, and the sorted key order of a map. A golden document
  per type is the frozen statement of that contract, and `🧬️schema/🔣️.json` is the same statement
  as a JSON Schema. The oracle is `ajv`, a real draft 2020-12 validator: it judges every golden
  against the schema before re-serialising it, so a schema that has drifted away from the two
  implementations fails here rather than agreeing with itself.

  @id-golden-documents-round-trip
  @level-fundamental
  @mode-differential
  Scenario: Decoding a golden and encoding it again reproduces it byte for byte
    Given the golden documents local://🔣️goldens.json
    And the model schema asset://🧬️schema/🔣️.json
    When each implementation decodes every golden into its own type and encodes it again
    Then every implementation projects the same text for every type, and the oracle finds every golden schema-valid
