Feature: Every declared subcommand of repo tools reaches its implementation
  Scenario: The catalogue and the build entry agree on the subcommands of tools
    Given the operations catalogue declares the subcommands of tools
    When the build entry dispatches repo tools
    Then setup, verify, list and exec each have a dispatch site that runs their implementation
    And a subcommand without a dispatch site would fall into the generic usage refusal
