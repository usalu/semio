Feature: Only the previews near the reader are live, within a weight budget
  Scenario: A grid with more cards in the viewport than the budget holds
    Given a grid of asset cards whose previews together weigh more than the live budget
    And every card is inside the viewport
    When the ambient clock steps to another frame
    Then the previews that moved weigh no more than the live budget together
    And at least one preview moved
    And every other preview shows a still figure that did not move

  Scenario: A grid taller than the viewport
    Given a grid of asset cards of which some are inside the viewport, some within one row of it and the rest further away
    When the ambient clock steps to another frame
    Then no preview further away than one row is live
    And every preview further away shows a still figure

  Scenario: A card further away is scrolled near
    Given a still preview further away than one row
    When its card comes within one row of the viewport
    Then the preview is live

  Scenario: A still preview is hovered or focused
    Given more cards in the viewport than the budget holds
    When the reader hovers the card of a still preview
    Then that preview is live and the live previews still weigh no more than the budget
    When the reader moves the focus into the card of another still preview
    Then that preview is live and the live previews still weigh no more than the budget

  Scenario: Seats are served by nearness and weight
    Given a live set with a budget of 100 points
    When previews of 60, 60 and 30 points inside the viewport are weighed
    Then the first and the third are live and the second rests
    And a preview heavier than the whole budget is live when it is the only one wanted
    And a released seat frees its weight for the next preview

  Scenario: A still preview holds no drawing lease
    Given previews drawn by the drawing worker and none of them wanted live
    When every still figure has arrived
    Then the drawing worker is ended
    And a preview that goes live starts it again
