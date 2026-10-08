# 🏛️ Architecture — quizze.architektur-und-technologie.de

The quiz site for architecture and technology students: it trains the feeling for the numbers behind energy-efficient
buildings. The site `https://quizze.architektur-und-technologie.de` offers the catalog [`❓️quiz/🔣️.json`](❓️quiz/🔣️.json)
(id `architecture`, title "Architecture and Technology Quizzes" / "Quizze zu Architektur und Technologie") through the
[proctor](../🛂️proctor) at `https://semio.iek.uni-hannover.de`, which records runs, scores, badges and the leaderboard.

| Part | Where |
|---|---|
| Catalog: introduction, quiz paths, badges | `❓️quiz/🔣️.json` |
| Web package `@teaching/architecture-quiz`: static build for the CDN with the proctor origin baked in; in development vite on port 6061 proxies the gateway routes to the proctor on 8791 (develop, check, test and publish via the dashboard commands in [its README](❓️quiz/README.md)) | `❓️quiz/📦️packages/🟦️typescript` |
| Deployment: the site as a static artifact on a CDN; the proctor as an API-only Docker image with Caddy (TLS with a certificate it obtains itself or one the operator supplies) for `semio.iek.uni-hannover.de` | see the site and [proctor](../🛂️proctor/README.md) READMEs |
| Quizzes | `⚡️energy/<topic>/❓️quiz/🔣️.json` |
| Pets: twenty animated companions, each the likeness of a thing the quiz items are about (sunny, cloudy, housy, solary, radiatory, pumpy, windowy, waly, battery, windy, boily, roofy, insuly, shady, venty, chilly, kettly, flamy, thermy, servy), with their bonds and one cast for the home screen and one per quiz; the site loads them lazily and only for a learner who wants pets (roster, casts, bonds and how to add one in [its README](🐾️pets/README.md)) | `🐾️pets/🔣️.json`, `🐾️pets/<species>/🔣️.json`, `🐾️pets/🟦️.ts` |

## Quizzes

| Quiz | Tasks (kind, items, drawn per run) | What it trains |
|---|---|---|
| 🧲 `physics` — Physical Understanding / Physikalisches Verständnis | power or energy? (classification, 16, 12); powers from tea light to Sun (sorting, 14, 10, W); energies from phone charge to world energy use (sorting, 12, 9, Wh) | telling rates from amounts; orders of magnitude over 25 decades |
| 🔥 `heating` — Heating / Heizen | U-values of building components (matching, 17, 10, W/(m²·K)); heating load and heating demand of buildings (matching with two dimensions, 11, 8, W/m² and kWh/(m²·a)) | envelope quality from single glazing to the passive-house roof; building age and standard |
| ❄️ `cooling` — Cooling / Kühlen | air change rates by use (matching, 15, 10, 1/h); cooling load and cooling demand (matching with two dimensions, 9, 7, W/m² and kWh/(m²·a)) | ventilation from warehouse to cleanroom; internal and solar loads, full-load hours |
| 📊 `demand` — Energy Demand / Energiebedarf | energy standards and their spider-diagram profiles (classification, 6 standards onto 6 profiles); final energy demand of buildings with their supply systems (matching, 9, 7, kWh/(m²·a)) | how envelope, airtightness, heat recovery and supply system add up |

All magnitudes use logarithmic scales, so confusing neighbours costs little and confusing orders of magnitude costs a
lot (design §6 of the ticket `QUIZ-PRODUCT-AND-TEACHING-PROCTOR`).

## Badges

| Badge | Label (en / de) | Rule |
|---|---|---|
| 🧲 `physics-expert` | Physics Expert / Physik-Profi | a run of `physics` scored 100 % |
| 🔥 `heating-expert` | Heating Expert / Heiz-Profi | a run of `heating` scored 100 % |
| ❄️ `cooling-expert` | Cooling Expert / Kühl-Profi | a run of `cooling` scored 100 % |
| 📊 `demand-expert` | Energy Demand Expert / Energiebedarfs-Profi | a run of `demand` scored 100 % |
| 🧮 `numerical-brain` | Numerical Brain / Zahlenhirn | every sorting task of the catalog scored 100 % in some run |
| 🔍 `pattern-seer` | Pattern Seer / Musterblick | every classification task of the catalog scored 100 % in some run |
| 🏁 `completionist` | Completionist / Rundum dabei | every quiz has a submitted run |

## Sources

Values are representative, rounded and stated in every item's explanation together with the reasoning. Derived values
follow the conventions below the tables.

### Physical Understanding

| Source | Used for |
|---|---|
| AG Energiebilanzen, Energieverbrauch in Deutschland im Jahr 2024 (2025) | primary energy 10,529 PJ; gross electricity consumption 527 TWh → 60 GW mean load |
| Energy Institute, Statistical Review of World Energy 2025 | world primary energy 592 EJ (2024) → 164,000 TWh, 18.7 TW |
| Deutsche WindGuard, Status des Windenergieausbaus an Land 2024 | 635 new turbines with 3,251 MW → 5.1 MW mean rating |
| IAU 2015 Resolution B3; solar constant 1,361 W/m² | solar luminosity 3.828 × 10²⁶ W; 174 PW intercepted by the Earth |
| ISO 7730 / ISO 8996 | 1 met = 58 W/m², a sitting person ≈ 100 W |
| IEC 60904-3 | 1,000 W/m² standard irradiance, module peak power |
| DIN 51603-1; calorific value of petrol | heating oil ≈ 10 kWh/l, petrol ≈ 8.8 kWh/l |
| Stromspiegel für Deutschland; Fraunhofer ISE, Aktuelle Fakten zur Photovoltaik | household electricity 2,500 kWh/a; 950 kWh/kWp |
| Kernkraftwerk Isar 2 operating data; DB class 403 | 1,410 MW net; ICE 3 8 MW |
| IWU, Deutsche Wohngebäudetypologie (2015) | heating demand and load of the 1960s house (type EFH_E) |

