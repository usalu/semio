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
| `🔣️.json` | the ensemble (`semio.pets.ensemble/v1`, id `architecture`): the species paths in roster order, the bonds, the casts, the chemistry |
| `<emoji><id>/🔣️.json` | one species each (`$defs/Species`): names and thing in English and German, grounds, size, palette, bones, parts, face, clips, repertoire, locomotion, temperament, states, tricks, purr, emitters, gear, grip, reach, mood (and optionally a canopy) |
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

## States, tricks and gear

Every pet is in one state at a time; the first is its resting state, a state that `lasts` gives way to its `then`
after that many seconds. A trick is set off by its cues — `click` (the second and third click of a streak, in this
order), `circle` and `countercircle` (the pointer circling the pet clockwise or counter-clockwise: one rung up or down
the states its `from` lists, bottom first), `stroke`, `shake` (shaken while held), `whim` (on its own, in a willing
mood) and `show` (to a partner) — and only in the states its `from` names; it leaves the pet in its `to`. A trick
without cues (`[—]`) is played only when the chemistry asks for it. Gear brings what a pet can do to get somewhere:
`climb` (walls), `ladder`, `grapple` (a hook on a line) and `parachute`; the two floaters and venty own none.

| Pet | Gear | States (resting first; lasts → then) | Tricks [cues] (from; → to) |
|---|---|---|---|
| sunny | — | `shining`, `blazing` 30 s → `shining`, `dim` 60 s → `shining`, `sunset` 45 s → `dim` | `corona` [click, whim], `prominence` [click, shake, show], `high-noon` [circle] and `sundown` [countercircle] along sunset/dim/shining/blazing, `rainbow` [—] (shining, blazing) |
| cloudy | — | `fluffy`, `wispy` 60 s → `fluffy`, `heavy` 40 s → `raining`, `raining` 12 s → `fluffy` | `rain` [click, shake] → `raining`, `thunder` [click] → `heavy`, `condense` [circle] and `evaporate` [countercircle] along wispy/fluffy/heavy/raining, `gust` [show], `snow` [—] (fluffy, heavy) |
| housy | ladder, grapple | `cosy`, `cold` 45 s → `cosy`, `wrapped` 90 s → `cosy`, `hot` 30 s → `cosy` | `energy-label` [click], `smoke-rings` [click, whim, show], `pull-on-coat` [circle] (cosy, wrapped → `wrapped`), `warm-through` [circle] (cold → `cosy`), `air-out` [countercircle] along cold/cosy/wrapped, `heat-loss` [whim] (cosy → `cold`), `rattle-roof` [shake] |
| solary | climb, ladder, parachute | `generating`, `shaded` 30 s → `generating`, `peak` 40 s → `hot`, `hot` 20 s → `generating` | `sun-track` [click, whim], `cell-wave` [click], `power-up` [circle] and `duck` [countercircle, shake] along shaded/generating/peak, `feed-battery` [show] |
| radiatory | climb, ladder | `warm`, `cold` 60 s → `warm`, `hot` 40 s → `warm` | `bleed` [click, shake] → `warm`, `glow-wave` [click, show] → `hot`, `turn-up` [circle] and `turn-down` [countercircle] along cold/warm/hot, `towel-dry` [whim] (warm, hot) |
| pumpy | parachute | `heating`, `standby` 60 s → `heating`, `cooling` 40 s → `heating`, `defrost` 8 s → `heating` | `three-for-one` [click] → `heating`, `reverse-cycle` [click] (heating, standby, defrost → `cooling`), `ramp-up` [circle] and `whisper` [countercircle] along standby/heating, `flow-temperature` [show] (heating), `defrost-shake` [whim, shake] (heating, standby → `defrost`) |
| windowy | climb, parachute | `shut`, `tilted` 25 s → `shut`, `open` 20 s → `tilted`, `fogged` 12 s → `shut` | `crack-open` [click] → `tilted`, `fog-draw` [click] (shut, tilted → `fogged`), `swing-wide` [circle] and `close-up` [countercircle, shake] along shut/tilted/open, `glaze-show` [whim, show] → `shut` |
| waly | ladder | `bare`, `warm` 90 s → `bare`, `wrapped` 120 s → `bare`, `passive` 120 s → `wrapped` | `brick-swap` [click], `layer-reveal` [click, show], `warm-up` [circle] and `cool-down` [countercircle] along bare/warm, `salute` [countercircle] (wrapped), `u-value-duel` [—], `crumble` [shake] |
| battery | climb, ladder, grapple | `charged`, `empty` 90 s → `charging`, `charging` 8 s → `full`, `full` 40 s → `charged` | `bar-count` [click, whim] (charged, full → `full`), `zap` [click, show] → `charging`, `charge-up` [circle] and `discharge` [countercircle] along empty/charged/full, `slosh` [shake] → `charged`, `power-vs-energy` [—] |
| windy | parachute | `turning`, `rated` 12 s → `turning`, `becalmed` 45 s → `turning`, `feathered` 8 s → `turning` | `full-power` [click, whim] → `rated`, `yaw-turn` [click], `spin-up` [circle] and `wind-down` [countercircle] along becalmed/turning/rated/feathered, `pinwheel` [whim, show] |
| boily | ladder, grapple | `burning`, `pilot` 60 s → `burning`, `roaring` 20 s → `burning` | `whoomp` [click, show] → `roaring`, `smoke-ring` [click, shake, whim] → `burning`, `fire-up` [circle] and `bank-down` [countercircle] along pilot/burning/roaring, `efficiency-bars` [show], `relight` [—] |
| roofy | ladder, parachute | `dry`, `snow-capped` 60 s → `dry`, `sun-baked` 30 s → `dry`, `wet` 15 s → `dry` | `tile-flip` [click], `tile-wave` [click], `sun-bake` [circle] and `shake-off` [countercircle] along snow-capped/dry/sun-baked, `carry-pv` [show, whim], `clatter` [shake] |
| insuly | climb, parachute | `fluffy`, `compressed` 30 s → `fluffy`, `thick` 40 s → `fluffy`, `soaked` 40 s → `fluffy` | `fluff-burst` [click] (fluffy, thick → `thick`), `tuck-in` [click], `bulk-up` [circle] and `flatten` [countercircle] along compressed/fluffy/thick, `dry-shake` [whim, click, circle, countercircle] (soaked → `fluffy`), `itch` [shake] |
| shady | ladder, parachute | `lowered`, `raised` 40 s → `lowered`, `tilted` 40 s → `lowered` | `slat-wave` [click] → `tilted`, `roll-down` [click] (lowered, tilted → `lowered`), `lower` [circle] and `raise` [countercircle] along raised/tilted/lowered, `sun-block` [—], `pose` [whim, show], `clatter` [shake] |
| venty | — | `nominal`, `low` (until circled up), `boost` 15 s → `nominal`, `bypass` 30 s → `nominal` | `spin-up` [click] → `boost`, `heat-recovery` [click], `air-up` [circle] and `air-down` [countercircle, shake] along low/nominal/boost, `night-cool` [whim, show] → `bypass` |
| chilly | climb, parachute | `cooling`, `standby` (until circled up or crowned), `frosted` 30 s → `cooling`, `overloaded` 30 s → `cooling` | `ice-crown` [click] (standby, cooling, overloaded → `frosted`), `flurry` [click, whim, shake] → `cooling`, `cool-down` [circle] and `thaw` [countercircle] along standby/cooling/frosted, `chill-out` [show] → `overloaded` |
| kettly | grapple, parachute | `cold`, `lukewarm` 40 s → `cold`, `heating` 6 s → `boiling`, `boiling` 4 s → `lukewarm` | `boil` [click, shake] → `boiling`, `whistle` [click] → `boiling`, `heat-up` [circle] and `cool-down` [countercircle] along cold/lukewarm/heating/boiling, `boast` [show, whim], `pour` [whim] |
| flamy | parachute | `burning`, `ember` 40 s → `burning`, `high` 15 s → `burning`, `snuffed` 20 s → `ember` | `spark-dance` [click, whim], `ember-whoosh` [click] → `high`, `stoke` [circle] and `shield` [countercircle] along ember/burning/high, `relight` [click] (snuffed → `burning`), `blow-out` [shake] → `snuffed`, `look-up` [—] (burning, high) |
| thermy | grapple, parachute | `mild`, `cold` 30 s → `mild`, `hot` 20 s → `mild` | `self-reading` [click] → `mild`, `tap-check` [click, shake], `warm-read` [circle] and `cool-read` [countercircle] along cold/mild/hot, `measure` [whim, show] |
| servy | grapple | `idle`, `busy` 20 s → `idle`, `hot` 30 s → `throttled`, `throttled` 20 s → `idle` | `led-wave` [click, whim] → `busy`, `overheat` [click] → `hot`, `load-up` [circle] along idle/busy/hot, `throttle-down` [countercircle] (busy, hot → `throttled`), `reboot` [shake] → `idle` |

