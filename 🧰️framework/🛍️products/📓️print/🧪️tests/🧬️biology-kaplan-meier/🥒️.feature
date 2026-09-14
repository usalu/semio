@capability-viz-scientific-survival
@no-oracle-survival-product-limit
@comparison-viz-probe-v1
Feature: The Kaplan–Meier estimator is computed from the event records, censoring included
  `sci-survival` walks the event records of a group in time order and multiplies the conditional
  survival of every event into a running product, skipping censored observations but removing them from
  the risk set. The catalogue's survival curve, Kaplan–Meier plot and cumulative-hazard plot are the
  same walk drawn three ways, so the walk itself is what this case measures — and it measures the
  millimetre geometry the page receives, so the data-to-canvas projection is covered at the same time.

  The reference is an independent TypeScript product-limit implementation in the adapter. `jstat` and
  survival-analysis packages are not in the print oracle registry; the request for `jstat` is recorded in
  the ticket's SCIENTIFIC status note, and until then the estimator's own definition — S(t) is the
  product of (1 − d/n) over the event times at or before t — is this case's specification.

  @id-product-limit
  @level-long
  @mode-conformance
  Scenario: One censored group produces the product-limit steps at the right millimetres
    Given the committed probe document local://biology-kaplan-meier.tex and the group
      | label     | records                                       | final |
      | treatment | 2:1 4:1 5:0 7:1 11:1 12:0 15:1 18:0 21:1 24:0 | 3/14  |
    Then the compiled probe and the reference implementation agree on every value

  @id-two-groups
  @level-long
  @mode-conformance
  Scenario: Two groups are estimated independently and drawn on the same window
    Given the committed probe document local://biology-kaplan-meier.tex and the groups
      | label | records         |
      | a     | 1:1 2:1 3:1 4:1 |
      | b     | 1:0 2:1 3:0 4:1 |
    Then the compiled probe and the reference implementation agree on every value

  @id-cumulative-hazard
  @level-long
  @mode-conformance
  Scenario: The cumulative hazard is the negative logarithm of the same survival estimate
    Given the committed probe document local://biology-kaplan-meier.tex and the group
      | label     | records                                       |
      | treatment | 2:1 4:1 5:0 7:1 11:1 12:0 15:1 18:0 21:1 24:0 |
    Then the compiled probe and the reference implementation agree on every value
