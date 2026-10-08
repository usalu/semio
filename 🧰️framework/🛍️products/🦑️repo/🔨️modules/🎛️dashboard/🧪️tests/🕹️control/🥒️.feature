Feature: Tasks are controlled without a terminal
  `semio run`, `tasks` and `logs` use the same sessions as the dashboard view: they start a launch and
  wait for it, list and act on tasks, and read the log files with or without a daemon.

  Scenario: A launch starts services first and returns its ready address
    Given a compound with a required service, a server that declares a port and a client
    When the launch runs and waits until it is ready
    Then the sessions start in order and the launch returns the ready address of the server
    And a stop of the server ends the group together
    And a service that already runs is reused by another launch

  Scenario: A member that ends before it is ready fails the run
    Given a compound whose first member ends before it printed its address
    When the launch runs and waits until it is ready
    Then it fails naming that member
    And the members after it fail too

  Scenario: A cancelled run starts nothing more
    Given a launch that waits for a service
    When the run is cancelled
    Then the run ends as cancelled and the members that still wait never start

  Scenario: The log files are read without a daemon
    Given an ended task and no running daemon
    When the developer asks for its logs
    Then the log files are written out
