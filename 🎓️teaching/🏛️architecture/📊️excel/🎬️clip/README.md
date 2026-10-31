# Four Excel Videos · Vier Excel-Videos

## English

The four room programs are proposed teaching studies. The shared historical reconstruction uses six geometric capsule families and the documented A76/B64 floor schedule.

Four independent architecture videos teach spreadsheet calculation with Microsoft 365 Excel with **one concrete example**: the room program and the heating demand of the Nakagin Capsule Tower in Ginza, Tokyo, by Kisho Kurokawa (1970–1972, demolished 2022), shown as a rounded teaching model (140 capsules in two towers; one shared model in [🏢️nakagin](../../🏢️nakagin/README.md)). Room programs and thermal boundary choices are teaching assumptions. Climate normals and cited dimensions retain their individual source provenance; the model is not a measured preservation survey. The workbook `Nakagin.xlsx` has the sheets Program, one sheet per capsule type (Type A to Type D), Climate and Energy.

**Concept and drawing.** Every concept appears in the workbook window and on the Nakagin at the same time: a cell is the hatched surface or the counted capsules it computes, a reference is a coloured frame with the same colour on the drawing, a changed width changes every dependent cell and the drawing follows. No box is abstract and no number appears without its object and unit. English and German are equally authored; choose a language explicitly. The German version shows German function names and `;` as separator, as the German Excel does.

| Video ID | What the viewer sees | Folder |
| --- | --- | --- |
| excel_anatomy | The workbook `Nakagin.xlsx`: worksheets (Program, one per capsule type, Climate, Energy), column, row, cell address, range, name box and formula bar; constants and text, displayed format against stored value (a rounded number with a unit), the capsule floor area as the first formula with coloured references and the area hatched on the capsule, the typical mistake of missing brackets (operator precedence) and its fix, recalculation when the width changes, precedents and dependents, functions and typical errors | [🦴️anatomy](🦴️anatomy/🐍️.py) · [entry](../🦴️anatomy/🎬️clip/🐍️.py) |
| excel_references | Relative references filled down the surface rows of Type A (U·A per face, each filled row lights its capsule face), the absolute `$D$10` for the design temperature difference with the four F4 forms and the mistake of a forgotten dollar sign, a two-way table of mixed references, the same pattern counting capsules by tower and type, references across sheets (a sliding sheet reference edited per copy), the 3D reference over the type sheets with its two failures (a sheet moved out of the span, a deleted sheet that silently shrinks the sum), an absolute reference broken by a deleted row, defined names, and a recap | [📍️references](📍️references/🐍️.py) · [entry](../📍️references/🎬️clip/🐍️.py) |
| excel_program | The room program: fittings and net floor area of a capsule on the type sheets, free floor and volume, the 140 capsule rows formatted as the table `Capsules` (a selected row lights its capsule), COUNTIF and COUNTIFS per type and tower, XLOOKUP of count and free floor by type, SUMPRODUCT of counts × free floor, a bar chart of the four totals, SUMIF per tower, swapping one capsule type to show the update, and a sanity check with the typo of a trailing space (139 and FALSE) and its repair, closing with a recap | [📋️program](📋️program/🐍️.py) · [entry](../📋️program/🎬️clip/🐍️.py) |
| excel_energy | Envelope surfaces from the nominal dimensions (reference casing kept separate), the attached back wall with factor b, U-values, U·A per surface as arrows, H_T by SUM and by SUMPRODUCT, the Climate sheet with defined names and heating degree days as an area, the mistake of treating the back wall as exposed and its repair, H_V from air change rate and volume, loss per kelvin, heating load, annual heating demand and kWh/m²; the Energy sheet multiplies by the capsule count of the Program sheet, a comparison of the four studies, a what-if with a better window U-value traced through dependents, and a typed building total that stops following the change, then its repair | [⚡️energy](⚡️energy/🐍️.py) · [entry](../⚡️energy/🎬️clip/🐍️.py) |

### Structure

