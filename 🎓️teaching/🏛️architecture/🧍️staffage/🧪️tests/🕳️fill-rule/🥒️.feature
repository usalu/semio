@capability-staffage-evaluation
Feature: One fill rule everywhere: even-odd
  A filled shape is painted where a ray from the point crosses its rings an odd number of times, whatever the winding of the rings. A ring inside a ring is a hole whether it winds
  the same way or the opposite way, an island inside a hole is painted again, and the overlap of two rings of one shape is a hole too (so parts that overlap are separate shapes).
  The SVG writer, the PDF writer (`f*`) and the Manim consumer (Cairo, non-zero natively) paint the same area.

  THE REFERENCES are shapely (the planar arrangement of the rings and the symmetric difference of their polygons) for the expected area and points, MuPDF for the SVG raster and PDFium for the PDF raster.

  Scenario Outline: A case paints the even-odd area in every renderer
    Given the case "<case>" of the fill-rule fixture as one filled path
    When it is written as SVG (long and compact form) and as PDF and rasterised at 10 pixels per millimetre
    Then the painted area is the shapely even-odd area within 1.5 percent
    And every sample point is painted exactly where shapely says it is
