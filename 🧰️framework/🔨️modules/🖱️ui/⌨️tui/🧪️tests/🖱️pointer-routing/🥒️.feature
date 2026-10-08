@capability-tui-pointer-routing
@comparison-independent-model-and-third-party-text-width
Feature: Route terminal pointer events to the right control
  The terminal engine owns hover, capture, click counting, the focus tree, splitters and tab drags. The table
  `🧫️fixtures/🖱️pointer-routing/🔣️.json` declares a two-stack dashboard scene, its painted chrome rows, the
  geometry of every chip and control, and the signals each pointer sequence must produce.

  Three readers share that table and nothing else. The Rust engine adapter replays every case. The vitest oracle in
  this folder re-derives every chrome case from the declared geometry with its own small model, checks the
  declared geometry against the golden frame rows, and uses `string-width` and `cli-truncate` as third-party
  oracles for cell width and ellipsis.

  @id-chrome-controls-carry-their-tab-index
  @level-fundamental
  @mode-conformance
  Scenario: Every chrome control names the tab it acts on and only the left button fires
    Given the two-stack scene painted at 80 by 40 cells
    When the left button presses the close glyph of the second, inactive chip
    Then the window reports a close for tab 1 and focus does not move
    And a right or middle press on the same glyph reports nothing
    And a right press on a chip asks for that tab's context menu at the pointer cell
    And the maximize control names the active tab of its own window

  @id-press-focuses-the-containing-focusable
  @level-fundamental
  @mode-conformance
  Scenario: A press focuses the containing focusable and tells the app which window took focus
    Given the two-stack scene painted at 80 by 40 cells
    When the left button presses a chip or the empty cap row of a window
    Then keyboard focus moves to the first widget inside that window
    And the window reports that it took focus once
    And a press on a widget row focuses the widget and selects the row, and a double press activates it

  @id-wheel-and-capture
  @level-fundamental
  @mode-conformance
  Scenario: The wheel goes to the widget under the pointer and capture overrides it
    Given the focus rests on the wizard of the first window
    When the wheel turns over the tabs widget of the second window
    Then only the tabs widget changes and focus stays on the wizard
    And the wheel over window chrome reaches no widget
    And a captured node receives the wheel wherever the pointer is

  @id-click-counting
  @level-quick
  @mode-conformance
  Scenario: Presses on one cell within half a second count as a double click
    Given the tabs widget of the second window
    When the same tab is pressed twice within 500 milliseconds
    Then the second press activates the tab
    And two presses a second apart are two single clicks

  @id-hover-and-gutter
  @level-quick
  @mode-conformance
  Scenario: Hover follows the pointer across widgets, chrome and the gutter between stacks
    Given the two-stack scene painted at 80 by 40 cells
    When the pointer moves over a widget, a window chip and the gutter
    Then the hovered node is the widget, the window and the splitter axis in turn

  @id-drags
  @level-fundamental
  @mode-conformance
  Scenario: Dragging a chip reorders tabs and dragging the gutter resizes the stacks
    Given the two-stack scene painted at 80 by 40 cells
    When a chip is pressed, dragged over a later chip and released
    Then the window reports the tab move once, on release
    And dragging the gutter reports incremental splitter deltas for the axis path
    And releasing or dragging without a press is harmless

  @id-painted-geometry-is-hit-geometry
  @level-fundamental
  @mode-conformance
  Scenario: The hit-tested geometry is the painted geometry
    Given the golden chrome rows and the declared chips and controls
    Then every declared wall, close glyph and maximize glyph is at its declared column
    And the Rust painter reproduces the golden rows cell for cell
    And an independent width model agrees every row is exactly as wide as the terminal

  @id-elision-matches-cli-truncate
  @level-quick
  @mode-conformance
  Scenario: Labels that do not fit end in an ellipsis exactly where cli-truncate puts it
    Given the elision vectors, including double-width text
    When Rust elides each label to its budget
    Then the result equals the declared output and cli-truncate's result
