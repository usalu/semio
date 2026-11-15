Feature: Parallel setup children share the dependency store without failing
  Scenario: A sibling holds the funding parent
    Given two setup tasks run in parallel and one holds the dependency funding parent
    When the other task finds the parent busy
    Then it says in English and German that it waits, with the elapsed and the maximum seconds
    And it retries with a doubling, jittered pause until the sibling lets go
    And it never fails because the parent was busy

  Scenario: The sibling never lets go
    Given the parent stays busy beyond the queue bound of the store sync
    When the bound is reached
    Then the task is refused with a bilingual timeout, and a cancellation ends the wait at once
