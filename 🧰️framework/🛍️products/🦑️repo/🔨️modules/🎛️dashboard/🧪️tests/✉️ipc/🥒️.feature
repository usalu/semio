Feature: The daemon and its views speak one language-neutral protocol
  Control messages are JSON envelopes, session output and input are binary frames, and both travel in
  length-prefixed frames. The daemon schema, the shared vectors and independent libraries state the
  same protocol as the native codec.

  Scenario: Every valid protocol vector survives the codec unchanged
    Given the shared control-plane message vectors that are valid
    When the native codec reads and writes each of them
    Then the output equals the vector

  Scenario: Every invalid protocol vector is refused
    Given the shared control-plane message vectors that are invalid
    When the native codec reads each of them
    Then each one is refused

  Scenario: The vectors cover the whole schema
    Given the daemon schema with its message types, session statuses and error codes
    When the vectors are compared with the schema
    Then every message type, every status and every error code appears in a valid vector
    And the error codes of the native wire are the codes of the schema

  Scenario: Frames carry the same bytes in every implementation
    Given the shared output and input frames
    When the native codec encodes them
    Then the bytes equal the shared bytes
    And control messages come out whole however the received bytes are cut

  Scenario: A missing greeting, a foreign protocol and an unreadable message are refused loudly
    Given a daemon and a connection that starts a task without a greeting
    When it greets with another build, then another connection greets with another protocol revision, then a third sends nonsense
    Then the first receives an error with the code hello_required, and the other build is served
    And the foreign protocol receives an error with the code protocol and is disconnected
    And the unreadable message receives an error with the code decode

  Scenario: A difference between client and daemon is described
    Given a client and a daemon of the same protocol and different builds, and another pair of different protocols
    When the client compares what the daemon greeted with
    Then the difference of build names both builds
    And a different protocol is told apart as incompatible and names both revisions
