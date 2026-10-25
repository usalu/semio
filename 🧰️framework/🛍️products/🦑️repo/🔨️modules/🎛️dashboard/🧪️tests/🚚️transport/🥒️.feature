Feature: Views reach the daemon of their workspace and nobody else
  The endpoint of a daemon derives from the workspace path. Reads and writes never wait, and a view is
  woken when a message arrives or its queued sends drain, so that it repaints at once.

  Scenario: The endpoint name derives from the workspace path
    Given a workspace path
    When the endpoint key is derived twice
    Then it is sixteen hexadecimal digits and the same both times
    And the hash of the empty input is the published XXH3 constant

  Scenario: A second daemon cannot take over a running workspace
    Given a daemon that serves a workspace
    When another daemon starts for the same workspace and a listener binds the same endpoint
    Then both fail
    And a view still connects to the first daemon without a difference of build

  Scenario: Daemons that start at the same moment all listen
    Given many workspaces whose sockets live in one runtime directory
    When their daemons bind their endpoints at the same moment, round after round
    Then every one of them listens and none is refused by a daemon that only inspects the directory

  Scenario: A connection wakes its owner
    Given a view with a notifier
    When a message arrives and when its queued sends drain
    Then the notifier is called for each
