# Four Excel Videos · Vier Excel-Videos

## English

Four independent architecture videos teach spreadsheet calculation with Microsoft 365 Excel with **one concrete example**: the room program and the heating demand of the Nakagin Capsule Tower in Ginza, Tokyo, by Kisho Kurokawa (1970–1972, demolished 2022), shown as a rounded teaching model (140 capsules in two towers; one shared model in [🏢️nakagin](../../🏢️nakagin/README.md)). Values that are assumed (areas, U-values, air change rate, Tokyo climate) are marked as assumed in the videos and are not survey data. The workbook `Nakagin.xlsx` has the sheets Program, one sheet per capsule type (Type A to Type D), Climate and Energy.

**Concept and drawing.** Every concept appears in the workbook window and on the Nakagin at the same time: a cell is the hatched surface or the counted capsules it computes, a reference is a coloured frame with the same colour on the drawing, a changed width changes every dependent cell and the drawing follows. No box is abstract and no number appears without its object and unit. English and German are equally authored; choose a language explicitly. The German version shows German function names and `;` as separator, as the German Excel does.

| Video ID | What the viewer sees | Folder |
| --- | --- | --- |
| excel_anatomy | The workbook `Nakagin.xlsx`: worksheet, column, row, cell address, range, name box and formula bar; constant, formula and displayed format (2.3 shown as 2.30 m); operators and functions; the capsule floor area as the first formula with the area hatched on the capsule; recalculation when the width changes; precedents and dependents; typical errors | [🦴️anatomy](🦴️anatomy/🐍️.py) · [entry](../🦴️anatomy/🎬️clip/🐍️.py) |
| excel_references | Relative references filled down the surface rows of Type A (U·A per face, each filled row lights its capsule face), the absolute `$D$10` for the design temperature difference with the four F4 forms, mixed references in the Program tower-by-type table, references across sheets, the 3D reference `SUM('Type A:Type D'!B13)` and defined names; what breaks (a reference that slides, a Type B tab moved out of the span, a deleted sheet with `#REF!`) shown as coloured frames and values before and after, and a six-row legend | [📍️references](📍️references/🐍️.py) · [entry](../📍️references/🎬️clip/🐍️.py) |
| excel_program | The room program: net floor area and volume of a capsule from its fittings, the table `Capsules` listing all 140 capsules with tower, storey, facade and type, COUNTIF and COUNTIFS per type and tower, XLOOKUP of type values, SUMPRODUCT of counts × areas, SUMIF totals per tower and a bar chart of areas, with every count lighting the matching capsules on the tower; a swapped-in value updates the dependents, and a check, a typo with a trailing space and its repair show how a count goes wrong | [📋️program](📋️program/🐍️.py) · [entry](../📋️program/🎬️clip/🐍️.py) |
| excel_energy | Envelope surfaces from the dimensions, exposed against attached to the core (back wall factor b), U-values, U·A per surface, H_T by SUM and by SUMPRODUCT, the Tokyo climate with defined names, a wrong climate value and its repair, H_V from air change rate and volume, heating load, annual heating demand and kWh/(m²·a); the Energy sheet sums all capsules by type count and compares the types in a chart; a what-if with a better window U-value updates every dependent cell, arrow and total, and a frozen typed total shows a stale cell and its repair | [⚡️energy](⚡️energy/🐍️.py) · [entry](../⚡️energy/🎬️clip/🐍️.py) |

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

Vier eigenständige Architekturvideos lehren Tabellenkalkulation mit Microsoft 365 Excel an **einem konkreten Beispiel**: dem Raumprogramm und dem Heizwärmebedarf des Nakagin Capsule Tower in Ginza, Tokio, von Kisho Kurokawa (1970–1972, 2022 abgerissen), gezeigt als gerundetes Lehrmodell (140 Kapseln in zwei Türmen; ein gemeinsames Modell in [🏢️nakagin](../../🏢️nakagin/README.md)). Angenommene Werte (Flächen, U-Werte, Luftwechselrate, Klima von Tokio) sind in den Videos als angenommen gekennzeichnet und keine Aufmaßdaten. Die Arbeitsmappe `Nakagin.xlsx` hat die Tabellenblätter Program, je ein Blatt pro Kapseltyp (Type A bis Type D), Climate und Energy.

