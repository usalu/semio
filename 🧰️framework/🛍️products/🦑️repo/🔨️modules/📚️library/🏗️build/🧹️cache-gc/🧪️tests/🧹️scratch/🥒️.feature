Feature: Stale Scratch Is Swept By The Cache Collector

  Scenario: A Dead Owner's Old Scratch Goes
    Given scratch directories named by their owner process
    When `repo cache gc` collects the declared scratch area
    Then every directory whose process is gone and that is old enough is removed

  Scenario: A Live Owner's Scratch Stays
    Given a scratch directory whose process is alive
    When it is collected, however old it is
    Then it is kept, as is every directory that is young or not named by an owner
