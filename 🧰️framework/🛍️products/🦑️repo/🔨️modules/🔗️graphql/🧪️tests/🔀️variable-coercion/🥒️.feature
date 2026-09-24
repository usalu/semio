@capability-graphql-variable-coercion
@oracle-graphql-js
@comparison-ordered-json-v1
Feature: Argument values resolve against variables and declared defaults identically
  What the executor hands a resolver is not the AST but the COERCED argument map: variables
  substituted, lists and input objects walked, and declared field defaults filled in. The rule that
  decides a whole class of bugs is the one about absence: a default fills an argument NAME that is
  absent, and an argument bound to an unbound variable is present-and-null, so it never takes the
  default. The reference implementation is the `graphql` npm package's own `valueFromASTUntyped`,
  which coerces an untyped literal against a variable map by the same rule.

  @id-arguments-resolve-against-variables
  @level-fundamental
  @mode-differential
  Scenario: Every root selection's arguments coerce to the same values
    Given the coercion corpus shared://🔀️variable-coercion/🔣️coercions.json
    When each implementation parses the request and coerces each root selection's arguments
    Then every implementation produces the same argument map for every case

  @id-defaults-fill-only-absent-arguments
  @level-quick
  @mode-conformance
  Scenario: A declared default fills an absent argument name and nothing else
    Given the coercion corpus shared://🔀️variable-coercion/🔣️coercions.json
    When each implementation coerces the cases that declare defaults
    Then an absent argument takes its default and a null-valued variable does not
