# Five Grasshopper Videos · Fünf Grasshopper-Videos

## English

Five independent architecture videos teach visual programming with Grasshopper 1 (Rhino 8) with **one concrete example**: the Nakagin Capsule Tower in Ginza, Tokyo, by Kisho Kurokawa (1970–1972, demolished 2022), shown as a detailed teaching reconstruction (140 documented capsules: 76 in A and 64 in B; one shared model in [🏢️nakagin](../../🏢️nakagin/README.md)). The original section supplies the 2.80 m pitch, A roof 42.14 m and A head 11.36 m. Ground/podium levels and B roof/head values remain provisional derived datums; A–D programs and free slider edits are design studies. Component and parameter names are the real Grasshopper names.

**Concept and drawing.** Every concept appears on the canvas that computes it and on the Nakagin at the same time: a Number Slider is a dimension line on the capsule, a Panel item lights the capsule it names, a tree branch is one occupied facade group of a capsule level, a wire shares the colour of the element it carries. No box is abstract and no number appears without its object and unit. English and German are equally authored; choose a language explicitly. German narration keeps the English component names, because the Grasshopper interface is English.

| Video ID | What the viewer sees | Folder |
| --- | --- | --- |
| grasshopper_anatomy | The capsule shell built on an empty canvas: script and canvas, a Number Slider (domain, decimals, value) wired into Center Box, component anatomy (name and nickname, inputs left, outputs right, wires, preview eye, grey, orange and red states), floating and input parameters, a Panel showing the output with its path and index, a Boolean Toggle for the round window, a Value List of the A–D studies, a plugin adding a ribbon tab, and item, list (Series), data tree and path plus index on the capsule levels of Tower B, closing with bilingual canvas and data glossaries | [🦴️anatomy](🦴️anatomy/🐍️.py) · [entry](../🦴️anatomy/🎬️clip/🐍️.py) |
| grasshopper_capsule | One parametric capsule: six sliders for the physical 2.6 × 4.2 m reference casing (width, length, height, corner radius, window diameter and offset) drive Center Box on a base plane, Plane Origin, Fillet Edge, the round window cut with Solid Difference and the four bolt points, with live dimension lines; slider domains as design rules (the envelope turns red past the limit), the A–D room-program studies sharing the nominal envelope and 1.30 m porthole, a typical mistake with a circle on the wrong plane, Orient from a base plane onto a rotating target plane, and List Item over eight planes with and without Wrap | [💊️capsule](💊️capsule/🐍️.py) · [entry](../💊️capsule/🎬️clip/🐍️.py) |
| grasshopper_tower | The core envelope and its detailed assembly, a Series of planes at the 2.80 m pitch from the podium roof, Construct Plane door-port frames with visible axes, authored floor masks selecting the occupied positions, Orient from local to world door port, a type pattern read with List Item, the tower revealed level by level, the mistake of an unmasked rectangular grid and its correction with Cull Index (A: 76, B: 64, together 140 read from the definition), the machinery head (A 11.36 m sourced, B provisional), the podium envelope with named assemblies and a room-program switch from B to C | [🗼️tower](🗼️tower/🐍️.py) · [entry](../🗼️tower/🎬️clip/🐍️.py) |
| grasshopper_casting | Every parameter has a type and mismatched data is cast on one capsule slot: text to Number via a Panel, Number to Integer with its rounding rule (and Floor to decide it yourself), Point to Plane (the capsule faces north instead of west), Line to Plane with guessed X and Y and the explicit plane that fixes it, number to Domain with Divide Domain, Circle to Curve, Box to Brep, Boolean to Integer and number to Boolean, the decimal-comma pitfall, and the cast that fails with a red component and its message, repaired with a Value List; a casting matrix of what was shown (green worked, red failed, dashed still to check in Rhino 8) | [🫗️casting](🫗️casting/🐍️.py) · [entry](../🫗️casting/🎬️clip/🐍️.py) |
| grasshopper_trees | The tower of 140 capsules as a tree with the path `{BUILDING;TOWER;STORY;FACADE}` drawn beside the tower: one capsule as item 21 of a flat list against its address, branches as occupied facade groups of variable length, path plus index, Panel and Tree Statistics, Flatten, Graft, Simplify, Path Mapper (`{A;B;C;D}(i)` to `{A;B;C}`, 20 storey branches of two, seven or eight, then `{A;B}` with 76 and 64), Flip Matrix with null placeholders, the data-matching mistake of two lists with different depths (Custom Preview colours the wrong capsules, shown in the Param Viewer) and its repair with Path Mapper and Graft Tree, and a recap of ten terms | [🌳️trees](🌳️trees/🐍️.py) · [entry](../🌳️trees/🎬️clip/🐍️.py) |

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

