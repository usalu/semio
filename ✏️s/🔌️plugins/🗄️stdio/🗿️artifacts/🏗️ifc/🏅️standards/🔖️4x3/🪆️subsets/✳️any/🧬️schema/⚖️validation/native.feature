Feature: Every native schema rule uses the current canonical EXPRESS program
  Scenario: Native program data follows the complete retained ADD2 corpus
    Given the official complete ADD2 declaration catalog
    When every retained expression, function and global rule is compiled
    Then compilation emits zero diagnostics and admits the canonical AST schema
    And the native data digest equals the compiled program digest
    And the native corpus counts equal the language-neutral fixture
    And syntax coverage alone does not assert native semantic execution
