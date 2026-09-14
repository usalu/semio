@capability-repo-move-section-move
@no-oracle-repo-move-sections
@comparison-ordered-json-v1
Feature: Renaming a section rewrites both of its markers and nothing else
  A section is a pair of comment-prefixed region markers whose name carries an entity emoji, or, in
  Markdown, a heading. Renaming one rewrites the opening marker, then the closing marker, and for
  Markdown the heading line as well; every other byte of the file is left alone, so a name that
  appears nowhere leaves the file untouched rather than failing. A section path is split on the hash
  character and only its last segment names the section. See the recorded decision
  `repo-move-sections`.

  @id-markers-follow-the-new-name
  @level-fundamental
  @mode-conformance
  @seed-1
  Scenario: Each language markers carry the new name and the body is untouched
    Given the shared vector set shared://📑️section-move-trees.json
    When the host plans the section rename over each file and applies the plan
    Then every implementation projects the recorded file and output line per vector

  @id-renaming-back-restores-the-file
  @level-fundamental
  @mode-round-trip
  @seed-1
  Scenario: Renaming a section and renaming it back restores the file byte for byte
    Given the shared vector set shared://📑️section-move-trees.json
    When the host renames each section and then renames the new name back to the old one
    Then every implementation projects the original file again

  @id-a-missing-file-is-refused
  @level-fundamental
  @mode-error
  @seed-1
  Scenario: Renaming a section of a file that does not exist is refused
    Given the shared vector set shared://📑️section-move-trees.json
    When the host plans each refused rename
    Then every implementation projects the recorded refusal message per vector
