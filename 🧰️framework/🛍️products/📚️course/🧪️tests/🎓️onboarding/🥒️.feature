Feature: Overview and guided introduction
  Scenario: A first visit opens the overview
    Given a device with no learner and no completed introduction
    When the course arrives
    Then the overview is mounted beneath the shared guided introduction
    And there is no How it works card or page
    And dismissing the introduction leaves the overview open

  Scenario: Logging in asks above the overview
    Given a guest browsing the overview
    When the guest chooses to log in or start a quiz
    Then the identity question opens as a dismissible dialog above the overview
    And cancelling restores focus to the overview
    And identifying a learner returns to the overview

  Scenario: The introduction is remembered
    Given a device with a dismissed introduction
    When the application is reopened
    Then the overview opens without the introduction
    And the introduction can be replayed from navigation
