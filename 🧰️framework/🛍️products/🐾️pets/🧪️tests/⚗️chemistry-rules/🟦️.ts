/** ⚗️ Subject adapter of the chemistry-rules case: the pets feeling module answers every committed vector on the states, tricks and reactions of the committed menagerie, and judges the sample menagerie of the product as it stands.
 *
 * @see ./🥒️.feature
 * @see ../../🔨️modules/💗️feeling/🟦️.ts
 */
import { type AdapterContext, defineTestAdapter } from "../../../../🔨️modules/🧪️test/🔌️adapter/🟦️.ts";
import { CUES, type Cooling, type Feeling, type Menagerie, type Rapport, type Reaction, type Slug, type Species, type Trait } from "../../🧬️schema/🟦️.ts";
import { type Chemistry, type Sighting, atRest, clickTrick, heldTicks, ladderOf, lastingTicks, nearby, reactionsOf, rungsOf, seen, showTrick, stateAfterTrick, stateAt, stateEnds, stepState, tricksFor, trialsOf, whimTrick } from "../../🔨️modules/💗️feeling/🟦️.ts";

const VECTORS = "shared://⚗️chemistry-rules/🔣️.json";
const SAMPLE = "shared://🧬️schema-conformance/🔣️.json";

type Vectors = {
  readonly menagerie: Menagerie;
  readonly relations: readonly { readonly id: string; readonly first: Sighting; readonly second: Sighting; readonly reaches: readonly number[] }[];
  readonly states: readonly { readonly id: string; readonly species: Slug; readonly state: Slug; readonly since: number; readonly ticks: readonly number[] }[];
  readonly ladders: readonly { readonly id: Slug }[];
  readonly tricks: readonly { readonly id: Slug; readonly feelings: readonly (Feeling & { readonly id: string })[]; readonly clicks: readonly number[]; readonly units: readonly number[] }[];
  readonly reachability: readonly { readonly id: Slug }[];
  readonly matching: readonly { readonly id: string; readonly sightings: readonly Sighting[]; readonly coolings: readonly Cooling[]; readonly tick: number; readonly rapports: readonly Rapport[] }[];
  readonly reactions: readonly { readonly id: string; readonly beats: readonly { readonly tick: number; readonly sightings: readonly Sighting[]; readonly units: readonly number[]; readonly rapports: readonly Rapport[] }[] }[];
};

/** 🧫️ The committed vectors. */
function vectors(ctx: AdapterContext): Vectors {
  return JSON.parse(new TextDecoder().decode(ctx.inputBytes(VECTORS))) as Vectors;
}

/** 🧬️ One species of the menagerie the vectors carry. */
function kindOf(document: Vectors, id: Slug): Species {
  return document.menagerie.species.find((species) => species.id === id)!;
}

/** 🔭️ What the first of two bodies is to the second. */
function relation(vector: Vectors["relations"][number]): unknown {
  return { nearby: vector.reaches.map((reach) => nearby(vector.first, vector.second, reach)), above: seen(vector.first, vector.second, "above"), below: seen(vector.first, vector.second, "below"), beside: seen(vector.first, vector.second, "beside"), any: seen(vector.first, vector.second, "any") };
}

/** 🕰️ The state of a species at every committed tick, when it gives way and how long it lasts. */
function standings(document: Vectors, vector: Vectors["states"][number]): unknown {
  const species = kindOf(document, vector.species);
  const state = species.states.find((entry) => entry.id === vector.state);
  return { standings: vector.ticks.map((tick) => stateAt(species, vector.state, vector.since, tick)), ends: stateEnds(species, vector.state, vector.since), lasts: state === undefined ? 0 : lastingTicks(state) };
}

/** 🪜️ The ladder of a species, the rungs of every trick, every step and the state every trick leaves. */
function ladder(species: Species): unknown {
  const names = ladderOf(species);
  return {
    ladder: names,
    rungs: Object.fromEntries(species.tricks.map((trick) => [trick.id, rungsOf(species, trick)])),
    steps: Object.fromEntries([...names, "nowhere"].map((name) => [name, { up: stepState(species, name, 1), down: stepState(species, name, -1), stay: stepState(species, name, 0) }])),
    after: Object.fromEntries(species.tricks.map((trick) => [trick.id, Object.fromEntries(names.map((name) => [name, stateAfterTrick(species, name, trick)]))])),
  };
}

