# Six BIM Videos · Sechs BIM-Videos

## English

Six independent architecture videos teach building information modelling with **one concrete example**: the Villa Savoye in Poissy by Le Corbusier and Pierre Jeanneret (1928–1931), reconstructed for the original completed state of 1930–1931, with explicit cellar, ground, first and roof storeys. Each value retains its own sourced, inferred or assumed basis; exact archival datums such as the 9.325 m crest are preserved. Registered physical section envelopes do not establish measured concrete cores. Vehicle dimensions, loads, unmeasured profiles and incomplete service routes retain explicit assumptions and uncertainty. Names follow IFC 4.3 (ISO 16739-1:2024, schema `IFC4X3_ADD2`).

**Concept and drawing.** Every concept is shown on the villa at the same time as its value: a dimension line between two pilotis, a hatched clear width, a filled volume, load arrows, a highlighted element, a revision badge. A class tree, a property table or a STEP line never stands alone; a leader or a matching colour ties each token to the drawn element. Nothing is an abstract box, and no number appears without its object and unit. English and German are equally authored; choose a language explicitly. The primary roof and the covered-garden canopy are separate slabs. The numbered support register includes circular, polygonal and hollow profiles. Native multipart bodies retain the solarium aperture/cap, roof stair enclosure and canopies, compensated U stairs, open metal ramp guard, hollow cellar assemblies, separate hatch covers, regional facade leaves, L/U/shelf members and original service reconstruction. Class counts refer to IFC products; an assembly may contain many physical parts. Counts are read from the source and checked against explicitly declared fixture deltas. Historical descriptions state capacity for three cars; fitting and manoeuvring three assumed modern cars remain unverified. Arrival and static-pose controls must be checked against the current ground enclosure.

| Video ID | What the viewer sees | Folder |
| --- | --- | --- |
| bim_information | A bare number becomes a piloti spacing: unit, context, place and provenance on the ground-floor plan; the clear width between two pilotis, the car with safety margins, the swept path and the translucent entrance surround and timber door leaves, then the properties and relationships of the piloti handed over as STEP tokens | [📊️information](📊️information/🐍️.py) · [entry](../📊️information/🎬️clip/🐍️.py) |
| bim_objects | The IFC class tree beside the villa; one piloti among all occurrences of the piloti type, its property set, representations, radius and height as dimensions and its geometric section volume; a change followed by occurrences of that selected type; a wall with its opening and window (has-a, not is-a) | [🧬️objects](🧬️objects/🐍️.py) · [entry](../🧬️objects/🎬️clip/🐍️.py) |
| bim_representations | One first-floor wall with its ribbon window in plan, section, elevation and 3D, always with the same GlobalId; stored Axis, FootPrint and Body against derived views, drawing scales 1:200 and 1:20, a design event that regenerates every view, and a hand-drafted copy that goes stale | [👁️representations](👁️representations/🐍️.py) · [entry](../👁️representations/🎬️clip/🐍️.py) |
| bim_models | Architecture, structure and services models of the villa on one schema: class rows summed per model, federation by GlobalId on shared coordinates, the shared slab, a clash between a downpipe and a piloti, and the translation to a structural analysis model with members, supports, loads and a list of what is lost | [🧩️models](🧩️models/🐍️.py) · [entry](../🧩️models/🎬️clip/🐍️.py) |
| bim_exchange | One piloti and the first-floor slab leave the model three ways: STEP lines tied to the drawn piloti, a triangle mesh without class and properties, and an analysis model with mid-surface slab, tributary areas and reactions; an IDS check and the way results come back | [🔌️exchange](🔌️exchange/🐍️.py) · [entry](../🔌️exchange/🎬️clip/🐍️.py) |
| bim_consistency | An explicit paving-reserve scenario on the roof slab: the GlobalId stays, revisions rise, the load and every piloti reaction follow, the linked analysis model turns stale, a check and a BCF topic report it, and resolution re-translates; two edits of one revision need a person to decide | [🔗️consistency](🔗️consistency/🐍️.py) · [entry](../🔗️consistency/🎬️clip/🐍️.py) |

### Structure

