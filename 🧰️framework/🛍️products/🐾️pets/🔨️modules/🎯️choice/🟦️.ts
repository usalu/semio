/** 🎯️ Choice: what an actor does next when its time is up — an idle one picks an activity by its weights and its mood, with its dwell, its clip and its goal, its target or its trick (`decide`); everything else ends the way its activity ends (`conclude`), a trick with the state and the mood it leaves; the beat of the stage on which moods travel and the chemistry between species is looked at (`beat`), which can set a pet off on something it would choose by itself; and the mischief of the stage (`mischief`): which pet plays with which marked element of the page, sent to its post beside it, pushing the element's copy out of its stack and letting it slide home, and thrown off when the learner takes the element back (`reclaimed`).
 *
 * A new activity an actor may choose by itself gets its branch in `decide`, and its end in `conclude`.
 * A part of the stage, not of the package: `@semio-tech/pets` exports nothing of it.
 *
 * @see ../🎪️stage/🟦️.ts — the façade of the stage and the normative order of a tick
 * @see ../💗️feeling/🟦️.ts — `moodWeights`, `whimTrick`, `stateAfterTrick`, `performed`, `reactionsOf`
 * @see ../🪄️mischief/🟦️.ts — the pick, the gates, the lift and the throw of mischief
 * @see ./🦀️.rs — the Rust twin
 */

import { ACTIVITIES, type Activity, type Feeling, type Menagerie, type Perch, type PetMode, type Slug, type Ticks } from "../../🧬️schema/🟦️.ts";
import { clipTicks } from "../🎞️animation/🟦️.ts";
import { CHEMISTRY_STREAM, GEAR_STREAM, MISCHIEF_STREAM, randomPick, randomWords, unitOf } from "../🎲️randomness/🟦️.ts";
import { perchAt } from "../🏞️terrain/🟦️.ts";
import { PURR_TICKS, evicted, loose, watched } from "../👀️attention/🟦️.ts";
import { CHEMISTRY_BEAT, CONTAGION_BEAT, drawsOf, drowsed, moodWeights, performed, reactionsOf, stateAfterTrick, stirred, trialsOf, tricksFor, whimTrick } from "../💗️feeling/🟦️.ts";
import { circled } from "../👆️gesture/🟦️.ts";
import { prankTick, prospectsOf, sightingsOf } from "../🗓️schedule/🟦️.ts";
import { contagion, nudge, pledge, reconcile, sulk } from "../💞️sociability/🟦️.ts";
import { COMFORT_GAP, clearway, roomsFor } from "../📏️spacing/🟦️.ts";
import { clamp } from "../📐️trigonometry/🟦️.ts";
import { actorKey, astir, clipAt, clipOf, enter, feel, indexOf, perform, present, purr, release, settle, shift, standing, stand, trickOf, type Draft, type Launch } from "../📝️draft/🟦️.ts";
import { daze, geared, hopsOf, launch, leave, paced, postTo, pushAt, reaches, setOut, stroll, unfoot, unpush, type Venture } from "../🚶️locomotion/🟦️.ts";
import { FRIENDS, MODE_LIMITS, activityWeights, affinityOf, dwellOf, needsAfter } from "../🧠️behavior/🟦️.ts";
import { gripFor, ladderTo } from "../🧗️climbing/🟦️.ts";
import { LIFT_BRACE, LIFT_RETURNS, LIFT_SHOVE, LIFT_TICKS, LIFT_WOBBLE, THROW_LIFT, THROW_SPEED, chosenFixture, thrownOff } from "../🪄️mischief/🟦️.ts";

//#region 🔖️Constants
const STROLL_LEAST = 0.75;
const VENTURE_SHARES: { readonly [mode in PetMode]: number } = { still: 0, calm: 0.25, lively: 0.6 };
const CLIMB_SHARE = 0.4;

/** 😳️ How strongly a pet the learner threw off its prank feels sheepish once it is down again: an impulse of `sad` at 0.3, as its species takes it — the nine shared moods have no sheepishness of their own, and sadness at that strength lowers its head and lids and bends its mouth a little for some eight seconds before it fades. */
export const SHEEPISH = 0.3;

/** 🫸️ For how many ticks a pet holds its post from the tick it arrives: the tick before its prank begins, the whole lift, and the tick on which the stage itself ends its push. */
export const PUSH_HOLD = LIFT_TICKS + 2;
//#endregion 🔖️Constants