/** 🎪️ The tricks on offer per cue, state and feeling, and the trick of every committed click and draw. */
function offers(species: Species, vector: Vectors["tricks"][number]): unknown {
  const names = ladderOf(species);
  const calm = atRest("content", 0);
  return {
    offers: Object.fromEntries(CUES.map((cue) => [cue, Object.fromEntries(names.map((name) => [name, Object.fromEntries(vector.feelings.map((feeling) => [feeling.id, tricksFor(species, cue, name, feeling).map((trick) => trick.id)]))]))])),
    clicks: Object.fromEntries(names.map((name) => [name, vector.clicks.map((index) => clickTrick(species, name, calm, index)?.id ?? null)])),
    whims: Object.fromEntries(names.map((name) => [name, Object.fromEntries(vector.feelings.map((feeling) => [feeling.id, vector.units.map((unit) => whimTrick(species, name, feeling, unit)?.id ?? null)]))])),
    shows: Object.fromEntries(names.map((name) => [name, Object.fromEntries(vector.feelings.map((feeling) => [feeling.id, vector.units.map((unit) => showTrick(species, name, feeling, unit)?.id ?? null)]))])),
  };
}

/** 🕸️ Every way from one state of a species into another — by a trick on offer there, by time, by a reaction — and what a breadth-first walk reaches from the resting state. */
function reach(chemistry: readonly Reaction[], species: Species): { edges: [Slug, Slug][]; reachable: Slug[]; unreachable: Slug[] } {
  const names = ladderOf(species);
  const edges: [Slug, Slug][] = [];
  const add = (start: Slug, end: Slug): void => {
    if (start !== end && !edges.some((edge) => edge[0] === start && edge[1] === end)) edges.push([start, end]);
  };
  for (const trick of species.tricks) for (const name of names) if (trick.from === undefined || trick.from.includes(name)) add(name, stateAfterTrick(species, name, trick));
  for (const state of species.states) if (stateEnds(species, state.id, 0) !== null) add(state.id, stateAt(species, state.id, 0, lastingTicks(state)).state);
  for (const reaction of chemistry) {
    for (const effect of reaction.then) {
      const trait: Trait = reaction[effect.on];
      if ((trait.species !== undefined && trait.species !== species.id) || effect.state === undefined || !names.includes(effect.state)) continue;
      for (const name of trait.state === undefined ? names : [trait.state]) if (names.includes(name)) add(name, effect.state);
    }
  }
  edges.sort((left, right) => names.indexOf(left[0]) - names.indexOf(right[0]) || names.indexOf(left[1]) - names.indexOf(right[1]));
  const reached = [names[0]!];
  for (let index = 0; index < reached.length; index++) for (const edge of edges) if (edge[0] === reached[index] && !reached.includes(edge[1])) reached.push(edge[1]);
  return { edges, reachable: names.filter((name) => reached.includes(name)), unreachable: names.filter((name) => !reached.includes(name)) };
}

/** 🔥️ A story beat by beat, the coolings carried from one beat to the next. */
function story(menagerie: Menagerie, vector: Vectors["reactions"][number]): Chemistry[] {
  const beats: Chemistry[] = [];
  let coolings: readonly Cooling[] = [];
  for (const beat of vector.beats) {
    const outcome = reactionsOf(menagerie, beat.sightings, beat.tick, coolings, beat.units, beat.rapports);
    coolings = outcome.coolings;
    beats.push(outcome);
  }
  return beats;
}

/** 🎟️ The species that stand for the two sides of a reaction in a rehearsal: the one a side names, else the first of `names` that is not the other side's. */
function cast(reaction: Reaction, names: readonly Slug[]): [Slug | undefined, Slug | undefined] {
  const when = reaction.when.species ?? names.find((name) => name !== reaction.near.species);
  const near = reaction.near.species ?? names.find((name) => name !== when);
  return [when, near];
}

/** 🎬️ Two actors of the species `first` and `second` that stand as a reaction asks: each in the state its trait names (its resting state otherwise), held as long as the trait asks, in the mood, activity and trick it names (its resting mood, no trick and idle otherwise — performing when it names a trick), the first above, below or beside the second with half the reach between their bodies. */
function staged(menagerie: Menagerie, reaction: Reaction, first: Slug, second: Slug): Sighting[] {
  const actor = (trait: Trait, species: Slug): Sighting => {
    const kind = menagerie.species.find((entry) => entry.id === species)!;
    return { species: kind.id, state: trait.state ?? kind.states[0]!.id, held: trait.held === undefined ? 0 : heldTicks(trait.held), mood: trait.mood ?? kind.mood, intensity: 0.5, activity: trait.activity ?? (trait.trick === undefined ? "idle" : "trick"), trick: trait.trick ?? null, x: 400, y: 400, width: kind.size.width, height: kind.size.height };
  };
  const one = actor(reaction.when, first);
  const other = actor(reaction.near, second);
  return placed(reaction, one, other);
}

