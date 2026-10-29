# Five Grasshopper Videos · Fünf Grasshopper-Videos

## English

Five independent architecture videos teach visual programming with Grasshopper 1 (Rhino 8) with **one concrete example**: the Nakagin Capsule Tower in Ginza, Tokyo, by Kisho Kurokawa (1970–1972, demolished 2022), shown as a rounded teaching model (140 capsules in two towers; one shared model in [🏢️nakagin](../../🏢️nakagin/README.md)). Values that are assumed (capsule types, storey height, core size, capital, podium) are marked as assumed in the videos and are not survey data. Component and parameter names are the real Grasshopper names.

**Concept and drawing.** Every concept appears on the canvas that computes it and on the Nakagin at the same time: a Number Slider is a dimension line on the capsule, a Panel item lights the capsule it names, a tree branch is one facade of one storey on the tower, a wire shares the colour of the element it carries. No box is abstract and no number appears without its object and unit. English and German are equally authored; choose a language explicitly. German narration keeps the English component names, because the Grasshopper interface is English.

| Video ID | What the viewer sees | Folder |
| --- | --- | --- |
| grasshopper_anatomy | The capsule shell built on an empty canvas: script and canvas, a Number Slider wired into a Box component, component anatomy (inputs left, outputs right, wires, preview, grey, orange and red states), floating and input parameters, a Panel showing the output and its path, a plugin adding a tab, and item, list and tree on the capsules of one tower | [🦴️anatomy](🦴️anatomy/🐍️.py) · [entry](../🦴️anatomy/🎬️clip/🐍️.py) |
| grasshopper_capsule | One parametric capsule: sliders for width, length, height, radius, window diameter and window offset drive a box with rounded edges, the round window and the four bolt points, with live dimension lines; the capsule types as variants of one definition, a typical mistake with a circle on the wrong plane, each type moved with Orient from its base plane onto a target plane and picked with List Item (also with Wrap) over eight planes | [💊️capsule](💊️capsule/🐍️.py) · [entry](../💊️capsule/🎬️clip/🐍️.py) |
| grasshopper_tower | The core with square plan, storey height and storey count, a series of planes for every storey and facade, capsule orientation with Orient and List Item, the capital on top of each core and the lower building as simple massing; counts read from the definition (A 9 × 3 × 2 = 54, B 11 × 4 × 2 = 88 minus 2 culled planes = 86), including the typical mistake of a forgotten Cull Index and its fix | [🗼️tower](🗼️tower/🐍️.py) · [entry](../🗼️tower/🎬️clip/🐍️.py) |
| grasshopper_casting | Every parameter has a type and mismatched data is cast on one capsule slot of Tower B: Number to Integer with its rounding rule, Boolean to Integer, Point to Plane (the capsule faces the wrong way), Line to Plane, text to Number, number to Domain, Circle to Curve, Box to Brep, number to Boolean, and the cast that fails with a red component and its message; rows not yet verified in the running app carry an on-screen marker; a casting matrix built from what was shown | [🫗️casting](🫗️casting/🐍️.py) · [entry](../🫗️casting/🎬️clip/🐍️.py) |
| grasshopper_trees | The aggregated tower of 140 capsules as a tree with the path `{BUILDING;TOWER;STORY;FACADE}` drawn as a real tree beside the tower: path anatomy, Panel and Param Viewer, Flatten Tree, Graft Tree, Simplify Tree, Path Mapper, Flip Matrix, counts per tower and storey as lit capsules, the data-matching mistake of two lists with different depths (Custom Preview colours the wrong capsules) and its repair with Path Mapper and Graft Tree, and a recap of ten terms | [🌳️trees](🌳️trees/🐍️.py) · [entry](../🌳️trees/🎬️clip/🐍️.py) |

### Structure

- Model: [🧱️model/🐍️.py](🧱️model/🐍️.py) (stdlib only: data trees, parameters, casting, components and the Nakagin definitions), data in [🧱️model/🔣️.json](🧱️model/🔣️.json), schema in [🧱️model/🧬️schema/🔣️.schema.json](🧱️model/🧬️schema/🔣️.schema.json). Building values come from [🏢️nakagin](../../🏢️nakagin/🔣️.json). Every panel value, count and drawing on screen comes from evaluating the definition.
- Cases: [🧪️cases/🔣️.json](🧪️cases/🔣️.json) holds the portable expectations.
- Tests: [🧪️tests/🐍️.py](🧪️tests/🐍️.py) re-derives the cases with independent oracles (`rhino3dm` for planes, transforms and boxes; a separate data-tree implementation); [🧪️tests/🟨️.cjs](🧪️tests/🟨️.cjs) validates the schema with AJV; [🧪️tests/🎭️presentation/🐍️.py](🧪️tests/🎭️presentation/🐍️.py) covers the scenes. The oracles are test-only dependencies.
- Toolkit: [🎨️visuals/🐍️.py](🎨️visuals/🐍️.py) (`ClipScene`: language, numbers, live values) and [🦗️canvas](🎨️visuals/🦗️canvas/🐍️.py) (components, parameters, sliders, panels, wires, groups, ribbon tabs, Param Viewer). Styling comes from the shared energy authoring toolkit; the Nakagin drawings come from [🏢️nakagin/🎨️visuals](../../🏢️nakagin/🎨️visuals/🐍️.py).
- Each topic folder holds one scene class with its bilingual title and narration. No runtime dependency is added.