- Model: [🧱️model/🐍️.py](🧱️model/🐍️.py) (stdlib only), data in [🧱️model/🔣️.json](🧱️model/🔣️.json), schema in [🧱️model/🧬️schema/🔣️.schema.json](🧱️model/🧬️schema/🔣️.schema.json). STEP, mesh and IDS exchange in [🧱️model/🔌️exchange/🐍️.py](🧱️model/🔌️exchange/🐍️.py), the analysis translation in [🧱️model/🧮️analysis/🐍️.py](🧱️model/🧮️analysis/🐍️.py). Every drawing and every number on screen is derived from the model.
- Cases: [🧪️cases/🔣️.json](🧪️cases/🔣️.json) holds the portable expectations (GlobalId vectors, information, class chains, representations, STEP, mesh, IDS, analysis, tributary areas, clash, events, invalid models and the six chapters).
- Tests: [🧪️tests/🐍️.py](🧪️tests/🐍️.py) re-derives the cases with NumPy, SciPy and independent oracles (GlobalId encoder against `uuid`, bisection, mesh volume and hull, kd-tree tributary areas); [🧪️tests/🟨️.cjs](🧪️tests/🟨️.cjs) validates the schema with AJV and rejects invalid mutations; [🧪️tests/🎭️presentation/🐍️.py](🧪️tests/🎭️presentation/🐍️.py) covers the scenes.
- Toolkit: [🎨️visuals/🐍️.py](🎨️visuals/🐍️.py) (`ClipScene`: language, numbers, live values), [📐️drafting](🎨️visuals/📐️drafting/🐍️.py) (dimensions, hatches, leaders), [🧾️panels](🎨️visuals/🧾️panels/🐍️.py) (class trees, property tables, STEP panels, model cards), [🏛️savoye](🎨️visuals/🏛️savoye/🐍️.py) (plan, section, elevation, axonometric, detail) and [🧮️analysis](🎨️visuals/🧮️analysis/🐍️.py). Styling comes from the shared energy authoring toolkit.
- Each topic folder holds one scene class with its bilingual title and narration. No runtime dependency is added.

### Render and Test

The standalone entries live in `🏗️bim/<topic>/🎬️clip/🐍️.py`. Canonical scenes remain under this shared clip folder so geometry, model data, narration and visuals have one source. The root entry renders the six-video playlist.

Run one video from the repository root:

```text
repo 🎓️teaching/🏛️architecture/⚡️energy/🏗️authoring/📜️script.ts render --video bim_information --language en --audience speaker
repo 🎓️teaching/🏛️architecture/⚡️energy/🏗️authoring/📜️script.ts render --video bim_objects --language en --audience speaker
repo 🎓️teaching/🏛️architecture/⚡️energy/🏗️authoring/📜️script.ts render --video bim_representations --language en --audience speaker
repo 🎓️teaching/🏛️architecture/⚡️energy/🏗️authoring/📜️script.ts render --video bim_models --language en --audience speaker
repo 🎓️teaching/🏛️architecture/⚡️energy/🏗️authoring/📜️script.ts render --video bim_exchange --language en --audience speaker
repo 🎓️teaching/🏛️architecture/⚡️energy/🏗️authoring/📜️script.ts render --video bim_consistency --language en --audience speaker
```

Render the entire six-video playlist in German:

```text
repo 🎓️teaching/🏛️architecture/⚡️energy/🏗️authoring/📜️script.ts render --video bim_information --video bim_objects --video bim_representations --video bim_models --video bim_exchange --video bim_consistency --language de --audience spectator
```

Choose `--language en` or `--language de` (required) and `--audience speaker` or `--audience spectator` for any video or playlist; omitting `--audience` renders both. Use `--output-root` for a chosen output location, and `--quality h --fps 30` for Full HD. The shared renderer provides bounded work, progress, cancellation, cache provenance and the speaker/spectator presentation.

Every source scene contains complete English and German narration text. This source folder contains no pre-recorded speech or final videos. The shared renderer attaches existing configured clause recordings when available; narration text alone does not synthesize a recording.

Run the contract and independent numerical checks:

```text
repo 🎓️teaching/🏛️architecture/⚡️energy/🏗️authoring/📜️script.ts test
```

### Transcripts