/** 📍️ The first of two staged actors moved to where a reaction wants it from the second. */
function placed(reaction: Reaction, first: Sighting, second: Sighting): Sighting[] {
  const gap = reaction.within / 2;
  const where = reaction.where ?? "any";
  if (where === "above") return [{ ...first, y: second.y - second.height - gap }, second];
  if (where === "below") return [{ ...first, y: second.y + first.height + gap }, second];
  return [{ ...first, x: second.x + (first.width + second.width) / 2 + gap }, second];
}

/** 🎭️ What a whole menagerie says about itself: per species its ladder and what can be reached, per reaction what happens when two of its species stand as it asks (a side that names no species is played by the first species that the other side is not) with their authored bond and every draw lucky. Species that carry no states, tricks or mood yet are passed over. */
function rehearsal(document: { readonly species?: readonly Partial<Species>[]; readonly bonds?: Menagerie["bonds"]; readonly chemistry?: readonly Reaction[] }): unknown {
  const chemistry = document.chemistry ?? [];
  const kinds = (document.species ?? []).filter((species): species is Species => species.states !== undefined && species.states.length > 0 && species.tricks !== undefined && species.mood !== undefined);
  const names = kinds.map((species) => species.id);
  const menagerie = { species: document.species ?? [], bonds: document.bonds ?? [], chemistry } as unknown as Menagerie;
  const plays: Record<string, unknown> = {};
  for (const reaction of chemistry) {
    const [first, second] = cast(reaction, names);
    if (first === undefined || second === undefined || !names.includes(first) || !names.includes(second) || first === second) continue;
    const sightings = staged(menagerie, reaction, first, second);
    plays[reaction.id] = { due: trialsOf(menagerie, sightings, [], 0, []).map((trial) => chemistry[trial.reaction]!.id), beat: reactionsOf(menagerie, sightings, 0, [], chemistry.map(() => 0), []) };
  }
  return { species: Object.fromEntries(kinds.map((species) => [species.id, { ladder: ladderOf(species), ...reach(chemistry, species) }])), reactions: plays };
}

export default defineTestAdapter({
  implementation: "typescript",
  scenarios: {
    relations: { subject: (ctx) => ({ projection: Object.fromEntries(vectors(ctx).relations.map((vector) => [vector.id, relation(vector)])) }) },
    states: {
      subject: (ctx) => {
        const document = vectors(ctx);
        return { projection: Object.fromEntries(document.states.map((vector) => [vector.id, standings(document, vector)])) };
      },
    },
    ladders: {
      subject: (ctx) => {
        const document = vectors(ctx);
        return { projection: Object.fromEntries(document.ladders.map((vector) => [vector.id, ladder(kindOf(document, vector.id))])) };
      },
    },
    tricks: {
      subject: (ctx) => {
        const document = vectors(ctx);
        return { projection: Object.fromEntries(document.tricks.map((vector) => [vector.id, offers(kindOf(document, vector.id), vector)])) };
      },
    },
    reachability: {
      subject: (ctx) => {
        const document = vectors(ctx);
        return { projection: Object.fromEntries(document.reachability.map((vector) => [vector.id, reach(document.menagerie.chemistry, kindOf(document, vector.id))])) };
      },
    },
    matching: {
      subject: (ctx) => {
        const document = vectors(ctx);
        return { projection: Object.fromEntries(document.matching.map((vector) => [vector.id, trialsOf(document.menagerie, vector.sightings, vector.coolings, vector.tick, vector.rapports)])) };
      },
    },
    reactions: {
      subject: (ctx) => {
        const document = vectors(ctx);
        return { projection: Object.fromEntries(document.reactions.map((vector) => [vector.id, story(document.menagerie, vector)])) };
      },
    },
    sample: { subject: (ctx) => ({ projection: { sample: rehearsal((JSON.parse(new TextDecoder().decode(ctx.inputBytes(SAMPLE))) as { menagerie: { species?: readonly Partial<Species>[]; bonds?: Menagerie["bonds"]; chemistry?: readonly Reaction[] } }).menagerie) } }) },
  },
});
