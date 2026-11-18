Feature: Zipf-distributed question pools
  A pooled task may declare a Zipf distribution over its slots. The slots are ranked by their smallest magnitude (a sorting item's absolute value,
  a matching item's absolute value of its first dimension, a classification item's definition position), the task draws as many slots as `draw`
  without replacement, each pick proportional to 1 / rank^exponent, and presents one variant of every drawn slot.

  Scenario: Small magnitudes dominate and large magnitudes stay reachable
    Given the shared zipf pool of twelve magnitude slots drawing five with exponent 1.5
    When thousands of seeded sheets are dealt
    Then the lowest third of the items makes up the clear majority of the dealt items
    And the top decile makes up less than ten percent of them and still occurs

  Scenario: The slot inclusion matches the exact Zipf weights
    Given the exact inclusion probabilities of sequential weighted picks without replacement computed with mathjs
    When thousands of seeded sheets are dealt
    Then every slot is included as often as its probability within sampling error

  Scenario: The draw is deterministic and valid
    Given the same seed twice
    Then the same items are dealt, one variant per distinct slot

  Scenario: A distribution must leave slots undrawn
    Given a pool whose draw equals its slot count and a distribution
    Then validation reports value-invalid at the draw
