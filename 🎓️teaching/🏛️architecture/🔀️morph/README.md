# 🔀 Morph Toolkit

**English.** Declarative, identity-preserving morphs for Manim clips. Clips never build morph geometry: each clip folder owns one `🔀️transitions/🔣️.json` that declares every transition (stable `id`, semantic `intent`, `source`/`target` endpoints, `relation`, `correspondence`, `recipe`, `timing`, `placement`, `final` options). The JSON Schema in `🧬️schema/🔣️.schema.json` is the single source of truth; `📋️spec` loads and validates documents (JSON path, stable problem code, how to fix) and `🎭️stage` executes them.

```python
morph = importlib.import_module("🎓️teaching.🏛️architecture.🔀️morph.🐍️")
stage = morph.stage(self, "<clip folder>/🔀️transitions/🔣️.json")
stage.bind("tower.capsules", {"a": capsule_a, "b": capsule_b})
stage.bind("diagram.leaves", lambda lod: build_leaves(lod), bounds=((0, 0, 0), (4, 2, 0)))
self.play(stage.transition("trees.birth.capsules-to-leaves"))
```

- `bind` takes a Mobject, a mapping, a list or a factory `lod -> Mobject`; factories need explicit logical bounds, run lazily and only for the lod the active profile asks for. Logical bounds and anchors freeze at bind and are never re-derived from reduced-detail geometry.
- `stage.transition(id)` reads the active render profile and returns one animation: the preview swap (source only, translate/scale, exact target at the terminal frame) or the full morph. Durations, ids and endpoints are identical for every profile. `stage.cut(id)` is the atomic switch for `duration: 0`; `stage.played` lists the requested ids.
- Recipes: `parts`, `hull`, `scatter`, `gather`, `split`, `unfold`, `retext`, `flight`, `move`, `replace` (Manim ReplacementTransform), `vanish` (`target: null`), `appear` (`source: null`); `copy: true` keeps the source (TransformFromCopy). Relations one-to-many and many-to-one require explicit `pairs` or a named `recipe` correspondence.
- `validate(document, stage)` additionally checks bound entities, revisions, supported view/scale/pose/scenario, keys, orders and pair parts. `load`, `parse`, `dump` persist and reload a document without loss.
- The staffage hook is `Stage.register_binder(kind, fn)` plus `stage.bind_with(kind, ...)`.
- Final execution lives in `🎞️full` (`execute`, recipes incl. `fade`, named `recipe` correspondences `auto`, `fan-in`, `fan-out`, `capsule-item`, `item-capsule`, `capsule-items`); `🫥️silhouette` holds the shared outline geometry and `🪙️token` the shared `token`, `travel`, `tether`. `copy: true` also works for `parts`, `hull` and `fade`.
- Legacy helpers (`silhouette`, `proxy`, `morph`, `scatter`, `gather`, `split`, `token`, `travel`, `tether`, `unfold`, `retext`) remain importable from `🐍️.py` while clips migrate. Import literally with `importlib.import_module("🎓️teaching.🏛️architecture.🔀️morph.🐍️")`.

Tests: `python -m unittest 🎓️teaching.🏛️architecture.🔀️morph.🧪️tests.📋️spec.🐍️` (JSON cases in `🧪️tests/📋️spec/🧫️cases`, cross-checked against the `jsonschema` validator) and `...🧪️tests.🐍️` for the toolkit.

**Deutsch.** Deklarative, identitätserhaltende Morphs für Manim-Clips. Clips bauen keine Morph-Geometrie: Jeder Clip-Ordner besitzt eine `🔀️transitions/🔣️.json`, die jeden Übergang festlegt (stabile `id`, semantische `intent`, Endpunkte `source`/`target`, `relation`, `correspondence`, `recipe`, `timing`, `placement`, `final`-Optionen). Das JSON-Schema in `🧬️schema/🔣️.schema.json` ist die einzige Quelle der Wahrheit; `📋️spec` lädt und prüft Dokumente (JSON-Pfad, stabiler Fehlercode, Hinweis zur Behebung) und `🎭️stage` führt sie aus.

- `bind` nimmt ein Mobject, eine Abbildung, eine Liste oder eine Fabrik `lod -> Mobject`; Fabriken brauchen explizite logische Grenzen, laufen träge und nur für die Detailstufe des aktiven Profils. Logische Grenzen und Anker werden beim Binden eingefroren.
- `stage.transition(id)` liest das aktive Render-Profil und liefert genau eine Animation: den Vorschau-Tausch (nur die Quelle, verschieben/skalieren, exaktes Ziel im letzten Bild) oder den vollen Morph. Dauern, IDs und Endpunkte sind in allen Profilen gleich. `stage.cut(id)` ist der atomare Wechsel für `duration: 0`; `stage.played` listet die angeforderten IDs.
- Rezepte: `parts`, `hull`, `scatter`, `gather`, `split`, `unfold`, `retext`, `flight`, `move`, `replace` (Manim ReplacementTransform), `vanish` (`target: null`), `appear` (`source: null`); `copy: true` behält die Quelle (TransformFromCopy). Beziehungen eins-zu-viele und viele-zu-eins verlangen explizite `pairs` oder eine benannte `recipe`-Zuordnung.
- `validate(document, stage)` prüft zusätzlich gebundene Entitäten, Revisionen, unterstützte Ansicht/Maßstab/Pose/Szenario, Schlüssel, Reihenfolgen und Paar-Teile. `load`, `parse`, `dump` speichern und laden ein Dokument verlustfrei.
- Der Staffage-Haken ist `Stage.register_binder(kind, fn)` mit `stage.bind_with(kind, ...)`.
- Die finale Ausführung liegt in `🎞️full` (`execute`, Rezepte inkl. `fade`, benannte `recipe`-Zuordnungen `auto`, `fan-in`, `fan-out`, `capsule-item`, `item-capsule`, `capsule-items`); `🫥️silhouette` enthält die gemeinsame Umriss-Geometrie und `🪙️token` die gemeinsamen `token`, `travel`, `tether`. `copy: true` gilt auch für `parts`, `hull` und `fade`.
- Die bisherigen Hilfsfunktionen (`silhouette`, `proxy`, `morph`, `scatter`, `gather`, `split`, `token`, `travel`, `tether`, `unfold`, `retext`) bleiben in `🐍️.py` importierbar, bis die Clips migriert sind.
