@capability-graphql-query-execution
@oracle-graphql-js
@comparison-ordered-json-v1
Feature: One query over one frozen repository answers the same way in every implementation
  The repo CLI answers `repo://` queries with a hand-rolled executor over a hand-rolled schema. The
  grammar is already pinned by the `document-parsing` case; what this case pins is EXECUTION — which
  field is resolved from which source, how an alias renames a key, how `__typename` is answered, how
  a list, a nullable and a non-null are shaped, how an enum member is serialised, and how an argument
  is coerced before a resolver ever sees it.

  Every implementation loads the same frozen repository records shared://🔣️repo-records.json into its
  context port and runs every query of local://🔣️queries.json
  in order. No filesystem, no clock and no process is reachable from a scenario, so the only thing two implementations can disagree about is the
  meaning of the query.

  The reference is `graphql-js`: the committed SDL asset://🧬️schema/🔣️schema.graphql
  is built into a real GraphQL schema and the same documents are executed against it. The oracle's source objects are
  a DECLARED mapping of the same records — it restates the field derivations, and `graphql-js` alone
  decides what executing a selection set against them means. It is not a second executor: nothing in
  the oracle adapter walks a selection set.

  @id-corpus-executes-identically
  @level-fundamental
  @mode-differential
  Scenario: Every query of the corpus returns the same data payload
    Given the frozen repository shared://🔣️repo-records.json
    And the query corpus local://🔣️queries.json
    When each implementation executes every query against the schema
    Then every implementation returns the same data payload for every query

  @id-arguments-coerce-before-resolution
  @level-quick
  @mode-differential
  Scenario: An enum member and a variable reach the resolver as the value the schema declares
    Given the frozen repository shared://🔣️repo-records.json
    And the query corpus local://🔣️queries.json
    When each implementation executes only the queries that carry an argument or a variable
    Then every implementation filters by the same coerced argument value
