Feature: Stable identities in positional diff replay
  Scenario Outline: Applying whole-row patches preserves event identities atomically
    Given the base rows and ordered patches of a stable-identity vector
    When the central applier materializes the positional diff
    Then acceptance and the projection equal the language-neutral vector
    And every accepted diff inverse restores the exact base rows
    And a refusal leaves the base rows unchanged
    And a SQLite transaction with immutable row identities gives the same result

    Examples:
      | vector                          |
      | stable-replacement              |
      | renamed-row                     |
      | colliding-row                   |
      | later-refusal-is-atomic          |
      | missing-target                  |
      | ordered-repeated-patches        |
      | two-independent-patches         |

  Scenario Outline: Inverse replay composes repeated nested row patches
    Given two nested rows in one identified parent row
    When the ordered nested patches of <vector> are applied to that parent
    Then the projection equals the portable vector and SQLite transaction
    And the inverse of the full diff restores the exact original nested rows
    And composing the patches gives the same forward and inverse projections

    Examples:
      | vector                             |
      | insert-then-remove-shifted-row      |
      | remove-then-insert-at-same-position  |