//#region 🔖️Decision
/** 🧗️ Where an idle actor on `perch` that gets around with gear could set out for, in this order: the perch of every friend (affinity at least `FRIENDS`) that stands on another perch, the first friend first; while the stage is lively or company crowds its perch (`crowd` others on its surface) every other perch with room for it (`roomsFor`, more room than where it stands) — each only when its gear has a way there, in one leg or by a perch on the way (`reaches`, over the ladders that stand free) —; and then, with climbing gear, every wall line it takes hold of from its perch (`gripFor`) or, carrying a ladder too, from the exit of its own ladder raised against it (`ladderTo`), once per wall and side, for a rest at a spot the stage draws — the life of a pet in the gutters of a page whose cards leave no room on top. None for a species without gear. */
function venturesOf(draft: Draft, index: number, perch: Perch, crowd: number): Venture[] {
  const body = draft.actors[index]!;
  const kind = draft.kinds[index]!;
  if (!geared(kind)) return [];
  const ventures: Venture[] = [];
  const known = (there: Perch): boolean => there === perch || ventures.some((venture) => venture.kind === "perch" && venture.perch === there) || !reaches(draft, kind, body.x, perch, there, draft.tick);
  for (let other = 0; other < draft.actors.length; other++) {
    const friend = draft.actors[other]!;
    if (other === index || friend.leaving || friend.footing !== "perch" || friend.perch === null) continue;
    if (affinityOf(draft.menagerie, draft.rapports, body.species, friend.species) < FRIENDS) continue;
    const there = perchAt(draft.perches, friend.perch, friend.x);
    if (there !== null && !known(there)) ventures.push({ kind: "perch", perch: there });
  }
  const restless = draft.mode === "lively" || crowd > 0;
  if (restless) for (const room of roomsFor(draft, kind)) if (!known(room.perch)) ventures.push({ kind: "perch", perch: room.perch });
  if (!restless || !kind.gear.includes("climb")) return ventures;
  const leans = kind.gear.includes("ladder");
  for (const pitch of draft.pitches) {
    if ((gripFor(perch, pitch, kind.size) === null && (!leans || ladderTo(perch, pitch, draft.keepouts, kind.size) === null)) || ventures.some((venture) => venture.kind === "wall" && venture.pitch.wall === pitch.wall && venture.pitch.side === pitch.side)) continue;
    ventures.push({ kind: "wall", pitch, y: null });
  }
  return ventures;
}

/** 🧮️ What an idle actor could choose at tick `now`, without changing anything: the feeling it decides in (`felt`, and `feeling` once it has grown as sleepy as its needs settled to `now` make it, `drowsed`), the state it stands in, the clear way of its perch (`low`, `high`), its launches, whether another perch has room for it (`elsewhere`) and where its gear could take it (`ventures`, {@link venturesOf}), and the weight of every activity — `activityWeights` times the multiplier of the mood it feels (`moodWeights`), a trick on a whim weighed while its species offers one in its state and mood. Whoever walks, hops or is on a trip with its gear moves (`astir`). While the pointer circles it (`circled`) it neither walks nor hops nor sets out with its gear: the circle holds its attention. */
function optionsOf(draft: Draft, index: number, now: Ticks): { readonly felt: Feeling; readonly feeling: Feeling; readonly state: Slug; readonly low: number; readonly high: number; readonly launches: readonly Launch[]; readonly elsewhere: boolean; readonly ventures: readonly Venture[]; readonly weights: readonly number[] } {
  const body = draft.actors[index]!;
  const kind = draft.kinds[index]!;
  const limits = MODE_LIMITS[draft.mode];
  const needs = needsAfter(body.needs, body.activity, now - body.since, kind.temperament);
  const felt = present(draft, index, now);
  const feeling = drowsed(felt, needs.energy, now);
  const state = standing(draft, index, now).state;
  let movers = 0;
  let fidgeters = 0;
  let crowd = 0;
  for (let other = 0; other < draft.actors.length; other++) {
    if (other === index) continue;
    const activity = draft.actors[other]!.activity;
    if (astir(draft, draft.actors[other]!)) movers++;
    if (activity === "fidget") fidgeters++;
    if (body.footing === "perch" && body.perch !== null && draft.actors[other]!.footing === "perch" && draft.actors[other]!.perch === body.perch && !draft.actors[other]!.leaving) crowd++;
  }
  const perch = body.footing !== "perch" || body.perch === null ? null : perchAt(draft.perches, body.perch, body.x);
  const [low, high] = clearway(draft, index, COMFORT_GAP, -1);
  const held = circled(body.hover, now);
  const restless = !draft.quiet && movers < limits.movers && limits.hop > 0 && perch !== null && !held;
  const launches = restless ? hopsOf(draft, index) : [];
  const elsewhere = restless && launches.length === 0 && roomsFor(draft, kind).some((room) => room.perch !== perch);
  const ventures = restless ? venturesOf(draft, index, perch, crowd) : [];
  const whims = tricksFor(kind, "whim", state, feeling).length > 0;
  const wishes = moodWeights(feeling.mood, feeling.intensity);
  const weights = activityWeights({ ...body, needs }, kind, { mode: draft.mode, quiet: draft.quiet, movers, fidgeters, roam: !held && Math.max(body.x - low, high - body.x) >= STROLL_LEAST * kind.size.width, hops: launches.length > 0 || elsewhere || ventures.length > 0, crowd, watched: watched(draft.pointer, body, kind), whims }).map((weight, at) => weight * wishes[at]!);
  return { felt, feeling, state, low, high, launches, elsewhere, ventures, weights };
}

