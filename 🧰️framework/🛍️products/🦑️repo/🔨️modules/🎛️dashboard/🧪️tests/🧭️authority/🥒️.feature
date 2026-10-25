Feature: Developer surfaces share neutral command authority
  Build targets and owner declarations are the command authority. The dashboard
  dispatches tasks through the repository build interface and editors retain source navigation.

  Scenario: Preserve canonical bootstraps and neutral task aliases
    Given the repository root package manifest
    Then repo, dashboard and dashboard:install retain their canonical commands
    And additional aliases select literal targets through repo run
    And direct backend commands and shell composition are refused
    And an independent JSON Schema validator reaches the same decision

  Scenario: Refuse all current editor launch authority
    Given current root and nested editor paths and instructions
    Then executable editor launch files and seeds are absent
    And dashboard declarations preserve typed owner commands
    And independent JSON Schema validation reaches the same decision

  Scenario: Active inventory fails closed and preserves authored source traversal
    Given a caller-owned fixture tree and an explicit generated-root policy
    Then authored build, dist, target and nested cache directories remain inspected
    And only declared generated roots and dependency boundaries are excluded
    And a traversal failure publishes no partial cache
    And a second successful call preserves the complete inventory
    And missing or broadened inventory policies are refused by independent JSON Schema validation

  Scenario: Root agent instructions name the dashboard as the human surface
    Given the root AGENTS document
    Then it states once that all human devs are using the repo dashboard
    And only that exact sentence at that exact path is modeled
    And mutable README and nested agent instructions are not modeled
    And the whole document validates without any editor exemption

  Scenario: Preserve every root runnable capability through its build owner
    Given the root project manifest and its dashboard declarations
    Then root commands are discoverable from targets or declarations
    And no root package script is required to discover them

  Scenario: Every active owner resolves executable commands through canonical scripts
    Given every active owner build manifest and configuration override
    Then each runnable target invokes Bun with an existing permanent canonical script and explicit command
    And dependency-only repository noop targets remain valid
    And shell pipelines, substitutions, direct compiler commands and direct temporary ticket probes are refused
    And each dashboard declaration validates against the canonical registry schema
    And independent JSON Schema validation agrees with the shared owner command and target vectors

  Scenario: Active developer documents retain dashboard-only instructions
    Given every active README, contributing guide, development guide and agent instruction file
    Then no document prescribes retired IDE configuration or startup
    And shared instruction vectors allow compiler configuration and historical planning evidence
    And independent JSON Schema validation agrees with every instruction decision

  Scenario: Container lifecycle belongs to dashboard build owners
    Given the development runtime container
    Then create, start and attach invoke their workspace build owners
    And no IDE extension, settings customization or IDE configuration volume is provisioned
    And compiler toolchains and retained headless agent volumes remain valid
    And independent JSON Schema validation agrees with every shared container vector

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

  Scenario: Editors use the dashboard registry
    Given runnable commands are declared in owning build manifests or open ticket declarations
    When a developer selects a command in the dashboard
    Then its parameters and dependencies are resolved by the native registry
    And one workspace daemon retains its identity and output
    And no active editor or agent launch catalog exists
    And no dashboard route generates an editor launch projection

  Scenario: Derive configuration only for retained agent clients
    Given MCP servers are declared by their owning dashboard tools
    When the workspace derives agent configuration
    Then only Claude Code and Codex are accepted clients
    And native validation and an independent JSON Schema validator agree
    And no IDE tasks, launches or MCP files are generated

  Scenario: Own the course and proctor compound beside the course project
    Given the architecture course owner declares course-with-proctor
    When the developer starts the compound
    Then the proctor becomes ready before the course starts
    And the course uses its declared HTTP readiness endpoint
    And cancelling either member stops both

  Scenario: A task owns its original process invocation
    Given a requesting view has its own process policy, capabilities and deadline
    When it starts a separately supervised dashboard task
    Then the view's process authority is not inherited by that task
    And explicitly selected task environment is retained
    And the task bootstrap creates authority for its actual selected command

  Scenario: Original receiving and compiler controls remain executable
    Given the original receiving native test is declared by the OS owner
    And Cargo synchronization is declared by its workspace owner
    When editor launch catalogs are removed
    Then both commands remain discoverable as build target identities
    And their current execution controls are provided by the build caller

  Scenario: Bazel phases belong to typed dashboard parameters
    Given the current build owner declares every allowed integration phase
    Then all nine choice effects dispatch their literal phase arguments to the actual permanent owner
    And an unselected phase retains the complete suite
    And both English and German metadata remain present
    And unknown, composed and multiple phase selections are refused
    And independent JSON Schema validation reaches the same phase decisions
    And native dry-run vectors retain each explicit dashboard phase

  Scenario: Go and TypeScript enforce the same active source authority
    Given the shared normalized and Windows path vectors and explicit inventory roots
    Then both implementations refuse root and nested IDE configurations
    And authored build, dist, target and nested cache directories are inspected
    And missing inventory roots fail closed
    And the Go toolchain executes both path forms and two inventory scans

  Scenario: A daemon socket path always fits the platform
    Given a runtime directory of any depth and the sun_path capacity of macOS and Linux
    When the socket path of a daemon is derived
    Then it is the runtime path while that fits and otherwise a digest named socket in the short per-user directory
    And an independent xxh3 names the same digest file
