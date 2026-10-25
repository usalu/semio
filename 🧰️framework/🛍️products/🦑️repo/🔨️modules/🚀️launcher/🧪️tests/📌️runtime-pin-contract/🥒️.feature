Feature: Any Bun on the machine runs the repository as the pinned Bun
  Scenario: A different Bun re-runs scripts and tests under the provisioned pinned Bun
    Given the launcher registry pins one Bun and the pinned Bun is provisioned in the tools partition or the user cache
    When a script or the test runner is started with another Bun in the checkout
    Then the exact command line, including flags and arguments with spaces, runs under the pinned Bun
    And the exit code of the pinned Bun is the exit code of the start

  Scenario: A pinned Bun that is not provisioned is provisioned by the launcher or refused with the command to use
    Given the pinned Bun is not provisioned
    When another Bun starts a script or the test runner
    Then the repo launcher gets the script as a script route and the tests through tools exec, which provision the pinned Bun
    And without a launcher the start is refused with exit code 2 and the install command, in the chosen language or in every language when none is chosen

  Scenario: An install counts only when it is verified
    Given installs in the tools partition and in the user cache with and without receipt, executable bit and matching size
    When the pinned Bun is located for every host of the registry
    Then the tools partition is preferred, an install whose receipt names another archive is ignored, and an independent implementation derives the same answers