Every pet also purrs (its own `purr` clip and particles) and has the clips of the hand and of its gear. A few states
are reached only through the chemistry: housy `hot`, waly `wrapped` and `passive`, roofy `wet`, insuly `soaked`.

## Chemistry

The `chemistry` of `🔣️.json` is what the pets do to each other, by what they are and what state and mood they are in
(design-v2 §19; the rules come from `📓️explore2-species-content.md` §7 of ticket `2026/10/02/QUIZ-PETS`). A reaction
reads: *when* an actor that matches `when` (a species, optionally its state — held at least `held` seconds —, its
mood, its activity or the trick it performs) is within `within` pixels of one that matches `near` (the gap between
their bodies; `where` it is seen from the second, anywhere when absent), while no third actor that matches `unless` is
within `within` of the second and the affinity of the two lies within `affinity`, then the effects of `then` happen
to the side they name: a state (begun anew when it is the state already), a mood with an `amount` (0.6 when absent;
`content` soothes a worse mood by the amount), a step of their rapport, the encounter they have next (`greet`,
`cuddle`, `squabble`), a trick (when the species offers it in the state it is in) or something it could start by
itself (`fidget`, `walk`, `hop`, `sleep`). The stage looks twice a second, pair by pair in the order of the species
above, and a reaction rests `every` seconds per pair of species after its turn — whether it happened or not, so a
`chance` is tried once per `every`. Within one look an actor takes the first state, mood, trick and activity that
reach it. A running time of concentration (a run of a quiz) and a still stage have no chemistry.