/** 🙋️ An idle actor decides what to do next ({@link optionsOf}): it grows as sleepy as it is tired, then one `randomPick` over the weights picks the activity — `forced` names it instead (what the chemistry set it off on), and only when the actor could choose it by itself right now (its weight is above 0; else nothing changes) —, then its dwell, its clip and, for a walk, a goal on the clear way of its perch (within the stroll of the mode, at least three quarters of a body away — a hopping gait at least one whole hop —, never past or into anybody; without such a goal it stays idle instead) — in a lively stage a pet that could rest on a wall or go up to a higher perch with its gear ({@link venturesOf}) sets out for one of these instead of that walk `CLIMB_SHARE` of the time — an explorer of the page (the first of five words on the gear stream, the second picks the wall line or the perch, the rest the spot; a trip that is not clear leaves it to its walk) —, for a hop one of its launches, and for a trick one of its whim tricks (picked with the clip word). An actor that feels like a hop and could set out with its gear ({@link venturesOf}) draws five words on the gear stream (`[seed, GEAR_STREAM, its stream, the counter of this decision]`, so no other draw moves): when no launch is in reach, or when the first is below the share of its mode (`VENTURE_SHARES`: a quarter calm, three in five lively), it sets out (`setOut` with the second as the pick and the rest for the way); a trip that is not clear leaves it to its hop. An actor that feels like a hop where no perch is in reach wanders off instead when another perch has room for it: it leaves, and arrives anew a moment later — unless the hop was forced or no other perch has room: then it stays. */
function decide(draft: Draft, index: number, now: Ticks, forced: Activity | null): void {
  const body = draft.actors[index]!;
  const kind = draft.kinds[index]!;
  const limits = MODE_LIMITS[draft.mode];
  const width = kind.size.width;
  const { felt, feeling, state, low, high, launches, elsewhere, ventures, weights } = optionsOf(draft, index, now);
  if (forced !== null && !(weights[ACTIVITIES.indexOf(forced)]! > 0)) return;
  shift(draft, index, "idle", now);
  if (feeling !== felt) body.feeling = feeling;
  const key = actorKey(draft, index);
  const pick = randomPick(key, weights);
  const words = randomWords(key, 4);
  const activity = forced ?? (pick < 0 ? "idle" : ACTIVITIES[pick]!);
  const dwell = unitOf(words[1]!);
  const clip = clipAt(kind, activity, unitOf(words[2]!));
  const place = unitOf(words[3]!);
  const walls = activity === "walk" && draft.mode === "lively" ? ventures.filter((venture) => venture.kind === "wall" || venture.perch.y < body.y) : [];
  if (walls.length > 0) {
    const gear = randomWords([draft.seed, GEAR_STREAM, draft.streams[index]!, key[2]!], 5).map(unitOf);
    if (gear[0]! < CLIMB_SHARE && setOut(draft, index, walls, gear[1]!, gear.slice(2), now)) return;
  }
  if (activity === "walk") {
    const reach = limits.stroll * width;
    const least = STROLL_LEAST * width;
    let goal = clamp(low + (high - low) * place, body.x - reach, body.x + reach);
    if (Math.abs(goal - body.x) < least) goal = body.x - low > high - body.x ? body.x - least : body.x + least;
    goal = paced(kind, clip, body.x, clamp(goal, low, high));
    if (kind.locomotion.gait === "hop" ? goal !== body.x : Math.abs(goal - body.x) >= least) {
      stroll(draft, index, goal, clip, now);
      return;
    }
    body.clip = clipAt(kind, "idle", unitOf(words[2]!));
    body.until = now + dwellOf("idle", draft.mode, dwell);
    return;
  }
  if (activity === "trick") {
    const trick = whimTrick(kind, state, feeling, unitOf(words[2]!));
    if (trick !== null) {
      perform(draft, index, trick, now);
      return;
    }
    body.clip = clipAt(kind, "idle", unitOf(words[2]!));
    body.until = now + dwellOf("idle", draft.mode, dwell);
    return;
  }
  if (activity === "hop") {
    if (ventures.length > 0) {
      const gear = randomWords([draft.seed, GEAR_STREAM, draft.streams[index]!, key[2]!], 5).map(unitOf);
      if ((launches.length === 0 || gear[0]! < VENTURE_SHARES[draft.mode]) && setOut(draft, index, ventures, gear[1]!, gear.slice(2), now)) return;
    }
    if (launches.length === 0 && (forced !== null || !elsewhere)) {
      body.clip = clipAt(kind, "idle", unitOf(words[2]!));
      body.until = now + dwellOf("idle", draft.mode, dwell);
      return;
    }
    if (launches.length === 0) {
      leave(draft, index, now);
      return;
    }
    const chosen = Math.floor(place * launches.length);
    if (launch(draft, index, launches[chosen < launches.length ? chosen : launches.length - 1]!, clip, now)) return;
    body.clip = clipAt(kind, "idle", unitOf(words[2]!));
    body.until = now + dwellOf("idle", draft.mode, dwell);
    return;
  }
  body.activity = activity;
  body.clip = clip;
  const played = clipOf(kind, clip);
  body.until = now + (activity === "fidget" && played !== null && !played.loop ? clipTicks(played) : dwellOf(activity, draft.mode, dwell));
}

