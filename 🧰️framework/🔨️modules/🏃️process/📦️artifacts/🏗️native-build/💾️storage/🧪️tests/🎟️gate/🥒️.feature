Feature: A native storage reservation serves only the control that issued it and the live controls beneath it
  Scenario: The issuing control or a live control derived beneath it presents the reservation
    Given a reservation issued under one resource control of an Original
    When that control, a narrowed child, a renewable child or a grandchild presents it
    Then the reservation is accepted

  Scenario: Any other control cannot present the reservation
    Given a reservation issued under one resource control of an Original
    When a retired child, a sibling, the issuer's parent, an unrelated control or a fabricated control presents it
    Then the reservation is refused with the original issued reservation requirement
