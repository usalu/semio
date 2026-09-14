@capability-repo-definition-parsing
@oracle-typescript-compiler
@comparison-ordered-json-v1
Feature: A file's definitions are found where the language's own compiler finds them
  A definition is a named top-level declaration with a line range and a kind. For TypeScript the
  reference is the TypeScript compiler's own parser: it reads the same source with the real language
  grammar and reports the same declaration names, the same first and last line of each declaration,
  and — for a `const` bound to an arrow function, a function expression or a class expression — the
  same promotion of the binding to a callable. That makes the compiler a credible oracle for exactly
  the projection below, and both subject implementations are held to it.

  @id-typescript-top-level-declarations
  @level-fundamental
  @mode-differential
  Scenario: Every top-level TypeScript declaration is found with its name and line range
    Given the shared source shared://🟦️sample.ts
    When each implementation parses its definitions
    Then every implementation projects the same names and the same first and last line

  @id-callable-const-is-a-function
  @level-fundamental
  @mode-differential
  Scenario: A const bound to a callable initialiser is reported as a function, not a constant
    Given the case source local://🔤️callables.ts
    When each implementation parses its definitions
    Then every implementation reports the same names and the same callable flag per name
