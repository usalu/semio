@capability-repo-move-section-extract
@no-oracle-repo-move-sections
@comparison-ordered-json-v1
Feature: A section leaves its file with the header, package declaration and imports it needs
  Extracting lifts the lines strictly between a section two markers into a new file, prefixed by the
  source header region, its package declaration where the language has one, and its import list
  formatted the way that language spells one. The source keeps everything outside the section, the
  two marker lines included in the removal. See the recorded decision `repo-move-sections`.

  @id-the-section-leaves-with-its-imports
  @level-fundamental
  @mode-conformance
  @seed-1
  Scenario: Each vector produces the recorded new file and the recorded remaining source
    Given the shared vector set shared://🧲️section-extract-trees.json
    When the host plans the extraction over each file and applies the plan
    Then every implementation projects the recorded target, source and output line per vector

  @id-extraction-removes-exactly-the-section
  @level-fundamental
  @mode-property
  @seed-1
  Scenario: The lines the source loses are exactly the section own line range
    Given the shared vector set shared://🧲️section-extract-trees.json
    When the host counts the source lines before and after each extraction
    Then every implementation projects the same line arithmetic per vector

  @id-a-missing-section-is-refused
  @level-fundamental
  @mode-error
  @seed-1
  Scenario: A missing file, a missing section or a sectionless language is refused
    Given the shared vector set shared://🧲️section-extract-trees.json
    When the host plans each refused extraction
    Then every implementation projects the recorded refusal message per vector
