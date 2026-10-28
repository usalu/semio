# BIM Videos

Complete transcripts of the six BIM videos. All values come from one rounded teaching model of the Villa Savoye in Poissy (Le Corbusier and Pierre Jeanneret, 1928–1931); values that are assumed are marked as such in the videos. One paragraph per narrated step, in narration order.

## Contents

1. [Data Become Information](#data-become-information)
2. [From Schema to Object](#from-schema-to-object)
3. [One Object, Many Views](#one-object-many-views)
4. [Models Share a Schema](#models-share-a-schema)
5. [Interfaces and Translation](#interfaces-and-translation)
6. [Identity and Consistency](#identity-and-consistency)

## Data Become Information

Video: `bim_information`

This number is data. Alone, it does not say what was measured.

Metres, kilograms, degrees: without a unit it could be anything.

Context: it is the distance between two piloti axes on the ground floor of the Villa Savoye.

The model names the place: site, building, storey and the two grid axes.

Does it add up? The bays sum to the grid width; what remains to the building edge is still open.

The data model adds meaning: property, unit, grid, and whether the value is sourced or assumed.

A value carries its origin. The first grid was wider and was reduced to save cost.

Information serves a question: can a car (size assumed) drive between the pilotis and turn under the house?

Spacing counts from axis to axis, but the car needs the free space between the piloti surfaces.

The clear width is the spacing minus the piloti diameter. In this model, that is the room the car has.

Driving straight through, the car plus a safety margin on each side must fit into it. With these inputs, it does.

Whether the grid is the first or the reduced one, the car fits.

How sure is the answer? The verdict flips only below a threshold, far under this spacing.

A thicker piloti or a wider car shifts the threshold, but not enough to change the answer.

The car drives through the bay and turns under the house.

On the curve, two pilotis come closer than the straight-driving margin; the car still clears them, but only just.

The curved glass wall follows the car's turning circle: its curve is calculated from the car.

A piloti carries more properties: its material, whether it bears load, and whether it is external.

Relationships complete the meaning: storey, type, the floor slab it carries and its grid position.

BIM keeps the data in one structured model, so people read the drawing and applications read the data.

Both describe the same piloti: the GlobalId links the drawn column to its STEP lines.

## From Schema to Object

Video: `bim_objects`

A schema defines vocabulary and rules for objects and relationships. For buildings, that schema is IFC 4.3.

A class describes a kind of thing: IfcColumn, IfcSlab, IfcWall and IfcWindow are kinds of built element.

An instance is one particular object: this piloti of the Villa Savoye, with its own GlobalId.

A type holds what all pilotis share; every piloti placed in the building is an occurrence of it.

A property set adds named values to the type: LoadBearing says that each piloti carries its share of the floor.

One object can have several representations: its axis, its footprint and its body.

The body is a circle extruded along its axis: the radius and the height are values, drawn as dimension lines.

A method is behaviour: the application computes the cross-section and the volume from these values.

The same method runs for every occurrence and adds up the concrete of all pilotis.

Change the diameter in the type and every occurrence follows; volume and total update, the GlobalId stays.

Inheritance: each class passes attributes down to IfcColumn, which adds only its PredefinedType.

Here six inherited attributes have values, among them GlobalId, name, placement and tag; three stay empty.

Elements are contained in a storey; the storey belongs to the building, the building to the site.

Composition means parts: the wall has an opening, which IfcRelVoidsElement cuts out of it.

A window fills that opening through IfcRelFillsElement; the glass takes the place of the opening.

The wall also has a material: a layer set of render, brick and plaster adds up to its thickness.

Keep is-a and has-a apart: a piloti is a built element; a window is not a wall, it fills an opening.

IFC exchanges declarative data and relationships: class, GlobalId, radius, height and properties travel as text.

The volume method does not travel: the receiving application runs its own method and gets the same value.

## One Object, Many Views

Video: `bim_representations`

Teaching model of the Villa Savoye: one object, the south wall of the first floor with its ribbon window.

The plan cuts the wall at window height: the wall in poché, the ribbon window as thin glass lines.

The section cuts through slab, pilotis and window: sill and head heights become dimension lines.

The elevation shows the whole south façade: ribbon band, dark green ground floor, pilotis, solarium screens.

The 3D view shows the body: a solid wall, voided by an opening that the window fills.

In this model the wall is stored as Axis, FootPrint and Body; plan, section and elevation are derived from it.

Three levels of detail for one wall: Axis as a line, FootPrint as an outline, Body as a solid with layers.

At 1:200 the outline is enough: on paper the wall is a thin line and its layers vanish.

At 1:20 the plan needs the body: the layers of the wall become visible, millimetre by millimetre.

Move the cut plane: the plan changes from solid wall to window band and back, because it is derived.

The same geometry gives the areas: gross wall, opening, net wall and the glazing share.

A design change lowers the sill. The ribbon window grows; all views and numbers are regenerated.

The take-off reads the body: net wall area, window area and glazing share follow the sill live.

The wall volume comes from the same body: the void is subtracted, so the lowered sill shrinks it.

Copied drawings would stay behind: each copy needs manual coordination, view by view.

A line drafted only in the plan is not in the model: the other views do not know it.

On regeneration the drafted line is lost: a view shows the model and keeps nothing of its own.

A representation shows the object; a design change updates it, and its GlobalId stays the same.

## Models Share a Schema

Video: `bim_models`

Different disciplines organize the building information they need.

The architecture model holds walls, openings, windows, ramp, curved glass wall, slabs and rooms.

Rooms are objects too: six spaces, each with its area, belong to their storey.

The wall build-up is part of the model: render, brick infill and plaster in three layers.

The structure model holds the pilotis, slabs and ramp. It leaves out render, windows and rooms.

The services model holds boiler, radiator, roof drains and downpipes: the same house, other objects.

Three models, one schema: the same vocabulary and rules, with different content.

Coordination puts the models on one origin. In this teaching model one GlobalId marks the shared slab.

The shared slab keeps one GlobalId: each model reads from it the properties it needs.

On the ground-floor plan every line belongs to one model: walls and glass, pilotis, pipes.

In this teaching scenario, a clash check finds what no single model shows: a downpipe too close to a piloti.

For one use case, the structure model is translated into a structural analysis model: IFC's analysis domain.

The first-floor slab becomes surface members on the grid, each piloti a line member.

The slab is cut at its mid-surface: the piloti line reaches it, the thickness becomes an attribute.

Assumptions add what was never drawn: every piloti foot becomes a fixed support node.

The load is assumed too: self-weight, finishes and imposed load add up to the area load.

Ribbon windows, infill walls, render, the curved glass wall and more are not in the analysis.

Even kept pilotis lose properties: fire rating and external flag have no place in the analysis.

Each member keeps its source GlobalId, so results trace back; the translation is not reversible.

## Interfaces and Translation

Video: `bim_exchange`

An interface defines the exchange. IFC is a standard schema, ISO 16739-1, that both sides know.

Semantic exchange sends an IfcColumn at its place, with a stable GlobalId. Instance numbers (#n) are only local.

Its properties travel along: type COLUMN, LoadBearing flag, material, circular profile and extrusion depth.

A triangle mesh keeps only vertices and faces. The shape survives roughly; class, identity, properties do not.

Round trip: IFC to a mesh and back. The shape returns, but class, GlobalId and LoadBearing stay lost.

To check the first-floor slab structurally, we choose an analysis schema and idealise the slab at its mid-surface.

The slab becomes shell elements on the grid, each piloti a member. Every analysis item records its source GlobalId.

The slab is the villa's own: the ramp opening is cut out, the salon rests on it, and the net area follows.

The analysis needs assumptions: self-weight from the thickness, finishes, imposed load, and the support conditions.

The load is a stack: the slab's own weight, the finishes and the imposed load add up to one area load.

Each piloti owns the slab up to the midlines to its neighbours. Beside the ramp opening, B3 owns a notched cell.

Each piloti is assigned its tributary area. This is a simplified estimate, not a finite element result.

The tool sends back reactions. They return to the model as a result linked to B3; geometry never flows back.

Ribbon windows, infill walls and render colour do not cross over. Translating back cannot restore them.

IFC has structural analysis entities, but analysis tools often use their own schemas. This is one chosen example.

Before trusting the exchange, validate what the receiver needs, for example with an IDS requirement.

## Identity and Consistency

Video: `bim_consistency`

A GlobalId names this roof slab in every exchange; the tag SL-R and the number #101 only work locally.

Architecture and structure each keep this slab in their own model and revision, under one GlobalId.

The slab record holds class, properties, author and revision; none of these is the identity.

Reports say water came through the flat roof each autumn; our teaching scenario starts from this leak.

Water passes soil and screed; wherever the waterproofing has a gap, it enters the slab and stains the ceiling.

After the repair, a design event thickens the soil from 0.10 to 0.30 m; the slab keeps its GlobalId.

Thickness and revision change; GlobalId, class and place in the villa stay what they were.

Soil weighs 18 kN/m³: 0.20 m more soil adds 3.6 kN/m², and the roof planting load becomes 12.92 kN/m².

Each piloti carries the roof around it: A3 collects 4.75 m × 2.375 m = 11.28 m², C3 only 2.38 m².

The load on A3 follows: 11.28 m² × 12.92 kN/m² ≈ 146 kN, not 105 kN.

Each piloti changes by 3.6 kN/m² times the area of its roof planting: D3 gains most, C4 under the terrace nothing.

LK-ROOF links the analysis to the slab, but it still holds revision 1: 105 kN, not 146 kN.

A check compares link and model: revision 1 against 2, 9.32 against 12.92 kN/m², 105 against 146 kN.

A requirement check (IDS) passes because the information exists; only the link check finds the old result.

The finding goes to the structural engineer as a BCF topic that points to the slab by its GlobalId.

Re-translating the slab gives 24 surface members and 25 columns of this teaching model; a person reviews the assumptions.

With the link set to revision 2, the analysis shows 146 kN too, and the check finds no issue.

Two people edit revision 1 at once: both save revision 2 under the same GlobalId, with different soil.

The GlobalId cannot merge 12.92 and 11.12 kN/m²; a person decides which edit the model keeps.

A schema gives data meaning, a GlobalId names the object, a revision dates it, and a link ties the analysis to it.
