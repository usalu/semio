Feature: Reuse Complete Current Ownership Evidence
  Scenario: A settled unchanged frontier avoids rediscovery
    Given exact original positive and negative path states and directory projections
    When a custody barrier rechecks unchanged physical evidence
    Then package discovery is not rerun
    And every policy and manifest byte remains checked
  Scenario Outline: Source changes preserve the original ownership result
    Given the original controlled ownership computation
    When <change> occurs before or during recheck
    Then <outcome> retains the complete derivation as the conservative fallback
    Examples:
      | change | outcome |
      | package addition removal rename | rejection |
      | ancestor or symlink replacement | rejection |
      | preserved-mtime manifest rewrite | rejection |
      | irrelevant source or generated output churn | acceptance |
      | unsettled incomplete observation | full verification |
      | original cancellation or final-yield mutation | refusal |

  Scenario: An authenticated semantically equal topology becomes the current frontier
    Given complete required manifest bytes and a physically changed decision topology
    When a fresh derivation has the same scopes, patterns and selected packages
    And its complete frontier passes replay and the final synchronous fence
    Then the next barrier replays the adopted frontier without rediscovery
    And newly observed nonmember decision manifests remain byte-authenticated
    And cancellation or mutation in the last fresh continuation refuses adoption
