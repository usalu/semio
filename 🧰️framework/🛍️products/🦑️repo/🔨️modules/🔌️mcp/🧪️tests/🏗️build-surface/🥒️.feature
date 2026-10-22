@capability-mcp-build-surface
@oracle-ajv
@comparison-ordered-json-v1
Feature: The repo MCP exposes the neutral build catalogue as tools
  Agents never shell out to build tools. Every exposed row of the tool-neutral `repo` operations catalogue is one MCP tool whose input schema, argument vector, bilingual description, annotations and task support are derived from the catalogue and one authored exposure policy. Every catalogue row is either exposed or excluded with a reason in English and German.

  @id-tool-set
  @level-fundamental
  @mode-differential
  Scenario: The derived tool set equals the golden tool set
    Given the catalogue shared://📚️library/🏗️build/⚙️operations/🔣️catalog.json and the policy asset://🏗️build-surface.json
    When every implementation derives the tools
    Then the tool names equal shared://🏗️build-surface.json and no catalogue row is neither exposed nor excluded

  @id-argument-verdicts
  @level-fundamental
  @mode-differential
  Scenario: The input schema accepts and refuses exactly the golden argument vectors
    Given the golden vectors of shared://🏗️build-surface.json
    When each vector is checked against the derived input schema by the implementation and by a JSON Schema validator
    Then both give the verdict of the vector, and an accepted vector yields the golden argument vector and environment

  @id-queries-read-the-sealed-graph
  @level-quick
  @mode-differential
  Scenario: Graph queries default to the sealed graph
    Given the tools show_projects, show_project, show_targets and graph
    When no graph_policy is given
    Then the argument vector carries --graph-policy=current and --json

  @id-agent-attribution
  @level-quick
  @mode-differential
  Scenario: A call runs under the agent that made it
    Given a call with the argument agent alpha
    When the child process starts
    Then SEMIO_AGENT_SESSION is the address of the session that repo session open issued for alpha, no per-tool identity variable is set, and the variables of an enclosing invocation are removed

  @id-cancellation
  @level-quick
  @mode-differential
  Scenario: Cancelling a call or a task interrupts the child
    Given a running build call
    When the client sends notifications/cancelled or tasks/cancel
    Then the child receives an interrupt, may clean up within the grace period and is killed after it
