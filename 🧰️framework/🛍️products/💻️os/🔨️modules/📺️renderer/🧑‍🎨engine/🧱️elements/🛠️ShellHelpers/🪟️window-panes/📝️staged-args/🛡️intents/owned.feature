Feature: Original Owned Staged Controls
  Scenario Outline: Synchronously Consume A Current Issued Control
    Given a status and ordinary control issued by the same real surface
    When a language neutral ownership case changes the read authority
    Then only the exact current issued read may edit host owned arguments
    And an old active read cannot authorize an event after publication changes
    And resident arguments never escape to an asynchronous intent sink

  Scenario: Preserve An Incomplete Text Draft While The Query Is Pending
    Given a current text preserving control and ready status
    When the user enters two incomplete text drafts
    Then the second host owned draft is displayed immediately
    And execution remains unavailable until a current ready status is committed
