@capability-staffage-caption-fit
Feature: Captions and scale-bar labels fit their drawing and their sheet cell
  Text is measured with the Helvetica advance widths the renderer uses (Adobe core font metrics, Arial as metric twin), so the scene reserves exactly the room its titles, subtitles and scale-bar labels need,
  and the caption above each cell of a views sheet wraps to the width of that cell.

  THE REFERENCE is PyMuPDF (🐍️.py): its built-in Helvetica and Helvetica-Bold give the width of every string.

  Scenario: The width table agrees with PyMuPDF
    Given every printable ASCII character and German strings with umlauts, regular and bold
    When their widths are measured
    Then the table and PyMuPDF agree within 0.1 %

  Scenario: Every text of a scene stays inside the scene
    Given the gas boiler and the gas hob at 1:200, the gas hob at 1:100 and the ceiling diffuser at 1:50 in all viewpoints and both languages
    When the extent of every text is measured with PyMuPDF
    Then no text starts left of the scene or ends right of it

  Scenario: Sheet captions wrap to their cell
    Given a long caption and a narrow cell
    When it is wrapped
    Then no line is wider than the cell unless it is a single word
