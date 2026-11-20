Feature: Card interlock responds to changes in seam geometry
  Scenario: A translation preserves the existing interlock
    Given a settled stack of cards with a shared cap and footer band
    When the entire stack moves as one
    Then the lift and band styles are left untouched

  Scenario: A footer gains a chip without growing
    Given a settled stack of cards with a footer row
    When a chip is added to the footer
    Then the chip is measured and the shared band is resolved again
