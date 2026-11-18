Feature: Assignments and Learning Clips on the Home Overview
  Scenario: A course with assignments and clips has two more general cards
    Given a course with assignments and learning clips
    When the overview lists its general pages
    Then assignments and clips follow the quizzes, and a course without them shows its five cards as before
    And no list of assignments stands above the overview
  Scenario: An assignment shows its PDF in the learner's language
    Given the assignments page is open
    When the learner opens an assignment
    Then its PDF of the learner's language is shown inside the page
    And the page offers a download of that PDF, a tab of its own and the way back
    And no PDF loads before the page is opened
  Scenario: A clip shows its poster and its hosting state
    Given the clips page is open
    Then every clip shows its poster
    And a clip with an address for the learner's language links to that site in a tab of its own
    And a clip without one says it is coming soon
