Feature: A continuous build task is ready when its announced address answers
  A continuous target declares the local port it serves on at `metadata.semio.dashboard.ready`. Every build
  tool and the dashboard daemon decide readiness the same way: the task is ready after its visible output
  shows `http://(127.0.0.1|localhost|0.0.0.0):<port>` and the exact ready address answers HTTP with a 2xx or 3xx
  status. The ready address is the printed match followed by the declared path, or with `printed` the whole
  printed address up to whitespace. A task without a declaration is ready when it starts.

  Scenario: Decide every case of the shared table
    Given the table of declared readiness, output chunks and expected address shared with the dashboard daemon
    When the build matcher reads the chunks and the output ends
    And the independent TypeScript implementation strips the escape sequences with Node `util.stripVTControlCharacters` (`strip-ansi` is not an installed dependency) and matches with a regular expression
    Then both agree on the address of every case, including those that have none

  Scenario: Decide the same way however the output is cut
    Given every case of the shared table
    When the output reaches the matcher one character at a time
    Then every case has the same address as when it arrives in chunks

  Scenario: Validate declarations
    Given the declaration vectors
    When the build schema validates them
    And `ajv` independently validates them against the dashboard schema of the same declaration
    Then both accept the valid vectors and refuse the invalid ones

  Scenario: Resolve the port of one run
    Given a declaration and an environment
    When the port is resolved
    Then the variable named by `portEnv` wins over the declared `port`, and a declaration without any port value is refused

  Scenario: Probe the announced address
    Given a local HTTP server that answers a chosen status
    When the announced address is probed
    Then a 2xx or 3xx status is ready, any other status or a closed port is not, and a redirect is not followed

  Scenario: Wait until the announced address answers
    Given a task whose output announces its address before its server listens
    When the server starts listening later
    Then the watch becomes ready only after the server answers

  Scenario: Stop waiting
    Given a watch that has not seen an address
    When the invocation is cancelled
    Then the watch never becomes ready and leaves no probe running
