Feature: Original native declared attribute query
  Scenario: The original receiving owner drives the shared query kernel
    Given an admitted native source borrow and original native decoding owner
    When qualified metadata and serialized slots are queried
    Then exact borrowed declarations and source addresses are scoped to the owned callback
    And missing, unique, ambiguous and late unknown remain distinct
  Scenario: Every interrupted prefix remains originally owned
    Given a finite original five-axis grant and installed retirement recipient
    When cancellation or a source or output failure interrupts the query
    Then actual accepted metadata progress remains in the original wallet
    And partial query and output owners retire before publication under original paid close
