Feature: Staffage can recover when a cached document references code removed by a release
  Scenario Outline: The first missing module after a release
    Given the document was opened on a URL with a query and a fragment
    When the <module> fails to load
    Then Staffage replaces the current history entry with a fresh recovery path that the static host serves through the archive's 404 document
    And the original query and fragment remain intact
    Examples:
      | module       |
      | entry script |
      | lazy chunk   |

  Scenario: The replacement document starts the archive
    Given the current URL uses a recovery path issued by this tab
    When the archive starts
    Then Staffage restores the original pathname without loading a document or adding a history entry

  Scenario: An unrelated 404 page starts the archive
    Given this tab has already attempted to recover another URL
    When the archive starts on an unrelated 404 path
    Then Staffage keeps the current address

  Scenario: A module fails again during the cooldown
    Given a recovery navigation began less than a minute ago
    When another entry or lazy module fails
    Then the page does not navigate again
    And the failure remains available to the page

  Scenario: Storage is unavailable
    Given session storage throws
    When an entry or lazy module fails
    Then the page does not navigate
    And the failure remains available to the page

  Scenario: An unrelated asset fails
    When an image fails to load
    Then the page does not navigate
