@capability-repo.codebase.artifact-ids
@no-oracle-repo-codebase-artifact-ids
@comparison-ordered-json-v1
Feature: Folder, file, section and definition ids are built the same way everywhere
  An artifact id is the address every other module hands around: an MCP resource uri, a breach
  scope, a contribution record and a tree node all carry one. The id is a concatenation of entity
  emojis and flattened names walking down from the owning bundle, so it depends on the bundle
  detection, on the folder-kind derivation and on the entity-emoji vocabulary at once — three places
  a second implementation can drift. This emoji id grammar is this repository's own; the TypeScript
  library's `🔍️discovery` module was checked for an equivalent builder and has none, so there is no
  reference implementation to compare against and the recorded no-oracle decision
  `repo-codebase-artifact-ids` carries the case on two independent implementations agreeing over one
  committed vector set. The flattening keeps every code point above `0x7F`, so a name that already
  carries a leading taxonomy emoji keeps it and the id names the kind emoji and the name emoji both;
  `emoji-named-file-keeps-its-leading-emoji` and `emoji-named-folder-keeps-its-leading-emoji` pin
  that against an implementation that strips one of the two.

  @id-ids-agree-for-every-vector
  @level-fundamental
  @mode-differential
  Scenario: Every folder, file, section and definition vector gets the same id
    Given the repository tree shared://📡️repo-tree.json materialised into the work directory
    And the id vectors shared://📡️artifact-id-vectors.json
    When each implementation builds the id its vector's kind selects
    Then every implementation projects the same id per vector

  @id-uris-agree-for-every-file-vector
  @level-fundamental
  @mode-differential
  Scenario: The file uri built from an id agrees
    Given the repository tree shared://📡️repo-tree.json materialised into the work directory
    And the id vectors shared://📡️artifact-id-vectors.json
    When each implementation builds the file uri of every file vector
    Then every implementation projects the same uri per vector
