Feature: The dashboard is the only control plane for developer processes
  Real binary, real workspace daemon, fixture workspace (🧫️fixtures/🧭️journeys). Scenarios tagged @cli run
  the non-interactive verbs of `semio` (TypeScript adapter); scenarios tagged @pty drive the interactive view
  inside a real pseudo-terminal (Rust adapter with portable-pty as the driver and vt100 as the screen model).
  Every scenario title is the name of one test in its adapter.

  @source
  Scenario: Actual Rust journey fixture source resolves the shared authored script
    Given the actual journey crate manifest and shared source contract
    Then its Rust imports and compile-time fixture include resolve the authored files
    And its immutable fixture script agrees with the independent filesystem oracle
  @source
  Scenario: Actual Rust journey binary selection requires the owner-supplied executable
    Given the shared executable selection contract
    Then an absent or empty owner-supplied executable is refused
    And an explicit owner-supplied executable path is retained without reading an installation record

  @pty
  Scenario: A draft starts and is discarded after visible confirmation
    Given an isolated workspace without the requested draft
    When a developer fills the required draft title in its launcher form
    Then no draft exists before the visible mutation confirmation
    When confirmation is cancelled and the developer returns to confirm the same form
    Then the actual supervised task creates the exact draft directory and shows its result
    When the required discard identifier is submitted and confirmed
    Then the actual supervised task removes only that draft and shows its result

  @pty
  Scenario: A closed ticket reopens with the required durable human prompt
    Given an isolated closed ticket with its prior interaction history
    When the developer tries its visible reopen form with an empty prompt
    Then no task starts and the closed document stays byte-for-byte unchanged
    When the exact human prompt is entered and the mutation is confirmed
    Then the supervised task shows the reopened ticket and exits successfully
    And independent JSON parsing proves open status, retained history and the exact new prompt
    And the important document is restored as an empty file

  @pty
  Scenario: The terminal driver reports physical emoji widths
    Given portable scalar and cluster width samples in an isolated Windows pseudo-terminal
    When the immutable fixture writes each sample and queries its actual console cursor through the system API
    Then the third-party VT100 model independently records the same emitted terminal bytes and cursor
    And both physical and model cursor facts are retained without substituting a terminal query reply

  @pty
  Scenario: A workspace terminal streams shell input and exits inside the dashboard
    Given an isolated workspace and the immutable production developer terminal operation
    When its visible parameter form selects the platform shell and workspace directory
    And the developer submits echo SEMIO_TERMINAL_PROBE and exit through the task view
    Then the independent terminal shows the probe and the task ends with exit code 0
    And its raw process log retains exactly SEMIO_TERMINAL_PROBE before a newline or terminal cursor movement
    And an independent durable JSON receipt confirms the untouched parameters and isolated root

  @pty
  Scenario: Tabs occupy every corner and own fullscreen and close controls
    Given the dashboard with a task overview and an inactive command tab
    When a tab is dragged to all four corners
    Then the independent terminal model shows every corner slot preview and committed tab position
    And each three-row preview chip keeps its controls and matches the committed chip and seam exactly
    When its own fullscreen and restore controls are clicked
    Then its body receives focus without creating another window
    And changing tabs within fullscreen changes the visible body while retaining fullscreen
    When its close control is clicked
    Then only that tab disappears and the overview remains

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
  @pty
  Scenario: Tabs dock with a preview and Escape cancels the drag
    Given the real dashboard with an overview and a launcher tab
    When a tab is held over the right edge of its group
    Then the independent VT100 screen shows the exact proposed destination tab and its controls
    When Escape cancels the gesture
    Then both tabs remain in their original group
    When the same gesture is released at the right edge
    Then two independently tiled windows appear
    And the committed destination chip and seam match the proposed glyphs at the same coordinates
    And a held divider follows repeated pointer movements across rendered frames
    And the incremental frame agrees with a full repaint

  @pty
  Scenario: A live server moves between split axes and tabs without restarting
    Given a running server with a terminal window
    When its tab is dragged beside and above the task overview and back into its tabs
    Then the independent terminal screen shows each preview and committed arrangement
    And the daemon retains exactly one task with the same session and process identity
    And the original server output remains in its logs

  @pty
  Scenario: A German form submits choices flags quoted arguments and environment
    Given a real German dashboard and a required text choice and flag form
    When the developer types Unicode text chooses beta enables the flag and adds quoted arguments and environment
    Then the independent terminal shows localized required parameters and the start action
    And exactly one process receives the portable fixture output

  @pty
  Scenario: Settings changes persist when the view is reopened
    Given an isolated dashboard with local preference events
    When the developer changes language and appearance in the visible settings pane
    Then the independent terminal shows German light preferences and the durable journal records both events
    When the developer detaches and opens another view without preference overrides
    Then the visible settings remain German and light

  @pty
  Scenario: A ready server opens its address restarts and stops from the view
    Given a running ready server in the real dashboard
    When the developer activates its ready address
    Then an independent recording opener receives exactly its declared health address
    When another view opens with customized address and cancellation keys and starts a slow opener
    Then cancelling from the visible view ends only that opener and the server remains ready
    When the developer restarts the task through the visible view control
    Then the same task has a new process and its declared endpoint answers HTTP 200
    When the developer stops it through the visible view control
    Then the screen shows its end and its port refuses connections

  @pty
  Scenario: A read only repository action runs from the launcher
    Given one goal document in an isolated repository
    When the developer selects the repository goal list action and submits its visible form
    Then the independent terminal shows the exact goal line and successful exit
    And the goal document bytes are unchanged

  @pty
  Scenario: Native files browse edit Unicode save reopen conflict and discard without an external editor
    Given a real file in a nested isolated workspace directory
    When the developer browses it with the native Files action and filters the directory and file names
    And edits combining text and an emoji using the visible multiline hardware cursor
    And saves with the configurable native document action
    Then an independent filesystem read contains the exact Unicode document
    When a foreign writer replaces the file and the developer attempts another save
    Then the native view shows a conflict and preserves both the foreign file and local buffer
    And closing the dirty document retains it until explicit discard
    When the developer discards and reopens the file
    Then the native editor shows the foreign writer's document

  @pty
  Scenario: German native files use customized controls and retain unsaved changes on close
    Given a German dashboard with customized file and save keys
    When the developer browses and edits a real UTF8 file
    Then the native view shows German file status and follows the configured save key
    And closing unsaved changes requests explicit localized discard

  @pty
  Scenario: An unresolved language has no preselection and cancels without a preference event
    Given no supported system or explicit language and no stored choice
    Then English and Deutsch appear without an active selection
    And Enter publishes no event before a choice is selected
    When the developer selects Deutsch and cancels before confirmation
    Then the view exits and no preference journal exists

  @pty
  Scenario: An explicit language choice reopens from its durable event
    Given no supported system or explicit language and no stored choice
    When the developer explicitly selects and confirms Deutsch
    Then the real dashboard displays German controls and commits one language event
    And reopening with the same unresolved environment uses that event without prompting again

  @pty
  Scenario: Maximum native document stays responsive
    Given a 1 MiB Unicode document inside a dedicated directory of 4096 entries
    When I browse and load it within the authored 5000 millisecond budgets
    Then end navigation and Unicode editing repaint within 1000 milliseconds each
    And both viewport directions stay responsive
    And closing the dirty document retains its buffer and original disk bytes
    And returning to editing cancels the pending close rather than claiming worker cancellation
