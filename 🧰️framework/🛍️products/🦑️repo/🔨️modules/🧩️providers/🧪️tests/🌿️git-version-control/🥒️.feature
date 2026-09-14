@capability-repo.providers.git-version-control
@oracle-git-cli
@comparison-ordered-json-v1
Feature: The version control provider says exactly what git says
  Every method of the version control provider is a small argv sequence handed to git, so the only
  reference that can adjudicate it is git itself — registered as the `git-cli` third-party CLI
  oracle. Each host builds its own throwaway repository outside this checkout from the same
  deterministic script and then reads the same facts back: the current branch, whether HEAD is a full
  object name that two independent reads agree on, which paths are staged, and whether a checkpoint
  advanced HEAD. Object names themselves are never projected — they are a function of timestamps and
  author identity, so comparing them would compare the clock rather than the provider. The scenarios
  are `@level-long` because they spawn a real version control system: the Go snapshot these came from
  skips the same four behaviours under `go test -short` for exactly that reason.

  @id-a-fresh-repository-reports-its-branch-and-head
  @level-long
  @mode-differential
  Scenario: A fresh repository reports the branch it was initialised on and a stable HEAD
    Given a throwaway repository initialised on main with one commit
    When the current branch and the current checkpoint are read twice
    Then every implementation projects main, a forty-character object name and two agreeing reads

  @id-staging-lists-the-added-paths
  @level-long
  @mode-differential
  Scenario: Staging every change lists exactly the added paths
    Given a throwaway repository initialised on main with one commit
    When two new files are written and everything is staged
    Then every implementation projects an empty staged list before staging and both paths after it

  @id-a-checkpoint-commits-and-advances-head
  @level-long
  @mode-differential
  Scenario: A checkpoint commits the working tree and moves HEAD forward
    Given a throwaway repository initialised on main with one commit
    When a new file is written and a checkpoint is taken
    Then every implementation projects an advanced HEAD that the next read agrees with and an empty staged list

  @id-reading-outside-a-repository-is-an-error
  @level-long
  @mode-error
  Scenario: Reading a branch outside a repository is reported, never defaulted
    Given a directory that is not a git repository
    When the current branch and the current checkpoint are read
    Then every implementation projects a failure for both rather than an empty branch name
