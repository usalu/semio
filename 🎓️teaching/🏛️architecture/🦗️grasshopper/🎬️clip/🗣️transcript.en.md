# Grasshopper Videos

Complete transcripts of the five Grasshopper videos. All values come from one rounded teaching model of the Nakagin Capsule Tower in Ginza, Tokyo (Kisho Kurokawa, 1970–1972, demolished 2022); values that are assumed are marked as such in the videos. One paragraph per narrated step, in narration order.

## Contents

1. [Anatomy of a Grasshopper Script](#anatomy-of-a-grasshopper-script)
2. [One Capsule, Parametric](#one-capsule-parametric)
3. [From One Capsule to the Tower](#from-one-capsule-to-the-tower)
4. [Casting: When Types Meet](#casting-when-types-meet)
5. [Data Trees on the Tower](#data-trees-on-the-tower)

## Anatomy of a Grasshopper Script

Video: `grasshopper_anatomy`

This is one capsule of the Nakagin Capsule Tower, a simplified teaching model. How do we hand it to a computer?

We describe it as a script: a Grasshopper definition, saved as a .gh file. Its drawing area is the canvas.

A Number Slider is the first input: a name, a domain from 1.0 to 5.0, one decimal and a value, 2.5.

A component turns inputs into an output. Center Box, nickname Box, takes width, length and height and builds a box; the viewport shows it.

Every component has a name and a nickname, and the canvas shows the nickname. Inputs sit on the left, outputs on the right, each with a grip.

Wires carry data from an output grip to an input grip and normally flow from left to right.

Each slider is one dimension: wire and dimension line share its colour. Length 5.0 and height 3.0 metres make the box grow.

Drag a slider and the result follows. The width goes from 2.5 to 3.0 metres and back, and the box changes with it.

The preview eye hides only the drawing, not the data: the box disappears, the volume still reads 25 cubic metres.

Grey is fine. A number and a text in Z turn it orange, a warning: one box, one null. Replace the text by 2.5 and it is grey again.

With nothing usable it turns red, an error: a wire cannot make the text tall a height. Give Z the number 2.5 and the box returns.

A Panel shows what a wire carries: path {0}, index 0, value 2.5. It makes a hidden number readable.

A parameter holds typed data. Floating it stands alone; as input X of a component it belongs to a port. Both hold the Number 2.5.

A Boolean Toggle has two states. True gives the capsule its round window, 1.3 metres across, in the front face.

A Value List offers fixed choices: the proposed room-program studies A to D. They share the nominal 1.30 metre porthole.

A plugin, a .gha file, adds a tab to the ribbon. Ladybug Tools studies the sun on the capsule window: more sun, warmer tint.

Now the data. One value is an item: the elevation offset zero metres for the first capsule level of Tower B.

A Series with step 2.8 metres gives 9 offsets in one branch, indices 0 to 8: one capsule per level of Tower B.

Change Count from 9 to 11 and the list grows. Tower A has 11 capsule levels.

Many lists form a data tree. Each occupied facade group is a branch with its displayed address and a variable number of capsule items.

A common mistake is to read a tree as one long list. Index one exists in many branches; a singleton branch has no item at that index.

Only path plus index identifies exactly one capsule. The Panel shows the actual selected address and index one.

The canvas terms, row by row in English and German: script, Skript; component, Komponente; and six more.

The data terms in English and German: item, Element; list, Liste; tree, Datenbaum; branch, Ast; path, Pfad; index.

## One Capsule, Parametric

Video: `grasshopper_capsule`

The Nakagin tower hangs its capsules on two cores, yet a single Grasshopper definition builds every one of them.

Parametric means: parameters feed dependencies, dependencies build geometry. Change one value and everything follows.

Six Number Sliders control the physical reference casing: 2.6 metres wide and 4.2 metres long, its height, corner radius, window diameter and offset.

Everything starts on a plane: origin plus X, Y and Z axes in Rhino's red, green and blue. This is the capsule's base plane.

Center Box takes the plane and the sliders for width, length and height. It centres itself on the plane, so half of it sinks below the floor.

Plane Origin uses the physical rear datum minus 0.1 metres, half the casing length and half its height. The box now stands on the base plane.

Drag the length slider: the dimension line, the box and every component behind it follow in the same moment.

Fillet Edge rounds the edges with the corner radius slider: the soft shell of the capsule appears.

A typical mistake: a Circle on the base plane lies on the floor, not on the front face. A wire does not know where you mean.

The fix: the front datum is the rear datum plus casing length. The glazing plane sits at its source-derived recess, while the cutting cylinder starts on the steel face.

Solid Difference subtracts a cylinder of that circle from the shell: the round window is cut into the front face.

Four points on the back face mark the high-tension bolts that fix the capsule to the core.

Each slider domain is a design constraint; here the limits are assumptions, for example a transport envelope.

Push the width past its domain: the slider clamps at its upper limit and the envelope turns red. The domain is a design rule.

A to D are proposed room-program studies, not historical capsule families. They share the nominal capsule envelope.

All four studies keep the nominal centred 1.30 metre porthole. Slider edits explore dimensions; they are not surveyed types.

On the core the capsule needs a target plane on the wall; the base plane must land on it.

Orient moves the capsule in one step from its base plane A to the target plane B.

Rotate the target plane about the core axis and the capsule turns with it, because it follows plane B.

Each slot retains its historical physical family. List Item chooses among four proposed program variants: index 0 gives A, 1 gives B, 2 gives C, 3 gives D.

Eight planes need indices up to 7, but there are only four types: without Wrap, List Item warns from index 4.

Switch Wrap on and the index cycles: 4 gives type A again, 5 gives B, and every plane gets a capsule.

Recap: parameter, dependency, geometry, domain, plane, Orient, List Item and index.

## From One Capsule to the Tower

Video: `grasshopper_tower`

The Nakagin Capsule Tower has 140 capsules. How do they become two towers?

The tower splits into five parts: core, planes, capsules, capital and podium. Each part gets its own coloured group on the canvas.

Start with the square core envelope. Its height combines selected capsule crowns with the roof clearance of this tower.

The envelope sets out the shaft. Its detailed assembly contains slabs, stairs, landings and lift doors on every storey.

A Series starts at the provisional 7.72 metre podium roof, with the sourced 2.80 metre pitch. Authored phase offsets place the capsules.

Authored origins and handed axes define the door-port frames. Construct Plane assembles each one without an inferred rotation.

Every plane carries its axes: X runs along the facade, Y points outward, Z points up.

Each authored facade group has indexed candidate ports. The authored floor masks select the occupied positions.

Orient maps the local door port to the reconstructed world door port. Its result is the capsule body plane, including the handed layout.

A type pattern picks the capsule type for every plane. List Item reads the pattern by index; only the list differs.

The slider reveals one reconstructed capsule level after another. The total is the sum of the occupied positions on the selected levels.

The wires stay unchanged. Floor masks, capsule crowns and the tower-specific roof clearance determine the resulting height.

Without the floor masks, the regular candidate grid includes unoccupied positions. Its repeated pattern is not the documented building.

Cull Index removes the unoccupied positions on every level. The documented result is 76 capsules in A and 64 in B.

The machinery head is a separate part. A has a sourced height of 11.36 metres; B uses a provisional derived 10.83 metres.

The head starts at the core roof: 42.14 m in A, a provisional 36.62 m in B; each tower keeps its own clearance.

The footprint and height define the podium envelope. The detailed model adds its slabs, columns, glazing, entrance and parapets.

The envelope is the first construction step. Named assemblies carry the circulation, services and façade details throughout the building.

The same definition reads the authored floor masks for both towers: 76 capsules in A, 64 in B.

Together they give the historical count, 140 capsules, read from the definition and not typed in.

Metabolism proposes replaceable capsules. Here the switch changes the room program from type B to type C; the porthole stays.

Recap: core, planes, capsules, capital, podium. This is the teaching model of the Nakagin Capsule Tower.

## Casting: When Types Meet

Video: `grasshopper_casting`

This capsule should face west, but its window looks north. One silent cast put it there. Which one?

In this teaching model of the Nakagin Capsule Tower, every parameter has a type: width is a Number, storeys an Integer, placement a Plane.

A wire connects whatever you drag. When the types differ, Grasshopper tries to cast the data.

A Panel holds the text 2.5. The Number input reads it as the number 2.5, and the nominal study width becomes 2.5 metres. Its physical reference casing is 2.6 metres wide.

A slider says 8.6, but Storeys wants an Integer. Grasshopper rounds, here to 9; check this in Rhino 8.

Grasshopper rounds; at exactly .5 it rounds to the even number: 2.5 gives 2, 3.5 gives 4. Check this in Rhino 8.

To decide the rounding yourself, put a component such as Floor before the Integer input: the top storey disappears.

A Point at a Plane input becomes a plane at that point with the world axes. Here the point is the origin of a facade slot.

On the west facade the capsule must face west. The world plane points it north: the window turns by 90°.

A Line becomes a plane whose Z axis follows the line. From the core to the slot Z points outwards, but X and Y are still guessed.

Build the plane explicitly from origin and axes. The capsule turns into place and faces outwards.

A Number can stand in for a Domain: the nominal study length 4.0 becomes the interval 0 To 4, and Divide Domain cuts it.

A Circle is a Curve and a Box is a Brep: window outline and capsule body pass as the more general type.

A Boolean Toggle becomes an Integer: False is 0, True is 1, and the window follows.

The other way round, a number can switch the window: 0 hides it, any other value shows it.

Mind the decimal comma: the text 2,5 may fail as a Number in a German setup. Write 2.5 or use a slider.

Now a Panel holds the text Type A. The Number input cannot read it: the component turns red, Data conversion failed from Text to Number.

Why? Grasshopper tries in a fixed order: same type, then the source, then the target. Type A fails every step, so the slot stays blank.

A label is no number. A Value List delivers the real width of Type A, and the component turns grey again.

Casts follow families: along a family they work, across families you need a component.

Each marked cell is a cast you have seen: green worked, red failed, dashed outlines still need a check in Rhino 8.

Three habits: check with a Panel, build planes explicitly, and never feed labels into a Number input.

## Data Trees on the Tower

Video: `grasshopper_trees`

In this teaching model, the Nakagin Capsule Tower has two towers and 140 capsules. One list holds them all, but which one sits where?

Item 21 is one capsule of 140. The list does not say its tower, its storey or its facade.

A data tree keeps the address: building, tower, storey, facade. Each level splits into more branches.

One occupied facade group is one branch. Its length follows the reconstructed floor layout.

The selected capsule keeps its path: building, tower, capsule level and facade. Read its actual address on screen.

An index selects one capsule inside its branch. The path and index together identify the selected capsule.

A Panel lists the occupied facade branches. Tree Statistics reveals the different branch lengths.

Flatten on one occupied storey joins its facade branches into one list, preserving every capsule.

Flatten puts all 140 capsules into one list. Nothing is lost, except the structure.

Graft gives every capsule of that storey its own branch, with one item per branch.

Graft does the opposite: every capsule gets its own branch, and the path grows by one index.

Simplify removes the common building, tower and storey prefix. The occupied facade branches remain.

Simplify removes the leading 0 that every path shares, because there is only one building. The branches stay.

Path Mapper binds letters to the displayed address. The facade letter is dropped; the remaining address selects a storey.

Applied to all branches, the rule {A;B;C;D}(i) to {A;B;C} merges the facades: each branch is a storey, 20 branches.

Each branch now contains one occupied storey. The documented counts vary: two, seven or eight capsules.

Mapping to {A;B} gives one branch per tower: 76 capsules in A and 64 in B.

Flip Matrix swaps branches and item positions. Short branches receive null placeholders: empty addresses, not capsules.

A component pairs data branch by branch. Custom Preview takes capsules and a colour, and the paths decide who pairs with whom.

The classic mistake: the short colour list repeats in every branch. Alternating colours follow item indices instead of tower addresses.

The Param Viewer shows why: no path matches, so the short colour list repeats in every branch. Read both paths first.

The fix: map the capsules to {A;B} and graft the colour list. Now both trees have the paths {0;0} and {0;1}.

Matching paths send the tower colours to the documented totals: 76 teal capsules in A and 64 orange in B.

Recap: tree, branch, path, item, index, Flatten, Graft, Simplify, Path Mapper, Flip Matrix. Ten terms, one address.
