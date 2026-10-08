Feature: One workspace dashboard controls developer processes
  The daemon owns local-only sessions and persists lifecycle events. Attached views consume
  session projections and bounded output replay. Nx remains the task execution authority.

  Scenario: Start a task with its working directory and environment
    Given a workspace daemon and a command discovered from an Nx manifest or workspace script
    When a developer starts the command
    Then one process starts with the command environment and workspace directory
    And its output matches execution through the Bun reference runtime

  Scenario: Manage servers, builds and tests concurrently
    Given an Nx development server, build and test running concurrently
    When the build and test finish
    Then the server continues accepting connections and the daemon remains responsive
    And each session exposes its process identifier, status and exit code
    When the developer restarts the server
    Then a fresh process reports readiness and accepts connections on the same port
    And each independent task begins a fresh Nx invocation even when the dashboard was launched by Nx
    And workspace shutdown releases the port

  Scenario: Detach and reconnect without losing processes
    Given a running session with output
    When every dashboard view disconnects and a new view attaches
    Then the running session is restored with bounded output replay
    And connecting a second view does not start a second daemon or duplicate the command

  Scenario: Restart and cancel a task
    Given a task controlled by the daemon
    When the developer restarts it
    Then its previous process tree terminates before the same command starts again
    When the developer cancels it
    Then the task receives an interrupt and escalates to termination after a bounded grace period

  Scenario: Kill and shut down without orphan processes
    Given a task with a child development server that may create its own process group
    When the developer kills the task or shuts down the daemon
    Then the whole process tree terminates and its ports become available
    And the daemon releases its workspace instance lock

  Scenario: Validate the language-neutral protocol with an independent library
    Given the shared control-plane message vectors
    When the owned Rust codec and the Ajv reference library read the vectors
    Then both accept the valid messages and reject the invalid messages

  Scenario: Choose an accessible and customizable view
    Given a new dashboard view with no language preference
    When the developer selects English or German
    Then task controls and keyboard hints use the chosen language
    And process output and exit status remain visible
    And the developer can switch the language and appearance without restarting tasks

  Scenario: Enter through Nx with redirected standard output
    Given a native terminal and the actual Nx native command runner with streamed output
    When the developer opens the dashboard
    Then the native view renders on the owning terminal and offers a language choice
    And the view can detach without terminating the daemon or its task descendants

  Scenario: Release the launch wrapper while the daemon continues
    Given the dashboard launcher with piped output and a detached daemon
    When the launcher finishes or the view detaches
    Then its output streams reach end of file before daemon shutdown
    And the daemon keeps responding independently

  Scenario: Start a dev task while source watching initializes
    Given source graph construction while other developers keep editing
    When a dev task starts
    Then its initial invocation starts independently of watcher readiness
    When the watcher becomes ready
    Then the latest sources are activated once to reconcile startup changes
    And cancellation prevents late activation

  Scenario: Own each renderer source watcher independently
    Given React and WebGPU tasks with different activation and example bindings
    When both initialize source watching
    Then their graph stores and daemon sockets have independent stable identities
    When one task is stopped or restarted
    Then the other task retains its watcher ownership

  Scenario: Retain startup choices while component descriptors rebuild
    Given authored playgrounds whose compiled descriptors use an earlier channel
    When development catalog generation withholds those runtime components
    Then the dashboard retains the authored variants, renderers and examples
    And choosing a startup command schedules current component generation

  Scenario: Reconnect across a short named-pipe listener gap
    Given an existing workspace daemon and a listener instance gap
    When another dashboard view connects
    Then it retries the connection within its bounded transport budget
    And it attaches to the existing daemon without starting another instance

  Scenario: Select a task before the view finishes connecting
    Given a responsive native view whose daemon connection is pending
    When a developer selects a command in the searchable launcher
    Then the owned start request remains pending until the connection is ready
    And it can be cancelled without launching a process
    When the connection becomes ready for an uncancelled request
    Then the selected Nx command runs once and its actual output appears in the correctly sized pane

  Scenario: Start a task in the environment of the view that asks for it
    Given two views with different environments attached to one daemon started from a third environment
    When the second view starts a task
    Then the task sees the second view's variables and none that only the daemon has
    And its terminal is `xterm-256color` with `COLORTERM=truecolor`
    When a view starts a task without having said hello
    Then it receives an error with the code `hello_required`

  Scenario: Report a protocol or build mismatch instead of continuing silently
    Given a daemon of one build and protocol revision and a view of another
    When the view connects
    Then the view can read both revisions and both builds from the greeting
    And a differing protocol revision is refused with an error with the code `protocol`
    And a differing build is reported to the developer while the daemon keeps serving

  Scenario: Report how every task ended
    Given tasks that exit with a code, are interrupted, are terminated and are killed
    Then each session shows the exit code, `128 + signal` for a signal death and the start and end instants
    And a session whose daemon vanished while it ran is shown as interrupted after the next start

  Scenario: Take terminal input without freezing or dropping it
    Given a task that reads its input slowly
    When a view sends a large paste as input frames
    Then the daemon queues the bytes and retries until the task has taken all of them
    And a task that never takes its input yields an error with the code `input_backlog` instead of blocking the daemon

  Scenario: Escalate the stop of a task that ignores interrupts
    Given a task that ignores interrupts but ends on termination
    When the developer stops it
    Then it receives an interrupt, then a termination request after a bounded wait, then the end of its whole process tree

  Scenario: Reach the daemon only as the same user
    Given a daemon on Windows
    Then its named pipe grants access to the user it runs as, refuses remote clients and fails to create a second first instance
    And its endpoint name derives from the workspace path through a hash that does not change between toolchains

  Scenario: Control tasks without a terminal
    Given a daemon with running and ended tasks
    When a command line lists the tasks, follows the output of one, stops, restarts and kills others
    Then it uses the same sessions as the dashboard view and chooses a task by identifier, position or command identifier
    And it can start a launch and wait until the daemon reports it ready, returning the ready address
