Feature: Real monorepo commands run through the dashboard
  The smoke suite starts real Nx commands of this monorepo through an isolated dashboard daemon. The daemon of
  the developer's own workspace is never touched: the suite mirrors the workspace root with junctions under a
  temporary directory, so the workspace key, the socket and the daemon cache are the suite's own.

  Scenario: A finite real Nx command runs to completion with exit code 0
    Given the mirrored real workspace with its own daemon
    When a developer runs the dashboard's own execution test target through the command line
    Then the output shows the tests passing and the verb exits 0

  Scenario: A real development server starts detached, answers and stops
    Given the mirrored real workspace with its own daemon and a free quiz port
    When a developer runs the quiz development server detached and waits for readiness
    Then the printed address answers HTTP 200
    When the developer stops the task
    Then the task is exited and the port refuses connections

  Scenario: A real native build completes with a retained task log
    Given the real workspace with an isolated named dashboard daemon
    When a developer builds the native UI package through the dashboard with the Nx cache bypassed
    Then the build completes and its task exits 0
    And the retained task log records the successful Nx build