Choose `--language en` or `--language de` (required) and `--audience speaker` or `--audience spectator` for any video or playlist; omitting `--audience` renders both. Use `--output-root` for a chosen output location, and `--profile final` for Full HD. The shared renderer provides bounded work, progress, cancellation, cache provenance and the speaker/spectator presentation.

Every source scene contains complete English and German narration text. This source folder contains no pre-recorded speech or final videos. The shared renderer attaches existing configured clause recordings when available; narration text alone does not synthesize a recording.

Run the contract and independent checks:

```text
repo 🎓️teaching/🏛️architecture/⚡️energy/🏗️authoring/📜️script.ts test
```

### Transcripts

Complete accessible transcripts: [English](🗣️transcript.en.md) · [Deutsch](🗣️transcript.de.md).

## Deutsch

Fünf eigenständige Architekturvideos lehren visuelles Programmieren mit Grasshopper 1 (Rhino 8) an **einem konkreten Beispiel**: dem Nakagin Capsule Tower in Ginza, Tokio, von Kisho Kurokawa (1970–1972, 2022 abgerissen), gezeigt als detaillierte Lehrrekonstruktion (140 belegte Kapseln: 76 in A und 64 in B; ein gemeinsames Modell in [🏢️nakagin](../../🏢️nakagin/README.md)). Der Originalschnitt belegt 2,80 m Geschosshöhe, 42,14 m Kerndach und 11,36 m Kopf für A. Boden-/Sockelhöhen und Dach-/Kopfwerte von B bleiben vorläufig abgeleitete Höhen; A–D-Programme und freie Slideränderungen sind Entwurfsstudien. Bauteil- und Parameternamen sind die echten Grasshopper-Namen.

**Begriff und Zeichnung.** Jeder Begriff erscheint zugleich auf dem Canvas, der ihn berechnet, und am Nakagin: Ein Number Slider ist eine Maßlinie an der Kapsel, ein Panel-Eintrag lässt die genannte Kapsel aufleuchten, ein Datenast ist eine belegte Fassadengruppe einer Kapselebene, ein Draht trägt die Farbe des Elements, das er führt. Kein Kasten ist abstrakt, und keine Zahl steht ohne Objekt und Einheit. Englisch und Deutsch sind gleichwertig verfasst; die Sprache wird ausdrücklich gewählt. Die deutsche Sprechfassung behält die englischen Komponentennamen, weil die Grasshopper-Oberfläche englisch ist.

