@capability-diagram-sequence-messages
@no-oracle-diagram-sequence-messages
@comparison-viz-probe-v1
Feature: The sequence family stacks messages down the lifelines at a constant step
  `uml-sequence` in `semio-viz-diagram-uml.sty` draws the UML interaction diagram of §15 and the
  message-sequence chart of §56 from a participant table and a message table. Its geometry is two
  rules.

  Participants are spread evenly across the frame: with `n` participants and a frame width `w` the
  lifeline of participant `i` (1-based) stands at `1 + (i − 0.5)·(w − 2)/n`, and the head box is
  `min((w − 2)/n − 2, 24)` wide.

  Messages stack downward from below the head band. With `m` messages, a head height `head` and
  `step=0` the step is derived so the last message still clears the bottom of the frame:
  `step = (height − head − 3) / (m + 0.5)`, and message `k` (1-based) sits at
  `y = height − head − k·step`. A message whose source and target are the same participant is
  drawn as the self-call bracket instead of a straight arrow, but keeps the same `y`.

  **Why there is no third-party oracle.** No registered library lays out a sequence diagram:
  d3 has no interaction-diagram layer and dagre ranks a graph rather than stacking an ordered
  message list, so nothing can adjudicate a coordinate that the two rules above define completely.
  Each scenario therefore writes out the millimetres those rules produce. The probe emits
  `geometry/diagram-message` records of `x-from, y, x-to, y` in message-table order.

  @id-message-y-positions
  @level-quick
  @mode-conformance
  Scenario: Four messages between three participants
    Given the sequence messages
      | from | to | kind   | xfrom | y       | xto |
      | p    | q  | sync   | 14    | 33.3333 | 40  |
      | q    | r  | sync   | 40    | 24.6667 | 66  |
      | r    | q  | return | 66    | 16      | 40  |
      | q    | p  | return | 40    | 7.3333  | 14  |
    Then every message sits on the specified lifelines and step

  @id-self-call-keeps-its-step
  @level-quick
  @mode-conformance
  Scenario: A self-call occupies a step of its own without moving the messages after it
    Given the sequence messages
      | from | to | kind | xfrom | y       | xto |
      | p    | q  | sync | 14    | 30.8571 | 40  |
      | q    | q  | self | 40    | 19.7143 | 40  |
      | q    | r  | sync | 40    | 8.5714  | 66  |
    Then every message sits on the specified lifelines and step
