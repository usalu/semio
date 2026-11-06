# Design Notes

## Preserved Source Rationale

<a id="source-notes-75"></a>

### 🦀️.rs:75

🧩️ One typed contribution row, reduced to the three things a capability needs: a stable local
id, a human title, and the artifact kind it acts on. Implemented once per `ContributionSet`
category so {@link capability_from_contribution} stays a single generic function — the four
categories carry genuinely different shapes (an inference names schemas, an io entry names a
dialect pair), and flattening them back into one untyped row would discard exactly the fields
that make a contributed capability findable.
