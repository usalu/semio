Feature: The dashboard is the sole developer control plane
  Nx targets and owner declarations are the command authority. Editors and agents consume
  the same daemon through the dashboard. Package scripts only bootstrap the workspace.

  Scenario: Keep only the four root bootstrap scripts
    Given the repository root package manifest
    Then its scripts are exactly nx, setup, dashboard and dashboard:install
    And each script uses the canonical bootstrap or an owning Nx target
    And an independent JSON Schema validator reaches the same decision

  Scenario: Refuse competing editor and agent launch configuration
    Given every active source directory of the monorepo
    Then no editor or agent launch JSON or JSONC file exists
    And historical ticket inputs, dependencies and generated output are outside the active configuration scope

  Scenario: Preserve every root runnable capability through its Nx owner
    Given the root Nx manifest and its dashboard declarations
    Then root commands are discoverable from targets or declarations
    And no root package script is required to discover them

  Scenario: Run authored scripts through one declarative Nx entry
    Given an authored source or ticket script and its owning Nx project
    When a developer selects the script, project and working directory
    Then the dashboard resolves an argument vector through Nx exec
    And arguments and environment remain explicit task inputs

  Scenario: Workspace startup never waits for Nx analytics consent
    Given the canonical Nx workspace configuration
    When a developer starts the dashboard in a real terminal
    Then analytics consent is already explicitly configured
    And no Nx consent prompt blocks the selected task
