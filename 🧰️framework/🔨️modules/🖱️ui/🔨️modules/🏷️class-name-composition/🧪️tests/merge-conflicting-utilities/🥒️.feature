@capability-class-name-conflict-resolution
@no-oracle-repository-owned-utility-groups
@comparison-utf8-text-v1
Feature: Resolve conflicting utilities by last-winner
  `cn()` resolves conflicts across a finite table of REPOSITORY-OWNED utility families — `ui-surface`
  and `ui-glass` fills, `px-single`/`px-tiny` spacing, `text-element` sizing, `border-accent` colours.
  No third-party class merger knows this table, so none of them is a valid oracle: agreeing with
  `tailwind-merge` here would prove nothing, and disagreeing with it would prove nothing either.

  The specification is therefore the group table itself, and the evidence is the vector set below,
  carried over from the behaviour the owner already guaranteed. See the recorded no-oracle decision
  `repository-owned-utility-groups`.

  @id-last-surface-fill-wins
  @level-fundamental
  @mode-conformance
  Scenario: The last repository surface fill wins
    Given the composed class list and its specified result
      | input                        | expected         |
      | ui-surface ui-glass          | ui-glass         |
      | ui-glass ui-veil             | ui-veil          |
      | ui-veil bg-transparent       | bg-transparent   |
      | bg-transparent ui-surface    | ui-surface       |
    Then every composition matches its specified result

  @id-last-utility-in-a-family-wins
  @level-fundamental
  @mode-conformance
  Scenario: The last utility in a family wins, and narrower families survive wider ones
    Given the composed class list and its specified result
      | input                                                             | expected                              |
      | px-single px-tiny                                                 | px-tiny                               |
      | p-double px-single px-tiny                                        | p-double px-tiny                      |
      | px-single p-double                                                | p-double                              |
      | h-medium h-large                                                  | h-large                               |
      | flex-shrink-0 shrink-0                                            | shrink-0                              |
      | scroll-my-single scroll-my-double                                 | scroll-my-double                      |
      | border-normal border-accent                                       | border-accent                         |
      | rounded-sm rounded-md                                             | rounded-md                            |
      | text-xs text-element text-sm                                      | text-element text-sm                  |
      | hover:bg-hover-base hover:bg-active-base                          | hover:bg-active-base                  |
      | data-[state=open]:border-normal data-[state=open]:border-accent   | data-[state=open]:border-accent       |
      | w-auto !w-full                                                    | w-auto !w-full                        |
      | aspect-square aspect-auto                                         | aspect-auto                           |
      | aspect-auto aspect-square                                         | aspect-square                         |
    Then every composition matches its specified result

  @id-modifiers-scope-the-conflict
  @level-quick
  @mode-conformance
  Scenario: A modifier scopes the conflict to its own variant
    Given the composed class list and its specified result
      | input                              | expected                           |
      | px-single hover:px-tiny            | px-single hover:px-tiny            |
      | hover:px-single hover:px-tiny      | hover:px-tiny                      |
      | focus:h-medium hover:h-large       | focus:h-medium hover:h-large       |
    Then every composition matches its specified result