/** 🏁️ The end of what an actor does when its time is up: an idle one decides (or gives up waiting for its partner), a squabbler sulks, a sulker reconciles, partners that cuddled purr on by themselves, a trick that has played out leaves its state and its mood ({@link finish}), and everyone else comes to rest. Walks, hops and falls end by their motion. */
export function conclude(draft: Draft, index: number, now: Ticks): void {
  const body = draft.actors[index]!;
  const activity = body.activity;
  if (body.footing !== "perch" || activity === "walk" || activity === "hop" || activity === "fall" || activity === "scoot") return;
  if (activity === "land" && body.vy > 0) {
    daze(draft, index, now);
    return;
  }
  if (activity === "idle" && body.partner === null) {
    decide(draft, index, now, null);
    return;
  }
  if (activity === "squabble" && body.partner !== null) {
    sulk(draft, index, now);
    return;
  }
  if (activity === "cuddle" && body.partner !== null) {
    body.partner = null;
    purr(draft, index, PURR_TICKS, now);
    return;
  }
  if (activity === "trick") finish(draft, index, now);
  if (activity === "idle") release(draft, index, now);
  if (activity === "sulk") reconcile(draft, index, now);
  settle(draft, index, now);
}

/** 🎬️ A trick has played out at tick `now`: the actor enters the state the trick leaves it in (`stateAfterTrick` — its `to`, or a step along its rungs for a circling trick; a trick that names `to` begins that state anew) and feels as the trick leaves it (`performed`). */
function finish(draft: Draft, index: number, now: Ticks): void {
  const body = draft.actors[index]!;
  const kind = draft.kinds[index]!;
  const trick = trickOf(kind, body.trick);
  if (trick === null) return;
  stand(draft, index, now);
  const after = stateAfterTrick(kind, body.state, trick);
  if (after !== body.state || trick.to !== undefined) enter(draft, index, after, now, now);
  body.feeling = performed(present(draft, index, now), trick, kind, now);
}
//#endregion 🔖️Decision

