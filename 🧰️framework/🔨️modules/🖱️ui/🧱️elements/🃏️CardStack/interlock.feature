Feature: Stacked cards abut as rectangles
  Scenario: Chips live inside each card
    Given a stack of cards with title and footer chips
    When the stack is rendered
    Then every card is one rectangle with its chips overlaid inside it
    And no lift, band or margin separates two cards

  Scenario: Heights snap to device pixels
    Given a card whose natural height falls between device pixels
    When the stack is measured
    Then the card is raised to the next whole device pixel so abutting lines coincide

  Scenario: A settled stack stays untouched
    Given a settled stack of cards
    When the window resizes without changing any card
    Then no style is written
