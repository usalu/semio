@capability-tui-terminal-mouse
@oracle-prompt-toolkit
@comparison-mouse-report-v1
Feature: Pointer reports follow the child's reporting level and encoding and decode back to the event
  A child that enabled mouse tracking (1000, 1002, 1003, 9) in one of the encodings SGR (1006), X10 and urxvt
  (1015) reads a report whose button code carries the button, the modifiers, the motion bit and the wheel bit.
  prompt_toolkit's decode tables (`xterm_sgr_mouse_events`, `typical_mouse_events`, `urxvt_mouse_events`) are the
  reference: every report in the fixture must decode back to the button, event type, modifiers and cell of the
  event it was made from, and the Rust encoder must produce exactly those bytes. Events the child did not ask for
  (reporting off, drag below level 1002, release at level 9, a column beyond 223 in X10) produce nothing.

  The cases are the ones `shared://🔣️.json` pins for every language twin.

  @id-pointer-reports-round-trip
  @level-fundamental
  @mode-differential
  Scenario: Each shared pointer event encodes to a report prompt_toolkit decodes to the same event
    Given every case of `shared://🔣️.json` with a reporting level, an encoding, an event and a pane-local cell
    When prompt_toolkit decodes the fixture's report bytes and the Rust encoder encodes the event
    Then the decoded button, event type, modifiers and cell equal the event, and the encoded bytes equal the fixture bytes
