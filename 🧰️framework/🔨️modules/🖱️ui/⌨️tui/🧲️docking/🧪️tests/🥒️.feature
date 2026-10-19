@capability-tui-docking-contract
Feature: Docking Primitives Belong to the Shared Cell Model
  Scenario: Signals and frame layout use one lower docking contract
    Given a core widget docking signal and a frame layout consumer
    Then both use the core docking placement and stack corner types
    And core never depends on frame layout

  Scenario: Preview geometry agrees across implementations
    Given each tab, edge and corner placement in the neutral vectors
    When preview geometry is computed for even, odd and empty areas
    Then Rust and the independent SQLite oracle produce the declared rectangles

  Scenario: Hit selection respects corners and area boundaries
    Given a pointer in a neutral vector
    Then the shared contract selects the declared edge or corner
    And outside or undersized areas refuse a placement

  Scenario: Advertised TUI Verification Selects Its Current Crate Family
    Given the declared TUI verification package selection
    When the dashboard invokes the TUI test owner with level and Cargo arguments
    Then only those TUI crates and their native features are selected
    And the original invocation, signal, remaining deadline and issued Cargo control are retained
    And an independent TOML and SQLite oracle agrees with the package selection
