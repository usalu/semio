Feature: Acknowledged ordinary surface read leases
  Scenario: Only an internally issued successful target can acquire an acknowledged current source
    Given an original ordinary patch receipt and exact actor lifetime
    When its real ACK installs the view and the refresh issues one target certificate
    Then only that exact response object can acquire the original source
    And a copied hash, BuiltNode, target or foreign handle confers no read authority
  Scenario: Pending publications and close retain original read owners
    Given active original node reads
    When the next view is published before its real ACK
    Then new snapshots are unavailable until that exact view is acknowledged
    And instance close revokes leases before retiring the UI owner
    And failed unsubscribe or close retains the same owner for retry
