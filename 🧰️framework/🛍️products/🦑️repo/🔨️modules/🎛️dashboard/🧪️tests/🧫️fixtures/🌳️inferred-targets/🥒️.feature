Feature: Complete workspace command discovery
  Scenario: Nx infers a component materialization command
    Given the current workspace graph includes an inferred target
    And the owner has no authored project target for it
    When the dashboard discovers commands
    Then the target is selectable through its domain path
    And its command invokes the exact Nx project and target

  Scenario: A concurrent Nx command publishes its graph
    Given the graph file temporarily contains an incomplete document
    When Nx finishes publishing the current graph
    Then command discovery retains its inferred build and materialization targets
