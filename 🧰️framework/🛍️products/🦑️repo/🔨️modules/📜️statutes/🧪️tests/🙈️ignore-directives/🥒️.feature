@capability-repo-statutes-ignore-directives
@no-oracle-repo-statutes-owned-law
@comparison-ordered-json-v1
Feature: A compose-ignore directive suppresses exactly the breaches it names
  A `// compose-ignore-<prefix>` line suppresses every statute whose identifier starts with one of
  its comma-separated prefixes, for the hundred lines that follow it and never for the line it sits
  on or any line before it. The prefix is a plain string prefix, so `code/section` covers every
  section statute while `code/section/empty` covers only that one, and a directive naming a statute
  that never fires suppresses nothing.

  @id-a-directive-suppresses-its-prefixes
  @level-fundamental
  @mode-conformance
  @seed-1
  Scenario: Every directive vector suppresses the breaches it claims and no other
    Given the directive vectors shared://🙈️ignore-directives/🔣️vectors.json
    When the host parses every directive block and decides every suppression
    Then every implementation projects the same parsed directives and the same suppression decisions

  @id-a-directive-only-reaches-forward
  @level-fundamental
  @mode-error
  @seed-1
  Scenario: A breach before the directive or beyond its window survives
    Given the directive vectors shared://🙈️ignore-directives/🔣️vectors.json
    When the host decides suppression for the line of the directive, the line before it, the last line of its window and the line after its window
    Then every implementation keeps the breach outside the window and drops only the breach inside it
