Feature: A view that attaches sees what the task printed
  The daemon keeps the recent output of every session in memory and a longer tail in rotating log files
  under the dashboard cache. A view that attaches is sent each session's output from a line it can start
  at, preceded by the terminal modes the earlier output had established, then continues live. A view that
  falls behind is never disconnected: it is sent what it can still read, or a fresh replay.

  Scenario: Replay starts at a line and ends at the present
    Given a session that printed more than the memory ring holds
    When a view attaches
    Then it receives a replay-start message, bytes that begin at a line, a replay-complete message and then live output
    And the bytes it received are the tail of what the task printed

  Scenario: Replay restores the terminal modes
    Given a task that hid the cursor, enabled bracketed paste and mouse tracking and set colours, a scroll region and a title
    When a view attaches long after those sequences were printed
    Then the replay begins with the sequences that re-establish those modes
    And the modes after the replay equal the modes after the whole output

  Scenario: Rebuild a full-screen program from its first frame
    Given a program that took the alternate screen and kept redrawing it
    When a view attaches
    Then the replay starts where the program took the screen

  Scenario: Keep scrollback in log files
    Given a session whose output is larger than the memory ring
    Then the log files hold the output beyond the ring, the current file and the one before it
    And an ended session keeps answering from its files after the daemon restarted
    And `semio logs` reads the same files without a daemon

  Scenario: Never disconnect a stalled view
    Given a view that does not read at all and a task that prints megabytes
    When the task finishes
    Then the stalled view is still connected
    And a second view received everything at line rate
    And the stalled view receives a replay of the recent output when it reads again

  Scenario: List many sessions to a fresh view
    Given more than a hundred retained sessions and a stalled view
    When a fresh view attaches
    Then it receives every session in pages, a replay for each and a final replay-complete message

  Scenario: Restart appends to the log from clean terminal modes
    Given a session that left the alternate screen on when it ended
    When the developer restarts it
    Then the log continues with the sequences that restore the terminal and a new line
