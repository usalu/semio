@capability-repo-move-file-integrate
@no-oracle-repo-move-sections
@comparison-ordered-json-v1
Feature: A whole file is folded into a named section of another file
  Integrating splits both files into header, package declaration, imports and body; merges the two
  headers by inserting the source lines the target does not already carry above the target closing
  header marker; unions the two import lists in target-then-source order; keeps the target package
  declaration and drops the source one; and writes the source body between a new pair of section
  markers, either at the end of the target body or immediately before a named parent section closing
  marker. See the recorded decision `repo-move-sections`.

  @id-the-source-lands-inside-the-markers
  @level-fundamental
  @mode-conformance
  @seed-1
  Scenario: Each vector produces the recorded target file and output lines
    Given the shared vector set shared://📥️file-integrate-trees.json
    When the host plans the integration and applies the plan
    Then every implementation projects the recorded target file and output lines per vector

  @id-the-new-section-is-parseable
  @level-fundamental
  @mode-property
  @seed-1
  Scenario: The section written by an integration is found again by the section reader
    Given the shared vector set shared://📥️file-integrate-trees.json
    When the host reads the sections of each integrated file back
    Then every implementation projects the same section names per vector

  @id-an-unknown-parent-section-is-refused
  @level-fundamental
  @mode-error
  @seed-1
  Scenario: A missing source, a missing target, a sectionless target or an unknown parent is refused
    Given the shared vector set shared://📥️file-integrate-trees.json
    When the host plans each refused integration
    Then every implementation projects the recorded refusal message per vector
