Feature: Micro-commit prepare keeps the index that already exists
  Prepare writes the commit message for paths already staged. It does not run git add.
  Unstaged edits and untracked files stay out of the index. An empty index is an error.
  Git status porcelain and git diff --cached --name-only are the oracle: both are unchanged by prepare.
  Ajv checks the case vectors in shared://micro-commit-staged-index/cases.json before either case runs.

  Scenario: staged-note-stays-the-only-index-entry
    Given a repository whose index contains only staged-note.md
    And base.md is modified in the worktree and loose-note.md is untracked
    When micro-commit prepare runs for the staged note
    Then the exit code is 0 and git status is unchanged

  Scenario: empty-index-is-rejected
    Given a repository with no staged paths
    And base.md is modified in the worktree and loose-note.md is untracked
    When micro-commit prepare runs
    Then the exit code is 1, the error says nothing is staged, and git status is unchanged
