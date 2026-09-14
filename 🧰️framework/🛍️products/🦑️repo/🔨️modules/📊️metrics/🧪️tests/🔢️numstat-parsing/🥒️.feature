@capability-repo.metrics.numstat-parsing
@oracle-git-numstat-cli
@comparison-ordered-json-v1
Feature: A git numstat stream parses into per-commit line deltas
  `loc` reads its history from `git log --numstat`, so the parser has to survive every shape the
  real git binary emits: paths octal-escaped inside double quotes under `core.quotepath`, renames
  written as `old => new`, binary changes written as `-` and `-` with no line counts, merge commits
  that carry no numstat block at all under `--first-parent`, and paths the `loc` walk must never
  count (a dot-prefixed directory, the repository's own `.🧬semio` meta tree).

  The subjects replay the recorded transcript shared://🎞️git-transcript.json — the oracle rebuilds the
  same repository from shared://🌱️repository-recipe.json in a temporary directory, runs the real
  `git` binary over it, and answers from that — so agreement means the parser agrees with git and
  not merely with a file somebody typed.

  @id-recorded-transcript-matches-real-git
  @level-fundamental
  @mode-differential
  Scenario: The committed transcript is still exactly what git produces
    Given the recorded transcript and the recipe it was recorded from
    When the host reports the digests of the two log streams and the commit ids in them
    Then the recorded stream and a freshly recorded one agree digest for digest

  @id-parses-recorded-numstat-stream
  @level-fundamental
  @mode-differential
  Scenario: Every commit block becomes one record with weighted bucket sums
    Given the recorded `--no-merges` stream
    When the host parses it into commit records
    Then every implementation projects the same shas, timestamps, authors and per-bucket deltas

  @id-resolves-renames-and-quoted-paths
  @level-fundamental
  @mode-differential
  Scenario: Quoted Unicode paths are decoded and renames resolve to the new path
    Given the recorded `--no-merges` stream
    When the host reports every file row it saw
    Then the emoji paths are decoded, the rename names its source, the binary row carries no counts
    And the dot-prefixed and `.🧬semio` rows classify into no bucket

  @id-merge-commit-carries-no-file-rows
  @level-quick
  @mode-differential
  Scenario: A merge commit is a record with an empty file list, never a dropped commit
    Given the recorded stream taken without `--no-merges`
    When the host parses it into commit records
    Then the merge commit appears with no file rows and an empty delta
