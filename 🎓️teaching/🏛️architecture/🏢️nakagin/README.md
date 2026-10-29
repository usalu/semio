# Nakagin Teaching Model · Nakagin-Lehrmodell

## English

One shared, schema-first **teaching model of the Nakagin Capsule Tower** (Kisho Kurokawa, Ginza, Tokyo, 1970–1972, demolished 2022) for the Grasshopper and Excel video series. It is a simplified, rounded model, not a survey. Every value is a leaf `{value, unit, basis, source?}` with the basis `sourced`, `assumed` or `derived`; sources are listed once.

**What it holds.**

- **One parametric capsule**, outer 2.5 × 4.0 × 2.5 m (width × length × height, length perpendicular to the core facade), rounded profile (corner radius 0.25 m is assumed), round window ⌀ 1.3 m on the front face, four bolt points on the back face. The historical interior size 2.3 × 3.8 × 2.1 m is kept as sourced interior size (program and energy volume).
- **Four capsule types** as parameter variants of that one capsule: A Residence, B Residence with desk, C Studio (window ⌀ 1.6 m), D Splayed residence (side window, 5° yaw on its plane). Counts per type are derived from a modular pattern in the data (A 36, B 35, C 34, D 35).
- **Two towers** on square cores (5.5 m, storey pitch 2.75 m, two podium storeys below the first capsule storey): Tower A with 9 capsule storeys × 3 facades (its west facade faces Tower B and the bridges and has none), Tower B with 11 capsule storeys × 4 facades, except the top capsule storey with 3 facades. Each facade carries exactly 2 capsules side by side (item 1 is one third of a storey higher). That is 70 branches `{BUILDING;TOWER;STORY;FACADE}` × 2 = **140 capsules**, the historical count.
- **Planes**: every slot has a plane on the core facade (X along the facade, Y outward, Z up) with the type rotation, so a capsule is placed by Orient from the world plane to the slot plane.
- **Capital** (machine room box with overhang and taper) per tower and **podium** (extruded box, 2 storeys).
- **Program and energy**: net floor area from the interior size, fittings per type, five exposed faces plus the back face attached to the core, U-values (window 2.33 W/(m²·K) sourced, others assumed), air change 0.5 h⁻¹, θᵢ 20 °C, Tokyo monthly means (JMA 1991–2020 normals) giving 1,585.7 K·d heating degree days at the 18 °C base, H_T, H_V, heating load and annual demand in kWh/a and kWh/(m²·a).
- **Design variants** (`variants`, `variant(name)`, `variants()`, `heating(t, variant=None)`): `window_upgrade` (window U 1.1 W/(m²·K)), `back_wall_exposed` (temperature factor b = 1.0, the mistake of treating the core-side wall as outside), `shared_roof_floor` (roof and floor U = 0, heated neighbours); a variant name or a tuple of names overrides the energy inputs, the data stay untouched. The climate carries its source, period and method (`climate()["source"|"period"|"method"|"hdd_value"]`).

| File | Role |
| --- | --- |
| [🔣️.json](🔣️.json) | all values with unit, basis and source |
| [🧬️schema/🔣️.schema.json](🧬️schema/🔣️.schema.json) | JSON Schema (draft 2020-12) |
| [🐍️.py](🐍️.py) | stdlib-only derivations: `load`, `value`, `capsule_types`, `capsule_solid`, `towers`, `core_solid`, `capital_solid`, `podium_solid`, `slots`, `tree`, `place`, `program`, `envelope`, `climate`, `heating`, `totals` |
| [🧪️cases/🔣️.json](🧪️cases/🔣️.json) | portable expectations (counts, plane samples, program, climate, heating, invalid models) |
| [🧪️tests/🐍️.py](🧪️tests/🐍️.py) | oracles: NumPy (areas, volumes, orthonormal planes, degree days, envelope sums), `rhino3dm` (`Plane`, `Transform.PlaneToPlane`, `Box`, `Circle`), an independent count walk over the raw JSON |
| [🧪️tests/🟨️.cjs](🧪️tests/🟨️.cjs) | AJV schema validation, 24 invalid mutations, independent count and type-pattern oracle |
| [🎨️visuals/🐍️.py](🎨️visuals/🐍️.py) | Manim drawings (capsule, core, capital, podium, tower) |

### Test

From the repository root:

```text
repo node 🎓️teaching/🏛️architecture/🏢️nakagin/🧪️tests/🟨️.cjs
.🧬semio/🎓️teaching/⚡️cache/python/Scripts/python.exe 🎓️teaching/🏛️architecture/🏢️nakagin/🧪️tests/🐍️.py
```