### Primary References

- McNeel Developer Guides: [Grasshopper Data Types](https://developer.rhino3d.com/guides/grasshopper/grasshopper-data-types/), [Advanced Data Structures](https://developer.rhino3d.com/guides/grasshopper/gh-algorithms-and-data-structures/advanced-data-structures/), [Introduction to Data Structures](https://developer.rhino3d.com/guides/grasshopper/gh-algorithms-and-data-structures/data-structures/) and [List Components](https://developer.rhino3d.com/guides/grasshopper/list-components/).
- McNeel API: [GH_DataComparison](https://developer.rhino3d.com/api/grasshopper/html/T_Grasshopper_Kernel_GH_DataComparison.htm); plugins are distributed on [Food4Rhino](https://www.food4rhino.com).
- Building: Kisho Kurokawa, Nakagin Capsule Tower, [Wikipedia](https://en.wikipedia.org/wiki/Nakagin_Capsule_Tower) and the sources in the Nakagin README.

### Render and Test

The standalone entries live in `🦗️grasshopper/<topic>/🎬️clip/🐍️.py`. Canonical scenes remain under this shared clip folder so model data, narration and visuals have one source. The root entry renders the five-video playlist.

Run one video from the repository root:

```text
repo 🎓️teaching/🏛️architecture/⚡️energy/🏗️authoring/📜️script.ts render --video grasshopper_anatomy --language en --audience speaker
repo 🎓️teaching/🏛️architecture/⚡️energy/🏗️authoring/📜️script.ts render --video grasshopper_capsule --language en --audience speaker
repo 🎓️teaching/🏛️architecture/⚡️energy/🏗️authoring/📜️script.ts render --video grasshopper_tower --language en --audience speaker
repo 🎓️teaching/🏛️architecture/⚡️energy/🏗️authoring/📜️script.ts render --video grasshopper_casting --language en --audience speaker
repo 🎓️teaching/🏛️architecture/⚡️energy/🏗️authoring/📜️script.ts render --video grasshopper_trees --language en --audience speaker
```

Render the entire five-video playlist in German:

```text
repo 🎓️teaching/🏛️architecture/⚡️energy/🏗️authoring/📜️script.ts render --video grasshopper_anatomy --video grasshopper_capsule --video grasshopper_tower --video grasshopper_casting --video grasshopper_trees --language de --audience spectator
```

Choose `--language en` or `--language de` (required) and `--audience speaker` or `--audience spectator` for any video or playlist; omitting `--audience` renders both. Use `--output-root` for a chosen output location, and `--quality h --fps 30` for Full HD. The shared renderer provides bounded work, progress, cancellation, cache provenance and the speaker/spectator presentation.

Every source scene contains complete English and German narration text. This source folder contains no pre-recorded speech or final videos. The shared renderer attaches existing configured clause recordings when available; narration text alone does not synthesize a recording.

Run the contract and independent checks:

```text
repo 🎓️teaching/🏛️architecture/⚡️energy/🏗️authoring/📜️script.ts test
```

### Transcripts

Complete accessible transcripts: [English](🗣️transcript.en.md) · [Deutsch](🗣️transcript.de.md).

## Deutsch

Fünf eigenständige Architekturvideos lehren visuelles Programmieren mit Grasshopper 1 (Rhino 8) an **einem konkreten Beispiel**: dem Nakagin Capsule Tower in Ginza, Tokio, von Kisho Kurokawa (1970–1972, 2022 abgerissen), gezeigt als gerundetes Lehrmodell (140 Kapseln in zwei Türmen; ein gemeinsames Modell in [🏢️nakagin](../../🏢️nakagin/README.md)). Angenommene Werte (Kapseltypen, Geschosshöhe, Kerngröße, Kapitell, Sockelgebäude) sind in den Videos als angenommen gekennzeichnet und keine Aufmaßdaten. Bauteil- und Parameternamen sind die echten Grasshopper-Namen.

**Begriff und Zeichnung.** Jeder Begriff erscheint zugleich auf dem Canvas, der ihn berechnet, und am Nakagin: Ein Number Slider ist eine Maßlinie an der Kapsel, ein Panel-Eintrag lässt die genannte Kapsel aufleuchten, ein Datenast ist eine Fassade eines Geschosses am Turm, ein Draht trägt die Farbe des Elements, das er führt. Kein Kasten ist abstrakt, und keine Zahl steht ohne Objekt und Einheit. Englisch und Deutsch sind gleichwertig verfasst; die Sprache wird ausdrücklich gewählt. Die deutsche Sprechfassung behält die englischen Komponentennamen, weil die Grasshopper-Oberfläche englisch ist.

| Video-Kennung | Was die Zuschauenden sehen | Ordner |
| --- | --- | --- |
| grasshopper_anatomy | Die Kapselhülle auf leerem Canvas aufgebaut: Skript und Canvas, ein Number Slider an einer Box-Komponente, der Aufbau einer Komponente (Eingänge links, Ausgänge rechts, Drähte, Vorschau, graue, orange und rote Zustände), freie und Eingabeparameter, ein Panel mit Ausgabe und Pfad, ein Plugin mit eigenem Tab sowie Element, Liste und Datenbaum an den Kapseln eines Turms | [🦴️anatomy](🦴️anatomy/🐍️.py) · [Einstieg](../🦴️anatomy/🎬️clip/🐍️.py) |
| grasshopper_capsule | Eine parametrische Kapsel: Slider für Breite, Länge, Höhe, Radius, Fensterdurchmesser und Fensterversatz steuern eine Box mit gerundeten Kanten, das runde Fenster und die vier Schraubpunkte, mit Maßlinien in Echtzeit; die Kapseltypen als Varianten einer Definition, ein typischer Fehler mit einem Kreis auf der falschen Ebene, jeder Typ per Orient von seiner Grundebene auf eine Zielebene gesetzt und mit List Item (auch mit Wrap) über acht Ebenen gewählt | [💊️capsule](💊️capsule/🐍️.py) · [Einstieg](../💊️capsule/🎬️clip/🐍️.py) |
| grasshopper_tower | Der Kern mit quadratischem Grundriss, Geschosshöhe und Geschosszahl, eine Serie von Ebenen für jedes Geschoss und jede Fassade, Kapselausrichtung mit Orient und List Item, das Kapitell auf jedem Kern und das Sockelgebäude als einfache Kubatur; Anzahlen aus der Definition gelesen (A 9 × 3 × 2 = 54, B 11 × 4 × 2 = 88 minus 2 entfernte Ebenen = 86), samt typischem Fehler eines vergessenen Cull Index und seiner Korrektur | [🗼️tower](🗼️tower/🐍️.py) · [Einstieg](../🗼️tower/🎬️clip/🐍️.py) |
| grasshopper_casting | Jeder Parameter hat einen Typ, unpassende Daten werden an einem Kapselplatz von Turm B umgewandelt: Number zu Integer mit Rundungsregel, Boolean zu Integer, Point zu Plane (die Kapsel schaut in die falsche Richtung), Line zu Plane, Text zu Number, Zahl zu Domain, Circle zu Curve, Box zu Brep, Zahl zu Boolean und die Umwandlung, die mit roter Komponente und Meldung scheitert; noch nicht in der laufenden Anwendung geprüfte Zeilen tragen eine Markierung im Bild; eine Casting-Matrix aus dem Gezeigten | [🫗️casting](🫗️casting/🐍️.py) · [Einstieg](../🫗️casting/🎬️clip/🐍️.py) |
| grasshopper_trees | Der gesamte Turm mit 140 Kapseln als Datenbaum mit dem Pfad `{BUILDING;TOWER;STORY;FACADE}`, als echter Baum neben dem Turm gezeichnet: Pfadaufbau, Panel und Param Viewer, Flatten Tree, Graft Tree, Simplify Tree, Path Mapper, Flip Matrix, Anzahlen je Turm und Geschoss als leuchtende Kapseln, der Zuordnungsfehler zweier Listen unterschiedlicher Tiefe (Custom Preview färbt die falschen Kapseln) und seine Behebung mit Path Mapper und Graft Tree sowie ein Rückblick auf zehn Begriffe | [🌳️trees](🌳️trees/🐍️.py) · [Einstieg](../🌳️trees/🎬️clip/🐍️.py) |

### Aufbau

- Modell: [🧱️model/🐍️.py](🧱️model/🐍️.py) (nur Standardbibliothek: Datenbäume, Parameter, Casting, Komponenten und die Nakagin-Definitionen), Daten in [🧱️model/🔣️.json](🧱️model/🔣️.json), Schema in [🧱️model/🧬️schema/🔣️.schema.json](🧱️model/🧬️schema/🔣️.schema.json). Gebäudewerte stammen aus [🏢️nakagin](../../🏢️nakagin/🔣️.json). Jeder Panel-Wert, jede Anzahl und jede Zeichnung im Bild entsteht durch Auswertung der Definition.
- Fälle: [🧪️cases/🔣️.json](🧪️cases/🔣️.json) enthält die portablen Erwartungen.
- Tests: [🧪️tests/🐍️.py](🧪️tests/🐍️.py) leitet die Fälle mit unabhängigen Prüfverfahren neu her (`rhino3dm` für Ebenen, Transformationen und Boxen; eine getrennte Datenbaum-Implementierung); [🧪️tests/🟨️.cjs](🧪️tests/🟨️.cjs) prüft das Schema mit AJV; [🧪️tests/🎭️presentation/🐍️.py](🧪️tests/🎭️presentation/🐍️.py) deckt die Szenen ab. Die Prüfverfahren sind reine Testabhängigkeiten.
- Werkzeuge: [🎨️visuals/🐍️.py](🎨️visuals/🐍️.py) (`ClipScene`: Sprache, Zahlen, Live-Werte) und [🦗️canvas](🎨️visuals/🦗️canvas/🐍️.py) (Komponenten, Parameter, Slider, Panels, Drähte, Gruppen, Ribbon-Tabs, Param Viewer). Gestaltung kommt aus dem gemeinsamen Energie-Werkzeugkasten; die Nakagin-Zeichnungen aus [🏢️nakagin/🎨️visuals](../../🏢️nakagin/🎨️visuals/🐍️.py).
- Jeder Themenordner enthält eine Szenenklasse mit zweisprachigem Titel und Sprechtext. Es kommt keine Laufzeitabhängigkeit hinzu.

### Primärquellen

- McNeel Developer Guides: [Grasshopper Data Types](https://developer.rhino3d.com/guides/grasshopper/grasshopper-data-types/), [Advanced Data Structures](https://developer.rhino3d.com/guides/grasshopper/gh-algorithms-and-data-structures/advanced-data-structures/), [Introduction to Data Structures](https://developer.rhino3d.com/guides/grasshopper/gh-algorithms-and-data-structures/data-structures/) und [List Components](https://developer.rhino3d.com/guides/grasshopper/list-components/).
- McNeel API: [GH_DataComparison](https://developer.rhino3d.com/api/grasshopper/html/T_Grasshopper_Kernel_GH_DataComparison.htm); Plugins werden über [Food4Rhino](https://www.food4rhino.com) verteilt.
- Gebäude: Kisho Kurokawa, Nakagin Capsule Tower, [Wikipedia](https://en.wikipedia.org/wiki/Nakagin_Capsule_Tower) und die Quellen im Nakagin-README.

### Rendern und Testen

Die eigenständigen Einstiege liegen in `🦗️grasshopper/<Thema>/🎬️clip/🐍️.py`. Die kanonischen Szenen bleiben in diesem gemeinsamen Clip-Ordner, damit Modelldaten, Sprechtext und Bilder eine Quelle haben. Der Wurzel-Einstieg rendert die Playlist mit fünf Videos.

Ein Video vom Repository-Wurzelverzeichnis aus rendern:

```text
repo 🎓️teaching/🏛️architecture/⚡️energy/🏗️authoring/📜️script.ts render --video grasshopper_anatomy --language de --audience speaker
```

Die gesamte Playlist auf Englisch rendern:

```text
repo 🎓️teaching/🏛️architecture/⚡️energy/🏗️authoring/📜️script.ts render --video grasshopper_anatomy --video grasshopper_capsule --video grasshopper_tower --video grasshopper_casting --video grasshopper_trees --language en --audience spectator
```

`--language en` oder `--language de` ist verpflichtend, `--audience speaker` oder `--audience spectator` ist für jedes Video und jede Playlist wählbar; ohne `--audience` werden beide gerendert. Der gemeinsame Renderer liefert begrenzte Arbeit, Fortschritt, Abbruch, Cache-Herkunft und die Sprecher-/Zuschauerdarstellung.

Verträge und unabhängige Prüfungen ausführen:

```text
repo 🎓️teaching/🏛️architecture/⚡️energy/🏗️authoring/📜️script.ts test
```

### Transkripte

Vollständige barrierefreie Transkripte: [English](🗣️transcript.en.md) · [Deutsch](🗣️transcript.de.md).
