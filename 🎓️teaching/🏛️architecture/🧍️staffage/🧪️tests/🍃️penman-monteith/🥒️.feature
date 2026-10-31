Feature: Penman-Monteith evapotranspiration
  The combination equation with aerodynamic resistance 208/u2 and surface resistance 70 s/m is the FAO-56 reference crop.

  Scenario: FAO-56 example 18
    Given Uccle on 6 July with a mean temperature of 16.9 degrees Celsius, wind of 2.078 m/s at 2 m, net radiation of 13.28 MJ/m2/day and a vapour pressure deficit of 0.588 kPa
    When the latent heat flux is computed
    Then the evapotranspiration is 3.88 mm/day within 3 percent and agrees with pyet within 4 percent
