@capability-repo-scope-ids
@no-oracle-repo-scope-grammar
@comparison-ordered-json-v1
Feature: A file resolves to the same addressable scopes in every implementation
  A scope identifier addresses one artifact inside a file: `file:<path>` for the file,
  `section:<path>#<sectionPath>` for a region or heading, and `def:<path>#<sectionPath>::<name>` for
  a definition inside one — degrading to `def:<path>#<name>` when the definition sits outside every
  section. The grammar is this repository's own addressing scheme; nothing third party can produce
  one, so the recorded no-oracle decision `repo-scope-grammar` names the pinned specification vectors
  and two independently written builders as the substitutes.

  @id-scope-id-grammar
  @level-fundamental
  @mode-conformance
  Scenario: The four id shapes are built exactly as the grammar specifies
    Given the four scope kinds with and without a section path
    When each implementation builds an identifier for each
    Then every implementation produces the specified string

  @id-scopes-of-a-source-file
  @level-fundamental
  @mode-differential
  Scenario: A source file yields its file scope, its section scopes and its definition scopes
    Given the shared sources shared://🟦️sample.ts, shared://🐹️sample.go and shared://📰️sample.md
    When each implementation builds every scope of each file
    Then every implementation projects the same kinds, identifiers, section paths and line ranges
