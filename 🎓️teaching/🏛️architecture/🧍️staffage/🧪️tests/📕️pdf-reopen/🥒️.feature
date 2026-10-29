Feature: PDF export reopens in independent readers
  Scenario: A drawing scene written as PDF is a valid vector document at the right size
    Given the drawing scene of a 1.80 m figure at 1:100 on an 80 mm by 50 mm sheet
    When it is written as PDF 1.7 and rendered at 254 dpi
    Then qpdf --check reports no syntax errors
    And the MediaBox equals the sheet size times 72 divided by 25.4 points
    And the figure measures 18 mm plus or minus 0.2 mm in the raster
    And the extracted text keeps the middle dot, the superscript two and the degree sign
    And the Helvetica advance widths equal reportlab's Adobe metrics
