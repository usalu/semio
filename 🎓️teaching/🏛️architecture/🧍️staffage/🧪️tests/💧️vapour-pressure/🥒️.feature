Feature: Saturation vapour pressure
  The Magnus form with Alduchov-Eskridge coefficients reproduces IAPWS-95 water saturation pressure.

  Scenario Outline: Saturation pressure against IAPWS-95
    Given an air temperature of <temperature> degrees Celsius
    When the saturation vapour pressure is evaluated
    Then it equals <pascal> Pa within 0.2 percent

    Examples:
      | temperature | pascal   |
      | 0.01        | 611.655  |
      | 20          | 2339.318 |
      | 37          | 6282.292 |
      | 60          | 19946.43 |

  Scenario: Relative humidity scales the vapour pressure
    Given an air temperature of 20 degrees Celsius and a relative humidity of 0.5
    When the ambient vapour pressure is evaluated
    Then it equals half of the saturation pressure
