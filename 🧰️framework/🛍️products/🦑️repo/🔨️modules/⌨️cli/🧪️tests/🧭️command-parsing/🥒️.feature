@capability-repo.cli.command-parsing
@no-oracle-repo-cli-owned-command-grammar
@comparison-ordered-json-v1
Feature: An argv splits into a command path, its operands and its flag values
  The repo CLI owns one command grammar. An invocation names a verb, optionally a subcommand,
  then operands and `--flag [value]` pairs. Subcommand selection stops at the first operand, `--`
  ends flag parsing, `--help`/`-h` short-circuits to the usage text of the command selected so
  far, a boolean flag never consumes the next token, and a command whose flag parsing is disabled
  takes everything after its name as operands.

  Every vector in shared://🧭️command-parsing/🔣️argv-vectors.json states one argv and either the projection it
  owes — the command path, the operands, and ONLY the flags the caller passed — or the verbatim
  refusal message. A vector that states a message is red when the message drifts, so a reworded
  diagnostic is a failing test rather than an unequal projection.

  @id-argv-projects-into-a-command
  @level-fundamental
  @mode-conformance
  Scenario: Every accepted argv projects into its stated command path, operands and flags
    Given the vectors shared://🧭️command-parsing/🔣️argv-vectors.json
    When the host parses every accepting vector against the repo command tree
    Then each projection equals the stated path, operands, flag values and help decision

  @id-refusals-carry-their-verbatim-message
  @level-fundamental
  @mode-error
  Scenario: Every refused argv carries the message the grammar owes it
    Given the vectors shared://🧭️command-parsing/🔣️argv-vectors.json
    When the host parses every refusing vector against the repo command tree
    Then each refusal carries the stated message verbatim

  @id-parsing-is-idempotent
  @level-quick
  @mode-round-trip
  Scenario: Parsing the same argv twice produces the same projection
    Given the vectors shared://🧭️command-parsing/🔣️argv-vectors.json
    When the host parses every accepting vector twice
    Then the two projections are equal for every vector
