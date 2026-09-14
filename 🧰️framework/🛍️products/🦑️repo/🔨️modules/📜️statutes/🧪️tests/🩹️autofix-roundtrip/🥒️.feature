@capability-repo-statutes-autofix
@no-oracle-repo-statutes-owned-law
@comparison-ordered-json-v1
Feature: An autofixable breach is repaired into exactly the expected text
  A statute the catalog marks autofixable must be repairable without a human. `🧪️file-fixable`
  carries a blank line after its header region start marker, which the section policy raises as
  `code/section/wrong-format/newline-after-region`; `🧪️file-fixable-expected` is the byte-exact text
  the repair must produce. Repair is idempotent: applying it to the already repaired text changes
  nothing, and the repaired text raises no breach that the repair claims to have removed.

  @id-the-fixable-source-becomes-the-expected-source
  @level-fundamental
  @mode-round-trip
  @seed-1
  Scenario: The repair produces the expected text and nothing else
    Given the fixable source shared://📁️some/📁️folder/🧪️file-fixable/🟦️.tsx
    And the expected repaired source shared://📁️some/📁️folder/🧪️file-fixable-expected/🟦️.tsx
    When the host repairs the fixable source, repairs the result again and analyzes both
    Then every implementation projects the expected text, the repaired statutes, an idempotent second pass and no remaining breach
