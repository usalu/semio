Feature: Original Khronos geometry interoperability
  Scenario Outline: Import the unchanged official sample
    Given the exact sample bytes matching the pinned original SHA-256
    When the actual artifact is imported into IFC triangulated geometry
    Then the neutral primitive, vertex and triangle counts agree
    And every world coordinate agrees with independent three.js within eight binary64 epsilons
    And all reflected triangle index sequences agree exactly
    And the neutral world bounds agree
    And re-exported GLB passes the official Khronos validator
