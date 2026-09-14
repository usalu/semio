@capability-graphql-document-parsing
@oracle-graphql-js
@comparison-ordered-json-v1
Feature: One GraphQL request string parses to one canonical AST in every implementation
  The repo CLI answers `repo://` queries with a hand-rolled grammar, not with a library, so the
  grammar itself is the thing under test. Every implementation of this owner parses the same corpus
  of request strings and emits the AST projection defined in `🧬️schema/🔣️.json`; the reference
  implementation is the `graphql` npm package's own `parse`, projected through the same mapping.

  The mapping is declared, and it is lossy in exactly the places the grammar is: variable definitions
  and directives are parsed and discarded, an alias-free field carries a null alias, and a bare enum
  name projects as a string literal. Anything the grammar does NOT accept — fragments, block strings,
  a second operation — belongs to the `unsupported-syntax` case, never to this corpus.

  @id-corpus-projects-identically
  @level-fundamental
  @mode-differential
  Scenario: Every document of the corpus projects to the same AST
    Given the request corpus local://🔣️documents.json
    When each implementation parses every document
    Then every implementation projects the same operation kind, selections, aliases and arguments

  @id-operation-kind-is-recovered
  @level-quick
  @mode-conformance
  Scenario: The operation kind is recovered from the header, defaulting to query
    Given the request corpus local://🔣️documents.json
    When each implementation reads only the operation kind of every document
    Then a bare selection set reads as a query and a `mutation` header reads as a mutation
