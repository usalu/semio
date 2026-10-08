Feature: Every launch configuration, target and script is offered under its verb
  Scenario: A developer starts a registered launch configuration from the dashboard
    Given the editor launch document registers terminal commands with comments and trailing commas
    When the dashboard discovers commands
    Then every terminal configuration is offered under its verb with its declared command, working directory and environment
    And workspace and defaulted input variables are substituted
    And a plain argument list runs directly while a command needing expansion runs through the platform shell

  Scenario: A compound starts its members together
    Given a compound names two offered configurations
    When the developer selects the compound
    Then each member starts in its own managed terminal

  Scenario: A configuration that cannot run unattended is not offered
    Given a configuration needs an input without a default or is not a terminal command
    Then the dashboard does not offer it or any compound naming it

  Scenario: Build, test and publish commands are found by their verb and owner
    Given a project declares targets whose names begin with a verb
    When the developer searches for the verb, owner and subject
    Then the target, its workspace script and its launch configuration are listed together under that verb
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
