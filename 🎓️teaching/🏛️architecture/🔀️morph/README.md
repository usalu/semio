# 🔀 Morph Toolkit

**English.** Identity-preserving Manim morphs between concrete drawings (capsules, faces, storeys) and abstract stand-ins (tree leaves, cells, path chips, number badges, plane squares). A multi-part source collapses to one silhouette that flows into the target while its details fade out and the target's details fade in; colours carry over; the scene holds exactly the target afterwards.

Helpers in `🐍️.py`: `silhouette`, `proxy`, `morph`, `scatter`, `gather`, `split`, `token`, `travel`, `tether`, `unfold`, `retext`. Import literally with `importlib.import_module("🎓️teaching.🏛️architecture.🔀️morph.🐍️")`. Sources must be in the scene before, targets must not; play `scatter`, `gather`, `split` and `unfold` directly with `self.play`. Use `proxy` for towers with thousands of faces.

Tests: `python -m unittest 🎓️teaching.🏛️architecture.🔀️morph.🧪️tests.🐍️` (the hull is checked against SciPy).

**Deutsch.** Identitätserhaltende Manim-Morphs zwischen konkreten Zeichnungen (Kapseln, Flächen, Geschosse) und abstrakten Stellvertretern (Baumblätter, Zellen, Pfad-Chips, Zahlen-Plaketten, Ebenen-Quadrate). Eine mehrteilige Quelle wird zu einer Silhouette, die in das Ziel übergeht, während ihre Details ausblenden und die des Ziels einblenden; Farben bleiben erhalten; danach enthält die Szene genau das Ziel.

Hilfsfunktionen in `🐍️.py`: `silhouette`, `proxy`, `morph`, `scatter`, `gather`, `split`, `token`, `travel`, `tether`, `unfold`, `retext`. Quellen müssen vorher in der Szene sein, Ziele nicht; `scatter`, `gather`, `split` und `unfold` direkt mit `self.play` abspielen. Für Türme mit tausenden Flächen `proxy` verwenden.
