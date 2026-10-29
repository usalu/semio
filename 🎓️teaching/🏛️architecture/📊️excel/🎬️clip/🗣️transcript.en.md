# Excel Videos

Complete transcripts of the four Excel videos. All values come from one rounded teaching model of the Nakagin Capsule Tower in Ginza, Tokyo (Kisho Kurokawa, 1970–1972, demolished 2022); values that are assumed are marked as such in the videos. One paragraph per narrated step, in narration order.

## Contents

1. [The Anatomy of a Workbook](#the-anatomy-of-a-workbook)
2. [Cell References](#cell-references)
3. [The Room Program in Excel](#the-room-program-in-excel)
4. [Heating Demand in the Nakagin Workbook](#heating-demand-in-the-nakagin-workbook)

## The Anatomy of a Workbook

Video: `excel_anatomy`

The Nakagin tower in our teaching model: capsules in four types. How much floor does one give, and what if its width changes?

A workbook is the whole file: Nakagin.xlsx holds the calculation of the entire building.

Worksheets are the tabs inside it: Program, one sheet per capsule type, Climate and Energy.

A worksheet is a grid: columns have letters, rows have numbers. Type A describes one capsule.

A cell is named by column and row. B2 is the width of the capsule, shown with its unit.

A range joins cells: B2:B4 holds width, length and height.

The Name Box shows the address, the formula bar shows what the cell contains.

Typed numbers are constants, and so is text such as Width: Excel stores both as they are.

The cell shows a rounded number with a unit, but it stores the exact number. The format changes the display, not the value.

A formula starts with an equals sign. The floor area is interior width times interior length: B5 times B6.

Excel colours every reference and frames its cell: the formula points at its sources, and the same colours mark width and length on the floor.

The result is the floor of the capsule, drawn as a hatched surface.

A typical mistake: without brackets Excel multiplies first and adds after, so one side of the perimeter is missing. Brackets fix it.

A function is a named calculation. SUM adds a range: the fittings give their total area.

German Excel writes SUMME, separates arguments with a semicolon and decimals with a comma.

Widen the interior up to the outer wall: every dependent cell recalculates and the floor follows.

Trace arrows show precedents, the cells a formula uses, and dependents, the cells that use this one.

Typical errors have causes. An empty interior length leaves a floor area of zero, and dividing by it gives #DIV/0!.

A misspelled function gives #NAME?: SUMM does not exist.

Delete a source row and the reference has nothing to point to: #REF! in English, #BEZUG! in German Excel.

Text where a number is expected gives #VALUE! in English and #WERT! in German: here the interior length was typed with its unit.

Type the number again and the numbers return: floor area, volume and fittings.

Remember the terms: workbook, worksheet, cell, reference, formula, function, Name Box, formula bar. Next video: references.

## Cell References

Video: `excel_references`

In this teaching model of the Nakagin tower, the width of every type A capsule sits in one cell. What follows when it changes?

A reference is an address, not a number. D3 points to B2 and B3, so a wider capsule gives a larger roof area.

One formula per surface is tedious. Write U·A once in F3 and fill it down: the formula travels with the row.

Copied one row down, D3*E3 becomes D4*E4. Relative parts shift by the offset, here one row.

Each row computes one surface. Filled down, each surface lights up on the capsule as its U·A value appears.

The back wall touches the core, so its formula also carries the factor b: a filled formula must fit every row.

Add the column with SUM: the total equals the transmission H_T that cell B11 already holds.

Heat loss per surface needs the design temperature difference, kept in one cell, D10. Every row must point to that cell.

A dollar sign before column and row makes a reference absolute. The F4 key cycles through the four forms.

Filled down, the absolute reference stays on the input in every row: each surface sees the same temperature difference.

The typical mistake: forget the dollar sign and fill down. The reference slides off the input and the values go wrong.

Press F4 on the reference, fill again, and each row meets the same input. Values and surfaces match again.

A two-way table: U-values down, temperature differences across. One mixed formula, filled right and down, gives every heat flux.

The same pattern counts capsules: types across, towers down. In B11, one reference locks the column, the other the row.

Fill right and down in one move: each cell takes its own tower and its own type, and counts those capsules on the tower.

A sheet reference names the sheet first, then an exclamation mark. A name with a space needs single quotes.

Fill that down and only the cell part moves. The sheet name is text and stays, so the reference slides down the type A sheet.

So edit each copy: type B reads its own sheet, types C and D theirs, and every row shows its own net floor area again.

A 3D reference adds the same cell on all sheets between two tabs: here the heating load B13 of one capsule of each type.

The tabs between the two names define the span. Move a sheet out of it, and its value drops out of the sum.

The second failure: delete the sheet of type C. Direct references show an error, but the 3D sum quietly shrinks.

The dollar sign guards copies only. Delete the input row, and even an absolute reference breaks.

A defined name makes an absolute reference readable: theta_i and theta_e stand for the climate cells.

Recap: relative moves, absolute stays, mixed locks one direction, sheet and 3D references cross tabs, a name makes it readable.

## The Room Program in Excel

Video: `excel_program`

In our teaching model the Nakagin Capsule Tower carries 140 capsules of four types. How many of each, and how much floor do they offer?

A room program lists, per room, the fittings, the areas and the users. Here it is reduced to one capsule.

One worksheet per capsule type. The sheet of type A lists this capsule's fittings with width and length in metres.

A formula multiplies width, length and count. Filled down, every fitting gets its floor area and its own hatch.

The net floor area is the interior width times the interior length.

Net floor area minus the fittings leaves the free floor. The occupants are a typed-in constant: one person.

Net floor area times the interior height gives the volume in m³. The cell shows a unit but stores only the number.

Types B, C and D have their own sheets: the same shell, other fittings, so other free floor.

How many of each type? Select the 140 capsule rows and choose Format as Table: the list becomes the table Capsules.

Each table row is one capsule. Select a row and its capsule lights up on the tower.

COUNTIF counts one type in the table column. The counted capsules light up in the colour of type A.

Filled down, the criterion moves from A4 to A7 while the table column stays whole. Four colours cover the tower.

COUNTIFS adds a second condition: type and tower. The matrix shows how many of each type stand in each tower.

Each type's free floor comes from its own sheet. The net area is the same shell for all types; the free floor tells them apart.

XLOOKUP finds the type named in the cell and returns its count and its free floor: by key, not by position.

SUMPRODUCT multiplies each count by its free floor and adds the products: the free floor of all capsules in m².

Column D multiplies count by free floor. The bar chart shows the four totals; each bar grows as its cell computes.

Totals per tower: SUMIF adds the net floor areas of one tower from the table. Tower A and Tower B add up to the building.

Metabolism idea: capsules can be exchanged. Change one type on one storey in the list, and that capsule changes colour.

Counts and bars update at once; each capsule keeps its free floor. The total grows, because type A has more free floor than type C.

Sanity check: the counts add up to the rows of the list and to the building's 140 capsules.

A typo, a space behind the type, is not counted: that capsule turns grey and the check shows 139 and FALSE.

Delete the space: the capsule is counted again and the check is TRUE. A drop-down list prevents the typo.

Recap: from 140 capsules to floor area: table, COUNTIF(S), XLOOKUP, SUMPRODUCT, SUMIF.

## Heating Demand in the Nakagin Workbook

Video: `excel_energy`

How much heat does one Nakagin capsule need in a Tokyo winter? We calculate it in Excel, sheet by sheet.

Every capsule type has its own worksheet. Type A holds the dimensions, surfaces and results of this simplified teaching model.

Three constants describe the shell: width, length and height. On the capsule they are the three dimension lines.

Each surface area is a formula from these dimensions. The front wall is width times height minus the round window.

The back wall is bolted to the core, so only part of its heat escapes: its U·A is multiplied by the attached factor b.

The U-value tells how many watts pass one square metre for each kelvin of temperature difference. Glass leaks most.

Area times U-value is U·A in watts per kelvin. The larger the U·A of a surface, the longer and thicker its arrow.

H_T sums all U·A. SUMPRODUCT does it in one formula: area times U-value times temperature factor, back wall included.

The Climate sheet stores Tokyo under defined names: temperatures, degree days, b, air change and air heat capacity.

Heating degree days are an area: each month's shortfall below the 18 degree line, times its days, adds up over the year.

A typical mistake: treating the attached back wall like an exposed one. With b at one, H_T grows and the loss is overestimated.

The factor is one cell on the Climate sheet. Set it back, and the back-wall arrow and H_T return to their values.

Air leaks too: air heat capacity times air change rate times volume gives the ventilation loss H_V.

H_T plus H_V is the heat loss of the whole capsule per kelvin. Every later result is built on these two cells.

The heating load Φ is the loss per kelvin times the temperature difference: the watts the heater must deliver on the design day.

Yearly demand: loss per kelvin times degree days times 24 hours, divided by 1000, in kWh. Divided by floor area: kWh per m².

The Energy sheet multiplies each type's demand by its capsule count from the Program sheet and adds up the whole building.

A, B and D share one shell, and fittings do not change the heat loss. Only C, with its larger window, needs more.

Why so much for so little floor? Five faces and a window wrap a small room. Heated neighbours above and below would cut the demand.

Which cells depend on the window U-value? Trace the dependents: they run through the type sheets to the building total.

Enter a better window in this one input cell, and every dependent cell, arrow and total follows.

A typical mistake: someone types the building total as a constant. It no longer follows the change.

Put the formula back, and the total matches its rows again.

Recap: U·A per surface, H_T and H_V, load from the temperature difference, demand from degree days, times the capsule count.