Complete accessible transcripts: [English](🗣️transcript.en.md) · [Deutsch](🗣️transcript.de.md).

## Deutsch

Sechs eigenständige Architekturvideos lehren Building Information Modelling an **einem konkreten Beispiel**: der Villa Savoye in Poissy von Le Corbusier und Pierre Jeanneret (1928–1931), rekonstruiert für den ursprünglichen fertiggestellten Zustand von 1930–1931 mit explizitem Keller-, Erd-, Ober- und Dachgeschoss. Jeder Wert behält seine eigene belegte, gefolgerte oder angenommene Grundlage; genaue Archivkoten wie der 9,325 m hohe Scheitel bleiben erhalten. Registrierte physische Querschnittshüllen belegen keine vermessenen Betonkerne. Fahrzeugmaße, Lasten, unvermessene Profile und unvollständige Leitungswege behalten ausdrückliche Annahmen und Unsicherheiten. Die Namen folgen IFC 4.3 (ISO 16739-1:2024, Schema `IFC4X3_ADD2`).

**Begriff und Zeichnung.** Jeder Begriff wird zugleich mit seinem Wert an der Villa gezeigt: eine Maßlinie zwischen zwei Pilotis, eine schraffierte lichte Weite, ein gefülltes Volumen, Lastpfeile, ein hervorgehobenes Bauteil, ein Revisionsschild. Klassenbaum, Eigenschaftstabelle oder STEP-Zeile stehen nie allein; eine Hilfslinie oder dieselbe Farbe verbindet jedes Token mit dem gezeichneten Bauteil. Nichts ist ein abstrakter Kasten, und keine Zahl steht ohne Objekt und Einheit. Englisch und Deutsch sind gleichwertig verfasst; die Sprache wird ausdrücklich gewählt. Hauptdach und Überdachung des hängenden Gartens sind eigene Platten. Das nummerierte Stützenregister umfasst kreisförmige, polygonale und hohle Profile. Native mehrteilige Körper erhalten Solariumsöffnung und Abdeckung, Dachtreppenhaus und Überdachungen, verzogene U-Treppen, offenes Metallgeländer, hohle Kellerbauten, getrennte Lukendeckel, regionale Fassadenschalen, L-/U-/Konsolenglieder und die ursprüngliche Haustechnikrekonstruktion. Klassenzahlen zählen IFC-Objekte; eine Baugruppe kann viele physische Teile enthalten. Zahlen stammen aus der Quelle und werden gegen ausdrücklich erklärte Fixture-Änderungen geprüft. Historische Beschreibungen nennen Platz für drei Autos; Unterbringung und Manöver dreier angenommener moderner Autos bleiben ungeprüft. Anfahrt und statische Stellpositionen müssen gegen die aktuelle Erdgeschossumfassung geprüft werden.

