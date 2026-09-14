@capability-repo-move-file-folder-move
@no-oracle-repo-move-paths
@comparison-ordered-json-v1
Feature: Moving a file or a folder carries its subtree and rewrites the documentation index
  A folder move carries everything under it. Both a file move and a folder move then rewrite every
  `AGENTS.md` heading that names the old path, once per heading line, leaving every other line
  alone. A move onto an occupied path is refused before anything is changed. See the recorded
  decision `repo-move-paths`.

  @id-a-move-carries-the-subtree-and-the-docs
  @level-fundamental
  @mode-conformance
  @seed-1
  Scenario: Each vector reaches the recorded workspace and output line
    Given the shared vector set shared://🚚️file-folder-move-trees.json
    When the host plans the move over each before-tree and applies the plan
    Then every implementation projects the recorded tree and output line per vector

  @id-moving-back-restores-the-tree
  @level-fundamental
  @mode-round-trip
  @seed-1
  Scenario: Moving a path and moving it back restores the whole workspace
    Given the shared vector set shared://🚚️file-folder-move-trees.json
    When the host moves each path and then moves it back
    Then every implementation projects the original tree again

  @id-an-occupied-target-is-refused
  @level-fundamental
  @mode-error
  @seed-1
  Scenario: A missing source or an occupied target is refused with the recorded message
    Given the shared vector set shared://🚚️file-folder-move-trees.json
    When the host plans each refused move
    Then every implementation projects the recorded refusal message per vector
