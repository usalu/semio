@capability-staffage-region-merge
Feature: Drawn details survive the merge of the filled shapes of one region
  Filled shapes of one thermal region on the same or an adjacent bone hide each other's inner outlines, so a body made of several overlapping parts reads as one
  silhouette. That merge belongs to filled shapes only. An unfilled stroke (a ring, a spoke, a rib, an unfilled rectangle) is a drawn detail: it stays visible
  inside a body of its own region and is removed only where a filled shape with a higher z genuinely covers it.

  THE REFERENCE is shapely: it measures the visible outline length of every shape with its own geometry operations.

  Scenario: Two overlapping filled shapes of one region read as one silhouette
    Given a 0.4 by 0.1 m and a 0.4 by 0.16 m filled rectangle of the same region overlapping by 0.1 m
    When the visible outlines are measured at 1:100
    Then they are 7.0 mm and 10.2 mm long, together the 17.2 mm of the union boundary

  Scenario: An unfilled ring inside a body of its own region survives
    Given a 0.2 by 0.06 m unfilled rectangle inside the first filled rectangle
    When its visible outline is measured
    Then it is the whole 5.2 mm

  Scenario: A filled shape of a higher z still covers a detail below it
    Given the same unfilled rectangle below and above a covering filled shape of another region
    When the visible outlines are measured
    Then the one below is 0 mm long and the one above the whole 3.2 mm

  Scenario: A third-party library measures the same lengths
    Given the same rectangles
    When shapely subtracts the covering polygons from each outline
    Then its lengths equal those of the drawing
