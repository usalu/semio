Feature: Cooperative Quiz Experience
  Learners see one another and follow shared activity without changing their own work.

  Scenario: Continuous Cursors and Card Activity
    Given two identified learners use the same course in different languages
    When one learner moves between overview cards
    Then the other learner sees the cursor move continuously
    When that learner points at a card
    Then the cursor stays on that card across different viewport sizes
    And the card names the learner occupying or exploring it

  Scenario: Follow a Learner from the Footer
    Given the footer lists both online learners by name
    When a learner chooses to follow the other learner
    Then the observer sees the other learner's page and viewport
    When the other learner opens another page
    Then the observer sees that page
    When the observer stops following or presses Escape
    Then the observer's own page is restored
    And their persisted quiz answers are unchanged

  Scenario: Follow a Randomized Run
    Given the observer has an unfinished run
    And another learner has a separately randomized run
    When the observer follows that learner
    Then the observer sees the exact remote sheet order and selected task
    And their controls cannot alter the remote run
    When the learner answers, switches tasks, or submits
    Then the observer sees the updated answers, task, or results
    And the observer's persisted run is unchanged

  Scenario: Follow Target Leaves
    Given an observer follows another learner
    When the other learner disconnects
    Then the observer's own page is restored
    And the departed learner's cursor and card activity disappear

  Scenario: Follow Survives a Bounded Observer Roster Shortage
    Given an observer with saved answers follows another learner's page
    When only the observer's course roster socket disconnects for two seconds
    Then the latest observed page and the Stop Following control remain available
    And the observer's saved answers are unchanged
    When the observer's roster reconnects before ten seconds
    Then both learners are online and the same followed learner's latest page is shown
    When only the observer's roster disconnects for ten seconds
    Then the observer's own task is restored automatically
    And reconnecting does not restart following or change the saved answers

  Scenario: Follow a Phone from a Desktop
    Given a desktop learner follows a phone learner using automatic layout
    When the phone learner swipes across a wrapped column or into another row
    Then the observer sees the same settled card in swipe mode
    And the observer's own saved answers and layout choices are unchanged

  Scenario: Follow the Current Phone Card Scroll
    Given the current card overflows on both the phone and its desktop observer
    When the phone learner scrolls the card
    Then the observer sees the same normalized scroll position
    When the phone learner swipes to an unscrolled card
    Then the observer sees that card's unscrolled position