Distances used below: **24 px** — side by side, as close as two bodies come (resting neighbours keep 8 px, meeting
ones the sum of their `reach`, at most 21 px); **90 px** — neighbours on one card or on cards that touch; **100 px**
— in reach of the thermometer; **120 px** — across a card; **above** — the rain falls on it (they share a column). A
reaction rests 20 s per pair unless the table says otherwise. Ids are the rule of the brief, followed by the
variant where one rule needed several rows. Scenes: where both can be on stage at once (a quiz shows six of its seven,
home five of the owner's nine and one visitor); *gallery* means only a sandbox of the stories gallery puts them together.

| Rule | Ids | When … near … | Then | Why | Scenes |
|---|---|---|---|---|---|
| R01 | `r01-heavy`, `r01-raining` | cloudy heavy or raining, 24 px from sunny | sunny `dim`, grumpy; cloudy playful; rapport −0.05 | a thick cloud cuts the 1,000 W/m² of a clear noon to 100–300 W/m² — their tease | home, physics, cooling |
| R02 | `r02` | sunny blazing, 90 px from solary generating | solary `peak`; both happy; +0.05 | 1,000 W/m² is the test irradiance of the peak rating | home, physics |
| R03 | `r03-heavy`, `r03-raining`, `r03-dim`, `r03-comfort` | cloudy heavy or raining, or sunny dim, 90 px from solary; battery 90 px from a sad solary | solary `shaded`, sad (cloudy −0.05); battery soothes her (content 0.2) and is content | module output follows irradiance; the battery bridges the gap | home, physics |
| R04 | `r04`, `r04-glad` | sunny shining, 90 px from solary shaded (no cloudy within 90 px of her) or generating | `shaded` → `generating`, happy; generating: happy | PV turns sunlight into electricity | home, physics |
| R05 | `r05` | sunny blazing for 20 s, 90 px from cloudy fluffy | cloudy `heavy`, curious; sunny proud | sunshine drives convection, convection builds towering clouds | home, physics, cooling |
| R07 | `r07-shining`, `r07-blazing` | cloudy raining, 90 px from sunny shining or blazing; rests 180 s | sunny plays `rainbow`; both happy; +0.05 | sunlight refracted in the drops | home, physics, cooling |
| R08 | `r08-wispy`, `r08-fluffy` | cloudy wispy or fluffy, 24 px from sunny | sunny grumpy (0.4); cloudy playful | a thin cloud lets about 90 % of the light through: only teasing | home, physics, cooling |
| R09 | `r09` | windy rated, 90 px from cloudy | cloudy playful and drifts off (walk) | wind drives clouds | home, physics |
| R10 | `r10` | cloudy performing `gust`, 90 px from windy | windy `rated`, happy; +0.05 | a gust spins the rotor | home, physics |
| R11 | `r11` | sunny blazing, 90 px from windy becalmed | windy `turning`, happy; +0.05 | the Sun heats the air and drives the wind | home, physics |
| R12 | `r12-generating`, `r12-peak` | windy becalmed, 90 px from solary generating or peak | windy sad (0.5); solary cuddles it next | complementary renewables: sun by day and in summer, wind by night and in winter | home, physics |
| R13 | `r13-burning`, `r13-high` | cloudy raining, 90 px from flamy burning or high | flamy `snuffed`, scared; cloudy sad ("oops"); −0.1 | rain snuffs a flame | home, physics |
| R14 | `r14-damp-burning`, `r14-damp-high`, `r14-gust`, `r14-gale` | cloudy heavy, or windy rated, 90 px from flamy | heavy: flamy `ember`; rated: burning → `high`, high → `snuffed` (chance 0.3); flamy scared | damp air starves a flame, a gust fans it and then blows it out | home, physics (gusts: physics) |
| R15 | `r15-snuffed`, `r15-ember` | boily, 90 px from flamy snuffed or ember | boily plays `relight`, proud; flamy `burning`, happy; +0.05 | the same family of burners; a pilot flame is there to relight | gallery |
| R16 | `r16-heating`, `r16-boiling` | kettly heating or boiling, 90 px from flamy snuffed | flamy `burning` and plays its `relight`; kettly proud | an electric spark relights a wick | physics |
| R17 | `r17` | kettly boiling, 90 px from flamy | kettly plays `boast`, proud; flamy sad; −0.05 | 2 kW against 35 W: 57 times | physics |
| R18 | `r18-heating`, `r18-boiling` | kettly heating or boiling, 90 px from battery | battery plays `power-vs-energy`; both grumpy (0.4); −0.05 | a rate against an amount: a 2 kW kettle empties a 15 Wh phone charge in under a minute | home, physics |
| R19 | `r19-wispy`, `r19-fluffy` | kettly boiling, 90 px from cloudy wispy or fluffy | cloudy plays `condense` (one rung up); both happy; +0.05 | the kettle's steam looks like a baby cloud | home, physics |
| R20 | `r20-busy`, `r20-hot` | kettly boiling, 90 px from servy busy or hot | servy `hot`; both content; they cuddle next | fellow heaters: every watt ends as heat | gallery |
| R21 | `r21-warm`, `r21-hot` | radiatory warm or hot, 90 px from windowy open | radiatory grumpy, huffs (fidget); windowy playful; they squabble next; −0.05 | an open window throws away what the radiator delivers | home, heating |
| R22 | `r22-warm`, `r22-hot` | radiatory warm or hot, 90 px from windowy tilted; every 6 s with chance 0.5 | radiatory grumpy (0.35); windowy `shut` with its `close-up` | a tilted window still leaks | home, heating |
| R23 | `r23` | radiatory cold, 90 px from windowy shut | windowy `fogged`, sad | warm moist air condenses on cold glass | home, heating |
| R24 | `r24-fogged`, `r24-shut` | radiatory warm, 90 px from windowy fogged or shut | `fogged` → `shut`; windowy happy; radiatory content | a radiator under the window warms the pane | home, heating |
| R25 | `r25` | radiatory hot, 100 px from thermy | thermy `hot`, flusters (fidget); radiatory proud | the thermometer reads the heat it keeps at the 20 °C line | home, heating |
| R26 | `r26-cosy`, `r26-wrapped` | windowy open for 12 s, 90 px from housy cosy or wrapped | cosy → `cold`, sad; wrapped: sad (0.3), shivers (fidget); windowy playful | ventilation heat loss | home, heating |
| R27 | `r27-open`, `r27-tilted` | windowy open or tilted, 90 px from venty | venty grumpy, fusses (fidget); windowy sad ("I was only airing"); −0.05 | window ventilation loses the heat the unit recovers (75 %) | home |
| R28 | `r28` | waly, 90 px from windowy; rests 180 s | waly plays `u-value-duel`; they squabble next | 1.3 against 0.28 W/(m²·K): 4.6 times | home, heating |
| R29 | `r29` | boily roaring, 90 px from radiatory cold | radiatory `warm`; both content; +0.05 | an old couple: radiators at boiler flow temperatures | home, demand |
| R30 | `r30-warm`, `r30-hot` | radiatory warm or hot, 90 px from housy cold | housy `cosy`, happy; radiatory proud | the radiator delivers the heating load | home, heating, demand |
| R31 | `r31` | housy wrapped, 90 px from boily | boily `pilot`, sad ("0.8"); −0.05 | insulation shrinks the boiler's job (303 → 15 kWh/(m²·a)) | home, demand |
| R32 | `r32-cooling-warm`, `r32-cooling-hot`, `r32-frosted-warm`, `r32-frosted-hot` | chilly cooling or frosted, 90 px from radiatory warm or hot; rests 30 s | they squabble next (after it both are grumpy); −0.1 | heating and cooling at once wastes energy | home |
| R33 | `r33-chilly`, `r33-radiatory` | thermy, 120 px from chilly or radiatory squabbling | the squabbler is soothed (content); thermy proud | thermy takes orders from both lines, 20 and 26 °C | home, heating, cooling |
| R34 | `r34` | sunny blazing, 120 px from chilly, unless a lowered shady is within 120 px of chilly | chilly `overloaded`, grumpy; sunny proud | solar gains are the chiller's workload (50–100 W/m²) | home, cooling |
| R35 | `r35`, `r35-relief` | sunny blazing, 120 px from shady lowered; shady lowered, 120 px from chilly overloaded | shady plays `sun-block`, proud; sunny grumpy (0.4) ("the sunglasses"); chilly back to `cooling`, happy | 6 against 100 W/m²: the blind blocks the Sun's gains | home, cooling |
| R36 | `r36-blazing`, `r36-shining` | sunny blazing, 90 px from housy cosy; sunny shining, 90 px from housy | blazing: housy `hot`, sad; shining: housy happy | summer gains overheat, winter gains heat — the sun's state stands for the season | home |
| R37 | `r37` | sunny blazing, 90 px from roofy | roofy `sun-baked`, grumpy; sunny proud | the roof skin reaches 60–70 °C | home |
| R38 | `r38` | shady lowered, 90 px from solary | solary `shaded`, sad; shady `raised` with its `raise` ("sorry"); −0.05 | a shadow on a module drops its output | home |
| R39 | `r39-lowered`, `r39-tilted` | shady lowered or tilted, 90 px from windowy shut | windowy proud (0.4); +0.05 | the shading sits on the window's head and spares the glass | home |
| R40 | `r40` | cloudy, 90 px from shady lowered | shady `raised`, sleepy | a cloud is free shade: a day off for the blind | home, cooling |
| R41 | `r41-cosy`, `r41-cold` | insuly performing `tuck-in`, 90 px from housy cosy or cold | housy `wrapped`; both proud; +0.1 | insulation is the house's coat | home, heating, demand |
| R42 | `r42-bare`, `r42-warm`, `r42-passive` | insuly performing `tuck-in`, 90 px from waly bare or warm; a thick insuly's `tuck-in` of a wrapped waly | waly `wrapped`, then `passive`; both proud; +0.1 | U 1.0 → 0.28 → 0.15 W/(m²·K) | home, heating |
| R43 | `r43-baked`, `r43` | insuly performing `tuck-in`, 90 px from roofy | `sun-baked` → `dry`; roofy proud; +0.1 | 35 cm give U 0.10 instead of 2.1 | heating |
| R44 | `r44-insuly`, `r44-roofy` | cloudy raining above insuly (unless roofy is within 120 px of her) or above roofy dry, 120 px | insuly `soaked`, sad; roofy `wet` | wet insulation insulates badly; roofs get wet | home |
| R45 | `r45` (and the `unless` of `r44-insuly`) | roofy, 90 px from insuly | insuly happy; roofy proud; rain does not soak her while a roof is near | the roof keeps the insulation dry | heating |
| R46 | `r46-radiatory`, `r46-sunny` | radiatory hot for 20 s, or sunny 20 s in one state, 90 px from waly bare | waly `warm`, content | thermal mass stores heat | home, heating |
| R47 | `r47-shining`, `r47-blazing` | sunny shining or blazing, 90 px from windowy shut | shining: happy; blazing: scared (squints) | winter solar gains help, summer ones hurt — the sun's state stands for the season | home |
| R48 | `r48` | pumpy heating, 90 px from radiatory warm | pumpy plays `flow-temperature`, proud; radiatory `hot`, happy; +0.1 | low flow temperatures with a heat pump | home, demand |
| R49 | `r49` | pumpy, 90 px from boily; rests 30 s | boily plays `efficiency-bars`; both grumpy; they squabble next | seasonal performance 3 against an efficiency of 0.8–0.9 | home, demand |
| R50 | `r50` | pumpy heating, 90 px from solary peak | pumpy proud; solary happy; +0.05 | the PV roof feeds the heat pump (plus-energy house) | home, demand |
| R51 | `r51-insuly`, `r51-housy` | pumpy heating, 90 px from insuly thick or housy wrapped | pumpy plays `whisper` (→ `standby`), proud | a deep retrofit cuts the purchase from 122 to 23 kWh/(m²·a) | home, demand |
| R52 | `r52` | pumpy cooling, 90 px from chilly cooling | both content; they cuddle next | a heat pump is a chiller run backwards | home |
| R53 | `r53` | venty, 90 px from pumpy heating | venty proud; they cuddle (hum in step) next; +0.05 | the compact passive-house unit | home, demand |
| R54 | `r54-servy`, `r54-thermy` | servy hot or thermy hot, 90 px from venty | venty `boost`; servy `busy` (relieved) | server halls live on moving, filtered air; a hot room is purged | cooling |
| R55 | `r55` | servy hot, 90 px from chilly cooling | chilly `overloaded` (works harder); servy `busy`; +0.05 | the chiller works around the clock for the data centre | cooling |
| R56 | `r56` | servy hot, 100 px from thermy | thermy `hot`, flusters; servy grumpy ("stop looking") | the rack runs hot and the thermometer fusses | cooling |
| R57 | `r57-kettly`, `r57-roofy-baked`, `r57-boily`, `r57-flamy`, `r57-chilly`, `r57-cloudy`, `r57-radiatory`, `r57-roofy-snow` | a hot pet (kettly boiling, roofy sun-baked, boily roaring, flamy high) or a cold one (chilly frosted, cloudy raining, radiatory cold, roofy snow-capped), 100 px from thermy; rests 2 s | thermy `hot` or `cold` (mild again when that runs out); the hot partner grows proud, the cold one grumpy (0.1 a time) | a thermometer reads what is around it | home, heating, cooling (kettly, boily, flamy: gallery) |
| R58 | `r58-peak-empty`, `r58-peak-charged`, `r58-rated-empty`, `r58-rated-charged` | solary peak or windy rated, 90 px from battery empty or charged; rests 6 s | battery `charging` (→ `full`); both happy; +0.05 | the battery stores the surplus | home, physics |
| R59 | `r59` | solary peak, 90 px from battery full | solary sad (curtailed); battery proud | a full store takes no more | home, physics |
| R60 | `r60` | sunny, 90 px from flamy; chance 0.5 | flamy plays `look-up`, happy | the smallest and the largest of the powers | home, physics |
| R61 | `r61` | sunny blazing, 100 px from thermy | thermy `hot`; they squabble next | the Sun drives the mercury up | home, cooling |
| R62 | `r62` | chilly frosted, 90 px from roofy | roofy `snow-capped`, shivers; chilly proud | cold air and snow | gallery |
| R63 | `r63-cooling`, `r63-frosted` | chilly cooling or frosted, 90 px from cloudy | cloudy plays `snow` (fluffy, heavy); chilly happy | overcast days lower the cooling load | home, cooling |
| R64 | `r64` | chilly cooling, 100 px from thermy | thermy `cold`; chilly proud; +0.05 | the chiller takes orders from the 26 °C line | cooling |
| G1 | `g1` | a grumpy pet, 90 px from a grumpy pet, unless a calm one is within 90 px of the second | both stay grumpy (0.15 more); they squabble next | two grumpy pets bicker; a calm third keeps the peace | all |
| G3 | `g3-content`, `g3-happy`, `g3-playful`, `g3-proud` | a sad pet, 90 px from a friend (affinity 0.4…1) who is content, happy, playful or proud | the friend cuddles it next; its sadness eases (content 0.16) | friends comfort | all |
| G4 | `g4-housy`, `g4-roofy`, `g4-waly`, `g4-insuly` | housy, roofy, waly or insuly, 90 px from a scared pet | the scared one calms (content 0.6); they cuddle (shelter, lean on, snuggle) next | the envelope shelters | home, heating, demand |
| G5 | `g5`, `g5-spread` | a purring pet, 90 px from a sleepy one (chance 0.5, every 5 s); a sleepy one, 90 px from a purring one (chance 0.25, every 5 s) | the sleepy one falls asleep; the purrer grows sleepy (0.4) | a purr lulls | all |
| G6 | `g6` | a proud pet, 90 px from a curious one | they greet next, and a proud pet shows a trick when it greets; the curious one happy (0.3) | an audience | all |

What the shape of a reaction cannot say, and what stands in for it:

- **R06** (solary at peak for 40 s and alone gets hot and grumpy, generating again 20 s later) is species data: `peak`
  lasts 40 s and gives way to `hot`, which gives way to `generating` after 20 s. "Alone" and the grumpy mood are not
  expressible: a reaction needs a second actor, and nothing says "nobody near".
- **G2** (a happy or playful pet cheers its neighbours) is the stage's own mood contagion and the encounter bias of
  happy and playful pets; a reaction for it would count it twice.
- **Delays and durations as outcomes** — R12 and R55 "both content after 20 s", R22 "closes within 6 s", R26 "for
  12 s" — are approximated: a cuddle or a `chance` per `every` (a waiting time), `held` where the state is the cause.
  R05 asks for 20 s of blazing instead of 30 s, because sunny's `blazing` lasts 30 s and gives way before.
- **Shares of the encounters** (×1.5, ×2 for 30 s in R20, R21, R28, R32, R49, R52, R53 and G1–G4) become a promise of
  the next encounter of the pair (`encounter`, 30 s).
- **Three parties** (R03, R35) are chains of pairs; R33's thermy cannot turn a running squabble into a greet — it
  soothes both squabblers instead; R34's "no lowered shady near" and R45's shelter are `unless`.
- **The scene** (R36, R47: heating or cooling) is unknown to the stage; sunny meets housy and windowy only at home, so
  the sun's state stands for the season: `shining` is winter sun, `blazing` summer sun.
- **Behaviour** (R09 drift ×1.6, R60 look-up odds ×3, R50/R51 "fan quieter", R52 "mirror each other's fan") becomes
  what a reaction can start: a walk, a chance on a chemistry-only trick, `whisper`; R50 and R52 keep only the moods.
- **Two moods at once** (R16 "proud and sheepish") keep the first; R16's lamp flash has no clip — flamy relights itself.
- **Precedence** (CONTENT: "a scared pet ignores every rule but G4"; R57's "hottest or coldest") is the order of the
  species and of the reactions: the first state and mood that reach an actor in a look win. R57 leaves the pets with a
  rule of their own to it (radiatory hot R25, servy hot R56, sunny blazing R61, chilly cooling R64).
- **Memory** (G6 "repeats its last trick") is not kept; a proud pet that greets shows one of its `show` tricks.
- **Ids that differ from the brief**: cloudy has no `bluster` (R10 keys on `gust`); R35's duet is shady's `sun-block`
  (its `pose` is a whim and show trick); shady's `raised` lasts 40 s, not the 5 s of R38's "sorry"; boily's and flamy's
  `relight` both exist (R15 is boily's, R16 flamy's own).

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
   Then what the pet can become and do: `states` (at least the resting one), `tricks`, a `purr`, `emitters`, its
   `gear` (by its nature; a floater owns a parachute or nothing), `grip`, `reach` and its resting `mood`. Every pet
   has a clip for `hang`, `tumble`, `purr`, `dizzy`, `shrug` and `push`, and for what its gear brings (`climb` →
   `climb`, `mantle`, `slide`; `ladder` → `carry`, `climb`; `grapple` → `aim`, `reel`; `parachute` → `glide`).
3. **Grounds.** Name every quiz item, task or topic the pet stands for. A pet may only join the cast of a quiz it is
   grounded in.
4. **Ensemble.** In `🔣️.json` add the path to `species`, the bonds of the pet (each with a physical reason, recorded in
   the table above) and the pet to the rotation of `home` and to the casts of the quizzes it belongs to.
5. **Module.** In `🟦️.ts` add the import and the species to the list, in the order of the ensemble.
6. **Check.** `bun nx run @teaching/architecture-quiz:test` (suite `🐾️pet-cast`: schema with ajv, the product's own
   validators, grounds, casts, bonds, states, tricks, gear and chemistry, the module against the documents) and look at
   the pet in the gallery.

## Adding a state, a trick or a reaction

- **State.** Add it to `states` of the species, after the resting state, with an English and a German `name`, and give
  it what shows it: a `tint` (colours that keep 3:1 against both pages), an overlay `clip` that keys only the bones and
  channels the state changes (the idle loop keeps breathing on the others) and maybe an `emitter`. Make it reachable —
  a trick's `to`, a rung of a circling trick's `from`, a `then` of another state or a reaction — and let it come back:
  `lasts` and `then`, or a trick that leaves it. Add it to the table above.
- **Trick.** Add the clip (once, 0.8 to 3 s, starting and ending on rest values) to `clips` and the trick to `tricks`
  with its `cues`, the states it is on offer in (`from`), where it leaves the pet (`to`) and the mood it leaves; the
  first two `click` tricks are the second and third click of a streak. A circling trick without `to` steps along the
  rungs its `from` lists. A trick only the chemistry plays has no cues. Add it to the table above.
- **Reaction.** Add a row to `chemistry` in `🔣️.json` with an id of the rule it implements (`r<nn>` or `g<n>`, a
  `-<variant>` where one rule takes several rows), sides that name species, states and tricks that exist, a `within`
  no smaller than the `reach` of both species together (else they never react, not even when they meet), an `every` of
  at least a second, and effects that each do something and play only tricks the species offers in the state the side
  asks for. Record it with its physical reason in the table above. Then look at it on the stage:
  `bun <ticket>/stage_storyboard.ts --menagerie 🎓️teaching/🏛️architecture/🐾️pets/🟦️.ts --scene <scene> --mode lively`
  prints how often every reaction acted.

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
| `../❓️quiz/🧪️tests/🐾️pet-cast/🟦️.ts` (vitest) | every species and the ensemble against the draft-07 contract (ajv) and the product's validators; the assembled menagerie has no issue and equals what `🟦️.ts` exports; grounds exist in the quiz files; every cast member of a quiz is grounded in it; every quiz and `home` have a cast; bonds join existing species; every state, trick and purr names clips, particles and states its species has; every state can be reached from the resting state and leads back to it (tricks, states that run out, chemistry); circling a pet one way or the other changes its resting state; every pet has the clips of the hand and of its gear; the chemistry implements R01–R64 but R06 and G1, G3–G6, its sides and effects exist and can happen, it looks at least as far as both species reach and at most 400 px, rests 1 to 600 s, and every scene has something to react to |
| `../❓️quiz/🧪️tests/🐕️pet-walk/🟦️.ts` (Playwright, end-to-end gate, project `pets`) | pets appear on the home screen, hidden from assistive technology, never a hit target of their own and standing on what the cards show; a click says hello, the next asks for a trick, the ones after it for a purr; a pet picked up hangs in the hand and, let go high, opens its parachute, glides and lands; circling the pointer round a pet on a quiz's page changes its state; during a lively minute in which pets are dragged over each other and thrown no two bodies overlap on any frame; a card under a falling pet still receives its click; the play group of the settings (hello, trick, pet, toss) and the footer switch work by keyboard; the cast follows the learner into a quiz's page and run; `still` gives motionless pets and `off` removes them; a device that asks for reduced motion gives still pets that answer no hand until the learner chooses, and calm pets the learner chose walk (at the test tempo); forced colours show none; a phone shows at most two small ones; no console error, failed request or policy violation. Mischief on lifted copies and the routes of the gear wait for the stage (`fixme`) |
