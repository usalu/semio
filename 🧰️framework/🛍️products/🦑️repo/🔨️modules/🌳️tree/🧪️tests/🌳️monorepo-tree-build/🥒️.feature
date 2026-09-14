@capability-repo.tree.monorepo-build
@no-oracle-repo-tree-projection
@comparison-ordered-json-v1
Feature: A record set projects into the monorepo tree
  The tree domain never reads the repository. It is handed the already-loaded records — the
  technologies with their bundles, the folders, the files, the goals, the tickets, the drafts,
  the policies, the contributors, the checkpoints and the sessions — and decides only the shape:
  which category a record lands in, how folders nest, which bundle owns which file, how a subgoal
  hangs under its goal and which ticket hangs under which goal.

  Every scenario builds from shared://🌳️tree-source.json and holds the result against
  shared://📤️tree-build-expectations.json, which states the projection as a pre-order outline of
  `<depth>|<kind>|<id>|<label>` rows.

  Rendering an entity line and minting an artifact id belong to 🪪️identity and the CLI renderers,
  so both are ports. The host supplies the same stub in every language, stated once here:

    key(data)          = the first non-empty string among name, title, slug, id, path, else ""
    human(kind, data)  = "<kind>#<key>"
    markdownLink(...)  = "[<kind>#<key>]"
    markdown(...)      = "- [<kind>#<key>]"
    artifactId(k, d)   = "<d.parentId></k>:<key>"

  @id-projects-the-record-set
  @level-fundamental
  @mode-conformance
  Scenario: The record set becomes the seven categories in their stated order
    Given the record set shared://🌳️tree-source.json
    When the host builds the monorepo tree without sections
    Then the outline equals the stated one, folders sort before files at every level, the bundle view repeats the folders below each bundle root and the file with no folder hangs off the codebase category itself

  @id-includes-sections-when-requested
  @level-fundamental
  @mode-conformance
  Scenario: Requesting sections hangs the parsed sections under their file
    Given the record set shared://🌳️tree-source.json
    When the host builds the monorepo tree with sections included
    Then every file node that has sections carries them, with the definitions of a section before its nested sections

  @id-renders-text-and-markdown
  @level-fundamental
  @mode-conformance
  Scenario: The tree renders as connector text and as a markdown list
    Given the tree built without sections
    When the host renders it as text and as markdown through the stub entity renderer
    Then a category renders as a link, an entity renders through the renderer with a blanked parent id, and the last child of a level carries the closing connector

  @id-stamps-parent-artifact-ids
  @level-quick
  @mode-conformance
  Scenario: Every entity node learns the artifact id of its nearest entity ancestor
    Given the tree built without sections
    When the host propagates parent ids from the empty root id through the stub identifier
    Then a category passes its own parent id down unchanged and every entity node carries the id its nearest entity ancestor minted

  @id-caches-by-content-digest
  @level-quick
  @mode-round-trip
  Scenario: The cache key follows the content, the fingerprint and the section choice
    Given the trees built with and without sections
    When the host takes the content digest of each and builds the cache metadata of both
    Then the two digests differ, each digest is stable across two builds of the same records, the metadata validates against its own fingerprint and section choice, and it is refused for a different fingerprint, a different section choice or a different schema version
