@capability-repo-event-kind-catalog
@oracle-ajv-repo-events
@comparison-ordered-json-v1
Feature: Every implementation reports the same event kind catalog
  The kind strings a repo event can carry are declared once, in
  `🧬️schema/🔣️event-kinds.json`. Rust compiles that file in with `include_str!`; Go reads it at
  load time through a module-relative path; both mirror it as typed constants. A kind that exists
  in one implementation and not the other, or a constant that has drifted from the schema, is a
  broken wire contract, so the catalog is projected verbatim and compared.

  @id-catalog-is-the-schema-file
  @level-fundamental
  @mode-differential
  Scenario: The declared constants are exactly the schema file, in order
    Given the kind catalog 🧬️schema/🔣️event-kinds.json of this module
    When the implementation projects its own ordered list of event kinds
    Then the list equals the schema file, contains no duplicate and every kind is dotted

  @id-envelope-accepts-only-declared-kinds
  @level-fundamental
  @mode-conformance
  Scenario: An envelope carrying an undeclared kind is rejected by the schema
    Given an envelope whose kind is not in the catalog
    When it is judged against 🧬️schema/🔣️.json
    Then the schema rejects it, and the same envelope with a declared kind is accepted