Sources: Wikipedia (capsule size, 140 capsules, four bolts), Archeyes (window ⌀ 1.3 m), Archiweb (interior size), Tokyo Metropolitan Government 2019 (window U-value 2.33), Japan Meteorological Agency normals for Tokyo, and the repo IFC export (core 5.5 m, pitch 2.75 m, podium footprint). Everything else is marked assumed.

## Deutsch

Ein gemeinsames, schemabasiertes **Lehrmodell des Nakagin Capsule Tower** (Kisho Kurokawa, Ginza, Tokio, 1970–1972, 2022 abgerissen) für die Grasshopper- und Excel-Videoreihen. Es ist ein vereinfachtes, gerundetes Modell, keine Bauaufnahme. Jeder Wert ist ein Blatt `{value, unit, basis, source?}` mit der Grundlage `sourced` (belegt), `assumed` (angenommen) oder `derived` (abgeleitet); Quellen stehen einmal gesammelt.

**Inhalt.**

- **Eine parametrische Kapsel**, aussen 2,5 × 4,0 × 2,5 m (Breite × Länge × Höhe, Länge senkrecht zur Kernfassade), gerundetes Profil (Eckradius 0,25 m angenommen), rundes Fenster ⌀ 1,3 m an der Frontseite, vier Schraubpunkte an der Rückseite. Die historische Innenmasse 2,3 × 3,8 × 2,1 m bleibt als belegte Innenmasse für Raumprogramm und Energievolumen erhalten.
- **Vier Kapseltypen** als Parametervarianten dieser einen Kapsel: A Wohnkapsel, B Wohnkapsel mit Schreibtisch, C Atelier (Fenster ⌀ 1,6 m), D Gefächerte Wohnkapsel (Seitenfenster, 5° Drehung auf ihrer Ebene). Die Anzahl je Typ wird aus einem Modulo-Muster in den Daten abgeleitet (A 36, B 35, C 34, D 35).
- **Zwei Türme** auf quadratischen Kernen (5,5 m, Geschosshöhe 2,75 m, zwei Sockelgeschosse unter dem ersten Kapselgeschoss): Turm A mit 9 Kapselgeschossen × 3 Fassaden (seine Westfassade liegt zu Turm B und den Brücken hin und hat keine), Turm B mit 11 Kapselgeschossen × 4 Fassaden, ausser dem obersten Kapselgeschoss mit 3 Fassaden. Jede Fassade trägt genau 2 Kapseln nebeneinander (Element 1 liegt ein Drittel Geschosshöhe höher). Das sind 70 Äste `{BUILDING;TOWER;STORY;FACADE}` × 2 = **140 Kapseln**, die historische Anzahl.
- **Ebenen**: Jeder Platz hat eine Ebene auf der Kernfassade (X entlang der Fassade, Y nach aussen, Z nach oben) mit der Typdrehung; die Kapsel wird mit Orient von der Weltebene auf die Platzebene gesetzt.
- **Kopfbau** (Maschinenraum-Quader mit Überstand und Verjüngung) je Turm und **Sockel** (extrudierter Quader, 2 Geschosse).
- **Programm und Energie**: Nettogrundfläche aus der Innenmasse, Einbauten je Typ, fünf Aussenflächen plus die am Kern anliegende Rückwand, U-Werte (Fenster 2,33 W/(m²·K) belegt, übrige angenommen), Luftwechsel 0,5 h⁻¹, θᵢ 20 °C, Tokioter Monatsmittel (JMA-Normalwerte 1991–2020) mit 1.585,7 K·d Heizgradtagen bei 18 °C Heizgrenze, H_T, H_V, Heizlast und Jahresbedarf in kWh/a und kWh/(m²·a).
- **Entwurfsvarianten** (`variants`, `variant(name)`, `variants()`, `heating(t, variant=None)`): `window_upgrade` (Fenster-U 1,1 W/(m²·K)), `back_wall_exposed` (Temperaturfaktor b = 1,0, der Fehler, die kernseitige Wand als aussen zu behandeln), `shared_roof_floor` (Dach und Boden U = 0, beheizte Nachbarn); ein Variantenname oder ein Tupel von Namen überschreibt die Energieeingaben, die Daten bleiben unverändert. Das Klima trägt Quelle, Zeitraum und Methode.

Dateien, Tests und Quellen siehe die englische Tabelle oben; die Tests laufen wie dort angegeben vom Repository-Wurzelverzeichnis. Quellen: Wikipedia (Kapselmass, 140 Kapseln, vier Schrauben), Archeyes (Fenster ⌀ 1,3 m), Archiweb (Innenmasse), Tokioter Stadtverwaltung 2019 (Fenster-U-Wert 2,33), Normalwerte der Japanischen Meteorologischen Agentur für Tokio und der IFC-Export des Repos (Kern 5,5 m, Raster 2,75 m, Sockelgrundriss). Alles Übrige ist als angenommen gekennzeichnet.
