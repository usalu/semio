Feature: Every declared command and target is offered under its verb
  The dashboard is the only control plane. Runnable things are declared once, next to their owner,
  in `metadata.semio.dashboard` of a project manifest, a ticket command document or the generated
  playground catalog, and the registry offers each one as a command.

  Scenario: A developer starts a declared command from the dashboard
    Given a project manifest declares a tool, a compound and a target with a ready port
    When the dashboard discovers commands
    Then every declared command is offered under its verb with its command, working directory and environment
    And workspace and parameter tokens are substituted
    And the command runs as a plain argument list, never through a shell

  Scenario: A compound starts its members in order
    Given a compound names two offered commands and the first declares a ready port
    When the developer selects the compound
    Then each member starts in its own managed terminal
    And the second member starts after the first one is ready

  Scenario: A command with a required parameter is not started without it
    Given a declared parameter is required and has no default
    When the developer starts the command without choosing a value
    Then the registry refuses the start and names the parameter

  Scenario: A declaration that breaks the schema is reported
    Given a project manifest whose dashboard declaration breaks the registry schema
    When the dashboard discovers commands
    Then the problem is reported with its manifest and location
    And every valid command stays runnable

  Scenario: Build, test and publish commands are found by their verb and owner
    Given a project declares targets whose names begin with a verb
    When the developer searches for the verb, owner and subject
    Then the targets are listed together under that verb
    And a name that begins with no known verb is filed under task

  Scenario: A build starts from the published project graph
    Given the published project graph is at least as new as every discovered project manifest
    When the developer starts a build, test or publish command
    Then the command starts from the published graph without waiting for graph construction
    And a continuous dev, serve or watch command still builds its own graph

  Scenario: A changed project manifest rebuilds the project graph
    Given a project manifest, the workspace configuration or the root package manifest is newer than the published graph
    When the developer starts a build
    Then the command rebuilds and republishes the project graph before running
