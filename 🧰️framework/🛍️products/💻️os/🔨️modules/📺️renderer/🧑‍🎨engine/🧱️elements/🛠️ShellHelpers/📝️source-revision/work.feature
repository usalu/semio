Feature: Source work holds
  Scenario: A timed out operation is still source work
    Given one exact active document and loaded plugin
    And an admitted typed operation
    When its UI waiter returns no completion
    Then source readiness remains unavailable
    And only its matching terminal completion or owner retirement closes the hold

  Scenario: Terminal publication can precede host effects
    Given an original source hold and an exact admitted operation
    When its terminal completion arrives
    Then the hold remains while the host dispatch is active
    When that dispatch finishes
    Then a fresh admitted source capture may begin

  Scenario: An uncertain failed command cannot reuse a prior hash
    Given a command whose admission response was lost
    When its host dispatch finishes
    Then the hold remains until its document lifetime retires