//#region 🔖️Beat
/** 🥁️ The beat of the stage at tick `now`: on every {@link CONTAGION_BEAT} moods travel between neighbours (`contagion`); on every {@link CHEMISTRY_BEAT}, outside a time of concentration, the chemistry of the menagerie is looked at ({@link react}). */
export function beat(menagerie: Menagerie, draft: Draft, now: Ticks): void {
  if (now % CONTAGION_BEAT === 0) contagion(menagerie, draft, now);
  if (now % CHEMISTRY_BEAT === 0 && !draft.quiet && menagerie.chemistry.length > 0 && draft.actors.length > 1) react(menagerie, draft, now);
}

/** ⚗️ One beat of chemistry at tick `now`: every actor that stays is sighted as it stands (its state and how long it has held it, the mood it feels, its activity and trick, the box of its species at its feet); the reactions that are due draw their chances from the chemistry stream (`[seed, CHEMISTRY_STREAM, now ÷ CHEMISTRY_BEAT]`, so no other draw moves) and their consequences are applied in order: a state is entered (begun anew when it is the one it is in), a mood is stirred as the species takes it, the rapport of the pair is nudged, an encounter is promised, a trick is performed by an actor that is free for it, and an idle actor without a partner decides at once on the activity it was set off on. */
function react(menagerie: Menagerie, draft: Draft, now: Ticks): void {
  const sightings = sightingsOf(menagerie, draft, now);
  const trials = trialsOf(menagerie, sightings, draft.coolings, now, draft.rapports);
  if (trials.length === 0 && draft.coolings.every((cooling) => cooling.until > now)) return;
  const draws = drawsOf(trials);
  const units = draws === 0 ? [] : randomWords([draft.seed, CHEMISTRY_STREAM, now / CHEMISTRY_BEAT], draws).map(unitOf);
  const outcome = reactionsOf(menagerie, sightings, now, draft.coolings, units, draft.rapports);
  draft.coolings = outcome.coolings;
  for (const consequence of outcome.consequences) {
    const index = indexOf(draft.actors, consequence.on);
    const other = indexOf(draft.actors, consequence.other);
    if (index < 0) continue;
    const body = draft.actors[index]!;
    const kind = draft.kinds[index]!;
    if (consequence.state !== null) enter(draft, index, consequence.state, now, now);
    if (consequence.mood !== null) body.feeling = stirred(present(draft, index, now), consequence.mood, consequence.amount, kind, now);
    if (consequence.rapport !== 0 && other >= 0) nudge(draft, index, other, consequence.rapport, now);
    if (consequence.encounter !== null) pledge(draft, consequence.on, consequence.other, consequence.encounter, now);
    const trick = trickOf(kind, consequence.trick);
    if (trick !== null && loose(body)) perform(draft, index, trick, now);
    if (consequence.activity !== null && body.activity === "idle" && loose(body)) decide(draft, index, now, consequence.activity);
  }
}
//#endregion 🔖️Beat

//#region 🔖️Mischief
/** 🃏️ The stage picks a pet and a marked element of the page to play with at tick `now` and sends the pet to its post beside it. Among the prospects (`prospectsOf`: idle pets on a perch whose grounds cover the element's key and that have a post beside it, in menagerie and survey order) a unit draw on the mischief stream (`[seed, MISCHIEF_STREAM, now, 0]`, word 0) points at one (`chosenFixture`); when it cannot get there the next ones are tried, round and round — first every one by its gear or on foot, and only when none can go that way every one again, now slipping over in a puff as the last resort (`postTo`; its post held for {@link PUSH_HOLD} ticks). The first that goes is the pusher of the prank (`lift`), which begins the tick after it arrives, with word 1 for how far it will shove the copy, the way and the room of its post and the width of the element. When nobody can go, the stage waits the mode's cooldown (`rested`) before it looks again. Of the element only its key and its box are read, so the pick cannot follow an answer (rule 7). */
function prank(menagerie: Menagerie, draft: Draft, now: Ticks): void {
  const prospects = prospectsOf(menagerie, draft, now);
  const count = prospects.length;
  if (count === 0) return;
  const units = randomWords([draft.seed, MISCHIEF_STREAM, now, 0], 2).map(unitOf);
  const first = prospects.indexOf(chosenFixture(prospects, units[0]!)!);
  for (const slip of [false, true]) {
    for (let turn = 0; turn < count; turn++) {
      const prospect = prospects[(first + turn) % count]!;
      const arrives = postTo(draft, indexOf(draft.actors, prospect.actor), prospect.post, prospect.side, PUSH_HOLD, slip, now);
      if (arrives === null) continue;
      draft.lift = { fixture: prospect.fixture.id, pusher: prospect.actor, since: arrives + 1, side: prospect.side, room: prospect.room, span: prospect.fixture.width, unit: units[1]! };
      return;
    }
  }
  draft.rested = now;
}

