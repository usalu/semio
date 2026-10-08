Feature: The command registry covers the complete monorepo
  The dashboard is the only control plane, so every runnable thing of the real workspace must be a registry
  command that resolves. The Rust registry (`semio commands`, `semio run --dry-run`) is compared with an
  independent TypeScript oracle that reads the Nx project graph, the manifests, the generated playground
  catalog and the open tickets itself; Ajv validates every declaration against the registry schema.

  Scenario: Every Nx target, playground variant, declaration and open ticket command is listed
    Given the project graph Nx published, the project manifests, the playground catalog and the open tickets
    When the registry is listed with every command included
    Then each target, variant, tool, compound, group and ticket command of the sources is listed
    And no listed command exists in no source

  Scenario: The published project graph has no errors and no fixture manifest is a project
    Given the project graph Nx published
    When its errors and the roots of its projects are read
    Then there are no errors and no project lives in a fixtures folder

  Scenario: The workspace check reports zero problems
    Given the whole workspace
    When the registry checks every declaration, reference and default resolution
    Then it reports zero problems and exits 0

  Scenario: Every dashboard declaration of every project manifest and ticket document is valid
    Given every dashboard block of every project manifest and every open ticket command document
    When Ajv validates them against the registry schema
    Then all of them are valid

  Scenario: Every declared parameter, ready port, requirement, compound member and group names something that exists
    Given the declarations of the workspace
    When each reference is looked up in the sources
    Then no reference dangles

  Scenario: Every target configuration resolves through its colon id
    Given the Nx targets that define configurations
    When a configuration is run in a dry run by its project:target:configuration id
    Then it resolves and the launch names the configuration

  Scenario: Every command resolves in a dry run with the launch the oracle states
    Given every non-trivial command and a stable sample of the plain targets
    When each is run with the dry run switch
    Then it resolves, every Nx target it launches exists in the graph
    And its command, arguments, environment, working directory, ready address and requirements equal the oracle's

  Scenario: Every ready port is unique among commands that can run together or is documented as shared
    Given the ready ports of every command
    When they are grouped by port
    Then every port belongs to one command or to a group documented as sharing it
