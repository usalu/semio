@capability-repo.cli.mcp-verb
@no-oracle-repo-cli-owned-mcp-verb-wiring
@comparison-ordered-json-v1
Feature: The `mcp` verb serves the repo repository on stdio
  `semio mcp [kind]` starts the repo Model Context Protocol server on standard input and output
  with the profile the operand names or `SEMIO_REPO_MCP_CLIENT` supplies, and the shipped
  `repo` binary serves the same repository. The protocol itself belongs to 🔌️mcp; what
  this case proves is that the verb wires the production repository behind it, that `initialize`
  ignores members it does not know, and that the declared tool and resource vocabulary reaches a
  client unchanged.

  local://🤝️handshake.json states the conversation and the vocabulary it owes.

  @id-the-verb-completes-the-handshake
  @level-fundamental
  @mode-conformance
  Scenario: `semio mcp` answers initialize, tools/list and resources/list over stdio
    Given the conversation local://🤝️handshake.json
    When the host runs the conversation against the `mcp` verb
    Then the initialize result carries the stated protocol version, server name, server version and capabilities, and the declared tool and resource names equal the stated ones

  @id-initialize-ignores-unknown-members
  @level-fundamental
  @mode-conformance
  Scenario: An initialize request carrying an unknown member is still accepted
    Given the conversation local://🤝️handshake.json
    When the host runs the conversation against the `mcp` verb
    Then the initialize response is a result rather than an error

  @id-the-dry-run-starts-no-server
  @level-quick
  @mode-conformance
  Scenario: `--dry-run` initializes the verb and exits without serving
    Given the conversation local://🤝️handshake.json
    When the host runs the `mcp` verb with `--dry-run` and no standard input
    Then it exits successfully and writes nothing