/** 🙈️ The mischief of the stage at tick `now`, after everybody's turn, whenever `prankTick` says it is due:
 * - without a prank the stage picks one ({@link prank});
 * - the tick a prank begins its pet starts to push at its post (`pushAt`, until the tick after the lift) — when it is not ready there, or its way there ended early, or it is gone or leaving, the prank is off and the cooldown counts;
 * - a pusher that stops pushing while the copy is out (picked up, its wall gone, summoned away) leaves the copy to vanish at once while it is still on its way out, and to slide home by itself once it rests there (the lift is moved on to the beginning of its way back);
 * - when the lift is over the pusher stops pushing, playful (`tricked`), and the cooldown counts;
 * - a pusher the learner threw off ({@link reclaimed}) feels sheepish ({@link SHEEPISH}) once it is down again and its fright has passed, and its prank is over; one that is gone ends it too.
 */
export function mischief(menagerie: Menagerie, draft: Draft, now: Ticks): void {
  if (prankTick(menagerie, draft, now) !== now) return;
  const lift = draft.lift;
  if (lift === null) {
    prank(menagerie, draft, now);
    return;
  }
  const index = indexOf(draft.actors, lift.pusher);
  const age = now - lift.since;
  if (age > LIFT_TICKS) {
    draft.lift = null;
    if (index >= 0) draft.actors[index]!.feeling = stirred(present(draft, index, now), "sad", SHEEPISH, draft.kinds[index]!, now);
    return;
  }
  if (age === LIFT_TICKS) {
    draft.lift = null;
    draft.rested = now;
    if (index < 0 || draft.actors[index]!.activity !== "push") return;
    unpush(draft, index, now);
    feel(draft, index, "tricked", now);
    return;
  }
  if (age === 0 && index >= 0 && pushAt(draft, index, lift.side, now + LIFT_TICKS + 1, now)) return;
  if (age >= LIFT_BRACE + LIFT_SHOVE + LIFT_WOBBLE) {
    draft.lift = { fixture: lift.fixture, pusher: lift.pusher, since: now - LIFT_RETURNS, side: lift.side, room: lift.room, span: lift.span, unit: lift.unit };
    if (index >= 0) unpush(draft, index, now);
    return;
  }
  draft.lift = null;
  draft.rested = now;
  if (index >= 0) unpush(draft, index, now);
}

/** 🫱️ The learner took the fixture `fixture` back at tick `now` (the render target restored the element at once): when that is the fixture whose copy is out right now, the copy is gone and the cooldown counts from now; a pusher that still pushes is thrown off — away from the middle of the element and up (`thrownOff`, faster by word 0 of `[seed, MISCHIEF_STREAM, now, 1]`; backwards when the element is no longer surveyed), tumbling, its parachute opening where the landing would be hard, frightened (`evicted`) — and keeps its prank until it is down again (the lift is moved so far back that the copy is over). Any other fixture, and the fixture before or after its copy is out, changes nothing. */
export function reclaimed(draft: Draft, fixture: string, now: Ticks): void {
  const lift = draft.lift;
  if (lift === null || lift.fixture !== fixture || now < lift.since || now - lift.since >= LIFT_TICKS) return;
  draft.rested = now;
  const index = indexOf(draft.actors, lift.pusher);
  if (index < 0 || draft.actors[index]!.activity !== "push") {
    draft.lift = null;
    return;
  }
  const body = draft.actors[index]!;
  const box = draft.fixtures.find((entry) => entry.id === fixture);
  const toss = box === undefined ? { vx: 0 - lift.side * THROW_SPEED, vy: 0 - THROW_LIFT } : thrownOff({ x: body.x, y: body.y }, box, unitOf(randomWords([draft.seed, MISCHIEF_STREAM, now, 1], 1)[0]!));
  draft.lift = { fixture: lift.fixture, pusher: lift.pusher, since: now - LIFT_TICKS - 1, side: lift.side, room: lift.room, span: lift.span, unit: lift.unit };
  evicted(draft, index, now);
  unfoot(draft, index, toss.vx, toss.vy, "tumble", now);
}
//#endregion 🔖️Mischief
