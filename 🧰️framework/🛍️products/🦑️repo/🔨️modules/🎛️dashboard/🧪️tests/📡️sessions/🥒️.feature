Feature: The view stays usable while the daemon is away
  The view connects in the background, keeps what the developer started while it is not connected,
  restores what the daemon replays and tells the developer in their language what went wrong.

  Scenario: A selection before the connection is an owned, cancellable start
    Given a view whose daemon connection is pending
    When the developer selects a command in the launcher and then cancels
    Then the start is kept as a waiting task and the cancel ends it without a process
    And nothing remains queued

  Scenario: The limit is checked before a window exists and a failed send is kept
    Given a full queue of waiting messages, and separately a connection whose send fails
    When the developer starts a command
    Then with the full queue no window and no session is created and the launcher stays
    And with the failing send the start is queued again and the developer is told

  Scenario: Reconnecting backs off and starts over after a success
    Given a lost connection
    When the view schedules one attempt after another
    Then each wait is at least the one before and none exceeds eight seconds
    And after a success the wait starts at half a second again

  Scenario: Restored output and exit status render without stealing the focus
    Given an ended task that the daemon restores
    When the view shows it
    Then its output and its exit code are shown
    And the keyboard stays where it was

  Scenario: A replay clears only the terminal it replays
    Given two running tasks with output
    When the daemon replays one of them
    Then only that terminal is cleared and the view tells that the replay is restoring

  Scenario: Daemon errors reach the user in their language and never the terminal
    Given the preferences language en and language de
    When the daemon answers with an error code
    Then the notice names the error in that language

  Scenario: Shutdown waits for the confirmation of the daemon
    Given a running dashboard
    When the developer asks for shutting everything down
    Then the request is sent and the view ends only with the confirmation of the daemon
