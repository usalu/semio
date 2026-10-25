Feature: Capacity Grant Handover To A Restarted Command

  Scenario: A Restart Keeps Its Place Without Queueing Again
    Given a command that holds a machine capacity grant yields because its implementation moved
    When the outermost command starts again as a new process
    Then the grant stays in the ledger through the gap and is offered with a one-time token and the identity of the offering process
    And only the process the offering process started takes it over, with its own identity, without entering the queue
    And an unrelated process, a recycled process id, a stale offer, another operation or a wrong token queues like any other request and is told why in English or German
    And the grant is given back when the replacement ends or when nobody takes it within the declared bound

  Scenario: The Claim Never Holds The Gate Lock While It Reads The Process Table
    Given a replacement process that is offered the grant of the command it replaces
    When it reads the identity of its parent from the system process table
    Then the shared gate lock is not held during that read, so a slow listing can never let a waiter steal the lock
    And a refused claim leaves no lock behind
