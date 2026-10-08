Feature: The command line takes arguments, starts launches and addresses every task by one stable handle
  `semio commands`, `run`, `tasks`, `logs`, `stop`, `restart`, `kill` and `open` read their arguments the same way,
  hand a resolved launch to the workspace daemon and print and accept the session id of a task as its one handle.
  The Rust scenarios are unit tests of `🧭️cli`; the journey runs against an isolated daemon
  (`SEMIO_DASHBOARD_INSTANCE`) of a temporary workspace in `🟦️.ts`.

  Scenario: Flags, bare words and the tail after the double dash are told apart
    Given an argument list with valued flags, bare switches, words and a tail after `--`
    When a verb reads it
    Then valued flags take the next word or the text after `=`, switches take none and the tail is kept verbatim
    And a malformed flag is refused with its name

  Scenario: A request carries parameters, extra environment and arguments
    Given `--param`, `--env` and words after `--`
    When the verb builds the registry request
    Then each is in its place and a bare flag id switches the flag on
    And a bare choice or text id is refused with the way to write it

  Scenario: A launch becomes the group message the daemon starts
    Given a resolved launch with required services and several processes
    When the command line prepares the start
    Then the daemon gets the services first, then the processes, in one group with the launch's stop policy

  Scenario: A running task is the launch asked for when command words and parameters agree
    Given a task that runs with the same command id, words and chosen parameters
    When the same launch is asked for again
    Then the running task is reused whatever the daemon normalised in its directory and environment

  Scenario: Control sequences are dropped from the text of a log
    Given output that is not a terminal
    When a task's output is printed
    Then control sequences are removed, also across chunk borders, and the text stays unchanged

  Scenario: The handle printed by run is the handle of tasks, logs, open and stop
    Given a long-running command that declares a ready address
    When a developer runs it with `--detach --wait-ready`
    Then the first line carries the session id and the ready address is the last line alone
    And `tasks`, `logs`, `open` and `stop` find the same session by that id

  Scenario: A command id or a group id names the live session
    Given the task is running
    When the developer gives its command id or the group part of its id to `logs`, `open` or `stop`
    Then the verb works on the live session
    And a command without a live session is refused by `stop` with the latest session named

  Scenario: A position in a listing is no handle
    Given several tasks have run
    When the developer gives a bare number to `logs`
    Then the verb answers that no task matches and does not address another task

  Scenario: Running the same command again reuses the live task
    Given the task is running
    When the developer runs the same command and parameters again
    Then the same handle is printed and no second process starts
