Feature: Localized Printable Assignments
  Scenario: A topic offers a printable assignment in each course language
    Given a course-owned assignment with English and German PDF artifacts and explicit build producers
    When the course is validated and projected for learners
    Then both localized documents and the topic association are retained
    And the owned validator agrees with an independent JSON Schema validator
  Scenario: Assignment metadata remains bounded and safe
    Given an assignment with a missing translation, invalid producer, template namespace, non-PDF filename or a parent path
    When the course is validated
    Then the assignment is rejected
