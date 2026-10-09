Feature: The dashboard is the only control plane for developer processes
  Real binary, real workspace daemon, fixture workspace (🧫️fixtures/🧭️journeys). Scenarios tagged @cli run
  the non-interactive verbs of `semio` (TypeScript adapter); scenarios tagged @pty drive the interactive view
  inside a real pseudo-terminal (Rust adapter with portable-pty as the driver and vt100 as the screen model).
  Every scenario title is the name of one test in its adapter.

  @pty
  Scenario: Launcher text editing follows the visible hardware cursor
    Given a required text parameter containing wide text and a combining cluster
    When the developer moves its visible cursor, edits and pastes text
    Then the independent terminal model shows the cursor at the edited position
    And the started task receives the edited value

  @cli
  Scenario: A detached server prints its address once it is ready
    Given a fixture server command that declares a ready port
    When a developer runs it detached and waits for readiness
    Then the command prints the ready address and exits 0
    And an HTTP request to that address answers 200
    And the task list shows it running with that address
    And its log contains the line the server printed
    When the developer stops the task
    Then the task is exited and the port refuses connections

  @cli
  Scenario: A compound starts its members in order and stops them together
    Given a compound of two ready servers that stops together
    When a developer runs it detached and waits for readiness
    Then both addresses are printed and both servers answer 200
    And both tasks share one group
    When the developer stops one member with the group switch
    Then both tasks are exited and both ports refuse connections

  @cli
  Scenario: A command that requires a service starts it first and waits for it
    Given a probe command that requires a ready server
    When a developer runs the probe attached
    Then the required server is running and ready before the probe starts
    And the probe output shows the server answer and the probe exits 0

  @cli
  Scenario: Restarting a task keeps its environment and starts a new process
    Given a long-running ticker started with an extra environment value
    When the developer restarts the task
    Then a new process id replaces the old one
    And the new output carries the same environment value

  @cli
  Scenario: An attached run exits with the exit code of its task
    Given a fixture command that exits with a chosen code
    When a developer runs it attached with a code parameter
    Then the verb exits with that code
    And the task list records the code and the exited status

  @cli
  Scenario: A detached task survives its client and its log replays from the start
    Given a ticker started detached
    When the client that started it has ended and another client reads the log
    Then the log replays the first line and keeps growing

  @cli
  Scenario: Every verb refuses what it cannot do with a precise message and exit 2
    Given an unknown command id, a missing required parameter and an unknown task
    When a developer runs, stops and reads logs for them
    Then each verb exits 2 and names the problem on standard error

  @cli
  Scenario: Two workspaces never share a daemon
    Given two fixture workspaces
    When a task runs in the first
    Then the second lists no task and the first lists exactly that task

  @cli
  Scenario: A resolved launch is stable and independent of the daemon
    Given the fixture workspace and no running daemon
    When a developer runs a command with dry run twice
    Then both outputs are identical JSON and no daemon was started

  @pty
  Scenario: A task is started from the launcher, receives typed words and shows its exit
    Given the dashboard open in a pseudo-terminal on the fixture workspace
    When the developer opens the launcher with Ctrl+B n, types a search and presses Enter
    Then the task output is visible in a window titled after the task
    When the developer types words and presses Enter
    Then the task echoes them on the screen
    When the developer interrupts the task with Ctrl+B c
    Then the screen shows that the task ended

  @pty
  Scenario: A finished task shows its exit code
    Given the dashboard open in a pseudo-terminal on the fixture workspace
    When the developer starts the fixture command that exits with code 3
    Then the screen shows the exit code 3 for that task

  @pty
  Scenario: Resizing the terminal reflows the view and informs the task
    Given a running task that prints its terminal size
    When the pseudo-terminal is resized
    Then the screen is repainted for the new size
    And the task prints the new size

  @pty
  Scenario: Detaching leaves the task running and attaching restores its output
    Given a running ticker in the dashboard
    When the developer detaches with Ctrl+B d and opens a new view
    Then the task is still running with the same process id
    And the new view replays the earlier output of the task

  @pty
  Scenario: A reattached view replays a gapless run of the task output
    Given a ticker that has printed for twelve seconds before any view exists
    When a new view attaches, restores the task views and selects the task window
    Then the screen shows consecutive tick numbers without a gap

  @pty
  Scenario: Two views of one workspace show the same tasks
    Given a task started in the first view
    When a second view attaches to the same workspace
    Then the second view lists the same task

  @pty
  Scenario: The incremental screen equals a full repaint
    Given a running task in the dashboard
    When the screen is read and the terminal is resized there and back to force a full repaint
    Then both screens are equal outside the connection status

  @pty
  Scenario: A task started from the dashboard is listed by the command line
    Given a task started from the launcher
    When a developer lists the tasks with the command line
    Then the task appears with its command id and a running status

  @pty
  Scenario: The terminal driver preserves committed Unicode text
    Given the shared native console commit corpus
    When an independent raw terminal driver echoes each committed text
    Then its VT100 screen contains exactly the corpus text

  @pty
  Scenario: A configured command starts with one click
    Given a configured text parameter and keyboard focus in a text editor
    When the developer clicks the visible start action once
    Then the task starts once and receives the configured text

  @pty
  Scenario: Rendered form text remains inside its window
    Given a configuration form whose preview contains workspace paths with emoji
    When the form is painted and edited
    Then every body row stays between the window borders on the independent screen
  @pty
  Scenario: A control-space prefix arms the next key
    Given a dashboard configured with the control-space prefix
    When the terminal sends the NUL byte for control-space
    Then the footer reports that the next key is armed
    And the following new-task key opens the launcher
  @pty
  Scenario: Hover and selection are visible and select the clicked command
    Given a launcher displaying several fixture commands
    When the pointer moves over an unselected command and clicks it
    Then the independent terminal cell attributes show hover and selected states
    And activating the selection starts that command once
  @pty
  Scenario: The launcher uses all available window space
    Given a 34-row terminal displaying thirteen fixture commands
    When the parameter form is hidden and the command tree is visible
    Then all thirteen command leaves fit in the available window body
    And no invisible form reserves an empty half of the window

  @cli
  Scenario: A listening server waits for a successful HTTP response
    Given a real server listening on its declared port and returning HTTP 503
    When the dashboard reads its announced local address
    Then the task has no ready address while the service warms up
    When the same declared HTTP endpoint answers successfully
    Then the dashboard announces that address and the independent HTTP client receives HTTP 200
  @cli
  Scenario: A pending HTTP probe remains responsive and is cancelled with its task
    Given a listening server whose HTTP handler waits longer than the readiness probe budget
    When the dashboard checks readiness
    Then another control client lists tasks within the interaction budget
    When the developer stops the task
    Then the task exits and releases its port
    And a completed or cancelled probe cannot publish a ready address for that ended task