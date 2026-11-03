Feature: The multiview sheet and the view-aware exports
  The plan, the front elevation and the side elevation of one evaluated state share one sheet, one paper scale and one time.
  Every size on paper is metres times 1000 divided by the scale denominator, whichever file format carries it.

  Scenario Outline: Physical size of every view in every format
    Given an asset <asset> at the instant <instant>, the representation <mode> and the scale 1:<denominator>
    When the sheet is composed in <projection>-angle projection and written as SVG, PDF and DXF
    Then lxml reads the SVG, PyMuPDF the PDF and ezdxf the DXF
    And the figure of every view is exactly metres times 1000 divided by the denominator millimetres wide and tall in the SVG and in the PDF
    And the DXF model space holds the same extents in metres
    And the plan shares its x origin with the side elevation and the front shares its ground line with it
    And the plan is above and the front right of the side elevation in third-angle, below and left in first-angle
    And no title, description or PDF keyword spells the raw representation id

  Examples:
    | asset            | instant          | mode       | projection |
    | standing-adult   | pose pointing    | light-line | third      |
    | standing-adult   | pose pointing    | light-line | first      |
    | walking-commuter | clip walk-cycle at 0.3 s | dark-solid | third |
    | tilt-window      | pose tilted-open | thermal    | first      |

  Scenario: Same bytes on a rerun
    Given the same state, scale, mode, language and projection
    When the sheet is written twice as SVG, PDF and DXF
    Then every pair of outputs is byte-identical

  Scenario: Labels follow the language
    Given a first-angle sheet
    When it is written in German
    Then it names Projektionsmethode 1, Draufsicht, Vorderansicht and Seitenansicht and no English caption

  Scenario: Every output names its view
    Given the export command for a pose at 1:100
    When it runs for elevation, front, plan and for both sheet projections in SVG and DXF
    Then it writes ten different files, the viewpoint or projection is in each name, and a missing viewpoint is refused
