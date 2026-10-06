Feature: Prepare Owned Graph Sources Before Compilation
  Scenario: Compile a component with a transitive graph owner
    Given a component whose Cargo dependency owns a graph manifest
    And the owner's generated Rust registry is absent
    When the developer starts the component through Nx
    Then the task graph schedules the owner graph generator first
    And compilation consumes that owner's generated registry

  Scenario: Compile each graph owner independently
    Given the declared graph owners and their output catalogs
    When a native build is selected for an owner
    Then its graph generator is a prerequisite in the installed Nx scheduler
