# BIM Videos

Complete transcripts of the six BIM videos. The Villa Savoye model targets its original completed state in 1930–1931. Source-supported, inferred and assumed dimensions are distinguished; hypothetical teaching changes are identified explicitly. One paragraph per narrated step, in narration order.

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

Does it add up? The bays sum to the grid width; each building datum retains its own source or inference.

The data model adds meaning: property, unit, grid, evidence, and the reconstructed original state of 1930–1931.

A value carries its origin. The first grid was wider and was reduced to save cost.

Information serves a question: can an assumed car reach the entrance while clearing the modelled ground obstacles?

Spacing counts from axis to axis, but the car needs the free space between the piloti surfaces.

The clear width subtracts the actual support outlines from the axis spacing. Circular profiles give the familiar diameter subtraction.

Driving straight through, the car plus a safety margin on each side must fit into the available width.

The first and the reduced grid are compared as explicit spacing scenarios.

How sure is the answer? The gauge shows the threshold of this explicit straight passage scenario.

A thicker circular support or a wider car shifts the threshold; the full route still needs its own physical check.

The assumed arrival is a teaching scenario. Historical descriptions state capacity for three cars; fitting three full-size cars remains unverified.

The swept rectangle is checked against actual height-overlapping building parts, including services and the hoist jib; the smallest clearance governs.

The translucent entrance surround and opaque timber door leaves follow the source-informed plan. The assumed arrival is a separate teaching route.

A piloti carries more properties: its material, whether it bears load, and whether it is external.

Relationships complete the meaning: storey, type, the floor slab it carries and its grid position.

BIM keeps the data in one structured model, so people read the drawing and applications read the data.

Both describe the same piloti: the GlobalId links the drawn column to its STEP lines.

## From Schema to Object

Video: `bim_objects`

A schema defines vocabulary and rules for objects and relationships. For buildings, that schema is IFC 4.3.

A class describes a kind of thing: IfcColumn, IfcSlab, IfcWall and IfcWindow are kinds of built element.

An instance is one particular object: this piloti of the Villa Savoye, with its own GlobalId.

A type holds what its occurrences share. These pilotis share one type; hall and canopy columns use other types.

A property set adds named values to the type: LoadBearing says that each piloti carries its share of the floor.

One object can have several representations: its axis, its footprint and its body.

This selected body is a circular extrusion: radius and height become dimensions. Other support profiles retain their native bores.

A method is behaviour: the application computes the cross-section and the volume from these values.

The same method sums the geometric section envelopes of the selected type; a measured concrete core would need separate evidence.

Change the diameter in the type and every occurrence follows; volume and total update, the GlobalId stays.

Inheritance: each class passes attributes down to IfcColumn, which adds only its PredefinedType.

Here six inherited attributes have values, among them GlobalId, name, placement and tag; three stay empty.

Elements are contained in a storey; the storey belongs to the building, the building to the site.

Composition means parts: the wall has an opening, which IfcRelVoidsElement cuts out of it.

A window fills that opening through IfcRelFillsElement; its frame and glass occupy distinct physical parts.

The wall material set distinguishes solid leaves and an air cavity; their widths add to its envelope thickness.

Keep class and composition apart: a window fills an opening, while the hanging-garden opening remains unfilled.

IFC exchanges declarative data and relationships: class, GlobalId, radius, height and properties travel as text.

The volume method does not travel: the receiving application runs its own method and gets the same value.

## One Object, Many Views

Video: `bim_representations`

Teaching model of the Villa Savoye: one object, the northwest wall of the first floor with its ribbon window.

The plan cuts the wall at window height: the wall in poché, the ribbon window as thin glass lines.

The section cuts through slab, pilotis and window: sill and head heights become dimension lines.

The elevation shows the northwest façade and the asymmetric solarium screen with its landscape aperture and sloping cap.

The 3D view shows occupied wall leaves, separate structural members, and an opening filled by frame and glass.

In this model the wall is stored as Axis, FootPrint and Body; plan, section and elevation are derived from it.

Three levels of detail for one wall: Axis as a line, FootPrint as an outline, Body as a solid with layers.

At 1:200 the outline is enough: on paper the wall is a thin line and its layers vanish.

At 1:20 the plan needs the body: the layers of the wall become visible, millimetre by millimetre.

Move the cut plane: the plan changes from solid wall to window band and back, because it is derived.

The same geometry gives the areas: gross wall, opening, net wall and the glazing share.

The sill drops from 0.89 to 0.70 m; the head stays at 2.04 m. The opening grows from 1.15 to 1.34 m.

