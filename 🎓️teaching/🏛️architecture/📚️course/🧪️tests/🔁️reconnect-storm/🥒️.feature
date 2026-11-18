Feature: A Page Recovers After Every Proctor Restart
  Ten real browser learners stay signed in while the one proctor of the stack is killed and started again, the network drops and tabs hide, and every page must find its way back on its own.

  Scenario: Presence and the Connection Indicator Return After Every Kill
    Given ten browser learners are signed in, some in a run, some on the overview and some on the leaderboard
    When the proctor is killed and started again with down-times from a tenth of a second to ninety seconds
    And the proctor is frozen with its sockets open, then thawed, or killed while frozen
    And the proctor is killed while a presence socket handshakes
    And the proctor is killed while a command is in flight
    And the network of some learners drops and returns before, during and after the kill
    And tabs go hidden and visible around the kill
    Then every page shows its presence footer with everyone online within a bound derived from the backoff cap
    And every connection indicator is calm again within the same bound
    And only one proctor process ever owns the port

  Scenario: A Submission Cut Off by a Kill Is Delivered Once the Proctor Is Back
    Given a learner has answered every task of a run
    When the proctor is killed while the submission is in flight and started again
    Then the learner reaches the results of the run