| Video-Kennung | Was die Zuschauenden sehen | Ordner |
| --- | --- | --- |
| grasshopper_anatomy | Die Kapselhülle auf leerem Canvas aufgebaut: Skript und Canvas, ein Number Slider (Bereich, Dezimalstellen, Wert) an Center Box, der Aufbau einer Komponente (Name und Kurzname, Eingänge links, Ausgänge rechts, Drähte, Vorschau-Auge, graue, orange und rote Zustände), freie und Eingabeparameter, ein Panel mit Ausgabe, Pfad und Index, ein Boolean Toggle für das runde Fenster, eine Value List der Studien A–D, ein Plugin mit eigenem Reiter sowie Element, Liste (Series), Datenbaum und Pfad plus Index an den Kapselgeschossen von Turm B, abgeschlossen mit zweisprachigen Glossaren zu Canvas und Daten | [🦴️anatomy](🦴️anatomy/🐍️.py) · [Einstieg](../🦴️anatomy/🎬️clip/🐍️.py) |
| grasshopper_capsule | Eine parametrische Kapsel: sechs Slider für die physische Referenzhülle von 2,6 × 4,2 m (Breite, Länge, Höhe, Eckenradius, Fensterdurchmesser und -versatz) steuern Center Box auf einer Grundebene, Plane Origin, Fillet Edge, das mit Solid Difference geschnittene runde Fenster und die vier Schraubpunkte, mit Maßlinien in Echtzeit; Slider-Bereiche als Entwurfsregeln (die Hülle wird jenseits der Grenze rot), die Raumprogramm-Studien A–D mit gemeinsamer Nennhülle und 1,30 m Bullauge, ein typischer Fehler mit einem Kreis auf der falschen Ebene, Orient von einer Grundebene auf eine rotierende Zielebene und List Item über acht Ebenen mit und ohne Wrap | [💊️capsule](💊️capsule/🐍️.py) · [Einstieg](../💊️capsule/🎬️clip/🐍️.py) |
| grasshopper_tower | Die Kernhülle mit ihrem detaillierten Aufbau, eine Series von Ebenen im Raster von 2,80 m ab dem Sockeldach, Construct-Plane-Türanschlussrahmen mit sichtbaren Achsen, festgelegte Geschossmasken für die belegten Positionen, Orient vom lokalen auf den Welt-Türanschluss, ein mit List Item gelesenes Typmuster, der Turm Geschoss für Geschoss enthüllt, der Fehler eines unmaskierten Rechteckrasters und seine Korrektur mit Cull Index (A: 76, B: 64, zusammen 140, aus der Definition gelesen), der Maschinenkopf (A 11,36 m belegt, B vorläufig), die Sockelhülle mit benannten Baugruppen und ein Wechsel des Raumprogramms von B zu C | [🗼️tower](🗼️tower/🐍️.py) · [Einstieg](../🗼️tower/🎬️clip/🐍️.py) |
| grasshopper_casting | Jeder Parameter hat einen Typ, unpassende Daten werden an einem Kapselplatz umgewandelt: Text zu Number über ein Panel, Number zu Integer mit Rundungsregel (und Floor, um selbst zu entscheiden), Point zu Plane (die Kapsel schaut nach Norden statt nach Westen), Line zu Plane mit erratenen X und Y und die explizite Ebene als Korrektur, Zahl zu Domain mit Divide Domain, Circle zu Curve, Box zu Brep, Boolean zu Integer und Zahl zu Boolean, die Dezimalkomma-Falle und die Umwandlung, die mit roter Komponente und Meldung scheitert und mit einer Value List behoben wird; eine Casting-Matrix des Gezeigten (grün gelang, rot scheiterte, gestrichelt noch in Rhino 8 zu prüfen) | [🫗️casting](🫗️casting/🐍️.py) · [Einstieg](../🫗️casting/🎬️clip/🐍️.py) |
| grasshopper_trees | Der Turm mit 140 Kapseln als Datenbaum mit dem Pfad `{BUILDING;TOWER;STORY;FACADE}`, neben dem Turm gezeichnet: eine Kapsel als Element 21 einer flachen Liste im Vergleich zu ihrer Adresse, Äste als belegte Fassadengruppen variabler Länge, Pfad plus Index, Panel und Tree Statistics, Flatten, Graft, Simplify, Path Mapper (`{A;B;C;D}(i)` zu `{A;B;C}`, 20 Geschossäste mit zwei, sieben oder acht Kapseln, dann `{A;B}` mit 76 und 64), Flip Matrix mit Null-Platzhaltern, der Zuordnungsfehler zweier Listen unterschiedlicher Tiefe (Custom Preview färbt die falschen Kapseln, sichtbar im Param Viewer) und seine Behebung mit Path Mapper und Graft Tree sowie ein Rückblick auf zehn Begriffe | [🌳️trees](🌳️trees/🐍️.py) · [Einstieg](../🌳️trees/🎬️clip/🐍️.py) |

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


### Shared Architectural Datums · Gemeinsame Architekturhöhen

The floor grid uses a sourced 2.80 m pitch with three phases, including the two sparse floor-3 capsules. Provisional podium levels are 0 / 4.00 / 7.72 m above the chosen ground; basement depth is 5.00 m. A roof/head heights are 42.14 / 11.36 m; B uses provisional derived 36.62 / 10.83 m. Roof clearances are derived separately from the highest capsule crown of each tower, rather than a shared free overrun. See the provenance on each shared model quantity.

Das Geschossraster nutzt belegte 2,80 m Schrittweite mit drei Phasen, auch für die zwei einzelnen Kapseln im dritten Geschoss. Die vorläufigen Sockelhöhen liegen bei 0 / 4,00 / 7,72 m über dem gewählten Boden; die Kellertiefe beträgt 5,00 m. Dach-/Kopfhöhen von A sind 42,14 / 11,36 m; B nutzt vorläufig abgeleitete 36,62 / 10,83 m. Der Dachfreiraum wird für jeden Turm aus dessen höchster Kapselkrone abgeleitet. Die gemeinsame Modelldatei dokumentiert die Herkunft jedes Werts.