The take-off distinguishes rough openings from actual glass; net wall area and glazing share follow the sill.

The wall volume counts solid leaves and excludes the air cavity; the lowered sill removes more solid material.

Copied drawings would stay behind: each copy needs manual coordination, view by view.

A line drafted only in the plan is not in the model: the other views do not know it.

On regeneration the drafted line is lost: a view shows the model and keeps nothing of its own.

A representation shows the object; a design change updates it, and its GlobalId stays the same.

## Models Share a Schema

Video: `bim_models`

Different disciplines organize the information they need about the reconstructed original house of 1930–1931.

The architecture model retains rooms, entrance construction, the roof stair enclosure, its canopy and the open metal ramp guard.

Rooms are objects too: the modelled spaces, each with its area, belong to their storey.

The facade build-up distinguishes render, two brick leaves, an air cavity and plaster.

The structure model holds pilotis, slabs, ramp, stairs and auxiliary lintels, hollow U supports and shelves; several members are shared with architecture.

Original services include drains inside supports 2 and 4, heating and water risers, soil stack and flue. Detailed envelopes and incomplete routes retain their uncertainties.

Three models share one schema. Counts refer to IFC products; a stair assembly can contain many physical parts.

Coordination puts the models on one origin. In this teaching model one GlobalId marks the shared slab.

The shared slab keeps one GlobalId: each model reads from it the properties it needs.

On the ground-floor plan discipline memberships may overlap: the mechanical hoist jib belongs to architecture, structure and services.

In this teaching scenario, a clash check finds what no single model shows: a downpipe too close to a piloti.

For one use case, the structure model is translated into a structural analysis model: IFC's analysis domain.

The first-floor slab becomes clipped surface members; selected slab supports become line members.

The slab is cut at its mid-surface: the piloti line reaches it, the thickness becomes an attribute.

Each selected analytical birth receives an assumed fixed restraint, including transfer-born members.

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

The slab becomes clipped shell members; selected slab supports become line members. Every analytical item records its source GlobalId.

The native slab retains ramp, stair and service apertures. Cellar hatches have separate covers, while the floor area excludes its actual voids.

The analysis needs assumptions: self-weight from the thickness, finishes, imposed load, and the support conditions.

The load is a stack: the slab's own weight, the finishes and the imposed load add up to one area load.

Each selected support receives the slab region up to the midlines to its neighbours, clipped to the slab and its openings.

Each selected support is assigned its tributary area. These are regional slab estimates, not a finite-element stiffness solution.

The tool sends back reactions. They return to the model as a result linked to 19; geometry never flows back.

Ribbon windows, infill walls and render colour do not cross over. Translating back cannot restore them.

IFC has structural analysis entities, but analysis tools often use their own schemas. This is one chosen example.

Before trusting the exchange, validate what the receiver needs, for example with an IDS requirement.

## Identity and Consistency

Video: `bim_consistency`

A GlobalId names this roof slab in every exchange; the tag SL-R and the number #101 only work locally.

Architecture and structure each keep this slab in their own model and revision, under one GlobalId.

The slab record holds class, properties, author and revision; none of these is the identity.

Reports say water came through the flat roof each autumn; our teaching scenario starts from this leak.

The source distinguishes the original reconstructed state from later damage evidence. The illustrated water path is schematic, without an invented membrane layer.

In an explicit repair scenario, a design event increases the reference paving reserve from 0.09 to 0.14 m; the slab keeps its GlobalId.

Thickness and revision change; GlobalId, class and place in the villa stay what they were.

The added paving increases the area load; layer densities and thicknesses determine the displayed value.

Each roof support collects the slab nearest to it, excluding openings. The irregular cells have different areas.

The load on each support follows from its share of each roof zone and the corresponding area load.

The paving change affects each support according to its share of the roof zone.

LK-ROOF links the analysis to the slab, but still holds the earlier revision and its earlier result.

A check compares the linked revision and stored values with the current model and derived results.

A requirement check (IDS) passes because the information exists; only the link check finds the old result.

The finding goes to the structural engineer as a BCF topic that points to the slab by its GlobalId.

Re-translating generates surface members and supports from the current teaching model; a person reviews the assumptions.

With the link updated, the analysis and model show matching reactions and the check finds no issue.

Two people edit revision 1 at once: both save revision 2 under the same GlobalId, with different paving.

The GlobalId cannot merge competing paving edits; a person decides which edit the model keeps.

A schema gives data meaning, a GlobalId names the object, a revision dates it, and a link ties the analysis to it.
