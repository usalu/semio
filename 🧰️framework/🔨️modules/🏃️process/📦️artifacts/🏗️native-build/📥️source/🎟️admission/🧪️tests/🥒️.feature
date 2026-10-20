Feature: Observable ordinary Cargo source admission
  Scenario Outline: Source planning publishes bounded work checkpoints
    Given <units> workspace compiler units and <embedded> embedded source inputs per unit
    When source boundaries are admitted through their original control
    Then cfg progress is awaited before observing each compiler unit
    And read progress is awaited before its first read and after every eight completed reads
    And completion reports cumulative read operations, unique files and read bytes
    And all work progress has zero lease-wait elapsed milliseconds
    And SQLite independently produces the same progress events

    Examples:
      | units | embedded |
      | 1     | 0        |
      | 2     | 9        |

  Scenario: The observer has not resumed cfg preparation
    Given an awaited cfg progress observer
    When it has not resumed
    Then no compiler cfg or source read has started

  Scenario Outline: Preparation is stopped before the next expensive operation
    Given <stop> while publishing cfg progress or observing compiler cfg
    When source admission continues
    Then no source read or completion event is emitted

    Examples:
      | stop                |
      | observer rejection  |
      | original cancellation |

  Scenario Outline: Storage settlement is requested at unit boundaries instead of per read
    Given <units> workspace compiler units whose sources read <reads> files each
    When source boundaries are admitted through a port that settles storage
    Then storage settlement is requested once before each unit and once after the last unit
    And no storage settlement happens between the reads of one unit
    And SQLite independently produces the same event order

    Examples:
      | units | reads |
      | 1     | 1     |
      | 3     | 8     |

  Scenario: Actual storage settlement is throttled
    Given a settlement interval and a clock
    When unit boundaries arrive faster than the interval
    Then the actual port runs at most one durable storage recheck per interval
    And the completion phase always rechecks
