Feature: Additional unchanged original ADD2 Certification Simple-Scene corpus
  Scenario: Original source identities and license remain exact
    Given ten pinned original IFC4X3_ADD2 files and the source CC-BY4.0 notice
    When file bytes, tree identities and original headers are admitted
    Then all hashes and byte/entity counts match independent observations
    And the existing46 examples remain a separate corpus

  Scenario: Central editing and independent EXPRESS outcomes are exercised
    Given an unchanged original source and its actual queried Application.Version slot
    When STEP roundtrip and central edit/diff/inverse laws run
    Then the original graph is restored exactly
    When the full EXPRESS implementation is compared with fresh external observations
    Then actual errors or incomplete work remain explicit
    And no full view, geometry or native admission is inferred
