@capability-staffage-section-views
Feature: Sectioned front and plan views are named as sections and marked in the side view
  A representation may declare that its front or plan view is a section: `sections.front.plane` is the forward position of a vertical cut plane in metres, `sections.plan.plane` the height of a horizontal one,
  each with a bilingual label such as A-A. Captions then read "Section A-A" / "Schnitt A-A" instead of "Front elevation" / "Vorderansicht", the side elevation at 1:100 and 1:50 shows the cut-line marker at the plane
  (thin chain line with thick ends, viewing arrows and letters), and the cast index records the section. A plane outside the asset is a validation issue in English and German.

  THE REFERENCE is shapely (🐍️.py): it builds the cut line, cuts the thick ends and moves the end points along the viewing direction; the hand-computed numbers of the fixture agree with it.

  Scenario: The oracle reproduces the hand-computed marker
    Given the markers of a 1.4 m by 3.3 m object at 1:100 and 1:50
    When shapely places them
    Then every line, thick end and arrow tip equals the hand-computed millimetres

  Scenario: The side view draws the marker at the plane
    Given the curtain wall bay with sections A-A and B-B
    When its side view is depicted at 1:100 and 1:50
    Then the marker items equal the shapely placement for the evaluated extent
    And no marker is drawn at 1:200, in a textless depiction or in the front view

  Scenario: Captions name the section
    Given a front view declared as section A-A and one without a section
    When the captions are read in English and German
    Then they read "Section A-A", "Schnitt A-A", "Front elevation", "Vorderansicht", "Plan section B-B" and "Grundriss-Schnitt B-B"

  Scenario: A plane outside the asset is rejected
    Given the sliding door with a plan section at 9 m
    When the asset is validated
    Then the issue section-plane-outside is reported in English and German

  Scenario: The cast index records the section
    Given a cast of the sliding door plan
    When it is exported
    Then the index carries the plane and the label of the section
