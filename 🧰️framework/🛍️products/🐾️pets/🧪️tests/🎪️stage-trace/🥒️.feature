@capability-pets-stage-trace
@no-oracle-pets-stage-trace
@comparison-ordered-json-v1
Feature: The stage folds the same events into the same frames
  The stage is the heart of the pets: `advance(menagerie, stage, events)` folds events into actors that idle,
  fidget, walk, hop, fall, land, sleep, glance, blink and meet each other on surveyed perches, and
  `frameOf(menagerie, stage)` projects what a render target draws (design §4.7, §5). It is a pure fold: the same
  seed and the same events yield the same frames, bit for bit, in every language (design §2.4).

  THERE IS NO REFERENCE. No third-party library simulates this model, and a second engine written to be compared
  with it would be the model again; the recorded no-oracle decision `pets-stage-trace` therefore rests on
  specification vectors, on laws and on the two cores agreeing with each other (`independent-implementations`):
  the TypeScript core and its Rust twin are compared projection for projection, so every checkpoint digest is a
  proof of the same IEEE-754 bit patterns in both. Every building block the stage calls (trigonometry, randomness,
  rig, animation, terrain, behaviour) has a third-party oracle in a case of its own.

  The vectors are scripted event logs over a self-contained menagerie of five blobs (a walker everybody likes, a
  hopper, a quarrelsome walker, a sleepy walker and a floater; friends, rivals and strangers): a calm home, a
  lively card on which friends and rivals meet, a card that scrolls, vanishes and comes back, a time of
  concentration, a still stage, a change of scene with a rotating cast, pokes and glances, a narrow stage, a floor
  that shrinks to a strip beside a task and widens again (whoever does not fit waits off stage), a card with a
  title tab on a floor with a footer line (pets step between the two levels), a pointer that rests on one
  place after another (whoever stands there stays whole and perks up; only a pet in the air, on a wall, a ladder or
  a rope in front of what the page keeps free turns see-through), a pointer that crosses behind the pets, comes to
  rest beside them and jitters from side to side (they turn round to it, lean after it, greet it, and never flip
  back and forth; a time of concentration leaves them be), and a company that gathers on the only edge there is
  and finds new ground with the next survey (it spreads out at once, but not in a time of concentration), and
  mischief on a page of a quiz whose task rows are marked for the pets (a ground of a blob covers a row's key): a
  learner who stays still while a blob climbs down beside the row of its topic, pushes the row's copy out of its stack
  and lets it slide home; a learner who takes the row back, so the blob is thrown off, glides down under its
  parachute and is sheepish; and a change of scene while the copy is out.
  A script is replayed tick by tick — the events of a tick are folded, then the tick passes — and every frame is
  digested: FNV-1a (32 bit) over the IEEE-754 bit patterns of every number of the frame (tick, rate, wake, and per
  actor its species index, where its rig is placed, facing, the index of its activity, opacity, every bone matrix,
  every eye, its spirits, the indices of its footing, state and mood, the intensity, tilt and pivot, every tool and
  its body; then every standing ladder, every particle, every lifted copy, every puff of dust and the index of the pet
  held), running on from frame to frame; a checkpoint also records how many particles, ladders, lifted copies and
  puffs its frame shows and who is held. The committed trace of a script holds that digest at a checkpoint every ten seconds, with the
  actors on stage, so a difference is found within ten seconds of where it began.

  The subjects are `@semio-tech/pets` (`openStage`, `advance`, `frameOf`) and the `pets` crate (`open_stage`,
  `advance`, `frame_of`); they hold themselves to the committed trace and the harness holds them to each other. The
  Python file of this case simulates nothing: it checks on the committed trace what can be checked without the
  stage (actors stand on surveyed surfaces and outside keep-outs, the recorded bodies of no two actors overlap, within
  the mode's number of movers, one pair at most, at most 160 particles and one lifted copy, nothing of the kind on a
  still stage, and only a pet in the learner's hand held).

  The vectors shared://🎪️stage-trace/🔣️.json are generated, never hand-edited, from the repository root:
  `.venv/Scripts/python.exe .🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️02/QUIZ-PETS/generate_behavior_vectors.py`
  (`.venv/bin/python` outside Windows; the traces are recorded from the TypeScript subject by
  `record_stage_trace.ts` beside it and refused when they break a law).

  @id-traces
  @level-fundamental
  @mode-differential
  Scenario: Every scripted event log replays into its committed trace in both cores
    Given the committed vectors shared://🎪️stage-trace/🔣️.json
    When every script is replayed tick by tick from an empty stage with its seed and every frame is digested
    Then every implementation projects the committed number of frames, the committed digest at every checkpoint and the committed actors, exactly

  @id-laws
  @level-fundamental
  @mode-property
  Scenario: Every tick of every script keeps the laws of the stage
    Given the committed vectors shared://🎪️stage-trace/🔣️.json
    When the laws are checked after every tick of every script
    Then actors on a perch stand on it outside every keep-out, the bodies of no two actors overlap wherever they are, no more actors walk or hop than the mode allows, at most one pair has partners, activities follow the activity graph and every frame is well-formed

  @id-determinism
  @level-quick
  @mode-property
  Scenario: The same seed and events yield the same frames however time is cut
    Given the committed vectors shared://🎪️stage-trace/🔣️.json
    When every script is replayed a second time, once more with the next seed, and once with its ticks passed in uneven chunks instead of one by one
    Then the second replay yields the same trace, the other seed another digest, and the chunked replay ends in the same stage
