@capability-energy-model-1-simulate
@oracle-energyplus-ashrae-140
@comparison-ordered-json-v1
@simulate-energy-model-1-any
Feature: Simulate the committed ANSI/ASHRAE 140 §5.2 case models and compare them with EnergyPlus

  This case is the real-world validation of the `s.energy.model` SIMULATION engine, not of its
  document codecs. The subject is `semio-s-plugin-energy`'s own kernel; the oracle is EnergyPlus,
  reached through honeybee-energy → honeybee-openstudio → OpenStudio, and the judge is ANSI/ASHRAE
  Standard 140 §5.2 — the standard that exists precisely to say whether a building-energy program
  is right.

  ONE INPUT. Every case is authored once, in Rust, against the engine's own typed `Model`
  (`🔨️modules/⚡️simulation/⚙️engine/🏛️bestest/🦀️.rs`) and committed as this subset's own canonical
  JSON at `🧫️fixtures/🏛️bestest-<case>/🔋️model.json`. The oracle does NOT re-author the case from
  the standard; it TRANSLATES that committed model into a honeybee model and runs EnergyPlus on it,
  so a disagreement is a disagreement about physics and never about two people reading §5.2
  differently. The model file carries its own `schedules`, so setpoints, gain profiles and
  ventilation schedules travel with the geometry rather than being reconstructed by each side.

  ONE WEATHER FILE. Both sides read `🧫️fixtures/🌦️denver-tmy/🌦️.epw` — the semio side through the
  `🌦️epw` codec that `✏️s/🔌️plugins/🗄️stdio` already owns, the oracle side through EnergyPlus's own
  reader. Its sha256 is carried in both result documents so a silent weather substitution cannot
  hide inside an agreement.

  ONE RESULT SHAPE. Both producers emit `semio.energy.bestest-results/1`: annual heating/cooling
  kWh, peak heating/cooling kW and their hours, free-float min/max/mean °C and their hours, and the
  8760-long hourly zone-air temperature. The subject writes its own document to `⚙️semio.json` in
  the scenario work directory; only the oracle's `🔮️energyplus.json` is committed.

  ⚠️ HONEST BOUNDARIES, stated rather than tuned away.
  1. `hourly.heatingW` / `hourly.coolingW` come back EMPTY from the semio side. The engine's
     aggregation pass registers exactly one time series per zone (the zone air temperature) and
     routes heating and cooling into METERS, which retain a running total and a peak but no hourly
     trace. The comparison reports those two metrics as unavailable rather than comparing zeros.
  2. Cases 960 and the §5.2 analytical-verification series are NOT registered: the engine's
     interior solar distribution is a fixed floor/uniform/split-flux heuristic with no enclosure
     radiation model, and case 960 needs a two-zone sunspace with real interzone solar and radiant
     coupling.
  3. Cases 630/930 (fins) and 650/950 (night ventilation) carry committed MODELS and are covered by
     the parameter cross-check, but have NO simulation scenario: the oracle side has not translated
     them, and the Rust host has no skip channel — a missing registration, a panic and an error are
     all results — so registering them would only manufacture guaranteed reds. They come back the
     moment a reference exists.
  4. The window is stated as the whole-assembly simple-glazing pair the standard publishes
     (U = 3.0 W/(m²·K), SHGC 0.787), which both producers consume identically. The residual gap to a
     layer-by-layer EnergyPlus window (2.740 / 0.760) is a known, documented cooling offset of about
     +5.7…+8.1 %, which is why the cooling tolerance below is 10 % rather than 20 %.

  Tolerances (from `📓️bestest-contract.md` and its 2026-09-06 amendment): annual heating within 20 %
  and annual cooling within 10 % relative of the translated EnergyPlus run; peaks within 25 %;
  free-float min/max/mean within ±2.5 K; hourly zone temperature RMSE ≤ 2.0 K. Every metric is reported separately, so a case that agrees on annual
  energy but disagrees on peaks says exactly that.

  @id-case-parameters
  @level-quick
  @mode-differential
  Scenario Outline: Case <case> model states what ANSI/ASHRAE 140 §5.2 states
    Given the committed case model shared://🏛️bestest-<case>/🔋️model.json
    When both implementations derive the case parameters the standard publishes from that model alone
    Then the areas, air-to-air U-values, glazing, infiltration rate, internal gain and ground temperature agree
    Examples:
      | case   |
      | 600    |
      | 610    |
      | 620    |
      | 630    |
      | 640    |
      | 650    |
      | 900    |
      | 910    |
      | 920    |
      | 930    |
      | 940    |
      | 950    |
      | 600FF  |
      | 900FF  |

  @id-annual-energy
  @level-long
  @mode-differential
  Scenario Outline: Case <case> annual heating and cooling agree with EnergyPlus
    Given the committed case model shared://🏛️bestest-<case>/🔋️model.json
    And the committed annual weather shared://🌦️denver-tmy/🌦️.epw
    And the committed EnergyPlus reference shared://🏛️bestest-<case>/🔮️energyplus.json
    When both implementations simulate the committed case model for a full year at an hourly zone timestep
    Then the annual heating and cooling energies agree within the declared tolerance
    Examples:
      | case  |
      | 600   |
      | 610   |
      | 620   |
      | 640   |
      | 900   |
      | 910   |
      | 920   |
      | 940   |

  @id-peak-load
  @level-long
  @mode-differential
  Scenario Outline: Case <case> peak heating and cooling agree with EnergyPlus
    Given the committed case model shared://🏛️bestest-<case>/🔋️model.json
    And the committed annual weather shared://🌦️denver-tmy/🌦️.epw
    And the committed EnergyPlus reference shared://🏛️bestest-<case>/🔮️energyplus.json
    When both implementations simulate the committed case model for a full year at an hourly zone timestep
    Then the peak heating and cooling demands agree within the declared tolerance
    Examples:
      | case  |
      | 600   |
      | 900   |

  @id-free-float
  @level-long
  @mode-differential
  Scenario Outline: Case <case> free-float zone temperatures agree with EnergyPlus
    Given the committed case model shared://🏛️bestest-<case>/🔋️model.json
    And the committed annual weather shared://🌦️denver-tmy/🌦️.epw
    And the committed EnergyPlus reference shared://🏛️bestest-<case>/🔮️energyplus.json
    When both implementations simulate the unconditioned case model for a full year at an hourly zone timestep
    Then the annual minimum, maximum and mean zone air temperatures agree within the declared tolerance
    Examples:
      | case   |
      | 600FF  |
      | 900FF  |

  @id-hourly-temperature
  @level-exhaustive
  @mode-differential
  Scenario Outline: Case <case> hourly zone air temperature tracks EnergyPlus
    Given the committed case model shared://🏛️bestest-<case>/🔋️model.json
    And the committed annual weather shared://🌦️denver-tmy/🌦️.epw
    And the committed EnergyPlus reference shared://🏛️bestest-<case>/🔮️energyplus.json
    When both implementations simulate the committed case model for a full year at an hourly zone timestep
    Then the root-mean-square difference across all 8760 hourly zone air temperatures is within the declared tolerance
    Examples:
      | case   |
      | 600    |
      | 600FF  |
      | 900FF  |

  @id-model-round-trip
  @level-quick
  @mode-round-trip
  Scenario: The committed case models are exactly what the case builders produce
    Given the committed case model shared://🏛️bestest-600/🔋️model.json
    When the committed model is decoded through this subset's own canonical JSON and re-encoded
    Then the re-encoded bytes are identical and the decoded model equals the registered case builder's own output
