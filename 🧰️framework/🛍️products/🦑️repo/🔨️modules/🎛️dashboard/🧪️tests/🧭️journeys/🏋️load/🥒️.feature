Feature: The control plane holds its documented limits under load
  One workspace supports up to 128 retained sessions and 16 simultaneous views; a launcher searches tens of
  thousands of commands interactively; output bursts are flow-controlled instead of disconnecting a client.
  Scenarios tagged @cli run in the TypeScript adapter, @pty in the Rust adapter (portable-pty and vt100).

  @cli
  Scenario: One hundred and twenty-eight sessions run side by side and the next one is handled
    Given an idle command started with distinct environments
    When one hundred and twenty-eight of them run detached
    Then the daemon lists 128 running tasks and still answers within two seconds
    When a hundred and twenty-ninth is started
    Then it is refused with a message naming the limit or the oldest finished session is retired, and the daemon keeps answering
    When the daemon is stopped
    Then no idle process of the workspace remains

  @cli
  Scenario: Searching fifty thousand commands answers within the latency budget
    Given a workspace with fifty thousand declared tools
    When the registry is checked and searched by a word and by a rare prefix
    Then the check reports zero problems and every search answers within its budget with the right hits

  @cli
  Scenario: A ten megabyte output burst reaches an attached client without a disconnect
    Given a command that prints ten megabytes
    When a developer runs it attached while another client lists the tasks
    Then the attached client receives at least ten megabytes ending with the done line and exits 0
    And the other client is answered during the burst
    And the log of the finished task ends with the done line

  @pty
  Scenario: Sixteen views attach to one workspace and see the same output
    Given a running ticker
    When sixteen views attach to the workspace
    Then every view shows the ticker output and a seventeenth view is refused or tells why

  @pty
  Scenario: Typing in the launcher over fifty thousand commands repaints within the latency budget
    Given the dashboard on a workspace with fifty thousand declared tools
    When the developer opens the launcher and types a word key by key
    Then every key shows its new match count within the budget

  @pty
  Scenario: A view stays connected while a task prints ten megabytes
    Given the dashboard showing a task that prints ten megabytes
    When the burst runs to its end
    Then the view reports connected throughout and shows the done line
