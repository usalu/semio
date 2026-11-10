Feature: Hard per-job memory limits

  Scenario: An overrun fails one job and never the host
    Given a job limited to 160 MB with no swap
    When the job allocates 1024 MB in steps of 16 MB
    Then the job ends as memory-exceeded within 1500 ms
    And its peak never exceeds the limit
    And an unrelated sibling process keeps running

  Scenario: The limit is the learned peak with headroom
    Given a class with a learned peak of 2000 MB and a floor of 512 MB
    Then the limit is 2500 MB

  Scenario: A killed job teaches the estimate
    Given a job stopped at its limit of 1000 MB
    Then the next learned peak is 1250 MB

  Scenario: The failure is reported in the named language
    Given a memory-exceeded outcome
    Then the English and the German text name the limit and the peak
