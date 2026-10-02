@capability-pets-behavior-choice
@oracle-pets-scipy
@comparison-pets-float-v1
Feature: A pet chooses what to do next by weights, and no activity is a dead end
  By default pets are slightly active: mostly idle with life in it, a fidget now and then, a short walk now and
  then (design §1, §5.6). What an idle pet does next is one weighted draw: `activityWeights(actor, species,
  situation)` gives every activity a weight (0 = not eligible) from the limits of the mode (`MODE_LIMITS`), the
  needs of the actor and what goes on around it, and `randomPick` takes one index. `dwellOf(activity, mode, unit)`
  says how long an activity lasts, `moodOf(activity)` which mood it eases towards, `encounterOf(affinity, unit)`
  what two pets do when they meet — friends mostly cuddle, rivals mostly squabble, everyone else mostly greets —
  `followersOf(activity)` which activity may follow which, and `castOf(cast, capacity, epoch, seed)` who is on
  stage (design §4.6, §5.2, §5.4, §5.5).

  THE REFERENCE is numpy and scipy. The choice is `numpy.searchsorted` (side `right`) of `unit × total` in
  `numpy.cumsum` of the positive weights; the unit of a keyed draw comes from
  `numpy.random.SeedSequence(key).generate_state(1)`, a library that has never seen this repository; the rotation
  of a cast is `numpy.roll`. The activity graph is judged by `scipy.sparse.csgraph`: breadth-first search from
  every activity names everything reachable from it, and `connected_components(connection="strong")` must find
  exactly one component — from every activity a pet can get to every other one, so it can never be stuck. The
  tables themselves (limits, weights, dwells, moods, encounter shares, followers) are this product's own tuning;
  the oracle restates them from the design text with numpy arithmetic, so the committed numbers, the TypeScript
  twin (`@semio-tech/pets`) and the Rust twin (`pets` crate) are held to one reading. Floats compare within 1e-9,
  indices, ticks, names and counts exactly.

  The committed situations cover both lively modes and the still one, quiet times, a full and an empty mover
  budget, a perch without room, a species without a fidget, a watched and a tired pet, and company on its surface
  (which makes a pet restless: the weight of a hop grows with every neighbour, so a crowd thins out where another
  perch is open); 256 decisions per situation show what a mode feels like (a rested pet in calm stays idle in more
  than half of them).

  The vectors shared://🧠️behavior-choice/🔣️.json are generated, never hand-edited, from the Python reference in
  this directory, from the repository root:
  `.venv/Scripts/python.exe .🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️02/QUIZ-PETS/generate_behavior_vectors.py`
  (`.venv/bin/python` outside Windows).

  @id-limits
  @level-fundamental
  @mode-differential
  Scenario: The modes allow what the design says
    Given the committed vectors shared://🧠️behavior-choice/🔣️.json
    When the limits of every mode are read from MODE_LIMITS
    Then every implementation projects the same limits per mode: still allows nothing, calm one mover and idle dwells of 6 to 20 seconds, lively two movers and 3 to 10 seconds

  @id-weights
  @level-fundamental
  @mode-differential
  Scenario: The weights of an idle pet follow its needs and its surroundings
    Given the committed vectors shared://🧠️behavior-choice/🔣️.json
    When activityWeights is taken of every committed situation
    Then every implementation projects the same eleven weights per situation, 0 for every activity that is not eligible

  @id-picks
  @level-fundamental
  @mode-differential
  Scenario: A weighted pick follows the running sum of the positive weights
    Given the committed vectors shared://🧠️behavior-choice/🔣️.json
    When weightedIndex is taken of every committed weight list at every committed unit
    Then every implementation projects the index numpy's cumsum and searchsorted find, never an index whose weight is not positive, and −1 when no weight is positive

  @id-decisions
  @level-fundamental
  @mode-differential
  Scenario: An actor decides counter after counter
    Given the committed vectors shared://🧠️behavior-choice/🔣️.json
    When randomPick over the weights of every committed situation is taken of the keys [seed, stream, 0] … [seed, stream, count − 1]
    Then every implementation projects the same activities in counter order and the same count per activity

  @id-dwells
  @level-fundamental
  @mode-differential
  Scenario: A dwell is a whole number of ticks inside the range of its activity
    Given the committed vectors shared://🧠️behavior-choice/🔣️.json
    When dwellOf is taken of every committed activity and mode at every committed unit
    Then every implementation projects the same ticks, low + floor((high − low) × unit)

  @id-moods
  @level-fundamental
  @mode-differential
  Scenario: Every activity has its mood
    Given the committed vectors shared://🧠️behavior-choice/🔣️.json
    When moodOf is taken of every activity
    Then every implementation projects the same mood per activity, from −0.8 for a squabble to 1 for a cuddle

  @id-encounters
  @level-fundamental
  @mode-differential
  Scenario: Friends mostly cuddle, rivals mostly squabble, everyone else mostly greets
    Given the committed vectors shared://🧠️behavior-choice/🔣️.json
    When encounterShares and, at every committed unit, encounterOf are taken of every committed affinity
    Then every implementation projects the same three shares and the same kinds per affinity

  @id-reachability
  @level-fundamental
  @mode-differential
  Scenario: Every activity is reachable from every other one
    Given the committed vectors shared://🧠️behavior-choice/🔣️.json
    When followersOf is read for every activity and the graph is searched from every activity
    Then every implementation projects the committed followers, the reachable set scipy finds from every activity — all eleven — and one strongly connected component

  @id-casts
  @level-fundamental
  @mode-differential
  Scenario: A cast shows its core and keeps a seat for a visitor
    Given the committed vectors shared://🧠️behavior-choice/🔣️.json
    When castOf is taken of every committed cast, capacity and seed at every committed epoch
    Then every implementation projects the visitors of the rotation on the seats the core leaves free — one seat at least from two seats on — after a core that fits in its order, or a core that does not fit taking turns itself; whoever takes turns is started by the seed and moved on by one per epoch, never more than the capacity and never a species twice