| Video-Kennung | Was die Zuschauenden sehen | Ordner |
| --- | --- | --- |
| bim_information | Eine nackte Zahl wird zum Pilotiabstand: Einheit, Kontext, Ort und Herkunft im Grundriss des Erdgeschosses; die lichte Weite zwischen zwei Pilotis, das Auto mit Sicherheitsabständen, der überstrichene Weg und die transluzente Eingangsumfassung und Holztürblätter, dann Eigenschaften und Beziehungen der Pilotis, übergeben als STEP-Token | [📊️information](📊️information/🐍️.py) · [Einstieg](../📊️information/🎬️clip/🐍️.py) |
| bim_objects | Der IFC-Klassenbaum neben der Villa; eine Piloti unter allen Vorkommen des Pilotitypen, ihr Property Set, ihre Darstellungen, Radius und Höhe als Maße und ihr geometrisches Querschnittsvolumen; eine Änderung, der die Vorkommen dieses ausgewählten Typs folgen; eine Wand mit Öffnung und Fenster (hat-ein, nicht ist-ein) | [🧬️objects](🧬️objects/🐍️.py) · [Einstieg](../🧬️objects/🎬️clip/🐍️.py) |
| bim_representations | Eine Wand des Obergeschosses mit Fensterband in Grundriss, Schnitt, Ansicht und 3D, stets mit derselben GlobalId; gespeicherte Axis, FootPrint und Body gegen abgeleitete Ansichten, Zeichenmaßstäbe 1:200 und 1:20, ein Entwurfsereignis, das alle Ansichten neu erzeugt, und eine von Hand gezeichnete Kopie, die veraltet | [👁️representations](👁️representations/🐍️.py) · [Einstieg](../👁️representations/🎬️clip/🐍️.py) |
| bim_models | Architektur-, Tragwerks- und Haustechnikmodell der Villa auf einem Schema: Klassenzeilen je Modell summiert, Föderation über GlobalId in gemeinsamen Koordinaten, die gemeinsame Decke, eine Kollision zwischen Fallrohr und Piloti und die Übersetzung in ein Tragwerksmodell mit Stäben, Lagern, Lasten und einer Liste des Verlorenen | [🧩️models](🧩️models/🐍️.py) · [Einstieg](../🧩️models/🎬️clip/🐍️.py) |
| bim_exchange | Eine Piloti und die Decke über dem Erdgeschoss verlassen das Modell auf drei Wegen: STEP-Zeilen, verbunden mit der gezeichneten Piloti, ein Dreiecksnetz ohne Klasse und Eigenschaften und ein Berechnungsmodell mit Mittelfläche der Decke, Einflussflächen und Auflagerkräften; dazu eine IDS-Prüfung und der Rückweg der Ergebnisse | [🔌️exchange](🔌️exchange/🐍️.py) · [Einstieg](../🔌️exchange/🎬️clip/🐍️.py) |
| bim_consistency | Ein ausdrückliches Belagsreserven-Szenario an der Dachdecke: Die GlobalId bleibt, die Revisionen steigen, Last und jede Pilotikraft folgen, das verknüpfte Berechnungsmodell veraltet, eine Prüfung und ein BCF-Thema melden es, und die Auflösung übersetzt neu; zwei Änderungen derselben Revision muss ein Mensch entscheiden | [🔗️consistency](🔗️consistency/🐍️.py) · [Einstieg](../🔗️consistency/🎬️clip/🐍️.py) |

### Aufbau

- Modell: [🧱️model/🐍️.py](🧱️model/🐍️.py) (nur Standardbibliothek), Daten in [🧱️model/🔣️.json](🧱️model/🔣️.json), Schema in [🧱️model/🧬️schema/🔣️.schema.json](🧱️model/🧬️schema/🔣️.schema.json). STEP-, Netz- und IDS-Austausch in [🧱️model/🔌️exchange/🐍️.py](🧱️model/🔌️exchange/🐍️.py), die Übersetzung ins Berechnungsmodell in [🧱️model/🧮️analysis/🐍️.py](🧱️model/🧮️analysis/🐍️.py). Jede Zeichnung und jede Zahl im Bild wird aus dem Modell abgeleitet.
- Fälle: [🧪️cases/🔣️.json](🧪️cases/🔣️.json) enthält die portablen Erwartungen (GlobalId-Vektoren, Information, Klassenketten, Darstellungen, STEP, Netz, IDS, Berechnung, Einflussflächen, Kollision, Ereignisse, ungültige Modelle und die sechs Kapitel).
- Tests: [🧪️tests/🐍️.py](🧪️tests/🐍️.py) leitet die Fälle mit NumPy, SciPy und unabhängigen Prüfverfahren neu her (GlobalId-Kodierer gegen `uuid`, Bisektion, Netzvolumen und Hülle, Einflussflächen mit kd-Baum); [🧪️tests/🟨️.cjs](🧪️tests/🟨️.cjs) prüft das Schema mit AJV und weist ungültige Änderungen ab; [🧪️tests/🎭️presentation/🐍️.py](🧪️tests/🎭️presentation/🐍️.py) deckt die Szenen ab.
- Werkzeugkasten: [🎨️visuals/🐍️.py](🎨️visuals/🐍️.py) (`ClipScene`: Sprache, Zahlen, Live-Werte), [📐️drafting](🎨️visuals/📐️drafting/🐍️.py) (Maße, Schraffuren, Hilfslinien), [🧾️panels](🎨️visuals/🧾️panels/🐍️.py) (Klassenbäume, Eigenschaftstabellen, STEP-Tafeln, Modellkarten), [🏛️savoye](🎨️visuals/🏛️savoye/🐍️.py) (Grundriss, Schnitt, Ansicht, Axonometrie, Detail) und [🧮️analysis](🎨️visuals/🧮️analysis/🐍️.py). Das Styling stammt aus dem gemeinsamen Energie-Werkzeugkasten.
- Jeder Themenordner enthält eine Szenenklasse mit zweisprachigem Titel und Sprechtext. Es kommt keine Laufzeitabhängigkeit hinzu.

