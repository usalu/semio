@capability-graphql-subset-boundary
@no-oracle-graphql-owned-subset-boundary
@comparison-ordered-json-v1
Feature: The boundary of the accepted subset is the same in every implementation
  This grammar is a deliberate SUBSET of GraphQL, and it is also, in a few places, wider than
  GraphQL. Both edges are behaviour the executor depends on, and neither can be judged by a
  conforming reference implementation: a library accepts the fragments we reject and rejects the
  `{ }` we accept, so measuring against it would only restate that we are not a GraphQL parser. The
  recorded decision `graphql-owned-subset-boundary` says so, and the evidence is pairwise:
  independently written Go and Rust implementations must draw the same line, verbatim message
  included.

  @id-subset-boundary-is-identical
  @level-fundamental
  @mode-differential
  Scenario: Accepted, rejected and the exact rejection message agree across implementations
    Given the boundary corpus local://🔣️divergences.json
    When each implementation parses every input
    Then acceptance and the verbatim diagnostic agree for every input
