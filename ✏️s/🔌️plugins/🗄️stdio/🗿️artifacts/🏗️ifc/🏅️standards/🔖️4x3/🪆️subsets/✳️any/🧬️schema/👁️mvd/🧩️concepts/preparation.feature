Feature: Canonical original-owned MVD preparation
  Scenario: Direct and Terminal share one preparation implementation
    Given one original caller, epoch, request owner and source snapshot
    When the accepted request is indexed and its closed rows are admitted
    Then the same paid snapshot alias and request owner move into population preparation
    And prepared clauses borrow the one admitted population index and request index

  Scenario: A request index is qualified by its actual population route
    Given two completed preparations of the same immutable source and request
    When a clause receives the first population view and the second request index
    Then it refuses before mutation, allocation or receipt
    And an unavailable source exposes only its request view and request index

  Scenario: Every denied and cancelled prefix keeps its original owners
    Given any preparation turn with one insufficient original grant currency
    When the turn is attempted
    Then source owners, metadata, heap and receipts remain unchanged
    When the original operation is cancelled
    Then each retained prefix retires under paid original retirement

  Scenario: A rejected row posts its actual metadata prefix
    Given an empty request or an unknown closed row kind
    When the original funded request turn refuses
    Then its actual failure metadata is posted once
    And a later refused retry adds no receipt
