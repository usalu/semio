Feature: Authenticated source revision lifetime
  The canonical document hash comes from an admitted local interaction capture.
  It is a bootstrap revision, and native command admission rechecks it.

  Scenario Outline: Preserve only the current document lifetime and observation
    Given a real active local document owner and exact loaded plugin handle
    When <events> occur
    Then readiness equals <ready>
    And invalidation aborts the original observation before any late publication
    And equal hashes never restore an older owner or observation

  Scenario: Preserve only scalar identity from the completed capture
    Given a current admitted capture with a full lowercase 256 bit document revision
    When its source identity is retained
    Then later changes to the caller identity do not change the retained revision
    And interaction generation is not used as a form generation
