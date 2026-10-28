Feature: Exact File Descriptor Custody
  Scenario: Physical descriptor validation fails before retained ownership
    Given one successfully opened descriptor
    When fstat, named identity, or retained ownership transfer fails
    Then that exact descriptor closes once before the refusal escapes
    And no descriptor is represented as retained
  Scenario: A qualified POSIX process retains a physical compiler lock
    Then independent Python flock cannot acquire it while owned
    And Python can acquire it after actual owner close
    And progress and cancellation retain the actual Original invocation
