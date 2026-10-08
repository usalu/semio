Feature: Sessions outlive the daemon as events
  The daemon appends every session change to a journal and replays it at the next start. The journal is
  bounded and survives a torn last line.

  Scenario: The journal projects sessions, survives a torn line and compacts
    Given a journal of many changes of one session, a removed session and a torn last line
    When it is opened
    Then only the last state of the remaining session is restored
    And the journal is compacted to its projection

  Scenario: The journal retains a bounded number of sessions
    Given more ended sessions than the journal retains
    When it is opened
    Then the retained number of sessions is restored
    And the sessions that ended first are the ones that left

  Scenario: Sessions that were alive when no daemon ran come back interrupted
    Given a journal with running, stopping, waiting and ended sessions
    When it is read without a daemon, and when a daemon starts from it
    Then every running, stopping or waiting session is interrupted, without process and exit code
    And the ended session stays as it ended
