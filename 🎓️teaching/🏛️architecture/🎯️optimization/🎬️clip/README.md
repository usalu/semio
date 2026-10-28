# Six Optimization Videos · Sechs Optimierungsvideos

## English

Six independent architecture videos teach optimization with **one concrete design study**: a seminar room of 8 × 6 × 3 m (48 m²) with one glazed façade in a Zurich-like climate. The design variables are façade bearing θ (90°–450°), window-to-wall ratio WWR (10–80 %), overhang depth d (0–1.2 m) and insulation thickness t (10–30 cm). The responses are heating, cooling and lighting electricity (kWh/a), façade cost (CHF), mean daylight factor (%) and embodied carbon (kg CO₂e). Values come from a simplified annual teaching model, not from measurements.

**Plot and room.** Every value on a plot is shown at the same time in the room: a marker moves, and the plan, the section with sun rays, the lamps, the stacked electricity bar and the price chip show that exact design. Nothing is abstract and no value appears without its unit. English and German are equally authored; choose a language explicitly.

| Video ID | What the viewer sees | Folder |
| --- | --- | --- |
| optimization_extrema | The room turns through all bearings while its electricity curve is drawn as a stacked area of heating, cooling and lighting, with summer and winter sections and the live formula; why the north local minimum is one (neighbours at ±5°), the south global minimum, the courtyard constraint and the saving in kWh/a and CHF/a | [📉️extrema](../📉️extrema/🎬️clip/🐍️.py) |
| optimization_sampling | The WWR × overhang plane filled with simulated rooms, each plotted point shown by its room with daylight, lamps and the equation of its electricity: 3 × 3 grid, random draws with gap and cluster, Latin hypercube, and the best-so-far electricity of each plan | [🎲️sampling](../🎲️sampling/🎬️clip/🐍️.py) |
| optimization_experiments | Four summer rooms with sun rays are the 2² runs (WWR 30 / 70 %, overhang 0 / 1 m, cooling kWh/a): one sign convention (effect = high − low), main effects and interaction written with the printed operands, bilinear model, centre-run curvature and a confirmation run | [🧪️experiments](../🧪️experiments/🎬️clip/🐍️.py) |
| optimization_index | Five façade variants beside one persistent room panel: physical rulers for electricity, cost, daylight and carbon (kWh/(m²·a), CHF, %, kg CO₂e) that normalize with units, weights that re-rank the winner and its room, a budget limit, and a sixth design that breaks the old range | [⚖️index](../⚖️index/🎬️clip/🐍️.py) |
| optimization_pareto | 36 designs on cost against electricity with the cost anatomy of the room, one dominated pair, the front as steps of "+CHF → −kWh/a", payback and knee, a refined design that beats five members, daylight as a third objective and weights as the price of 1 kWh/a | [🏆️pareto](../🏆️pareto/🎬️clip/🐍️.py) |
| optimization_algorithms | A heat map of the surface beside winter and summer room sections, then gradient descent (difference quotients, overshoot), pattern search (four neighbour cards per round) and a genetic algorithm (population 8) paint the designs they simulate within a budget of 50 runs; stopping rules show where each method would stop | [🧬️algorithms](../🧬️algorithms/🎬️clip/🐍️.py) |

### Structure

- Model: [🧮️model/🐍️.py](🧮️model/🐍️.py) (stdlib only), coefficients in [🧮️model/🔣️.json](🧮️model/🔣️.json), schema in [🧮️model/🧬️schema/🔣️.schema.json](🧮️model/🧬️schema/🔣️.schema.json).
- Cases: [🧪️cases/🔣️.json](🧪️cases/🔣️.json) holds the portable expectations (chapters, extrema, samples, effects, front, solver paths, invalid models).
- Tests: [🧪️tests/🐍️.py](🧪️tests/🐍️.py) re-derives the cases with NumPy and SciPy; [🧪️tests/🟨️.cjs](🧪️tests/🟨️.cjs) validates the schema with AJV and rejects invalid mutations.
- Toolkit: [🎨️visuals/🐍️.py](🎨️visuals/🐍️.py) (`ClipScene`: language, numbers, plot axes, live marker) and [🎨️visuals/🏠️room/🐍️.py](🎨️visuals/🏠️room/🐍️.py) (`RoomVisuals`: section, façade, plan, thumbnail, bars). Styling comes from the shared energy authoring toolkit.
- Each scene folder holds one class with its bilingual title and narration; the title is metadata for the transcripts and the window chrome, while the video body shows one instructional heading per beat. No runtime dependency is added.

### Render and Test

Render one video:

~~~text
repo 🎓️teaching/🏛️architecture/⚡️energy/🏗️authoring/📜️script.ts render --video optimization_extrema --language en --audience speaker --no-play
~~~

Render all six German videos:

~~~text
repo 🎓️teaching/🏛️architecture/⚡️energy/🏗️authoring/📜️script.ts render --video optimization_extrema --video optimization_sampling --video optimization_experiments --video optimization_index --video optimization_pareto --video optimization_algorithms --language de --audience speaker --no-play
~~~

For the delivered Full HD format, add `--quality h --fps 30`. Use `--audience spectator` for the same animation without captions; omitting `--audience` renders both. Progress, cancellation, job count and cache provenance use the shared renderer.

Run the contract and independent numerical checks:

~~~text
repo 🎓️teaching/🏛️architecture/⚡️energy/🏗️authoring/📜️script.ts test
~~~

### Transcripts

Complete accessible transcripts: [English](🗣️transcript.en.md) · [Deutsch](🗣️transcript.de.md).

## Deutsch

Sechs eigenständige Architekturvideos lehren Optimierung an **einer konkreten Entwurfsstudie**: ein Seminarraum von 8 × 6 × 3 m (48 m²) mit einer verglasten Fassade im Klima von Zürich. Die Entwurfsvariablen sind die Fassadenrichtung θ (90°–450°), der Fensteranteil WWR (10–80 %), die Vordachtiefe d (0–1,2 m) und die Dämmstärke t (10–30 cm). Die Zielgrößen sind Heiz-, Kühl- und Beleuchtungsstrom (kWh/a), Fassadenkosten (CHF), mittlerer Tageslichtquotient (%) und graue Emissionen (kg CO₂e). Die Werte stammen aus einem vereinfachten Jahresmodell zu Lehrzwecken, nicht aus Messungen.

**Diagramm und Raum.** Jeder Wert im Diagramm wird gleichzeitig im Raum gezeigt: Bewegt sich eine Markierung, zeigen Grundriss, Schnitt mit Sonnenstrahlen, Lampen, gestapelter Strombalken und Preisschild genau diesen Entwurf. Nichts ist abstrakt und kein Wert steht ohne Einheit. Englisch und Deutsch sind gleichwertig verfasst; die Sprache wird ausdrücklich gewählt.

| Video-Kennung | Was die Zuschauenden sehen | Ordner |
| --- | --- | --- |
| optimization_extrema | Der Raum dreht sich durch alle Himmelsrichtungen, während seine Stromkurve als gestapelte Fläche aus Heizung, Kühlung und Beleuchtung entsteht, mit Sommer- und Winterschnitt und der Formel mit eingesetzten Zahlen; warum das lokale Minimum im Norden eines ist (Nachbarn bei ±5°), das globale Minimum im Süden, die Hofseiten-Randbedingung und die Einsparung in kWh/a und CHF/a | [📉️extrema](../📉️extrema/🎬️clip/🐍️.py) |
| optimization_sampling | Die Ebene aus WWR × Vordach, gefüllt mit simulierten Räumen; jeder gezeichnete Punkt wird durch seinen Raum mit Tageslicht, Lampen und der Gleichung seines Stroms gezeigt: 3 × 3-Raster, Zufallsziehungen mit Lücke und Häufung, Latin Hypercube und der bisher beste Strom jedes Plans | [🎲️sampling](../🎲️sampling/🎬️clip/🐍️.py) |
| optimization_experiments | Vier Sommerräume mit Sonnenstrahlen sind die 2²-Versuche (WWR 30 / 70 %, Vordach 0 / 1 m, Kühlstrom kWh/a): eine Vorzeichenkonvention (Effekt = hoch − tief), Haupteffekte und Wechselwirkung mit den gezeigten Operanden, bilineares Modell, Krümmung am Zentrumsversuch und ein Bestätigungsversuch | [🧪️experiments](../🧪️experiments/🎬️clip/🐍️.py) |
| optimization_index | Fünf Fassadenvarianten neben einem durchgehenden Raumbild: physikalische Skalen für Strom, Kosten, Tageslicht und graue Emissionen (kWh/(m²·a), CHF, %, kg CO₂e), die mit Einheiten normieren, Gewichte, die den Sieger und seinen Raum neu bestimmen, eine Budgetgrenze und ein sechster Entwurf, der den alten Bereich sprengt | [⚖️index](../⚖️index/🎬️clip/🐍️.py) |
| optimization_pareto | 36 Entwürfe in Kosten gegen Strom mit der Kostenaufschlüsselung des Raums, ein dominiertes Paar, die Front als Stufen „+CHF → −kWh/a“, Amortisation und Knick, ein verfeinerter Entwurf, der fünf Mitglieder schlägt, Tageslicht als drittes Ziel und Gewichte als Preis von 1 kWh/a | [🏆️pareto](../🏆️pareto/🎬️clip/🐍️.py) |
| optimization_algorithms | Eine Heatmap der Fläche neben Winter- und Sommerschnitt des Raums, dann malen Gradientenabstieg (Differenzenquotienten, Überschwingen), Mustersuche (vier Nachbarkarten je Runde) und ein genetischer Algorithmus (Population 8) die simulierten Entwürfe innerhalb eines Budgets von 50 Läufen; Abbruchregeln zeigen, wo jedes Verfahren aufhören würde | [🧬️algorithms](../🧬️algorithms/🎬️clip/🐍️.py) |

