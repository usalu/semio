Feature: A Whole Class Learns While Everything Fails
  About a hundred learners at once, some in real browsers and the rest speaking the site's own protocol, play every quiz while the network and the proctor misbehave.

  Scenario: Every Quiz Is Played to a Perfect Score Under Chaos
    Given real browser learners and protocol learners together make about a hundred simultaneous learners
    When every browser learner plays every quiz of the course end to end
    And the network of a learner drops and returns in the middle of an answer or a submission
    And pages reload, tabs close and reopen, and the requests of the page slow down
    And the proctor is killed and started again while the class is playing
    Then no browser shows a page error or a failed request outside an announced shortage
    And the connection indicator shows the shortage and recovers to saved
    And an answer given offline waits on the device and reaches the proctor after the reconnect

  Scenario: Every Submission Is Scored Exactly Once
    Given commands are lost before they arrive, answers are lost after they were decided, and commands are sent twice
    When all learners are done
    Then the proctor holds exactly one submitted run per learner and quiz
    And every learner's total is the points of the challenges played, read from the proctor itself

  Scenario: The Leaderboard and Presence Converge
    Given all learners are done and still online
    Then the leaderboard ranks everyone with the right total, in the proctor and on the leaderboard page of every browser
    And every browser counts all learners online
    When the protocol learners leave
    Then every browser counts only the browsers online

  Scenario: A Hundred Real Browsers Attend at Once
    Given a hundred real browser learners and no protocol learner, each with pets and icon animations switched off through the preferences
    When the class arrives over a couple of minutes and every browser plays one quiz, spread evenly over the course
    Then every browser stays connected and presence counts a hundred online
    And no browser shows a page error, every submission is scored exactly once and the leaderboard is right
    And the time to the first task and to the results of a submission are measured
