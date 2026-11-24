Feature: Heating and cooling finger exercise
  Scenario: Bilingual assignment that fixes only geometry and output
    Given the tiny home in Hannover with an air-source heat pump and ordinary insulating glazing
    When the German and English assignment sources are published
    Then each opens with a cover axonometry that shows the tiny home with staffage and without dimensions, labels or hidden edges
    And each recommends working directly in Grasshopper
    And each draws the tiny home on the geometry page as a technical axonometry with dimensions and states the exact envelope, south window, east sliding door and canopy coordinates
    And the German text addresses the student informally
    And each lists the exact rows of the CSV template and its file format
    And no concrete energy value is prescribed
    And each names the Stud.IP upload folder, the submission file name, the template, the reference solution and the deadline Friday 10:00
  Scenario: Geometric consistency
    Given the language-neutral geometry cases
    When the expressions are evaluated independently with mathjs
    Then the results agree with the declared expected values