### Heating

| Source | Used for |
|---|---|
| BMWi/BMI, Regeln zur Datenaufnahme und Datenverwendung im Wohngebäudebestand, BAnz AT 04.12.2020 B1, Tables 2 and 3 | U-values of existing components: single glazing 5.8, metal window 4.3, roller-shutter box 3.6, box-type window 2.7, solid roof 2.1, half-timbered wall 1.5, hollow-block wall 1.0, masonry 1984–1994 0.6 |
| Gebäudeenergiegesetz (GEG) 2024, Annex 1, § 15, § 16 | reference U-values: wall 0.28, floor 0.35, roof 0.20, window 1.3, door 1.8; primary energy ≤ 0.55 × reference (§ 15), envelope H′T ≤ 1.0 × reference (§ 16) |
| Passive House Institute, building and component criteria | window 0.80, opaque components ≤ 0.15, 10 W/m² and 15 kWh/(m²·a) |
| EN 673 | Ug of triple glazing ≈ 0.5 |
| IWU, Deutsche Wohngebäudetypologie (2015), appendix C.3 and Table 26 | heating demand and transmission per m² living area of types EFH_E/F/I/J, MFH_B, NBL_GMH_F; modernisation package 2 of MFH_B; EnEV 2016 / KfW 55 / KfW 40 variants |
| DIN EN 12831 | design heating load at −12 °C |

### Cooling

| Source | Used for |
|---|---|
| EN 16798-1, category II (7 l/s per person + 0.35/0.7 l/(s·m²)) | office, classroom, cinema, restaurant, warehouse |
| DIN 1946-6 | nominal ventilation of apartments |
| DIN 18032-1 | 60 m³/h per athlete |
| Garage ordinances of the German states (GaVO) | 6 m³/(h·m²) for garages with little traffic |
| DIN 1946-7 | laboratories 25 m³/(h·m²) |
| DIN 1946-4 | operating rooms 60 m³/(h·m²), ≥ 1,200 m³/h outdoor air |
| VDI 2052 | commercial kitchens 12–30 1/h |
| IEST-RP-CC012, ISO 14644-4 | cleanrooms ISO 7/6/5 |
| Passive House Institute | 30 m³/h per person; overheating ≤ 10 % of hours; non-residential criteria |
| VDI 2078, DIN 4108-2 | cooling loads, sol-air temperature, summer heat protection |

### Energy Demand

| Source | Used for |
|---|---|
| IWU, Deutsche Wohngebäudetypologie (2015) | heating demands of the standards and the deep-retrofit package |
| Passive House Institute | n50 ≤ 0.6 1/h, heat recovery ≥ 75 % |
| DIN V 4108-6 / DIN V 4701-10 | hot water 12.5 kWh/(m²·a); generation and distribution efficiencies |
| Fraunhofer ISE, heat pump field tests in existing buildings | seasonal performance factor ≈ 3 |
| GEG 2024, Annex 10 | efficiency class H above 250 kWh/(m²·a) |

## Conventions for derived values

- Heating load per m² living area of the IWU types = (transmission H_T/A from the IWU typology + ventilation
  0.34 Wh/(m³·K) × n × 2.5 m) × 32 K (20 °C inside, −12 °C outside), with n = 0.6 1/h before 2002 and 0.5 1/h from the
  EnEV 2002 on (airtight envelope required); with heat-recovery ventilation the ventilation term is ≈ 0.10 W/(m²·K).
  The GEG 2024 new build uses the design practice for new builds since 2016 (30–50 W/m²); KfW 55/40 follow from
  H′T ≤ 70 %/55 % of the reference building plus heat recovery; the passive house from the PHI criterion.
- GEG 2024 separates two requirements: the envelope loss H′T may reach that of the reference building (§ 16, 1.0 ×),
  while the primary energy must stay at or below 55 % of the reference (§ 15, since 2023). Heating load and heating
  demand follow the envelope, so a heat-pump house with the minimum envelope keeps ≈ 40 W/m² and ≈ 50 kWh/(m²·a); the
  EnEV 2014 new build (before the envelope tightening of EnEV 2016) sits at ≈ 55 kWh/(m²·a).
- Cooling demand = cooling load × full-load hours (homes ≈ 350, schools ≈ 250, offices ≈ 500–600, hospitals ≈ 1,300,
  data centres ≈ 8,000).
- Ventilation heat loss = 0.34 Wh/(m³·K) × (0.07 × n50 + 0.4 1/h × (1 − heat recovery)) × 2.5 m × 75 kKh.
- Final energy = heating ÷ generation efficiency (old gas boiler 0.8, low-temperature 0.85, condensing 0.9) + hot water
  12.5 ÷ its efficiency + auxiliary energy; heat pumps divide by their seasonal performance factor.
- Energy prices: gas 12 ct/kWh, heat-pump electricity 30 ct/kWh, other electricity 35 ct/kWh, PV feed-in 8 ct/kWh.
