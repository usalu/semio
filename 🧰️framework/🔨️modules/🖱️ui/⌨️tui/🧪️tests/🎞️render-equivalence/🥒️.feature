@capability-tui-incremental-render
@comparison-owned-terminal-emulator-and-full-render
Feature: Incremental terminal patches equal a full repaint
  `Tui::render` repaints only what changed, `Tui::render_full` repaints everything. Both are applied to the owned
  VT interpreter, which shares no code with the diff and patch emitter, and the screens must be identical after
  every step of a seeded random sequence of scene, pointer, key, overlay, layout and resize mutations
  (`⌨️tui/🧪️tests/🔬️render-equivalence`). No third-party terminal emulator is installed in this repository, so the
  oracle is the owned interpreter plus the full repaint; the mutation proof below shows the property is able to fail.

  @id-incremental-equals-full
  @level-fundamental
  @mode-conformance
  Scenario: Applying incremental patches reproduces the full repaint after every mutation
    Given 64 seeded random mutation sequences of 80 steps over a dashboard-shaped scene
    When each step renders incrementally and the same scene renders in full on a fresh screen
    Then both screens and both engine frames are identical
    And the same input produces the same signals in both engines

  @id-mutation-after-first-frame
  @level-quick
  @mode-conformance
  Scenario: A mutation after the first frame produces a patch and a quiet frame produces none
    Given a scene rendered once
    When a node's text changes three times
    Then every change yields a patch that names the new text and the next render is empty

  @id-minimal-patch
  @level-quick
  @mode-conformance
  Scenario: A one-cell change does not re-emit the frame
    Given a rendered list
    When one row gains a mark
    Then the patch is under a quarter of the size of the full frame

  @id-mutation-proof
  @level-quick
  @mode-manual
  Scenario: The property fails when the dirty bits are cleared only at the roots
    Given the scene clears dirty flags on the root and overlay root only
    When the properties run
    Then both the random-mutation property and the mutation test fail
