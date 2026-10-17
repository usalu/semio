Feature: Developer surfaces share neutral command authority
  Build targets and owner declarations are the command authority. The dashboard, editors
  and agents dispatch tasks through the repository build interface.

  Scenario: Preserve canonical bootstraps and neutral task aliases
    Given the repository root package manifest
    Then repo, setup, dashboard and dashboard:install retain their canonical commands
    And additional aliases select literal targets through bun repo run
    And direct backend commands and shell composition are refused
    And an independent JSON Schema validator reaches the same decision

  Scenario: Preserve required editor launch projections
    Given the registered launch configuration and launch seed
    Then both parse as version 0.2.0 launch projections
    And their build task commands do not invoke Nx or Bazel directly
    And authored Source helper and debugger declarations remain available

  Scenario: Preserve every root runnable capability through its build owner
    Given the root project manifest and its dashboard declarations
    Then root commands are discoverable from targets or declarations
    And no root package script is required to discover them

  Scenario: Run authored scripts through one declarative build entry
    Given an authored source or ticket script and its owning build project
    When a developer selects the script, project and working directory
    Then the dashboard resolves an argument vector through the neutral exec operation
    And arguments and environment remain explicit task inputs

  Scenario: Workspace startup never waits for Nx analytics consent
    Given the canonical Nx workspace configuration
    When a developer starts the dashboard in a real terminal
    Then analytics consent is already explicitly configured
    And no Nx consent prompt blocks the selected task
