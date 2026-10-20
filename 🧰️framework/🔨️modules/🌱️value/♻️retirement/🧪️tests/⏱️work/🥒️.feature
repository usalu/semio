Feature: Finite fixture-scaled native retirement work
  Scenario: Paid work and genuine unchanged receipts have independent finite limits
    Given a maximum of 65537 physical fixture rows and 256 close transitions per row plus 1024 fixed transitions
    When a close helper observes an unchanged selected public quote and an empty nonterminal receipt
    Then 16 consecutive unchanged turns refuse without changing any granted currency
    And paid progress remains admitted below its checked fixture-scaled total

  Scenario: A finite receiving observation uses its actual row count
    Given257rows under the shared per-row fixture work policy
    Then the finite maximum is66816turns
    And a row count beyond65537 is refused

  Scenario: Native receiving callers remain finite independently from exact grants
    Given zero, seventeen or257 authored rows
    When receiving or closure observes completed turns
    Then paid turns stay below the checked fixture bound
    And sixteen unchanged quotes with empty receipts refuse independently
    And selected copy, capacity, release and depth grants remain exact

  Scenario: One selected close event funds every physical currency together
    Given a close event with copy, capacity, release and depth from the shared neutral fixture
    When a source fixture funds the selected event
    Then every grant currency equals that event without independent-axis reconstruction or padding
    And SQLite projects the same exact grant
    And normal fixture work counters allocate no JSON data
