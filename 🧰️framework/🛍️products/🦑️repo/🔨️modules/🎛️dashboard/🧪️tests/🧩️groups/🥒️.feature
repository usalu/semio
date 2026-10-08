Feature: The daemon starts a launch as a group
  A launch is the processes of one command or compound in order, preceded by the services it requires.
  The daemon owns the order: it starts or reuses the required services, waits for their readiness, then
  starts each process after the previous one is ready when that one declares readiness. Processes that
  wait for an earlier one are visible as pending sessions. A compound that stops together ends with its
  first member.

  Scenario: Start the services a command requires first
    Given a command that requires a service which declares a port
    When the developer starts the command
    Then the service starts first
    And the command stays pending until the service printed its address
    And the command then starts

  Scenario: Reuse a running service
    Given a service that already runs under the same command identifier
    When another command that requires it starts
    Then no second process of the service starts
    And the command starts once the running service is ready

  Scenario: Start the members of a compound in order
    Given a compound of a server that declares a port and a client that does not
    When the developer starts the compound
    Then the client is pending while the server starts
    And the client starts after the server printed its address
    And both sessions carry the compound's group identifier

  Scenario: Stop the members of a compound together
    Given a running compound that stops together
    When one member ends or is stopped
    Then the other members receive the stop request and end
    And pending members that never started end without a process

  Scenario: Fail the launch when a member cannot become ready
    Given a compound whose first member ends before it printed its address
    When the developer starts the compound
    Then the pending members fail with the reason in their output
    And the requesting view receives an error naming the member

  Scenario: Cancel a launch that is still starting
    Given a launch that waits for a service
    When the developer cancels it
    Then the pending members end without ever starting a process
