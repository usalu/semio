Feature: Card grid on a short desktop or tablet
  A phone swipes one card at a time. A desktop or tablet keeps the card grid and scrolls it when the rows are taller than the window.

  Scenario: The overview stays a grid in a short window
    Given a desktop viewport of 1280 by 400
    When the archive overview is shown
    Then the overview is in strip mode
    And its cards sit in the staffage grid

  Scenario: A category of 24 assets stays a grid
    Given a desktop viewport of 1280 by 400
    And a category holding 24 assets
    When that category is opened
    Then the category is in strip mode
    And each row is at least 190 pixels tall
    And the 24 cards sit in the staffage grid

  Scenario: A phone swipes
    Given a viewport no wider than 767 pixels
    When the overview or a category is shown
    Then that level is in swipe mode
