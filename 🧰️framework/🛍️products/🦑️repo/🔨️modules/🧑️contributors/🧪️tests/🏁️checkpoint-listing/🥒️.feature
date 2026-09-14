@capability-repo.contributors.checkpoints
@no-oracle-repo-contributors-checkpoints
@comparison-ordered-json-v1
Feature: The checkpoint log parses into checkpoints, and refuses what is not one
  A checkpoint is one line of `git log` under a pretty format this repository chose: sha, author,
  date and subject, separated by `|`. A subject that contains a `|` keeps it, because only the
  first three separators are structural, and a line with fewer than four parts is not a checkpoint
  and is rejected rather than guessed at. The fixture is real output of this repository's own
  history plus the lines a reader has to refuse. `git` produced the fixture but is no reference for
  the parse, and nothing third party reads this format; see `repo-contributors-checkpoints`.

  @id-the-log-parses-into-checkpoints
  @level-fundamental
  @mode-differential
  Scenario: Every accepted line becomes a checkpoint and every malformed line is refused
    Given the real checkpoint log shared://🏁️checkpoint-log.json
    When each implementation parses the log, the malformed lines and the log again under a limit
    Then every implementation projects the same checkpoints, the same refusals, the same limited listing and the same artifact identifiers

  @id-a-limit-is-a-prefix-of-the-listing
  @level-quick
  @mode-property
  Scenario: Listing under a limit yields exactly the first entries of the unlimited listing
    Given the real checkpoint log shared://🏁️checkpoint-log.json
    When each implementation lists the checkpoints without a limit and under every limit up to the number of entries
    Then every implementation projects that each limited listing is the prefix of the unlimited one
