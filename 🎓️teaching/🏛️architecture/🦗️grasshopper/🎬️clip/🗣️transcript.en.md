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

A Value List offers fixed choices: the capsule types A to D. Pick one and the capsule takes that type's window.

A plugin, a .gha file, adds a tab to the ribbon. Ladybug Tools studies the sun on the capsule window: more sun, warmer tint.

Now the data. One value is an item: the height 0 m of the first storey, one capsule on Tower A.

A Series with step 2.75 metres makes a list: nine heights in one branch {0}, indices 0 to 8, one capsule on each of nine storeys.

Change Count from 9 to 11 and the list grows by two: Tower B has eleven capsule storeys.

Many lists form a data tree. Each facade of a storey is one branch with its own path, here {0;0;3;1}, holding two capsule items.

A common mistake is to read a tree as one long list. The index 1 alone fits all 27 branches of Tower A, so it names 27 capsules.

Only path plus index names exactly one capsule: path {0;0;3;1}, index 1. The Panel shows both.

The canvas terms, row by row in English and German: script, Skript; component, Komponente; and six more.

The data terms in English and German: item, Element; list, Liste; tree, Datenbaum; branch, Ast; path, Pfad; index.

## One Capsule, Parametric

Video: `grasshopper_capsule`

The Nakagin tower hangs 140 capsules on two cores, yet a single Grasshopper definition builds every one of them.

Parametric means: parameters feed dependencies, dependencies build geometry. Change one value and everything follows.

Six Number Sliders hold the parameters: width, length, height, corner radius, window diameter and window offset.

Everything starts on a plane: origin plus X, Y and Z axes in Rhino's red, green and blue. This is the capsule's base plane.

Center Box takes the plane and the sliders for width, length and height. It centres itself on the plane, so half of it sinks below the floor.

Plane Origin lifts the plane by half the length and half the height: now the box stands with its bottom face on the base plane.

Drag the length slider: the dimension line, the box and every component behind it follow in the same moment.

Fillet Edge rounds the edges with the corner radius slider: the soft shell of the capsule appears.

A typical mistake: a Circle on the base plane lies on the floor, not on the front face. A wire does not know where you mean.

The fix: a plane on the front face, moved by the length and the window offset, carries the circle to the right place.

Solid Difference subtracts a cylinder of that circle from the shell: the round window is cut into the front face.

Four points on the back face mark the high-tension bolts that fix the capsule to the core.

Each slider domain is a design constraint; here the limits are assumptions, for example a transport envelope.

Push the width past its domain: the slider clamps at its upper limit and the envelope turns red. The domain is a design rule.

The capsule types of the teaching model are one definition with different parameter values.

Feed type A to D into the same sliders: window offset and window diameter change, and the definition builds four capsules.

On the core the capsule needs a target plane on the wall; the base plane must land on it.

Orient moves the capsule in one step from its base plane A to the target plane B.

Rotate the target plane about the core axis and the capsule turns with it, because it follows plane B.

List Item picks the type for each plane: index 0 gives type A, 1 gives B, 2 gives C, 3 gives D.

Eight planes need indices up to 7, but there are only four types: without Wrap, List Item warns from index 4.

Switch Wrap on and the index cycles: 4 gives type A again, 5 gives B, and every plane gets a capsule.

Recap: parameter, dependency, geometry, domain, plane, Orient, List Item and index.

## From One Capsule to the Tower

Video: `grasshopper_tower`

The Nakagin Capsule Tower has 140 capsules. How do they become two towers?

The tower splits into five parts: core, planes, capsules, capital and podium. Each part gets its own coloured group on the canvas.

The core is a square extrusion. Plan side, storey height and storey count are the sliders; they decide the height of the box.

The result is the shaft: one box with a line on every storey, and the storey count is read straight from the definition.

A Series gives one elevation per storey. Each elevation becomes a level for the capsules on the shaft.

A second list holds the four facade angles. Rotating one plane by each angle gives one plane per facade.

Every plane carries its axes: X runs along the facade, Y points outward, Z points up.

Two capsules sit side by side on each facade: item 0 and item 1, one step to each side along X.

Orient moves the capsule from its base plane onto each target plane. One capsule, one component, every plane.

A type pattern picks the capsule type for every plane. List Item reads the pattern by index; only the list differs.

Now the capsule storeys slider: the count is storeys times facades times items, and the drawing grows with it.

