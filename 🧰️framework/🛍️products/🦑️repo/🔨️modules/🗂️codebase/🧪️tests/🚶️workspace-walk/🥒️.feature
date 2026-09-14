@capability-repo.codebase.workspace-walk
@no-oracle-repo-codebase-walk
@comparison-ordered-json-v1
Feature: Walking a repository yields the same bundles, folders and files everywhere
  Every repo verb that says anything about the codebase — analyze, tree, metrics, the MCP codebase
  resource — starts from one walk of the repository root. That walk decides which files are
  considered at all, which bundle owns each of them, and in which order they are reported; a
  divergence there silently changes every number downstream. The order is the lexical directory
  order the walk descends in, and the projection is the bundle, folder and file aggregate built on
  top of it. No third party walks a repository into this repository's own bundle vocabulary, so the
  confidence rests on the recorded no-oracle decision `repo-codebase-walk`: one committed tree, two
  independently written implementations, and the same projection from both.

  A file's extension carries its leading dot and its kind is derived from its name, and a folder
  carries the kind the walk derives for it rather than a constant. Those four members reach the
  GraphQL aggregate, `list`, `search` and the export snapshot unchanged, so the record scenario
  states them once for both implementations.

  A definition is addressed by the file it sits in, the section that encloses it, its kind and its
  name; the identity is built from all four. That is the aggregate `definition list`, `graphql`,
  `list`, `search` and `export` answer from, and the definition scenario states it.

  @id-walk-reports-the-same-considered-files
  @level-fundamental
  @mode-differential
  Scenario: The considered file list and its order agree
    Given the repository tree shared://📡️repo-tree.json materialised into the work directory
    When each implementation resolves the repository scope to its considered files
    Then every implementation projects the same file list in the same order

  @id-walk-projects-the-same-bundle-and-folder-aggregates
  @level-fundamental
  @mode-differential
  Scenario: The bundle and folder aggregates built on that walk agree
    Given the repository tree shared://📡️repo-tree.json materialised into the work directory
    When each implementation projects the bundle and folder aggregates of that walk
    Then every implementation reports the same ids, roots, paths and counts in the same order

  @id-walk-projects-the-same-file-and-folder-records
  @level-fundamental
  @mode-differential
  Scenario: The file and folder records built on that walk agree
    Given the repository tree shared://📡️repo-tree.json materialised into the work directory
    When each implementation projects the identity, name, extension and kind of every walked record
    Then every implementation reports the same members in the same order

  @id-walk-projects-the-same-definitions
  @level-fundamental
  @mode-differential
  Scenario: Every definition of every walked file agrees
    Given the repository tree shared://📡️repo-tree.json materialised into the work directory
    When each implementation projects every definition of every considered file
    Then every implementation reports the same identities, section paths, kinds and ranges in the same order
