Feature: Thermal comfort by ISO 7730 against pythermalcomfort
  The predicted mean vote and the predicted percentage dissatisfied follow from air temperature, mean radiant temperature, relative air speed, humidity, activity in met and clothing in clo.

  Scenario Outline: PMV and PPD of a clothed person
    Given air of <air> degrees Celsius, mean radiant temperature <radiant> degrees Celsius, relative air speed <speed> m/s and <humidity> percent humidity
    And an activity of <met> met and clothing of <clo> clo
    When the ISO 7730 comfort is evaluated
    Then the predicted mean vote is <pmv> within 0.01 and the predicted percentage dissatisfied <ppd> within 0.1

    Examples:
      | air | radiant | speed | humidity | met | clo | pmv    | ppd    |
      | 24  | 24      | 0.1   | 50       | 1.2 | 0.6 | -0.021 | 5.009  |
      | 22  | 22      | 0.1   | 60       | 1.2 | 0.5 | -0.752 | 16.921 |
      | 26  | 38      | 0.1   | 50       | 1.2 | 0.5 | 2.19   | 84.552 |

  Scenario: Inputs outside the ISO limits are flagged
    Given air of 5 degrees Celsius, mean radiant temperature 5 degrees Celsius, relative air speed 1 m/s and 70 percent humidity
    And an activity of 1.4 met and clothing of 1.5 clo
    When the ISO 7730 comfort is evaluated
    Then the result is marked as outside the validity limits
