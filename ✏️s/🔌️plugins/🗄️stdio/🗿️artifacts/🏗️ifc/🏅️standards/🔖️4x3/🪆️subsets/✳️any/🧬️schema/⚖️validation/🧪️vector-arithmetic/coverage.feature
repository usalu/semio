Feature: Issued vector arithmetic and isolated procedural source reads
  Scenario: One public query keeps EXPRESS entity aliases local
    Given the unchanged pinned IFC4X3_ADD2 function and source declarations
    When negative scalar, vector sum and vector difference queries execute
    Then all closed arithmetic values agree with fresh generated helpers and SQLite
    And original source values cannot change a later public query
    And local entity aliases retain shared writes while aggregates retain value assignment
    And derived attributes retain local fields and complete runtime type metadata
    And returned local references retain computed fields and original identities
    And cancellation, undefined failures, object failures and work denial preserve source
    And no full-population clone or full branch coverage is claimed