- Model: [🧱️model/🐍️.py](🧱️model/🐍️.py) (stdlib only: A1 references, copy translation, formula parser and evaluator, dependencies, English and German formula text and the Nakagin workbook), data in [🧱️model/🔣️.json](🧱️model/🔣️.json), schema in [🧱️model/🧬️schema/🔣️.schema.json](🧱️model/🧬️schema/🔣️.schema.json). Building values come from [🏢️nakagin](../../🏢️nakagin/🔣️.json). Every cell value on screen comes from evaluating the workbook.
- Cases: [🧪️cases/🔣️.json](🧪️cases/🔣️.json) holds the portable expectations.
- Tests: [🧪️tests/🐍️.py](🧪️tests/🐍️.py) re-derives the cases with independent oracles (`openpyxl` for references and the `.xlsx` file, `formulas` evaluating that file against every cell of our engine, NumPy for the energy sums); [🧪️tests/🟨️.cjs](🧪️tests/🟨️.cjs) validates the schema with AJV; [🧪️tests/🎭️presentation/🐍️.py](🧪️tests/🎭️presentation/🐍️.py) covers the scenes. The oracles are test-only dependencies.
- Toolkit: [🎨️visuals/🐍️.py](🎨️visuals/🐍️.py) (`ClipScene`: language, numbers, live values) and [📗️workbook](🎨️visuals/📗️workbook/🐍️.py) (window, ribbon, name box, formula bar, grid, sheet tabs, reference frames, fill handle, trace arrows, error tags, chart). Styling comes from the shared energy authoring toolkit; the Nakagin drawings come from [🏢️nakagin/🎨️visuals](../../🏢️nakagin/🎨️visuals/🐍️.py).
- Each topic folder holds one scene class with its bilingual title and narration. No runtime dependency is added.

### Primary References

