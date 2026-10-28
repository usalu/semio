Feature: Cooperative schema-owned GLB JSON emission
  Scenario: Exact JSON meanings survive bounded emission
    Given an admitted real Khronos Box snapshot
    And exact controls, Unicode, lone surrogates, numeric words and duplicate own data keys
    When the physical GLB JSON is emitted incrementally
    Then it retains independently specified JSON values and ECMAScript escaping
    And extras names remain data and ordinary empty defaults remain omitted
  Scenario: Byte limits govern long immutable strings
    Given a name larger than the admitted encoded byte ceiling
    When the JSON writer visits and encodes it
    Then it refuses with glb.limit before publishing a GLB
  Scenario Outline: Original cancellation remains authoritative
    Given a real loaded snapshot with a long immutable string
    When the original signal cancels inside or at completion of <phase>
    And the callback replaces the caller control object
    Then no GLB is published and the source stays unchanged
    Examples:
      | phase           |
      | glb-json-values |
      | glb-json-encode |
      | glb-json-bytes  |
