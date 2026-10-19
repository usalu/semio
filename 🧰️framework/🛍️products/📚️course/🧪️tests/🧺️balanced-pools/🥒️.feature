Feature: Balanced question pools
  Scenario: Equivalent questions rotate before repeating
    Given a pool with fixed semantic slots and four variants per slot
    When the same learner starts four attempts across challenges
    Then each sheet contains one item from every slot
    And no item appears in more than one attempt
    And every timed question allows 300 seconds

  Scenario: Numeric answers keep their individual values
    Given two items with the same correct value
    When a learner guesses different values for them
    Then the overview counts each exact answer under its own item and dimension

  Scenario: Replacement runs and reloads preserve rotation
    Given a registered learner and a balanced quiz
    When four runs start at alternating challenges and the earlier runs are voided
    Then the low seed bits advance from zero through three
    And no item repeats
    And replaying the persisted events restores the same seeds and runs
