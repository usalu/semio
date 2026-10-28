Feature: Ordered IFC entity diff composition
  Entity removals retain both stable identity and the position in the original snapshot.
  Applying a composed diff must equal sequential application, including entity order.

  Scenario: Removing a later entity keeps a front insertion at the front
    Given original entity ids [1, 2]
    When entity 99 is inserted at position 0
    And entity 2 is removed at position 2
    Then sequential and composed entity ids are [99, 1]
    And the inverse restores [1, 2]

  Scenario: Annihilating a front insertion shifts a surviving later insertion
    Given original entity ids [1, 2]
    When entity 99 is inserted at position 0
    And entity 100 is inserted at position 2
    And entity 99 is removed at position 0
    Then sequential and both associative compositions have ids [1, 100, 2]
    And the inverse restores [1, 2]

  Scenario: Removal positions are checked before applying any changes
    Given original entity ids [1, 2]
    When a diff removes entity 2 at position 0
    Then the central applier rejects the diff
    And the original snapshot is unchanged

  Scenario: Removing and reinserting the same identity retains its new position
    Given original entity ids [1, 2]
    When entity 1 is removed at position 0
    And entity 1 is inserted at position 1
    Then sequential and composed entity ids are [2, 1]
    And the inverse restores [1, 2]

  Scenario: Binary diff framing is exact
    Given a binary IFC diff has format 1 and only declared field flags
    When a decoder sees format 0, an unknown flag, or trailing bytes
    Then it rejects the frame before applying any state
