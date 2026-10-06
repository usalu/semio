Feature: Browser Worker Continuation Source Ownership
  Scenario: SQLite cooperates through the host continuation scheduler
    Given the SQLite worker imports the first-party host continuation scheduler
    When the browser worker is generated from its declared source inventory
    Then the continuation scheduler is admitted as a schema-owned source
    And the generator input contract declares that same source
    And Nx includes the source in the frame worker cache inputs
    And an independent browser bundler admits the same dependency
