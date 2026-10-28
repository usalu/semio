Feature: Original raw identity drafts
  Scenario: Incomplete identity text remains editable without numeric interpretation
    Given released entry and target fields with blank or partial identity text
    When the native original query reaches the planner IdentitySet frontier
    Then source controls retain the exact original identity text and paths
    And missing planner identities require explicit user input with no generated identifier
    And ready is false with a bounded bilingual identity diagnostic
    And full identity decoding resumes only after every draft is canonical and complete
    And cancellation closes the same original request and query owners
