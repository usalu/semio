@capability-quiz-learner-lifecycle
@oracle-quiz-python-reference
@comparison-quiz-score-v1
Feature: Registrations, runs, answers and submissions decide the same events and rejections everywhere
  The proctor wraps two pure deciders that both cores share (design §8). A handle decides identify-learner
  for a pseudonym or name: every handle key has a stream of its own, a free handle registers the command's
  learner under the normalized display (`learner-registered`), a claimed one is refused (`handle-claimed`)
  — recalling a handle is a read and writes nothing — and a handle outside the policy, a handle of another
  key and an anonymous identity are `handle-invalid`. The learner decides its own commands: an anonymous
  identify-learner registers the learner once (`learner-exists` afterwards), start-run, record-answer and
  submit-run run against the current quizzes and their revisions: unknown learners and quizzes are
  rejected, an open run of the current revision blocks a new one while a stale one is voided first,
  answers are checked against the run's sheet (`unknown-task`, `answer-invalid`, the latest answer per
  task wins) and refused on a revised quiz (`quiz-revised`), and a submission is voided on a revised quiz,
  refused while incomplete (`run-incomplete`), and otherwise emits `run-submitted` with the scored result
  followed by one `badge-awarded` per newly earned badge. Every decision first holds the command to its id
  and slug shapes (`id-invalid`) and to its own learner (`unknown-learner`), and to the caps of `Limits`:
  a registration beyond the cap of learners is `roster-full` (`registrationRejection`), a start after the
  cap of started runs — open, submitted or voided alike, per quiz or in total, so switching the challenge
  back and forth stops — `runs-exhausted`, an answer after the cap of recorded answers of a run
  `answers-exhausted`.

  A run is played at one challenge (challenge design §3.5), named by `start-run` and fixed by
  `run-started`, which starts the run at the instant the `start-run` carries — a run started offline and
  delivered late keeps the device's start, so a task opened on the device gains no time after the sync.
  Every claimed instant is first lowered to five minutes (`CLOCK_LEAD`) past the decider's clock: an honest
  device four minutes ahead decides alike on the device and at the proctor, at once or late, while a start
  or an opening dated an hour ahead is lowered and buys no time. An
  instant beyond 2^53 − 1 is `id-invalid`. An open run of the quiz at the current revision blocks a start at its own challenge
  (`run-open`) and is voided by a start at another one — after the caps, which refuse first. The run's
  sheet, its answer rules and its scoring follow the challenge; a result carries its challenge and its
  points (score × par). On a timed run (expert) a task is opened first: `open-task` (checked like
  `record-answer` for its ids and slug) decides `task-opened` at the instant the learner acted, raised to
  the run's start, is refused for a task already opened (`already-opened`), on an untimed run
  (`run-untimed`), a closed run, a revised quiz or an unknown task. An answer carries the instant the
  learner acted: on a timed run it is refused for a task not opened (`task-unopened`) and after the
  task's seconds (`time-up` — the instant raised to the opening, minus the opening, above
  `seconds × 1000`; exactly at the limit is in time, 1 ms past it is not, for every kind of task); on an
  untimed run the instant is raised to the run's start; `answer-recorded` keeps the raised instant. Apart
  from the lead the decider's clock never enters these instants: a device clock running less than five minutes
  ahead of the decider's or any amount behind it, an answer delivered at once or long after, all get the
  same verdict. A timed run is submitted with tasks unanswered or partly answered, what is missing
  scoring as a miss; an untimed run stays `run-incomplete` until every task is complete. The learner view
  names each run's challenge and, once submitted, its points; `best` is per quiz the submitted run with
  the most points (a later run replaces it only with strictly more) as its challenge, score and points,
  and `total` sums those points. A run view shows the sheet at the run's challenge, `opened` on a timed
  run, and on an open easy run the hints of every task that has any (challenge design §8: compare, profile,
  group and category questions about the learner's own answer, as the challenge-rules case states them). The badges `hard-cooling` and
  `expert-sorter` ask for a least challenge.

  THE REFERENCE is `🐍️.py` beside this file: a second implementation of §3–§8 written in Python from
  the design text alone (the handle policy over `unicodedata`, sheet over numpy's MT19937, answer rules,
  scoring, badges, deciders, folding, views). Only decisions and schema views are projected — never a
  core's private state — so each subject replays the committed `given` events and steps through its own
  `decide`/`evolve` (`@semio-tech/quiz`: `decideHandle`/`evolveHandle`, `registrationRejection`,
  `emptyLearnerState`, `decideLearner`/`evolveLearner`, `learnerView`, `runView`; the snake_case twins of
  the `quiz` crate). Every step carries its decision time `now`; a step may revise a quiz from then on, the
  way a redeployed quiz file changes its revision; a sequence may carry its own caps. Scores inside
  `run-submitted` compare under `quiz-score-v1`.

  The last scenario plays the catalog a site actually serves, read from the repository at run time
  (`🎓️teaching/🏛️architecture/❓️quiz/🔣️.json` and the quizzes it lists): a learner who answers every
  sheet task of every quiz perfectly — whatever items the run draws — earns every badge of the catalog,
  "Heating Expert", "Numerical Brain" and "Pattern Seer" among them, and a learner who makes exactly one
  mistake in one task earns every badge but those that depend on that task. Every play names the
  challenge of its runs — perfect tours at medium, hard, expert and easy, flawed ones taking turns at
  medium, hard and expert —, a tour below a badge's least challenge earns every badge but that one, and a
  timed task is opened before it is answered, every command carrying the instant it is decided at. Every
  implementation builds the answers from the quiz definitions and its own sheets: perfect answers guess
  the true values where the keys are hidden, a flawed sorting exchanges the smallest and the largest item
  with their guesses, a flawed matching exchanges the cards — or the guesses — of two items of different
  value in the first dimension.

  Command idempotency by command id is a property of the proctor's framework deciders, not of these
  pure functions, so every committed command id is distinct.

  The vectors shared://🧾️learner-lifecycle/🔣️.json are generated, never hand-edited, from the Python
  reference in this directory, from the repository root:
  `.venv/Scripts/python.exe .🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️28/QUIZ-PRODUCT-AND-TEACHING-PROCTOR/generate_quiz_vectors.py`
  (`.venv/bin/python` outside Windows).

  @id-registrations
  @level-fundamental
  @mode-differential
  Scenario: identify-learner registers a handle once, refuses a second claim, and the cap of learners refuses registrations
    Given the committed vectors shared://🧾️learner-lifecycle/🔣️.json
    When every committed sequence is decided and folded from its unclaimed handle key and every committed learner count is held to its cap
    Then every implementation projects the same decision per step and the same rejection per count

  @id-learner-decisions
  @level-fundamental
  @mode-differential
  Scenario: Anonymous registration, start-run, record-answer and submit-run decide the committed events and rejections
    Given the committed vectors shared://🧾️learner-lifecycle/🔣️.json
    When every committed learner sequence folds its given events and then decides and folds every step under its caps
    Then every implementation projects the same decision per step, scores within 1e-12

  @id-learner-views
  @level-quick
  @mode-differential
  Scenario: The folded learner answers with the committed learner view and run views
    Given the committed vectors shared://🧾️learner-lifecycle/🔣️.json
    When every committed sequence of a registered learner is replayed and viewed against the final quizzes
    Then every implementation projects the same learner view and the same view of every committed run

  @id-site-catalog
  @level-fundamental
  @mode-differential
  Scenario: Perfect runs of the site catalog earn every badge and a single mistake withholds the badges of its task
    Given the site catalog 🎓️teaching/🏛️architecture/❓️quiz/🔣️.json and its quizzes, read from the repository
    And the committed plays of shared://🧾️learner-lifecycle/🔣️.json
    When every play starts, answers and submits its runs — perfectly, or with one mistake in the named task
    Then every implementation projects the same score per run, the same badges per submission and the same badges held at the end
    And a perfect tour holds every badge of the catalog, a flawed one every badge but those its task decides