- Microsoft Support: [Create a 3-D reference to the same cell range on multiple worksheets](https://support.microsoft.com/en-us/excel/create-a-3-d-reference-to-the-same-cell-range-on-multiple-worksheets), [Dynamic array formulas and spilled array behavior](https://support.microsoft.com/en-us/office/dynamic-array-formulas-and-spilled-array-behavior-205c6b06-03ba-4151-89a1-87a7eb36e531) and [XLOOKUP function](https://support.microsoft.com/de-de/excel/get-started/xlookup-function).
- Microsoft Garage: [Functions Translator](https://www.microsoft.com/en-us/garage/profiles/functions-translator) for German function names.
- Building: Kisho Kurokawa, Nakagin Capsule Tower, [Wikipedia](https://en.wikipedia.org/wiki/Nakagin_Capsule_Tower) and the sources in the Nakagin README.

### Render and Test

The standalone entries live in `📊️excel/<topic>/🎬️clip/🐍️.py`. Canonical scenes remain under this shared clip folder so model data, narration and visuals have one source. The root entry renders the four-video playlist.

Run one video from the repository root:

```text
repo 🎓️teaching/🏛️architecture/⚡️energy/🏗️authoring/📜️script.ts render --video excel_anatomy --language en --audience speaker
repo 🎓️teaching/🏛️architecture/⚡️energy/🏗️authoring/📜️script.ts render --video excel_references --language en --audience speaker
repo 🎓️teaching/🏛️architecture/⚡️energy/🏗️authoring/📜️script.ts render --video excel_program --language en --audience speaker
repo 🎓️teaching/🏛️architecture/⚡️energy/🏗️authoring/📜️script.ts render --video excel_energy --language en --audience speaker
```

Render the entire four-video playlist in German:

```text
repo 🎓️teaching/🏛️architecture/⚡️energy/🏗️authoring/📜️script.ts render --video excel_anatomy --video excel_references --video excel_program --video excel_energy --language de --audience spectator
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

Die vier Raumprogramme sind vorgeschlagene Lehrstudien. Die gemeinsame historische Rekonstruktion verwendet sechs geometrische Kapselfamilien und den belegten Geschossbestand A76/B64.

Vier eigenständige Architekturvideos lehren Tabellenkalkulation mit Microsoft 365 Excel an **einem konkreten Beispiel**: dem Raumprogramm und dem Heizwärmebedarf des Nakagin Capsule Tower in Ginza, Tokio, von Kisho Kurokawa (1970–1972, 2022 abgerissen), gezeigt als gerundetes Lehrmodell (140 Kapseln in zwei Türmen; ein gemeinsames Modell in [🏢️nakagin](../../🏢️nakagin/README.md)). Raumprogramme und thermische Randbedingungen sind Lehrannahmen. Klimanormalwerte und zitierte Maße behalten ihre eigene Quellenherkunft; das Modell ist kein Aufmaß zur Denkmalpflege. Die Arbeitsmappe `Nakagin.xlsx` hat die Tabellenblätter Program, je ein Blatt pro Kapseltyp (Type A bis Type D), Climate und Energy.

**Begriff und Zeichnung.** Jeder Begriff erscheint zugleich im Arbeitsmappenfenster und am Nakagin: Eine Zelle ist die schraffierte Fläche oder die gezählten Kapseln, die sie berechnet, ein Bezug ist ein farbiger Rahmen mit derselben Farbe in der Zeichnung, eine geänderte Breite ändert alle abhängigen Zellen, und die Zeichnung folgt. Kein Kasten ist abstrakt, und keine Zahl steht ohne Objekt und Einheit. Englisch und Deutsch sind gleichwertig verfasst; die Sprache wird ausdrücklich gewählt. Die deutsche Fassung zeigt deutsche Funktionsnamen und `;` als Trennzeichen wie das deutsche Excel.

| Video-Kennung | Was die Zuschauenden sehen | Ordner |
| --- | --- | --- |
| excel_anatomy | Die Arbeitsmappe `Nakagin.xlsx`: Tabellenblätter (Program, eines je Kapseltyp, Climate, Energy), Spalte, Zeile, Zelladresse, Bereich, Namenfeld und Bearbeitungsleiste; Konstanten und Text, angezeigtes Format gegenüber gespeichertem Wert (gerundete Zahl mit Einheit), die Kapselgrundfläche als erste Formel mit farbigen Bezügen und schraffierter Fläche an der Kapsel, der typische Fehler fehlender Klammern (Punkt vor Strich) und seine Korrektur, Neuberechnung bei geänderter Breite, Vorgänger und Nachfolger, Funktionen und typische Fehler | [🦴️anatomy](🦴️anatomy/🐍️.py) · [Einstieg](../🦴️anatomy/🎬️clip/🐍️.py) |
| excel_references | Relative Bezüge, nach unten über die Flächenzeilen von Typ A gefüllt (U·A je Fläche, jede gefüllte Zeile lässt ihre Kapselfläche aufleuchten), der absolute `$D$10` für die Auslegungstemperaturdifferenz mit den vier F4-Formen und dem Fehler eines vergessenen Dollarzeichens, eine Kreuztabelle gemischter Bezüge, dasselbe Muster zählt Kapseln nach Turm und Typ, Bezüge über Blätter (ein wandernder Blattbezug, je Kopie angepasst), der 3D-Bezug über die Typblätter mit seinen zwei Fehlern (ein aus der Spanne verschobenes Blatt, ein gelöschtes Blatt, das die Summe unbemerkt schrumpfen lässt), ein durch gelöschte Zeile gebrochener absoluter Bezug, definierte Namen und ein Rückblick | [📍️references](📍️references/🐍️.py) · [Einstieg](../📍️references/🎬️clip/🐍️.py) |
| excel_program | Das Raumprogramm: Einbauten und Nettogrundfläche einer Kapsel auf den Typblättern, freie Fläche und Volumen, die 140 Kapselzeilen als Tabelle `Capsules` formatiert (eine gewählte Zeile lässt ihre Kapsel aufleuchten), ZÄHLENWENN und ZÄHLENWENNS je Typ und Turm, XVERWEIS von Anzahl und freier Fläche nach Typ, SUMMENPRODUKT aus Anzahl mal freier Fläche, ein Balkendiagramm der vier Summen, SUMMEWENN je Turm, ein ausgetauschter Kapseltyp zeigt die Aktualisierung, und eine Plausibilitätsprüfung mit dem Tippfehler eines Leerzeichens (139 und FALSCH) und seiner Korrektur, abgeschlossen mit einem Rückblick | [📋️program](📋️program/🐍️.py) · [Einstieg](../📋️program/🎬️clip/🐍️.py) |
| excel_energy | Hüllflächen aus den Nennmaßen (Referenzhülle getrennt gehalten), die am Kern angeschlossene Rückwand mit Faktor b, U-Werte, U·A je Fläche als Pfeile, H_T mit SUMME und mit SUMMENPRODUKT, das Blatt Climate mit definierten Namen und Heizgradtagen als Fläche, der Fehler, die Rückwand wie eine außenliegende zu behandeln, und seine Korrektur, H_V aus Luftwechselrate und Volumen, Verlust je Kelvin, Heizlast, jährlicher Heizwärmebedarf und kWh/m²; das Blatt Energy multipliziert mit der Kapselanzahl vom Blatt Program, ein Vergleich der vier Studien, ein Was-wäre-wenn mit besserem Fenster-U-Wert über die Nachfolger verfolgt und eine als Konstante eingetippte Gebäudesumme, die der Änderung nicht mehr folgt, samt Reparatur | [⚡️energy](⚡️energy/🐍️.py) · [Einstieg](../⚡️energy/🎬️clip/🐍️.py) |

### Aufbau

- Modell: [🧱️model/🐍️.py](🧱️model/🐍️.py) (nur Standardbibliothek: A1-Bezüge, Kopierübersetzung, Formelparser und -auswertung, Abhängigkeiten, englischer und deutscher Formeltext sowie die Nakagin-Arbeitsmappe), Daten in [🧱️model/🔣️.json](🧱️model/🔣️.json), Schema in [🧱️model/🧬️schema/🔣️.schema.json](🧱️model/🧬️schema/🔣️.schema.json). Gebäudewerte stammen aus [🏢️nakagin](../../🏢️nakagin/🔣️.json). Jeder Zellwert im Bild entsteht durch Auswertung der Arbeitsmappe.
- Fälle: [🧪️cases/🔣️.json](🧪️cases/🔣️.json) enthält die portablen Erwartungen.
- Tests: [🧪️tests/🐍️.py](🧪️tests/🐍️.py) leitet die Fälle mit unabhängigen Prüfverfahren neu her (`openpyxl` für Bezüge und die `.xlsx`-Datei, `formulas` wertet diese Datei gegen jede Zelle unserer Engine aus, NumPy für die Energiesummen); [🧪️tests/🟨️.cjs](🧪️tests/🟨️.cjs) prüft das Schema mit AJV; [🧪️tests/🎭️presentation/🐍️.py](🧪️tests/🎭️presentation/🐍️.py) deckt die Szenen ab. Die Prüfverfahren sind reine Testabhängigkeiten.
- Werkzeuge: [🎨️visuals/🐍️.py](🎨️visuals/🐍️.py) (`ClipScene`: Sprache, Zahlen, Live-Werte) und [📗️workbook](🎨️visuals/📗️workbook/🐍️.py) (Fenster, Menüband, Namenfeld, Bearbeitungsleiste, Raster, Blattregister, Bezugsrahmen, Ausfüllkästchen, Spurpfeile, Fehlermarken, Diagramm). Gestaltung kommt aus dem gemeinsamen Energie-Werkzeugkasten; die Nakagin-Zeichnungen aus [🏢️nakagin/🎨️visuals](../../🏢️nakagin/🎨️visuals/🐍️.py).
- Jeder Themenordner enthält eine Szenenklasse mit zweisprachigem Titel und Sprechtext. Es kommt keine Laufzeitabhängigkeit hinzu.

### Primärquellen

- Microsoft Support: [3D-Bezug auf denselben Zellbereich mehrerer Tabellenblätter](https://support.microsoft.com/en-us/excel/create-a-3-d-reference-to-the-same-cell-range-on-multiple-worksheets), [Dynamische Arrays und übergelaufenes Verhalten](https://support.microsoft.com/en-us/office/dynamic-array-formulas-and-spilled-array-behavior-205c6b06-03ba-4151-89a1-87a7eb36e531) und [XVERWEIS-Funktion](https://support.microsoft.com/de-de/excel/get-started/xlookup-function).
- Microsoft Garage: [Functions Translator](https://www.microsoft.com/en-us/garage/profiles/functions-translator) für deutsche Funktionsnamen.
- Gebäude: Kisho Kurokawa, Nakagin Capsule Tower, [Wikipedia](https://en.wikipedia.org/wiki/Nakagin_Capsule_Tower) und die Quellen im Nakagin-README.

### Rendern und Testen

Die eigenständigen Einstiege liegen in `📊️excel/<Thema>/🎬️clip/🐍️.py`. Die kanonischen Szenen bleiben in diesem gemeinsamen Clip-Ordner, damit Modelldaten, Sprechtext und Bilder eine Quelle haben. Der Wurzel-Einstieg rendert die Playlist mit vier Videos.

Ein Video vom Repository-Wurzelverzeichnis aus rendern:

```text
repo 🎓️teaching/🏛️architecture/⚡️energy/🏗️authoring/📜️script.ts render --video excel_anatomy --language de --audience speaker
```

Die gesamte Playlist auf Englisch rendern:

```text
repo 🎓️teaching/🏛️architecture/⚡️energy/🏗️authoring/📜️script.ts render --video excel_anatomy --video excel_references --video excel_program --video excel_energy --language en --audience spectator
```

`--language en` oder `--language de` ist verpflichtend, `--audience speaker` oder `--audience spectator` ist für jedes Video und jede Playlist wählbar; ohne `--audience` werden beide gerendert. Der gemeinsame Renderer liefert begrenzte Arbeit, Fortschritt, Abbruch, Cache-Herkunft und die Sprecher-/Zuschauerdarstellung.

Verträge und unabhängige Prüfungen ausführen:

```text
repo 🎓️teaching/🏛️architecture/⚡️energy/🏗️authoring/📜️script.ts test
```

### Transkripte

Vollständige barrierefreie Transkripte: [English](🗣️transcript.en.md) · [Deutsch](🗣️transcript.de.md).


### Placement Datums · Platzierungshöhen

The floor grid uses a sourced 2.80 m pitch with three phases, including the two sparse floor-3 capsules. Provisional podium levels are 0 / 4.00 / 7.72 m above the chosen ground; basement depth is 5.00 m. A roof/head heights are 42.14 / 11.36 m; B uses provisional derived 36.62 / 10.83 m. Roof clearances are derived separately from the highest capsule crown of each tower, rather than a shared free overrun. See the provenance on each shared model quantity.

Das Geschossraster nutzt belegte 2,80 m Schrittweite mit drei Phasen, auch für die zwei einzelnen Kapseln im dritten Geschoss. Die vorläufigen Sockelhöhen liegen bei 0 / 4,00 / 7,72 m über dem gewählten Boden; die Kellertiefe beträgt 5,00 m. Dach-/Kopfhöhen von A sind 42,14 / 11,36 m; B nutzt vorläufig abgeleitete 36,62 / 10,83 m. Der Dachfreiraum wird für jeden Turm aus dessen höchster Kapselkrone abgeleitet. Die gemeinsame Modelldatei dokumentiert die Herkunft jedes Werts.


The current proposed program assignment is A36 / B35 / C33 / D36 across the140 measured-placement slots. These are study labels, not historical capsule geometry families.

Die aktuelle vorgeschlagene Programmzuordnung lautet A36 / B35 / C33 / D36 für die140 Plätze mit messgestützter Anordnung. Das sind Studienbezeichnungen, keine historischen Kapsel-Geometriefamilien.

Reference capsule drawings measure the physical 2.60 × 4.20 × 2.50 m casing. Workbook 2.50 × 4.00 m dimensions remain explicitly nominal program/energy controls; the usable-volume diagram is an exact analytical box without fabrication allowances.

Referenzzeichnungen bemaßen das physische 2,60 × 4,20 × 2,50-m-Gehäuse. Die 2,50 × 4,00-m-Werte der Arbeitsmappe bleiben nominale Programm-/Energiewerte; das nutzbare Volumen erscheint als exakte analytische Box ohne Fertigungszugaben.
