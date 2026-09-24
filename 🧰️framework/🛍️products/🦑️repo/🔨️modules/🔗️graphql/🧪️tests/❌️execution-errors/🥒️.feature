@capability-graphql-execution-errors
@no-oracle-graphql-owned-executor-diagnostics
@comparison-ordered-json-v1
Feature: A request the executor cannot answer is refused with the executor's own words
  Half of an executor is what it refuses. A selection that names a field no type carries, a node id
  in no recognised shape, a mutation whose aggregate no record carries — each has to stop, and stop
  with a message a caller can act on rather than with a null the caller reads as an empty result.
  The `query-execution` case measures the accepting half against `graphql-js`; this case measures
  the refusing half, which `graphql-js` cannot judge because the wording is ours: a conforming
  executor refuses the same documents and says so in its own words, so comparing the two would
  measure a rendering choice instead of a behaviour.

  The evidence is therefore specification vectors, as the recorded decision
  `graphql-owned-executor-diagnostics` says: every input is paired with the verbatim message it must
  produce, asserted by the implementation itself, and the projection carries what was actually said
  so a message that drifts is visible rather than merely unequal.

  @id-every-refusal-carries-its-verbatim-message
  @level-fundamental
  @mode-error
  Scenario: Every refusal of the corpus produces exactly the recorded message
    Given the frozen repository shared://🔣️repo-records.json
    And the refusal corpus shared://❌️execution-errors/🔣️refusals.json
    When each implementation executes every input against the schema
    Then every input is refused with the recorded message and none is answered

  @id-a-refusal-writes-no-event-and-changes-no-record
  @level-quick
  @mode-conformance
  Scenario: A refused mutation leaves the record set exactly as it found it
    Given the frozen repository shared://🔣️repo-records.json
    And the refusal corpus shared://❌️execution-errors/🔣️refusals.json
    When each implementation runs only the mutations of the corpus against one context
    Then the record set is unchanged and no event was written