Not one wire changes. The definition does not know how tall the tower is; it only follows the number.

A typical mistake: without Cull Index, tower B gets 88 capsules, two of them west on the top storey, where the building has none.

Cull Index removes exactly those two planes, indices 6 and 7. Now the definition gives 86, the number the building really has.

The capital is its own small parametric part. Height and overhang are two sliders on top of the core.

It sits on top of the core at the height the capsule storeys slider gives, so it rises with the tower.

The lower building is only an extruded massing: one footprint, one height.

That is on purpose. The podium is context, not the subject: one extrusion is enough, while one capsule parameter changes every copy.

The same definition builds tower A and tower B. Capsule storeys and facades differ; the formulas show both.

Together they give the historical count, 140 capsules, read from the definition and not typed in.

Metabolism: capsules are replaceable. Swap type B for type C, and every capsule of that type gets the larger window at once.

Recap: core, planes, capsules, capital, podium. This is the teaching model of the Nakagin Capsule Tower.

## Casting: When Types Meet

Video: `grasshopper_casting`

This capsule should face west, but its window looks north. One silent cast put it there. Which one?

In this teaching model of the Nakagin Capsule Tower, every parameter has a type: width is a Number, storeys an Integer, placement a Plane.

A wire connects whatever you drag. When the types differ, Grasshopper tries to cast the data.

A Panel holds the text 2.5. The Number input reads it as the number 2.5, and the capsule becomes 2.5 metres wide.

A slider says 10.6, but Storeys wants an Integer. Grasshopper rounds, here to 11; check this in Rhino 8.

Grasshopper rounds; at exactly .5 it rounds to the even number: 2.5 gives 2, 3.5 gives 4. Check this in Rhino 8.

To decide the rounding yourself, put a component such as Floor before the Integer input: the eleventh storey disappears.

A Point at a Plane input becomes a plane at that point with the world axes. Here the point is the origin of a facade slot.

On the west facade the capsule must face west. The world plane points it north: the window turns by 90°.

A Line becomes a plane whose Z axis follows the line. From the core to the slot Z points outwards, but X and Y are still guessed.

Build the plane explicitly from origin and axes. The capsule turns into place and faces outwards.

A Number can stand in for a Domain: the capsule length 4.0 becomes the interval 0 To 4, and Divide Domain cuts it.

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

The smallest piece: tower A, storey 0, facade 0. Its two capsules form one branch, a list of two items.

Capsule 21 has the path {0;0;3;1}: building 0, tower 0, which is A, storey 3, facade 1.

The branch is a list. An index addresses one item in it: {0;0;3;1}[1] is capsule 21, the right capsule of that facade.

A Panel in tree mode lists every branch with its path. Tree Statistics counts 70 branches of two items each.

Flatten first, on one storey of tower A: its three branches become one list of six items.

Flatten puts all 140 capsules into one list. Nothing is lost, except the structure.

Graft on the same storey: each of the six capsules gets its own branch, six branches of one item.

Graft does the opposite: every capsule gets its own branch, and the path grows by one index.

Simplify on that storey strips the path prefix {0;0;0} that its three branches share. The branches stay.

Simplify removes the leading 0 that every path shares, because there is only one building. The branches stay.

Path Mapper binds the letters to a path. Capsule 21: A is 0, B is 0, C is 3, D is 1. D, the facade, is dropped.

Applied to all branches, the rule {A;B;C;D}(i) to {A;B;C} merges the facades: each branch is a storey, 20 branches.

Now each branch is a storey: six capsules where it has three facades, eight where it has four.

Mapping to {A;B} gives one branch per tower: 54 capsules in tower A, 86 in tower B.

Flip Matrix swaps branches and items, so all left capsules form one list and all right ones another: two branches of 70.

A component pairs data branch by branch. Custom Preview takes capsules and a colour, and the paths decide who pairs with whom.

The classic mistake: the short colour list repeats in every branch. Left capsules turn teal, right ones orange, in both towers.

The Param Viewer shows why: no path matches, so the short colour list repeats in every branch. Read both paths first.

The fix: map the capsules to {A;B} and graft the colour list. Now both trees have the paths {0;0} and {0;1}.

With matching paths, each tower's colour reaches all its capsules: 54 teal in tower A, 86 orange in tower B.

Recap: tree, branch, path, item, index, Flatten, Graft, Simplify, Path Mapper, Flip Matrix. Ten terms, one address.
