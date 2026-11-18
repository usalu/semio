Feature: A tab opened before a redeploy recovers from chunks the CDN no longer serves
  Scenario: First failed chunk load
    Given a page whose dynamic import of a hashed chunk fails
    Then the page reloads once to fetch the current document and its chunks, and the failure is not reported further
  Scenario: Repeated failure right after that reload
    Given the page reloaded for a failed chunk less than a minute ago
    When a dynamic import fails again
    Then the page does not reload again and the failure reaches the page's own retry
  Scenario: Storage is unavailable
    Given session storage throws
    When a dynamic import fails
    Then the page does not reload and the failure reaches the page's own retry
