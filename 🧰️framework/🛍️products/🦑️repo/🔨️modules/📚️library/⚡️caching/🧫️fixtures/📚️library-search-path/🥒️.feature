Feature: Cargo library search paths fit the operating system loader
  Scenario: A large static dependency graph retains its procedural macro
    Given 180 source libraries and one procedural macro
    And a compiler build directory whose combined unit paths exceed 32767 characters
    When the pinned Cargo builds and runs the application with its current build directory layout
    Then every compiler and build script starts successfully
    And the procedural macro loads successfully
    And the application prints 187
    And no compiler wrapper is required

  Scenario: The current Cargo consumes the metadata policy
    Given the repository disables embedded crate metadata
    When the pinned Cargo reads its configuration
    Then the current embed-metadata setting is DoNotEmbed
    And no obsolete metadata key remains
