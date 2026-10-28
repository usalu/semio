Feature: Caller-authored required SELECT inputs
  Scenario: Explicit constructor and scalar admission
    Given the actual source and complete released SELECT declaration
    Then no constructor is initially selected
    When the caller chooses a member
    Then the itemless typed constructor remains incomplete
    When the caller supplies its scalar
    Then the exact typed payload reaches the central strict candidate and inverse
  Scenario: Editable exact member transitions
    Given a caller-authored typed scalar
    When the caller chooses a member with the same primitive kind
    Then the original child payload is preserved exactly
    When the caller chooses a member with another primitive kind
    Then the constructor is itemless until another scalar is supplied
  Scenario: Closed carrier and original operation
    Then nonmembers and malformed constructor children are refused
    And all original cancellation prefixes and observer failures preserve source and request
