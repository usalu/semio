Feature: Real monorepo commands run through the dashboard
  The smoke suite starts real build-runner commands of this monorepo through an isolated named dashboard daemon.
  It uses the actual workspace. Its instance name separates the workspace key, socket and daemon state from
  the developer's daemon, and the suite verifies that separation before starting tasks.

  Scenario: A finite real command runs to completion with exit code 0
    Given the real workspace with an isolated named dashboard daemon
    When a developer runs the dashboard's own execution test target through the command line
    Then the output shows the tests passing and the verb exits 0

  Scenario: A real development server starts detached, answers and stops
    Given the real workspace with an isolated named dashboard daemon and a free course port
    When a developer runs the architecture course development server detached and waits for readiness
    Then the printed address answers HTTP 200
    When the developer stops the task
    Then the task is exited and the port refuses connections

  Scenario: A real native build completes with a retained task log
    Given the real workspace with an isolated named dashboard daemon
    When a developer builds the native UI package through the dashboard with the build cache bypassed
    Then the build completes and its task exits 0
    And the retained task log records the successful build
