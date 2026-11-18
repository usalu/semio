Feature: Total power of an asset

  The power of an asset is the sum over all its elements, derived from its authored thermal model: the metabolic heat of every living body in the default pose plus the rated power of every machine, device, lamp, stove or heating circuit. An asset without such an element has 0 W.

  Scenario: A person cooking includes the hob
    Given the person cooking at the hob
    When the total power is derived
    Then it is the metabolic heat of the body plus the 1500 W the hob draws

  Scenario: A machine is rated, not measured in its default pose
    Given the diesel generator whose default pose is switched off
    When the total power is derived
    Then it is the sum of the rated heat of its silencer, stack, hood and enclosure

  Scenario: No power-drawing element
    Given an oak tree
    When the total power is derived
    Then it is 0 W, stated explicitly

  Scenario: The manifest agrees with the assets
    Given every catalog asset and its manifest entry
    Then the manifest power equals the derived power within 0.5 W, and the validator reports no mismatch
