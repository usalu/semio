# Polygon Fragment Ordering / Polygonfragment-Sortierung

## English

This shared geometry module orders actual planar polygon material from far to near for an orthographic view. It uses only Python's standard library. Rendering, colour, opacity and scene-object ownership belong to the caller.

Face contains source and polygon integer identities, a tuple of 3D boundary vertices and its geometric normal. Keep disconnected boundaries as separate faces. Supply finite, simple, planar boundaries; derive normals from local vertex differences rather than a distant global-coordinate accumulation. The schema and portable cases define the interface.

ordered_fragments(faces, towards, progress=None, cancelled=None) returns immutable Face fragments with the original identities and material coverage. Concave boundaries are decomposed without filling voids. Actual source planes partition intersecting regions; traversal uses an iterative stack. Coplanar order is deterministic. No artificial cut edge is an architectural boundary: renderers must preserve only original boundary strokes.

Progress reports phase-local dictionaries with phase, completed and total; splitting may change a phase's total. A true cancellation callback raises FragmentCancelled without publishing a partial result. source_files() identifies implementation and schema inputs for reproducible validation. NumPy, Pillow and schema validators are test oracles, with no runtime dependency added.

## Deutsch

Dieses gemeinsame Geometriemodul sortiert tatsächliches ebenes Polygonmaterial für orthogonale Ansichten von hinten nach vorn. Es verwendet ausschließlich die Python-Standardbibliothek. Darstellung, Farbe, Transparenz und die Zuordnung zu Szenenobjekten bleiben beim Aufrufer.

Face enthält ganzzahlige Quell- und Polygonkennungen, ein Tupel räumlicher Randpunkte und die geometrische Normale. Getrennte Ränder bleiben getrennte Flächen. Die Grenzen müssen endlich, einfach und eben sein. Normalen werden aus lokalen Punktdifferenzen bestimmt. Schema und sprachunabhängige Fälle beschreiben die Schnittstelle.

ordered_fragments erhält Kennungen und Materialabdeckung, zerlegt konkave Ränder ohne Hohlräume zu füllen und teilt sich kreuzende Bereiche an tatsächlichen Quellebenen. Ein iterativer Stapel vermeidet rekursive Aufruftiefen. Neue Schnittkanten erhalten keine erfundenen Konturlinien.

Fortschrittsmeldungen enthalten phase, completed und total je Arbeitsphase; Teilungen können die Gesamtzahl ändern. Ein bestätigter Abbruch löst FragmentCancelled aus und liefert kein Teilergebnis. source_files() nennt Implementierung und Schema für reproduzierbare Prüfungen. Externe Bibliotheken dienen ausschließlich als unabhängige Testorakel.
