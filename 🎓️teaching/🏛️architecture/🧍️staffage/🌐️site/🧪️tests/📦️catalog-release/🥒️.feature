Feature: A Staffage release only ships valid catalog assets
  Scenario: An invalid authored asset reaches a release build
    Given a catalog document with a zero physical dimension
    When its asset module is loaded for a release
    Then the build rejects it and names the authored file and validation finding

  Scenario: Authors inspect an invalid asset before a release
    Given the same catalog document
    When its asset module is loaded outside a release
    Then the module carries the finding so the archive can explain why the asset is unavailable

  Scenario: A valid authored asset reaches a release build
    Given a valid catalog document
    When its asset module is loaded for a release
    Then the build retains the versioned document without findings
