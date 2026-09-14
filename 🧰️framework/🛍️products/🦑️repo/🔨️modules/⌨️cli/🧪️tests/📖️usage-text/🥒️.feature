@capability-repo.cli.usage-text
@no-oracle-repo-cli-owned-usage-presentation
@comparison-ordered-json-v1
Feature: Every command states its own usage text
  `--help` or `-h` anywhere in an invocation short-circuits to the usage text of the command
  selected so far. That text is `<short>`, a blank line, `Usage:` and the two-space-indented usage
  line; a command with subcommands then adds a blank line, `Commands:` and one line per child in
  declaration order, the child name padded to twenty columns and its own summary after a single
  space. A child name longer than twenty columns is not truncated — the summary simply moves right.

  local://📖️usage-goldens.json states that text verbatim for representative commands, states
  the structural law every command in the tree obeys, and states the verbs the root registers in
  the order it registers them.

  @id-representative-commands-state-their-text
  @level-fundamental
  @mode-conformance
  Scenario: Every golden command produces its stated usage text byte for byte
    Given the goldens local://📖️usage-goldens.json
    When the host renders the usage text of each golden command path
    Then the produced text equals the stated one

  @id-every-command-obeys-the-usage-law
  @level-fundamental
  @mode-conformance
  Scenario: Every command in the tree obeys the stated structure
    Given the goldens local://📖️usage-goldens.json
    When the host renders the usage text of every command path of the tree
    Then each text opens with its summary, carries the usage marker and its usage line, and a command with children lists every child once on its own indented line

  @id-the-root-registers-its-stated-verbs
  @level-quick
  @mode-conformance
  Scenario: The root registers exactly the stated verbs in the stated order
    Given the goldens local://📖️usage-goldens.json
    When the host lists the verbs the root command registers
    Then the list equals the stated one, in order
