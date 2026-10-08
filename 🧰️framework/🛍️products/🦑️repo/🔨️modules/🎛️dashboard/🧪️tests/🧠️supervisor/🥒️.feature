Feature: The supervisor owns the processes of a workspace
  Tasks run under the daemon, not under a view. They end with an exit code, stop in escalating steps,
  take their input without losing it, and start in the environment of the view that asked.

  Scenario: A task runs to its exit code and later views and a restarted daemon replay its output
    Given a task that prints and exits with code 7
    When a second view attaches, and after a restart of the daemon a third view attaches
    Then each of them receives the output and the exit code
    And the replay of the second view starts, completes and ends in order
    And the log files answer without a daemon

  Scenario: A restart runs the task again into the same log
    Given an ended task
    When the developer restarts it
    Then a new process runs the same command
    And the output of both runs is in one log

  Scenario: A child title and a full-screen program reach every view
    Given a task that sets a title and a task that takes the alternate screen
    When a view attaches after both printed
    Then it reads the title and the screen of the program

  Scenario: A task starts in the environment of the view that asked for it
    Given two views with different environments attached to one daemon
    When the second view starts a task
    Then the task sees the second view's variables and xterm-256color as its terminal
    And it sees none of the daemon's own variables

  Scenario: Stopping escalates and killing reports the signal death
    Given a task that ends on interrupt, a task that ignores interrupts and a task to kill
    When the developer stops the first, kills the third and then stops the second
    Then an interrupted task reports 130 and a killed task 137
    And the task that ignores interrupts ends by termination after the interrupt grace

  Scenario: Terminal input is queued and delivered in full
    Given a task that counts the lines it reads
    When a view pastes two thousand lines at once
    Then the task read every line and ends
    And input for an ended task is answered with the error not_running
