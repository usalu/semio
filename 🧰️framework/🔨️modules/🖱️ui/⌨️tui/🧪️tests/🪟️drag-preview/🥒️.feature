Feature: Exact Window Drag Tab Preview and Handles
  Scenario: Drag a window through its visible handle
    Given a window tab with a localized drag label
    Then its leading padding cell shows the Unicode or ASCII drag grip
    And its handle hit target belongs to that tab
    And close and maximize remain independent controls
    When the pointer presses and drags the grip
    Then the engine begins the same window drag as the tab label

  Scenario Outline: Preview the actual committed destination tab
    Given the window layout in drag-preview case "<case>"
    When the source window is dragged to the destination
    Then the preview shows the source tab label and controls at the proposed insertion slot
    And every preview chip and seam glyph agrees with the actual committed drop
    And removing an empty source stack is included in proposed geometry
    And the source tab dimensions agree with an independent Taffy layout
    Examples:
      | case                     |
      | last-source-top-left     |
      | last-source-top-right    |
      | last-source-bottom-left  |
      | last-source-bottom-right |
      | peer-source-top-right    |
      | last-source-left         |
      | last-source-right        |
      | last-source-top          |
      | last-source-bottom       |
      | same-stack-corner        |
      | same-stack-reorder       |
      | compact-top-right        |
      | clipped-source-chip      |
      | overflow-insertion       |
      | overflow-arrow-corner    |
      | overflow-arrow-cap       |

  Scenario: Keep an overflow arrow in its occupied corner strip
    Given a narrow top-left strip whose next arrow lies in the right half of the window
    When a source tab is dragged onto the arrow
    Then the proposal remains in the occupied top-left corner
    And its insertion index uses the visible tab's full model position
    And its raised cap resolves to the same occupied strip corner
    And overflow arrows cannot remove the active tab or its drag and close controls
    And counted overflow markers shrink to single glyphs before any marker is hidden
    And a single navigation marker remains when two cannot fit beside the active controls

  Scenario: Cancel a drag without changing the mounted layout
    Given a window drag with a destination tab preview
    When Escape or pointer cancellation occurs
    Then the preview disappears
    And the source and target stacks retain their original order

  Scenario: Scope repeated local window ids to their mounted canvas
    Given two canvases whose source and target window ids are equal
    When a source in the second canvas is dragged onto its target
    Then the proposal uses the second canvas and both concrete chrome node identities
    And the preview agrees with the committed drop in that canvas
    And the first canvas retains its layout
    And dragging a source across canvases with repeated ids produces no window docking mutation

  Scenario: Remove the captured source during a window drag
    Given a held window drag with stable source node identity
    When the captured source node is removed
    Then pointer capture and the destination tab preview are cancelled
