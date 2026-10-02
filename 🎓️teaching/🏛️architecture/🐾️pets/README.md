# 🐾️ Pets — the architecture menagerie

The twenty pets of **quizze.architektur-und-technologie.de**: small animated companions that stand on the cards of the
quiz site, look at the pointer, blink, wander a little and meet each other. Every pet is the likeness of a thing the quiz
items are about, so the pets on screen always fit the topic: the home screen shows the owner's nine in turn together
with one visitor from the other eleven, a quiz's page and its runs show the troupe of that quiz.

This folder is the domain-specific half. The domain-neutral half is the pets product
([`🧰️framework/🛍️products/🐾️pets`](../../../🧰️framework/🛍️products/🐾️pets/README.md)): the schema of a species, the
deterministic stage that folds events into frames and the React layer that draws them. The quiz knows no species either:
the site hands this menagerie to `mountQuiz` (`../❓️quiz/🟦️.ts`, option `pets`) as a function that imports
`🟦️.ts` lazily, so the menagerie is a script chunk of its own that only a learner who wants pets downloads, and nothing
is fetched at run time (the release document's Content-Security-Policy admits connections to the proctor alone).

| Path | What |
|---|---|
| `🔣️.json` | the ensemble (`semio.pets.ensemble/v1`, id `architecture`): the species paths in roster order, the bonds, the casts |
| `<emoji><id>/🔣️.json` | one species each (`$defs/Species`): names and thing in English and German, grounds, size, palette, bones, parts, face, clips, repertoire, locomotion, temperament |
| `🟦️.ts` | static imports of all of the above → `ARCHITECTURE_MENAGERIE`, its only export |

## Roster

The owner's nine come first, in the owner's order; the other eleven were derived from the quiz items. Sizes are CSS
pixels at scale 1 (width × height). A ground is `<quiz>`, `<quiz>/<task>` or `<quiz>/<task>/<item>` and must exist in
the quiz files.

| # | Directory | Thing (en / de) | Size, gait | Grounded in |
|---:|---|---|---|---|
| 1 | `☀️sunny` | sun / Sonne | 46 × 51, floats | physics: the Sun, sunlight on Earth and per square metre, world primary energy, PV peak power; cooling: loads and demands (task) |
| 2 | `☁️cloudy` | cloud / Wolke | 58 × 43, floats | physics and cooling as topics; the clear-sky square metre, the PV annual yield, cooling loads and demands (task) |
| 3 | `🏠️housy` | house / Haus | 48 × 52, walks | heating: load and demand (task); demand: both tasks; physics: heating load, heating demand, energy certificate, the old house |
| 4 | `🔆️solary` | solar panel / Solarmodul | 46 × 52, walks | physics: PV peak power, PV annual yield, sunlight per square metre; demand: plus-energy house, KfW 55 with gas and solar |
| 5 | `♨️radiatory` | radiator / Heizkörper | 54 × 55, walks | heating: the passive-house window ("no radiator needed"), load and demand (task); physics: heating loads; demand: final energy (task) |
| 6 | `🌀️pumpy` | heat pump / Wärmepumpe | 46 × 46, walks | demand: the three heat-pump houses, KfW 40, passive house, plus-energy house; heating: GEG 2024 |
| 7 | `🪟️windowy` | window / Fenster | 40 × 52, hops | heating: six windows and glazings; cooling: classroom, attic flat, 1970s office; demand: plus-energy house |
| 8 | `🧱️waly` | wall / Wand | 48 × 46, walks | heating: five walls, the Plattenbau, the Gründerzeit building |
| 9 | `🔋️battery` | battery / Akku | 42 × 52, walks | physics: car battery, phone charge, wall box, petrol tank (in all three tasks) |
| 10 | `💨️windy` | wind turbine / Windenergieanlage | 48 × 56, walks | physics: wind turbine, its year of energy, the ICE train |
| 11 | `🔥️boily` | boiler / Heizkessel | 48 × 53, walks | demand: five gas houses, three standards; physics: a litre of heating oil |
| 12 | `🛖️roofy` | roof / Dach | 56 × 42, walks | heating: three roofs; cooling: attic flat; demand: old building, plus-energy house; physics: PV annual yield |
| 13 | `🧶️insuly` | insulation / Dämmung | 48 × 40, hops | heating: the insulated walls and roofs, load and demand (task); demand: WSchVO 1995, deep retrofit, final energy (task) |
| 14 | `😎️shady` | external sun shading / Sonnenschutz | 44 × 48, walks | cooling: six shaded and unshaded buildings; heating: roller-shutter box; demand: passive house |
| 15 | `🌬️venty` | ventilation unit / Lüftungsgerät | 44 × 41, floats | cooling: air change rates (task), passive-house office; demand: standard profiles (task), two houses; heating: KfW 40, Gründerzeit retrofit |
| 16 | `❄️chilly` | chiller / Kältemaschine | 52 × 44, walks | cooling as a topic; passive-house home, new school, hospital, 1970s office, data centre |
| 17 | `🫖️kettly` | kettle / Wasserkocher | 46 × 44, walks | physics: the kettle (power, twice), boiling water, the old house's "nine kettles" |
| 18 | `🕯️flamy` | tea light / Teelicht | 28 × 46, walks | physics: the tea light (twice), powers (task: "From Tea Light to Sun") |
| 19 | `🌡️thermy` | thermometer / Thermometer | 32 × 52, walks | heating and cooling as topics (20 °C, −12 °C, 26 °C); a 2000s house, cooling loads (task), attic flat, boiling water |
| 20 | `🖥️servy` | server rack / Serverschrank | 46 × 56, walks | cooling: the data centre |

The display name of a pet is `<Nickname>, the <thing>` / `<Nickname>, <Artikel> <Ding>` (`Species.name`). The pets
themselves are decoration and hidden from assistive technology, so they carry no accessible name; the preferences of the
quiz list the names of the pets that are on stage right now as text, in the learner's language.

## Casts

A scene is `home` or the id of a quiz. The layer shows at most six pets from 1024 px of width, four from 768 px and two
below; a scene the ensemble does not name falls back to `home`. Whenever a scene has a rotation and room for two, one
place belongs to a visitor from the rotation and the core has the others: a core that fits is on stage as a whole, a
larger one takes turns (at home on a desktop: five of the owner's nine and one visitor; on a phone: one and one). Places
the core leaves empty go to further visitors. Turns change every one to two minutes, never while the pets are `still` or
a run is on.

| Scene | Core | Rotation |
|---|---|---|
| `home` | sunny, cloudy, housy, solary, radiatory, pumpy, windowy, waly, battery | windy, boily, roofy, insuly, shady, venty, chilly, kettly, flamy, thermy, servy |
| `physics` | kettly, battery, sunny | flamy, windy, solary, cloudy |
| `heating` | radiatory, waly, windowy | housy, insuly, roofy, thermy |
| `cooling` | chilly, shady, venty | sunny, cloudy, servy, thermy |
| `demand` | pumpy, boily, solary | housy, insuly, venty, radiatory |

Every species in the cast of a quiz carries at least one ground in that quiz, every species is in the cast of `home`,
and the core of every quiz holds at least one pair with a bond, so there is always somebody to like or to bicker with.

## Bonds

A bond is the authored affinity of two species, from −1 (they squabble) to +1 (they adore each other); a pair that is
not listed is neutral. From 0.4 two pets mostly cuddle when they meet, from −0.3 downwards they mostly squabble and
sulk, in between they greet; shared history shifts the feeling a little and drifts back. Disputes stay small: the core
never lets the effective affinity fall below −0.6. All 67 bonds, strongest first:

| Pair | Affinity | Why |
|---|---:|---|
| pumpy – radiatory | +0.9 | the heat pump serves low-temperature radiators; the radiator delivers its warmth into the room |
| sunny – solary | +0.9 | PV turns sunlight (1,000 W/m² at test) into electricity |
| waly – insuly | +0.9 | insulation wraps the wall: U 1.0 → 0.28 → 0.15 W/(m²·K) |
| roofy – insuly | +0.8 | 35 cm of insulation give the roof U 0.10 instead of 2.1 |
| pumpy – insuly | +0.7 | a deep retrofit cuts what the heat pump has to buy from 122 to 23 kWh/(m²·a) |
| roofy – solary | +0.7 | the roof carries the PV |
| shady – chilly | +0.7 | external shading spares the chiller: 6 W/m² shaded against 100 W/m² unshaded |
| battery – solary | +0.6 | the battery stores the module's surplus: the amount behind the rate |
| chilly – pumpy | +0.6 | siblings: a heat pump is a chiller run backwards |
| cloudy – windy | +0.6 | wind drives clouds, and a gust spins the rotor |
| insuly – housy | +0.6 | insulation is the coat of the house |
| pumpy – solary | +0.6 | plus-energy house: the PV roof feeds the heat pump |
| pumpy – venty | +0.6 | the passive-house compact unit: heat recovery plus a small heat pump |
| radiatory – boily | +0.6 | an old couple: radiators at boiler flow temperatures |
| servy – chilly | +0.6 | the chiller works around the clock for the data centre |
| shady – windowy | +0.6 | the shading sits on the window's head and keeps the glass from overheating |
| waly – roofy | +0.6 | teammates of the opaque envelope |
| battery – windy | +0.5 | the battery buffers the turbine's gusty output |
| chilly – venty | +0.5 | teammates in air conditioning: filtered, cooled supply air |
| flamy – boily | +0.5 | the same family of burners |
| flamy – sunny | +0.5 | the smallest and the largest of the powers; the tea light looks up to the Sun |
| roofy – housy | +0.5 | the roof is the hat of the house |
| solary – windy | +0.5 | complementary renewables: sun by day and summer, wind by night and winter |
| waly – housy | +0.5 | walls make the house |
| windowy – housy | +0.5 | windows are the eyes of the house |
| chilly – thermy | +0.4 | the chiller takes orders from the 26 °C line |
| kettly – boily | +0.4 | both boil water |
| pumpy – housy | +0.4 | the house is the home the heat pump heats |
| radiatory – housy | +0.4 | the radiator delivers the heating load of the house |
| radiatory – thermy | +0.4 | the thermometer keeps the radiator at the 20 °C design line |
| servy – venty | +0.4 | server halls live on moving, filtered air |
| shady – cloudy | +0.4 | a cloud is shade for free: a day off for the blind |
| sunny – windy | +0.4 | winds are driven by the Sun heating the atmosphere |
| venty – housy | +0.4 | the house breathes through its ventilation |
| venty – insuly | +0.4 | the passive-house trio: insulation, airtightness, heat recovery |
| battery – cloudy | +0.3 | the battery bridges the gaps a cloud leaves in the solar supply |
| boily – housy | +0.3 | the boiler is the old heart of the house |
| chilly – cloudy | +0.3 | overcast days lower the cooling load |
| kettly – cloudy | +0.3 | the kettle's steam looks like a baby cloud |
| pumpy – thermy | +0.3 | the seasonal performance depends on the temperature lift |
| radiatory – waly | +0.3 | the radiator hangs on the wall, the wall stores the warmth |
| servy – kettly | +0.3 | fellow heaters: every watt ends as heat |
| shady – housy | +0.3 | the shading guards the house against the summer sun |
| solary – housy | +0.3 | the module lives on the roof of the house |
| sunny – housy | +0.3 | winter solar gains warm the house |
| venty – waly | +0.3 | an airtight envelope lets the ventilation do all the breathing |
| windowy – sunny | +0.2 | the window loves winter sun and flinches in summer |
| boily – insuly | −0.2 | insulation shrinks the boiler's job |
| roofy – windowy | −0.2 | roof windows leak heat and gain sun |
| battery – kettly | −0.3 | amount against rate: 2 kW would empty a phone charge in under a minute |
| boily – solary | −0.3 | fossil against sun |
| chilly – windowy | −0.3 | unshaded glazing is the chiller's biggest load |
| kettly – flamy | −0.3 | 2 kW against 35 W: the kettle boasts |
| radiatory – windowy | −0.3 | single glazing makes the radiator work overtime, a passive-house window makes it redundant |
| servy – thermy | −0.3 | the rack runs hot and the thermometer fusses |
| solary – shady | −0.3 | a shadow on a module drops its output |
| thermy – sunny | −0.3 | the Sun drives the mercury up |
| chilly – sunny | −0.4 | solar gains are the chiller's workload |
| roofy – sunny | −0.4 | the Sun heats the roof skin to 60–70 °C |
| sunny – cloudy | −0.4 | a playful tease: the cloud takes away the kilowatt per square metre of a clear noon |
| windowy – venty | −0.4 | window ventilation loses the heat a ventilation unit recovers |
| flamy – cloudy | −0.5 | rain and gusts snuff out a flame |
| sunny – shady | −0.5 | the blind blocks the Sun's gains |
| windowy – waly | −0.5 | heat-loss bickering: 1.3 against 0.28 W/(m²·K) |
| pumpy – boily | −0.6 | seasonal performance 3 against an efficiency of 0.8 to 0.9 for the same old house |
| chilly – radiatory | −0.7 | opposite jobs: heating and cooling at once wastes energy |
| solary – cloudy | −0.7 | module output follows irradiance; the module sulks under a cloud |

## Adding a pet

1. **Name and directory.** Pick the thing, its nickname (`<thing>y`) and an emoji no other species uses. Register the
   directory name `<emoji><id>` in the repo taxonomy (`🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json`,
   `semanticDirectoryMemberKinds` → `members-of-teaching-pets` → `memberNames`) and create the directory here; copy the
   emoji (with its variation selector) from the taxonomy, never retype it.
2. **Species document.** `<emoji><id>/🔣️.json` starting with
   `"$schema": "../../../../🧰️framework/🛍️products/🐾️pets/🧬️schema/🔣️.json#/$defs/Species"`: `id`, `name` and `thing`
   in English and German, `grounds`, `size`, `palette`, then the rig. Art direction: 40 to 56 px tall, feet on the
   origin, facing right, a 2 px ink outline, two eyes and a mouth; an idle loop, the gait clip, two signature fidgets,
   `greet`, `cuddle`, `squabble`, `sulk`, `sleep` and `land`. Clips that do not loop start and end on rest values.
3. **Grounds.** Name every quiz item, task or topic the pet stands for. A pet may only join the cast of a quiz it is
   grounded in.
4. **Ensemble.** In `🔣️.json` add the path to `species`, the bonds of the pet (each with a physical reason, recorded in
   the table above) and the pet to the rotation of `home` and to the casts of the quizzes it belongs to.
5. **Module.** In `🟦️.ts` add the import and the species to the list, in the order of the ensemble.
6. **Check.** `bun nx run @teaching/architecture-quiz:test` (suite `🐾️pet-cast`: schema with ajv, the product's own
   validators, grounds, casts, bonds, the module against the documents) and look at the pet in the gallery.

## Looking at them

The stories gallery of the pets product shows every species large — at rest and with every clip, on a light and a dark
ground — and a sandbox with mock cards on which a chosen cast lives (scene, mode, quiet, capacity, scale, seed).

| Where | Entry | Address |
|---|---|---|
| Claude preview (`.claude/launch.json`) | `architecture-pets-stories` | http://127.0.0.1:6074/ |
| VS Code (`.vscode/launch.json`, group `3_dev`) | `🛠️dev🎓️teaching🏛️architecture🐾️pets📖️stories` | http://127.0.0.1:6074/ |

Both run `bun nx run @semio-tech/pets-react:dev` with `PETS_STORIES_PORT=6074` and
`PETS_MENAGERIE=🎓️teaching/🏛️architecture/🐾️pets/🟦️.ts`. In the real site the pets are on by default (`calm`); the
preferences offer `off`, `still`, `calm` and `lively`, the footer of every screen carries a "Show pets" switch that
turns them off and back to the last other choice, and a device that asks for reduced motion decides the default only:
its learner gets them motionless until they choose (the preferences say so), and what a learner chose there or with
the switch holds on every device.

## Tests

| Suite | What |
|---|---|
| `../❓️quiz/🧪️tests/🐾️pet-cast/🟦️.ts` (vitest) | every species and the ensemble against the draft-07 contract (ajv) and the product's validators; the assembled menagerie has no issue and equals what `🟦️.ts` exports; grounds exist in the quiz files; every cast member of a quiz is grounded in it; every quiz and `home` have a cast; bonds join existing species |
| `../❓️quiz/🧪️tests/🐕️pet-walk/🟦️.ts` (Playwright, end-to-end gate, project `pets`) | pets appear on the home screen and stand on the cards, hidden from assistive technology and never the target of a click; pupils follow the pointer, a pet blinks, a pet walks and two pets meet (`lively`, at the test tempo); the cast follows the learner into a quiz's page and run; the footer switch works by keyboard on every screen and is remembered; `still` gives motionless pets and `off` removes them; a device that asks for reduced motion gives motionless pets and a note until the learner chooses, and calm pets the learner chose walk, also after a reload (at the test tempo); no console error, failed request or policy violation |
