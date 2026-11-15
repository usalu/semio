Feature: Native development uses the dashboard without an IDE
  Scenario: Configure each desktop compiler through Nx
    Given the six native host and build-mode presets
    When the dashboard launches the owning C++ configure target
    Then Ninja generates compiler build files
    And no IDE solution or architecture catalog is produced
    And an independent JSON Schema validator agrees with shared preset vectors

  Scenario: Activate standalone Microsoft C++ Build Tools
    Given a Windows host with the standalone compiler tools
    When a supervised native operation begins
    Then its compiler environment is activated automatically
    And command-file environment vectors agree with an independent dotenv parser
    And cancellation stops the owned command-file process tree

  Scenario: Pin the real compiler behind a hard-linked system shim
    Given a macOS host whose c++ on the path is a system shim with many hard links
    When the native setup pins its build inputs
    Then the single-link toolchain binary behind the shim is pinned
    And the toolchain c++ name stays the compiler command
    And an independent xcrun lookup agrees with the resolved toolchain
    And a host without such a toolchain is refused in English and German
