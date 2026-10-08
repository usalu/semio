Feature: The command registry is the only source of what the dashboard and the command line can start
  Runnable things are declared once, next to their owner (`metadata.semio.dashboard` of a project manifest, the
  `🎮️commands.json` of an open ticket, the generated playground catalog) or discovered from Nx. The registry offers
  each as one command with an identity, a verb, its parameters and the launch it resolves to. The interactive
  launcher and `semio commands`, `semio run` read the same registry. Everything resolves as plain argument lists;
  no launch file exists. Scenarios here run on the frozen workspace `🧫️fixtures/🎮️registry/🏗️workspace.json`; an
  independent resolver written from fleet-plan §2.2 and Ajv (the registry schema) produce the same results as the
  `semio` binary.

  Scenario: The registry lists exactly the commands its sources declare
    Given the fixture workspace with manifests, a published Nx graph, a playground catalog and tickets
    When a developer lists the registry with `semio commands --json --all`
    Then the ids equal those an independent walk of the sources derives
    And every entry validates against `RegistryEntry` of the registry schema
    And no root workspace script is a command and the commands of a closed ticket are unlisted

  Scenario: A selection resolves to the launch the declarations state
    Given a target, a tool, a group, a compound, a ticket tool and a playground with chosen parameters
    When a developer resolves each with `semio run <id> --param k=v --dry-run`
    Then the launch equals the one an independent resolver derives, requirements included
    And the launch validates against `Launch` of the registry schema

  Scenario: Global axes reach tools by verb and Nx flags stay with Nx
    Given a tool whose verb a global axis names and an axis that only changes Nx flags
    When the developer lists the tool's parameters
    Then the axis with environment effects is offered and the Nx-only axis is not
    And the Nx flags of an offered axis are ignored on the tool

  Scenario: Requirements and compound members pin parameters and environment
    Given a requirement and a compound member in object form with parameters and environment
    When the developer resolves the command
    Then the required command starts with the pinned parameters and the member environment

  Scenario: Playground facts come from the catalog row of the plugin
    Given a catalog row that declares a hub, data directories, local-only and a viewer path
    When the developer chooses a user slot, the hub, local-only and the viewer role
    Then the shell receives its slot's data directory, the hub address and the local-only switch
    And the viewer role opens the declared path

  Scenario: Free extra environment and arguments reach the launch
    Given a launch started with `--env KEY=value` and words after `--`
    When the registry resolves it
    Then every process of the launch carries the environment and the words follow `--`
    And a runner variable the dashboard owns is refused

  Scenario: A selection the declarations do not allow is refused before anything starts
    Given a missing required parameter, an unknown value, an unknown parameter and an unknown command
    When the developer asks for the launch
    Then the registry names the problem, prints nothing on stdout and exits with status 2

  Scenario: Declarations that break the schema are reported with their file
    Given a project manifest with a group that still uses the single `target` key
    When the developer proves the workspace with `semio commands --check`
    Then the problem names the manifest and the unknown field and the status is not zero

  Scenario: Two services never claim one port and the graph is whole
    Given commands of different owners that declare the same ready port, a playground renderer without its Nx target and a published graph that records plugin errors
    When the developer proves the workspace with `semio commands --check`
    Then each is a problem naming the port, the renderer target or the plugin error
    And targets and tools of one project that share a port are alternatives and are not reported

  Scenario: Semio has only the dashboard verbs
    Given a verb the dashboard does not answer, such as a route of the root script
    When the developer runs it with `semio`
    Then nothing is forwarded to another script, the usage is printed and the status is 2
    And `semio --help` prints the usage with every flag and the status is 0
