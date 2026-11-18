Feature: Learning Clips
  Scenario: A topic offers learning clips whose hosting is added later
    Given a course-owned clip with a poster file and no address yet
    And a clip with a hosted address in both languages or in one language only
    When the course is validated and projected for learners
    Then the posters and the addresses are retained
    And the owned validator agrees with an independent JSON Schema validator
  Scenario: Clip metadata remains bounded and safe
    Given a clip with a missing poster, a poster that is not a PNG or climbs a path, a missing translation, or an address that is not HTTPS
    When the course is validated
    Then the clip is rejected
  Scenario: Every page has one address
    Given a clip or assignment named like a quiz, a topic, a root page or another clip
    When the course is validated
    Then the course is rejected
