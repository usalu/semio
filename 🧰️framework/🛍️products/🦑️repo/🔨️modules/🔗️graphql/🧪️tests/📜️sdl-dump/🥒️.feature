@capability-graphql-schema-definition-language
@oracle-graphql-js
@comparison-ordered-json-v1
Feature: The schema the executor serves is the committed SDL, and the committed SDL is real GraphQL
  The executor builds its schema in code, and the repository commits that schema as SDL in
  asset://🧬️schema/🔣️schema.graphql so every implementation and every client reads one document
  rather than one implementation's field table. Two things have to hold and neither is obvious: the
  committed SDL must be a schema a conforming GraphQL reader accepts at all, and it must describe
  exactly the schema the executor actually serves — same types, same fields, same field types, same
  arguments, same enum members, same union members, same interfaces.

  The reference is `graphql-js`: it reads the committed document with `buildSchema` — which fails
  outright on a document that is not GraphQL — and reports the inventory it found. The subject
  reports the inventory of the schema it built in code. Nothing in the oracle adapter reads the
  executor's own type table.

  @id-served-schema-matches-the-committed-sdl
  @level-fundamental
  @mode-differential
  Scenario: The inventory of the built schema is the inventory of the committed document
    Given the committed schema document asset://🧬️schema/🔣️schema.graphql
    When each implementation reports the types, fields, arguments and members it finds
    Then every implementation reports the same inventory in the same order
