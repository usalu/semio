Feature: A draining Original never lets accepted work outlive it
  Scenario: Work accepted beneath a live owner while the Original joins its work is joined
    Given an Original whose callback returned and whose accepted work is still settling
    When that work's continuation admits a new operation beneath a still live owner right away
    Then the new operation is joined before the Original settles

  Scenario: Admission after the join observed the owner empty is refused
    Given an Original or resource loan whose join has just observed no accepted work
    When any continuation, however many asynchronous steps later, admits an operation beneath a live owner
    Then the operation is either joined before settlement or refused
    And no admitted operation completes after the Original or loan has settled

  Scenario: A draining Original admits only beneath a live owner
    Given an Original whose callback returned while accepted work is still settling
    When that work requests a new top-level control, a new top-level loan or a new nested process envelope
    Then every one of them is refused
    And an operation or narrowed control beneath the still live accepting owner is admitted

  Scenario: Cancelling a draining Original closes admission beneath its live owners
    Given an Original whose callback returned while accepted work is still settling
    When its cancellation signal fires and that work requests a new operation beneath a still live owner
    Then the request is refused with the cancellation reason
    And the accepted work is still joined before the Original settles
