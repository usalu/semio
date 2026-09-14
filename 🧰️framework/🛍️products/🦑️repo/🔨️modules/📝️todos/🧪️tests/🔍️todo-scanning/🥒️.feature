@capability-repo.todos.scanning
@no-oracle-repo-todos-scanning
@comparison-ordered-json-v1
Feature: Every todo a tree carries is found exactly once
  A todo has no store of its own: it IS a line of source. A directory contributes the `- TODO `
  items of its own `.todos.md`, a file contributes the `// TODO Name: description` comments it
  carries, and a markdown file that is not a `.todos.md` contributes nothing, because an item is
  only an item where a directory records its own work. Which files are read at all is a closed
  list of extensions, and which directories are entered at all is the filesystem tree's own rule:
  never a dependency directory, never a build output, never a dotted directory other than the
  repository meta root. The tree is stated as its own files rather than copied out of this
  checkout, which carries no `.todos.md`; see `repo-todos-scanning`.

  @id-every-todo-in-the-tree-is-found
  @level-fundamental
  @mode-differential
  Scenario: Scanning a stated tree yields the same todos in the same order
    Given the tree shared://🔍️scan-tree.json
    When each implementation scans the in-memory tree and searches it for every term
    Then every implementation projects the same todos in the same order and the same search results

  @id-the-scan-refuses-three-kinds-of-directory
  @level-fundamental
  @mode-conformance
  Scenario: A scan over a real directory enters neither a dependency, a build output nor a dotted directory
    Given the tree shared://🔍️scan-tree.json
    When each implementation writes the tree into its own work directory and scans it from disk
    Then every implementation projects the same walk and the same todos, and no entry under a dependency, build or dotted directory appears in either