### Rendern und Testen

Die eigenständigen Einstiege liegen in `🏗️bim/<Thema>/🎬️clip/🐍️.py`. Die maßgeblichen Szenen bleiben in diesem gemeinsamen Clip-Ordner, damit Geometrie, Modelldaten, Sprechtext und Zeichnungen nur eine Quelle haben. Der Einstieg im Wurzelordner rendert die Playlist mit sechs Videos.

Die sechs oben gezeigten Einzelaufrufe funktionieren auch mit `--language de`. Beispiel für ein deutsches Einzelvideo:

```text
repo 🎓️teaching/🏛️architecture/⚡️energy/🏗️authoring/📜️script.ts render --video bim_information --language de --audience speaker
```

Der Playlist-Aufruf mit `--language de` rendert alle sechs Videos getrennt. `--language` (Pflicht) und `--audience` (`speaker` oder `spectator`; ohne Angabe entstehen beide) gelten für jedes Video und jede Playlist; `--output-root` wählt den Ausgabeordner, `--quality h --fps 30` liefert Full HD. Fortschritt, Abbruch, Zwischenergebnisse und die Darstellung für Sprechende und Zuschauende stellt der gemeinsame Renderer bereit.

Die Szenen enthalten vollständige deutsche und englische Sprechtexte. Tonaufnahmen und fertige Videos sind keine eingecheckten Quelldateien. Der gemeinsame Renderer bindet vorhandene konfigurierte Abschnittsaufnahmen ein; Sprechtext allein erzeugt keine Aufnahme.

Vertrags- und unabhängige Zahlenprüfungen laufen über den obigen `repo … test`-Aufruf.

### Transkripte

Vollständige barrierefreie Transkripte: [English](🗣️transcript.en.md) · [Deutsch](🗣️transcript.de.md).

## Render Interface Contracts · Schnittstellenverträge

English: native presentation/provider tests declare the shared interface contracts. Passing portable geometry, source and callback checks does not verify native Manim frames or caption layout; that render gate remains outstanding.

- Canonical literal `importlib.import_module` paths expose every drawing and model provider to the render dependency graph and its source certificates.
- `villa_axon` selects contextual source elements with `ghost=(tag,)`.
- STEP export receives an explicit scene language (`language="en"` or `"de"`). Its `first_id` supports local-reference renumbering; IDS checks use `source="semantic"` or `source="mesh"`.
- The registered presentation tests bind narration clauses, animation cleanup and source identities. Portable recorders and NumPy/GEOS/SciPy checks separately verify physical callback geometry; final movie publication still requires its native render gate.

Deutsch: Native Präsentations- und Anbietertests erklären die gemeinsamen Schnittstellenverträge. Bestandene portable Geometrie-, Quell- und Callback-Prüfungen bestätigen keine nativen Manim-Bilder oder Untertitellayouts; diese Renderprüfung bleibt offen.

- Kanonische literale `importlib.import_module`-Pfade machen jeden Zeichnungs- und Modellanbieter für den Abhängigkeitsgraphen des Renderns und seine Quellzertifikate sichtbar.
- `villa_axon` wählt Kontextelemente der Quelle mit `ghost=(tag,)` aus.
- Der STEP-Export erhält eine ausdrückliche Szenensprache (`language="en"` oder `"de"`). Sein `first_id` ermöglicht die Umnummerierung lokaler Verweise; IDS-Prüfungen nutzen `source="semantic"` oder `source="mesh"`.
- Die registrierten Präsentationstests binden Sprechklauseln, das Aufräumen von Animationen und Quellidentitäten. Portable Recorder und NumPy-/GEOS-/SciPy-Prüfungen prüfen getrennt die physische Callback-Geometrie; die endgültige Videoveröffentlichung braucht weiterhin ihre native Renderprüfung.

