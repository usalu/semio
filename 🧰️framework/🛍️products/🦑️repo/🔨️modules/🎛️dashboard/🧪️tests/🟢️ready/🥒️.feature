Feature: The daemon knows when a server is ready
  A long-running task declares the port it serves on. The daemon reads the visible output of the task,
  never its control sequences, and the task is ready when the output shows a local web address with
  exactly that port. The ready address is the match followed by the declared path, or with the printed
  option the whole printed address up to the next whitespace. Views and `semio run --wait-ready` consume
  the session change that carries it.

  Scenario: Decide every case of the shared table
    Given the shared table of declared readiness, output chunks and expected address
    When the owned Rust tracker reads the chunks and the output ends
    And the independent TypeScript implementation strips the escape sequences with `strip-ansi` and matches with a regular expression
    Then both agree on the address of every case, including those that have none

  Scenario: Decide the same way however the output is cut
    Given every case of the shared table
    When the output reaches the tracker one byte at a time instead of in chunks
    Then every case has the same address as when it arrives in chunks

  Scenario: Wait for the byte that completes the port
    Given a task that declared port 6061 and printed an address that ends in 6061
    When no further byte arrives
    Then the address is undecided until the next byte, the end of the line or a short silence settles it

  Scenario: Announce an address once per run
    Given a task that printed its ready address
    When it prints the same address again
    Then no second address is announced
    When the task restarts
    Then its next address is announced

  Scenario: Report the ready address on the session
    Given a task started through the daemon with a declared port
    When its output shows the local address with that port
    Then every attached view receives a session change carrying the ready address
    And a launch that waits for readiness returns that address
