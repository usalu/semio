Feature: Card grid on a short desktop or tablet
  A phone scrolls a list of links. A desktop or tablet keeps the layered card grid.

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

  Scenario: Phone navigation uses native lists
    Given a 390-pixel viewport and a catalog with 24 human assets
    When the archive opens
    Then each archive destination has its own link without loading category documents
    When the reader follows the humans link
    Then all 24 assets are reachable in one scrolling list, without loading their drawings
    And search is visible while the secondary controls start collapsed
    When the reader follows the link to the last asset
    Then the detail opens and requests that asset alone
    When the reader goes up to the category
    Then the category list resumes at the previous scroll position

  Scenario: Landscape touch navigation
    Given a coarse pointer and an 844 by 390 viewport
    When the archive opens
    Then the phone list remains usable by vertical scrolling

  Scenario: Phone asset loading can be cancelled and retried
    Given a phone opened directly on an asset address
    And the selected asset request has not finished
    When the reader cancels the request and tries again
    Then the original request is cancelled
    And the archive starts a new request for the selected asset only

  Scenario: Phone asset load failure has one selected-asset retry
    Given the selected asset has a temporary loading failure
    Then the phone detail shows one retry control
    When the reader tries again
    Then only the selected asset is requested again

  Scenario: The phone opens the all-assets page without eager categories
    Given a 390-pixel viewport
    When the all-assets page opens
    Then all 24 assets are reachable without loading any category document
    When the reader searches by title, tag or summary
    Then the archive loads the categories needed to complete the search

  Scenario: The all-assets pane behind the glass shows its visible cards
    Given a catalog with 24 assets and two cells intersecting the inert preview
    Then the preview reserves 24 cells and mounts two cards
    When the all-assets page opens
    Then all 24 cards are accessible

  Scenario: The featured categories load independently
    Given the featured frieze needs two authored categories
    When the first category finishes while the second load is pending
    Then the second category keeps its original load

  Scenario: The overview requests the assets its cards show
    Given a desktop overview of a category with seven authored assets, a category with one, a planned asset and one featured asset
    When every card has had the time it takes to warm
    Then the four assets of the first frieze, the one of the second and the featured one were requested
    And no category document and no planned asset was requested

  Scenario: A desktop asset page requests that asset alone
    Given a desktop viewport of 1280 by 800 opened directly on an asset address
    When the overview cards behind the page have had the time they take to warm their categories
    Then the selected asset was requested once and no category document was requested
    When the reader goes up to the category
    Then the category grid requests its category

  Scenario: A loading asset says that it is loading, in the reader's language
    Given a desktop opened directly on an asset address whose request has not finished
    Then a live region says in English that the asset is loading and nothing calls it planned
    And a German reader is told the same in German
    When the request finishes without a document
    Then the page says the asset is planned and not authored yet

  Scenario: An asset is shown while its findings are still computed
    Given a desktop opened on an authored asset whose check has not finished
    Then the viewer shows the asset and a live region says how far the check is
    When the check ends with a finding
    Then the page lists the finding above the viewer, which keeps showing the asset