### Aufbau

- Modell: [🧮️model/🐍️.py](🧮️model/🐍️.py) (nur Standardbibliothek), Koeffizienten in [🧮️model/🔣️.json](🧮️model/🔣️.json), Schema in [🧮️model/🧬️schema/🔣️.schema.json](🧮️model/🧬️schema/🔣️.schema.json).
- Fälle: [🧪️cases/🔣️.json](🧪️cases/🔣️.json) enthält die portablen Erwartungen (Kapitel, Extrema, Stichproben, Effekte, Front, Lösungspfade, ungültige Modelle).
- Tests: [🧪️tests/🐍️.py](🧪️tests/🐍️.py) leitet die Fälle mit NumPy und SciPy unabhängig neu her; [🧪️tests/🟨️.cjs](🧪️tests/🟨️.cjs) prüft das Schema mit AJV und weist ungültige Änderungen ab.
- Werkzeugkasten: [🎨️visuals/🐍️.py](🎨️visuals/🐍️.py) (`ClipScene`: Sprache, Zahlen, Diagrammachsen, Live-Markierung) und [🎨️visuals/🏠️room/🐍️.py](🎨️visuals/🏠️room/🐍️.py) (`RoomVisuals`: Schnitt, Fassade, Grundriss, Vorschaubild, Balken). Das Styling stammt aus dem gemeinsamen Energie-Werkzeugkasten.
- Jeder Szenenordner enthält eine Klasse mit zweisprachigem Titel und Sprechtext; der Titel dient als Metadatum für die Transkripte und den Fensterrahmen, im Videobild steht je Schritt eine erläuternde Überschrift. Es kommt keine Laufzeitabhängigkeit hinzu.

### Rendern und Testen

Ein Video rendern:

~~~text
repo 🎓️teaching/🏛️architecture/⚡️energy/🏗️authoring/📜️script.ts render --video optimization_extrema --language en --audience speaker --no-play
~~~

Alle sechs deutschen Videos rendern:

~~~text
repo 🎓️teaching/🏛️architecture/⚡️energy/🏗️authoring/📜️script.ts render --video optimization_extrema --video optimization_sampling --video optimization_experiments --video optimization_index --video optimization_pareto --video optimization_algorithms --language de --audience speaker --no-play
~~~

Für das gelieferte Full-HD-Format `--quality h --fps 30` ergänzen. `--audience spectator` rendert dieselbe Animation ohne Untertitel; ohne `--audience` entstehen beide Varianten. Fortschritt, Abbruch, Job-Anzahl und Zwischenergebnisse stellt der gemeinsame Renderer bereit.

Vertrags- und unabhängige Zahlenprüfungen ausführen:

~~~text
repo 🎓️teaching/🏛️architecture/⚡️energy/🏗️authoring/📜️script.ts test
~~~

### Transkripte

Vollständige barrierefreie Transkripte: [English](🗣️transcript.en.md) · [Deutsch](🗣️transcript.de.md).

## Primary References · Primärquellen

- [NIST: Design of Experiments](https://www.itl.nist.gov/div898/handbook/pri/pri.htm): factors, effects, interactions and confirmation · Faktoren, Effekte, Wechselwirkungen und Bestätigung.
- [SciPy: Latin Hypercube](https://docs.scipy.org/doc/scipy/reference/generated/scipy.stats.qmc.LatinHypercube.html): marginal stratification · Randschichtung.
- [SciPy: Minimization](https://docs.scipy.org/doc/scipy/reference/generated/scipy.optimize.minimize.html): gradient and derivative-free method families · Gradienten- und ableitungsfreie Verfahren.
- [pymoo: Genetic Algorithm](https://pymoo.org/algorithms/soo/ga.html): evaluation, selection, variation and survival · Bewertung, Selektion, Variation und Überleben.

## Delivered Distribution Files · Gelieferte Verteildateien

English: final masters are rendered for `en` and `de`, each for `speaker` and `spectator`, and delivered under `.🧬semio/🎓️teaching/⚡️cache/videos/<language>/<audience>/delivery`. Format: H.264 High/level4.1, yuv420p, CRF18, faststart. `delivery.json` records exact frame counts, sizes and SHA-256 links to the validated canonical masters in the adjacent `rendered` folder.

Deutsch: Die finalen Master werden für `en` und `de`, je für `speaker` und `spectator`, gerendert und unter `.🧬semio/🎓️teaching/⚡️cache/videos/<language>/<audience>/delivery` ausgeliefert. Format: H.264 High/level4.1, yuv420p, CRF18, faststart. `delivery.json` hält exakte Bildzahlen, Größen und SHA-256-Verweise auf die geprüften Master im benachbarten Ordner `rendered` fest.