**Begriff und Zeichnung.** Jeder Begriff erscheint zugleich im Arbeitsmappenfenster und am Nakagin: Eine Zelle ist die schraffierte Fläche oder die gezählten Kapseln, die sie berechnet, ein Bezug ist ein farbiger Rahmen mit derselben Farbe in der Zeichnung, eine geänderte Breite ändert alle abhängigen Zellen, und die Zeichnung folgt. Kein Kasten ist abstrakt, und keine Zahl steht ohne Objekt und Einheit. Englisch und Deutsch sind gleichwertig verfasst; die Sprache wird ausdrücklich gewählt. Die deutsche Fassung zeigt deutsche Funktionsnamen und `;` als Trennzeichen wie das deutsche Excel.

| Video-Kennung | Was die Zuschauenden sehen | Ordner |
| --- | --- | --- |
| excel_anatomy | Die Arbeitsmappe `Nakagin.xlsx`: Tabellenblatt, Spalte, Zeile, Zelladresse, Bereich, Namenfeld und Bearbeitungsleiste; Konstante, Formel und angezeigtes Format (2,3 als 2,30 m); Operatoren und Funktionen; die Kapselgrundfläche als erste Formel mit schraffierter Fläche an der Kapsel; Neuberechnung bei geänderter Breite; Vorgänger und Nachfolger; typische Fehler | [🦴️anatomy](🦴️anatomy/🐍️.py) · [Einstieg](../🦴️anatomy/🎬️clip/🐍️.py) |
| excel_references | Relative Bezüge, nach unten über die Flächenzeilen von Typ A gefüllt (U·A je Fläche, jede gefüllte Zeile lässt ihre Kapselfläche aufleuchten), der absolute `$D$10` für die Auslegungstemperaturdifferenz mit den vier F4-Formen, gemischte Bezüge in der Kreuztabelle Turm mal Typ auf dem Blatt Program, Bezüge über Blätter, der 3D-Bezug `SUMME('Typ A:Typ D'!B13)` und definierte Namen; was bricht (ein wandernder Bezug, ein aus der Spanne verschobenes Blatt Typ B, ein gelöschtes Blatt mit `#BEZUG!`) als farbige Rahmen und Werte vorher und nachher sowie eine Legende mit sechs Zeilen | [📍️references](📍️references/🐍️.py) · [Einstieg](../📍️references/🎬️clip/🐍️.py) |
| excel_program | Das Raumprogramm: Nettogrundfläche und Volumen einer Kapsel aus ihren Einbauten, die Tabelle `Capsules` mit allen 140 Kapseln samt Turm, Geschoss, Fassade und Typ, ZÄHLENWENN und ZÄHLENWENNS je Typ und Turm, XVERWEIS der Typwerte, SUMMENPRODUKT aus Anzahl mal Fläche, SUMMEWENN-Summen je Turm und ein Balkendiagramm der Flächen, wobei jede Anzahl die passenden Kapseln am Turm aufleuchten lässt; ein ausgetauschter Wert aktualisiert die abhängigen Zellen, und eine Prüfung, ein Tippfehler mit Leerzeichen und seine Korrektur zeigen, wie eine Zählung falsch wird | [📋️program](📋️program/🐍️.py) · [Einstieg](../📋️program/🎬️clip/🐍️.py) |
| excel_energy | Hüllflächen aus den Maßen, außenliegend gegenüber am Kern angeschlossen (Faktor b der Rückwand), U-Werte, U·A je Fläche, H_T mit SUMME und mit SUMMENPRODUKT, das Klima von Tokio mit definierten Namen, ein falscher Klimawert und seine Korrektur, H_V aus Luftwechselrate und Volumen, Heizlast, jährlicher Heizwärmebedarf und kWh/(m²·a); das Blatt Energy summiert alle Kapseln nach Typanzahl und vergleicht die Typen im Diagramm; ein Was-wäre-wenn mit besserem Fenster-U-Wert aktualisiert alle abhängigen Zellen, Pfeile und Summen, und eine als Konstante eingetippte Summe zeigt eine veraltete Zelle und ihre Reparatur | [⚡️energy](⚡️energy/🐍️.py) · [Einstieg](../⚡️energy/🎬️clip/🐍️.py) |

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
