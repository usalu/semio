@capability-viz-render-scene-graph
@no-oracle-render-scene
@comparison-viz-probe-exact-v1
Feature: A chart specification renders into the scene graph and into TikZ
  Milestone 4 of the visualization library gives the LaTeX kernel a TypeScript twin
  (`🔨️modules/📊️viz-kernel`) so that every algorithm has a second, independently written subject.
  The twin's last stage is the renderer: it turns a taxonomy §79 chart specification into the
  dependency-free 2D scene graph of `🧰️framework/🔨️modules/◻️2d` and, from the SAME resolved item
  list, into TikZ source text.

  Nothing in d3 adjudicates that stage. d3 has no scene graph, no TikZ emitter and no notion of a
  chart specification — its shape generators stop at a path string. The arithmetic underneath the
  renderer is separately adjudicated against the registered d3 oracles by the kernel's own
  differential harness (`bun ./📜️script.ts test` in `📊️viz-kernel/📦️packages/🟦️typescript`,
  237 checks over thirteen modules), so this case covers only what that harness cannot: that the
  specification is turned into the primitives it names, that both emitters agree on the same item
  list, and that changing an option changes the projection.

  The vectors below are the specification. They are written out, not derived, because a derived
  expectation would restate the implementation rather than constrain it.

  @id-scene-graph-primitives
  @level-quick
  @mode-conformance
  Scenario: The scene graph carries exactly the primitives the specification names
    Given the demo bar chart specification and the expected scene
      | key             | values      |
      | size            | 160,100     |
      | count/line      | 8           |
      | count/text      | 8           |
      | count/rect      | 4           |
      | count/path      | 4           |
      | identity        | 1           |
      | unitColors      | 1           |
    Then the rendered scene graph matches those counts

  @id-bar-rectangles
  @level-quick
  @mode-conformance
  Scenario: Every bar rectangle stands where its scales put it
    Given the demo bar chart specification and the expected rectangles
      | key    | values                     |
      | rect/0 | 22.47619,54.8,25.904762,31.2   |
      | rect/1 | 54.857143,8,25.904762,78       |
      | rect/2 | 87.238095,67.8,25.904762,18.2  |
      | rect/3 | 119.619048,28.8,25.904762,57.2 |
    Then the scene's rectangles match those numbers to six decimals

  @id-tikz-mirrors-the-scene
  @level-quick
  @mode-conformance
  Scenario: The TikZ emitter draws the same item list as the scene graph
    Given the demo bar chart specification and the expected picture
      | key         | values |
      | statements  | 24     |
      | opens       | 1      |
      | closes      | 1      |
      | unitless    | 1      |
    Then the emitted TikZ carries one statement per resolved item, in millimetre coordinates

  @id-options-change-the-projection
  @level-quick
  @mode-conformance
  Scenario: Two specifications of one mark with different options render differently
    Given the demo bar chart specification and the expected distinctness
      | key       | values |
      | differs   | 1      |
    Then widening the band padding moves every bar