## Primary References · Primärquellen

- [buildingSMART: IFC 4.3 approved as a final standard](https://www.buildingsmart.org/ifc-4-3-approved-as-a-final-standard/): ISO 16739-1:2024 · ISO-Norm 16739-1:2024.
- [buildingSMART: IfcColumn](https://ifc43-docs.standards.buildingsmart.org/IFC/RELEASE/IFC4x3/HTML/lexical/IfcColumn.htm), [IfcBuiltElement](https://ifc43-docs.standards.buildingsmart.org/IFC/RELEASE/IFC4x3/HTML/lexical/IfcBuiltElement.htm) and [Pset_SlabCommon](https://ifc43-docs.standards.buildingsmart.org/IFC/RELEASE/IFC4x3/HTML/lexical/Pset_SlabCommon.htm): classes, supertypes and property sets · Klassen, Obertypen und Property Sets.
- [buildingSMART: IfcGloballyUniqueId](https://standards.buildingsmart.org/IFC/RELEASE/IFC4_3/HTML/lexical/IfcGloballyUniqueId.htm): the 22-character GlobalId · die 22-stellige GlobalId.
- [buildingSMART: IfcStructuralSurfaceMember](https://standards.buildingsmart.org/IFC/RELEASE/IFC4_3/HTML/lexical/IfcStructuralSurfaceMember.htm): structural analysis domain · Tragwerksanalyse.
- [buildingSMART: Information Delivery Specification](https://www.buildingsmart.org/standards/bsi-standards/information-delivery-specifications-ids/) and [BCF](https://github.com/BuildingSMART/BCF): checking and issue exchange · Prüfung und Austausch von Hinweisen.
- [UK BIM Framework: ISO 19650 guidance](https://ukbimframework.org/wp-content/uploads/2020/02/ISO_19650_Guidance_Part_2_Processes_for_Project_Delivery_ThirdEdition.pdf): information management · Informationsmanagement.
- [Association des Sites Le Corbusier: Villa Savoye](https://www.sites-le-corbusier.org/): history and listing of the building · Geschichte und Schutz des Gebäudes.
- [CMN Villa Savoye visitor plan and sections (2021)](https://www.villa-savoye.fr/content/download/9824393/file/document-visite-villa-savoye-FR-2021.pdf?inLanguage=fre-FR&version=3): native plan topology and solarium aperture · native Grundrisstopologie und Solariumsöffnung.
- [Savoye as Built (2024)](https://uvadoc.uva.es/bitstream/handle/10324/68427/9788413202839.pdf?sequence=1): original construction and services evidence, with per-value reconstruction limits · Belege zur ursprünglichen Konstruktion und Haustechnik mit wertbezogenen Rekonstruktionsgrenzen.
- Zaparaín Hernández et al., ["Los planos de estructura de la Villa Savoye"](https://informesdelaconstruccion.revistas.csic.es/index.php/informesdelaconstruccion/article/view/6189), *Informes de la Construcción* 74(566), 2022: structural drawings · Tragwerkspläne.
- Samuel and Blundell, "The making of architectural promenade: Villa Savoye and Schminke House", *arq* 16(2), 2012: the ramp · die Rampe.

English: the research notes record which statements were read on the primary page and which only through excerpts; measured structural registers and native primary plan/section traces inform positions, section envelopes, slab regions and roof datums. Individual jambs, profile cores, construction details, mechanisms and incomplete service routes remain explicitly inferred or unresolved. The assumed car route is separate from the source-informed glazing plan. Deutsch: Die Recherchenotizen halten fest, welche Aussagen auf der Primärseite gelesen und welche nur aus Auszügen entnommen wurden; vermessene Tragwerksregister und native primäre Grundriss-/Schnittspuren begründen Positionen, Querschnittshüllen, Deckenregionen und Dachkoten. Einzelne Laibungen, Profilkerne, Konstruktionsdetails, Mechanismen und unvollständige Leitungswege bleiben ausdrücklich gefolgert oder ungeklärt. Die angenommene Fahrzeugroute bleibt vom quellenbezogenen Verglasungsgrundriss getrennt.
