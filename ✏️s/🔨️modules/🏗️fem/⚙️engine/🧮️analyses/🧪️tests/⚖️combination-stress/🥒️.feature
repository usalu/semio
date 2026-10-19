Feature: Recover Combined Element Stress From Combined Displacements
  Scenario Outline: Stress invariants describe the combined stress tensor
    Given the portable <kind> stress-recovery vector
    And two independent load cases on one constrained finite element
    When the cases are combined with signed load factors
    Then recovered normal stresses equal the combined signed stresses
    And von Mises equals the principal-stress invariant of that combined tensor
    And von Mises is nonnegative on both shell surfaces

    Examples:
      | kind  |
      | plane |
      | solid |
      | shell |

  Scenario: A combined member load retains fixed-end forces
    Given two independent uniformly distributed loads on a cantilever
    When signed factors combine both cases
    Then recovered beam stations equal a direct solve with the combined member load
