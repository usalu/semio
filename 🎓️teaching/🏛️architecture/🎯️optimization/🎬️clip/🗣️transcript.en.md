# Optimization Videos

Complete transcripts of the six optimization videos. All values come from one simplified annual teaching model of a seminar room, not from measurements. One paragraph per act, in narration order.

## Contents

1. [Minimization and Maximization](#minimization-and-maximization)
2. [Sampling the Design Space](#sampling-the-design-space)
3. [Design of Experiments](#design-of-experiments)
4. [Multiple Criteria and a Normalized Index](#multiple-criteria-and-a-normalized-index)
5. [Pareto Optimality](#pareto-optimality)
6. [Algorithms and Evolutionary Search](#algorithms-and-evolutionary-search)

## Minimization and Maximization

Video: `optimization_extrema`

This seminar room has one glazed façade. Which compass direction should it face? The façade orientation θ is our design variable: east 90°, south 180°, west 270°, north 360°.

Everything else stays fixed: window share, overhang and insulation. Only the orientation θ turns.

Objective: the electricity for heating, cooling and lighting, in kWh/a. We minimize it, so lower is better. The values come from a simplified teaching model of one year.

We turn the façade from east, 90°, through all orientations and simulate electricity in kWh/a.

Turning south, the winter sun heats the room for free: heating falls. The high summer sun hits south glass steeply: cooling electricity drops to the lowest so far. Facing west, the low evening sun floods the room: cooling stays high, and the total peaks near 282°. Facing north, only diffuse light reaches the glass: cooling is least, heating and lighting rise. At 450° the façade faces east again, so the curve ends where it began.

Start facing northwest, 315°, and always turn toward less electricity. At 360°, north, cooling rises faster than heating falls: every neighbour is worse, a local minimum.

But south, 180°, is lower still: it is the global minimum of the whole range. From north to south the free winter sun saves far more heating than the summer sun adds cooling. Stopping in the north valley wastes electricity every year, and that costs CHF as well.

The optimal design is θ* = 180°, south. Its value, E at θ*, is the annual electricity, in kWh/a. Keep them apart: the design θ* is a direction; the value E at θ* is electricity in kWh/a.

With one variable we can scan every degree and compare all valleys, not just the nearest. With many variables, a search can end in the wrong valley, just as our start at 315° did.

Now the opposite: how much do we save against an east façade, in kWh/a and CHF/a?

Minimizing E equals maximizing the saving S: valleys turn into peaks and the curve flips over. The highest saving sits at the same orientation as the lowest electricity: θ* = 180°.

The street to the south is loud: the glazed façade must face the quiet courtyard, north ± 90°. A constraint limits which designs are allowed. It adds no objective and changes no value of E.

Among the allowed orientations, north, 360°, is now the best one. The quiet courtyard has a price: extra kWh/a and CHF/a compared with the free south optimum.

Another window share moves the optimum's value; an overhang moves its direction as well. With the courtyard rule 270°–450°, the answer here is north, 360°.

## Sampling the Design Space

Video: `optimization_sampling`

Which window share and overhang depth suit this south façade? Each pair is one design: point and room move together.

One simulation sums a year of heat, sun and daylight; two solstice days are shown. Winter sun cuts heating electricity, summer sun adds cooling, daylight cuts lighting. One number per design: electricity in kWh/a, carried by each point.

But only nine simulations are affordable. Which nine designs?

A full grid uses three levels per axis and combines them all. Simulate them one by one: room, sun rays and electricity bar for each. The ninth simulation empties the budget. The best of nine is ringed; its room shows why it beats the worst.

More design variables: orientation with four levels, insulation with three. Levels multiply: 3 · 3 · 4 · 3 makes 108 simulations. Every extra level multiplies the simulations: here twelve times our budget.

Random sampling draws nine independent designs, recorded so everyone sees the same points. Same budget, same model: nine more simulations.

No draw lies inside this box (edges excluded): WWR 35 to 70 %, overhang 0.15 to 1.20 m. Elsewhere draws crowd: two nearly identical rooms, two simulations. The best draw sits in the crowded pair: no even coverage.

Latin hypercube sampling: nine strata per axis, exactly one sample in each stratum.

This vertical strip is one window-share band with exactly one design. The horizontal strip also holds exactly one: each axis is covered evenly.

But the plane has 81 cells and only nine points: empty regions remain.

Until simulated, a point is only a position, with no result. Each point becomes a full building and runs through the same model.

To judge the point sets, reveal the surface with a dense reference grid. Darker means less electricity. The lowest point lies inside the range.

Compare each point set's best design with the optimum of the dense reference grid. Each best design is the best of nine, yet none reaches the optimum. The remaining gap is the price of having only nine simulations.

Sampling shows coverage and promising regions to search next. A finite sample cannot prove that it found the global optimum. The model itself still needs validation. Nine simulations found a good south façade design, no proven optimum.

## Design of Experiments

Video: `optimization_experiments`

Sampling asks where to simulate. Experiments start with a question about this room. How do window share and overhang change the summer cooling electricity of our seminar room? Sun, room and weather stay fixed. Only window share and overhang change.

Factor A is the window-to-wall ratio (WWR): 30 % or 70 %. Factor B is the overhang depth: 0 m (no overhang) or 1 m. Code every low level as minus one and every high level as plus one.

A full two-by-two plan simulates all four combinations of A and B. Each corner is one summer room: along A the window share grows, along B the overhang.

The summer sun enters all four rooms. Every ray carries the same amount of solar energy. The model returns each room's cooling electricity in kWh/a. Lower is better. A simplified annual teaching model, not a measurement.

We plot cooling electricity against window share, with one line per overhang depth. Each point is one room: orange without overhang, green with 1 m.

At 30 % the window catches few rays, so the overhang saves little cooling electricity. At 70 % the window catches many more rays, so the same overhang saves much more.

The savings differ, so the lines are not parallel: that is an interaction. The effect of the overhang depends on the window share.

Main effect of A: the mean at 70 % minus the mean at 30 % window share. Main effect of B: the mean with overhang minus the mean without.

The interaction effect is half the difference of the two overhang effects. Half that difference is the interaction: a bigger window share makes the overhang save more.

Four simulations fit a bilinear model: a constant, two slopes and a product term, A times B. Setting A and B to zero predicts the plan's middle: the mean of all simulations. Each coefficient is half an effect, in kWh/a per coded step.

Two levels only draw straight lines. The true cooling electricity may bend between them.

So we add a confirmation simulation at the middle: 50 % window share, 0.5 m overhang. The model predicts more cooling electricity than the simulation returns. The first 0.5 m of overhang already delivers most of the saving: the curve bends.

Running the same simulation again gives exactly the same value: no estimate of noise. Measured rooms scatter: physical experiments repeat measurements to estimate variability.

Plan simulations to answer the question, estimate effects and confirm. Add insulation as factor C: a full plan needs eight simulations, a fractional plan only four. Here C moves exactly with the interaction A×B: the two effects are confounded. For our room, insulation as factor C saves four simulations, but its effect hides in A×B.

In our room a 1 m overhang saves 279 kWh/a at a 70 % window share, but only 122 kWh/a at 30 %.

## Multiple Criteria and a Normalized Index

Video: `optimization_index`

Which of five façade designs suits this seminar room best? Only window share, overhang and insulation are compared.

Electricity intensity and cost: lower is better. Daylight factor: higher is better. Embodied carbon: lower is better.

No design is best everywhere.

Kilowatt-hours per m² and year plus francs? That sum has no unit. Added anyway, the biggest numbers decide, not the importance. Cost in kCHF, carbon in tonnes: the ranking changes.

Give each criterion its own ruler, starting with electricity. Each design sits at its real value; the room shows what the value means. Sun becomes cooling demand; the chiller divides it by its efficiency. In winter the room loses heat; the pump needs less power than heat. Lamps add the rest, less where daylight helps; the sum is the electricity.

Step one: subtract the best of these five values. Step two: divide by the span from best to worst.

Zero is the best of these five, one the worst of these five.

Façade cost gets the same ruler: the cheapest façade becomes zero. The price is the sum of its parts: fixed items, wall, insulation, windows and overhang.

Daylight factor: more is better, so the ruler is mirrored.

Embodied carbon is the sum of the façade's materials, per layer.

Embodied carbon follows the same rule: its five values now lie between zero and one.

Weights say how much each criterion matters; they add up to 100 %. Here: electricity intensity 40 %, cost 30 %, daylight factor 20 %, embodied carbon 10 %. A wider column means a stronger pull on the result.

Slide the filled parts together: their length is the index. The shortest bar has the lowest index and ranks first. Weight times normalized value, summed over four criteria: the index.

Its room: not best everywhere, but best overall. Strengths elsewhere outweigh its weakest criterion.

A cost-driven client puts more weight on cost. The bars change and the ranking reorders. Another façade wins, though no design changed.

Sweep the cost weight to see where the winner switches. Close calls are fragile: test the ranking before you decide.

The rulers depend on the designs compared: add a sixth façade. Premium is dearest and most frugal: costs shrink, electricity values shift a little. The order of the original five can flip, though they did not change.

Record weights, designs and value ranges with every decision.

Client budget 21,000 CHF: a mandatory limit, applied before weights. Within the budget, strengths still offset weaknesses; the limit itself is never traded off.

To see trade-offs before weighting, look at Pareto fronts.

For this room at these weights, the winner has the lowest index.

## Pareto Optimality

Video: `optimization_pareto`

Which window share, insulation and overhang suit this seminar room's glazed façade? A simplified annual teaching model gives every façade its cost and its electricity.

We want both small: cost to the left, annual electricity at the bottom. Every façade becomes one point: cost along the horizontal, electricity up.

We simulate every combination of six window shares, three insulation thicknesses and two overhang depths. Point at any tile: room, cost and electricity follow.

The cheapest and the most frugal façade lie far apart: none is both.

Compare two façades: this one has a larger window share and more insulation. That one is cheaper and uses less electricity. So the first is dominated: nobody would pick it.

Dominance: no worse in cost and no worse in electricity. And strictly better in at least one of them; here, in both.

Sweep from cheap to expensive: keep only façades whose electricity beats all cheaper ones. What remains is the Pareto front of the simulated designs: no gain without extra cost.

Off the grid, a bigger window shaded by a deeper overhang, paid for by thinner insulation, dominates a front member.

The room is design space, the plot is objective space; simulation maps one onto the other.

Walk along the front, starting at the cheapest façade. Step one: a 0.6 m overhang costs 192 CHF more and saves 65 kWh/a. Divide the two: 3.0 CHF for every kWh/a saved.

At 0.25 CHF per kWh the savings repay this step in about 12 years. A façade lasts about 30 years: only steps that pay back within that lifetime are worth it.

Walk on: each step costs more per kWh/a saved, until the payback exceeds the lifetime. Here is the knee: the next step would take about 34 years to pay back, longer than the façade lasts. At 0.40 CHF per kWh that step repays in 21 years, so the knee moves: same front, different best choice.

Add daylight as a third objective: tile colour shows the daylight factor in percent. In 2D it loses: a dimmer room is cheaper and needs less electricity. With daylight counted, nobody beats it on all three objectives. A two-dimensional picture can hide a trade-off: check every objective.

To pick one façade, weigh the objectives as in the index video: the weights fix what one kWh/a is worth. The winner always sits on the front and slides along it as the weights change.

The front removes dominated designs; it does not choose: tariff, lifetime and weights do. For this room, the knee: 30 % window share, 0.6 m overhang, 12 cm insulation at 0.25 CHF/kWh.

## Algorithms and Evolutionary Search

Video: `optimization_algorithms`

Seminar room, south façade: which window share and overhang need the least electricity? Every design costs electricity three ways: heating, cooling and the lamps. One simulation per design, budget 50: a solver chooses the next design.

The full map needs 1,681 simulations; colour shows annual electricity in kWh/a. The valley floor is the best design: less window share, deeper overhang. In practice we see only simulated points; the rest stays in the fog.

Gradient descent starts at a 75 % window share and 0.1 m overhang. Four close neighbours reveal the slope: five simulations per step. More window share raises the electricity here, more overhang lowers it. The step goes downhill: the room follows, the bar shrinks.

The first step overshoots the valley floor, to too small a window share. Repeating, the path zigzags; some steps even raise the electricity.

After 50 simulations, gradient descent's best design is within 1 % of the map minimum. Four of every five simulations probe slopes; noise or a second valley can mislead them.

Pattern search needs no slope: it only compares simulated designs. From the same start it tries window share and overhang, up and down: four simulations. The best neighbour wins and the room moves there. If no neighbour is better, halve the step. Round after round the steps shrink until no neighbour is better.

Pattern search gets within 1 % of the minimum first, after 16 simulations, with no slope.

A genetic algorithm keeps eight designs; window share and overhang are its genes. Each card shows one design and its simulated electricity: eight simulations. Spread over the map, they explore several regions.

Selection: draw two cards at random; the one with less electricity becomes a parent. Good designs win more often, yet even a mid-ranked card can win a duel.

Crossover: the child takes window share from one parent, overhang from the other. On the map the child sits at a corner of its parents' box.

Mutation nudges the child a little; here the change helps.

Six children, one simulation each: six simulations per generation.

Elitism: the two best parents survive, so the best value never gets worse.

Later a mutation can also hurt: the simulated child is worse than the crossover alone.

Repeat for seven generations: the cloud contracts and the best card improves in jumps.

After 50 simulations the best card is within 1 % of the map minimum. Within 1 %: pattern search needs 16 simulations, the others more than twice as many.

Same model, same budget: the best electricity so far, plotted against simulations used. Pattern search gets there first; all three end within 3.3 kWh/a of the map minimum.

Stop at a budget, a tolerance or stagnation, and say which. Tolerance fires first: the genetic run would stop after 15 simulations, still far from the best.

None of them proves the global optimum; for that we need the full map, 1,681 simulations.

Check the winner with a finer simulation, then judge it as architecture.

Parametric tools wire this up: sliders, simulation, solver. Choose by smoothness, noise and simulation budget; a population copes with several valleys.

Answer: 66 % window share, 0.99 m overhang, 494.8 kWh/a; the architect judges the rest.
