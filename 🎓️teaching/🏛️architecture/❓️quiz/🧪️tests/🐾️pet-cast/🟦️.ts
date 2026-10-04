/** 🐾️ The pets of the site fit its quizzes. Every species document and the ensemble of the architecture menagerie are
 * accepted by the draft-07 contract of the pets product through a third-party validator (ajv) and by the product's own
 * validators, and the menagerie the site ships is the one those documents assemble to, without an issue. Every ground a
 * species names exists in the quiz files, every species in the cast of a quiz is grounded in that quiz, every quiz of
 * the catalog has a cast, the home screen has one that knows every species, and every bond joins two of them.
 *
 * What the pets become and do holds together as well: every state, trick and purr shows only clips, particles and
 * states its species has; every state can be reached from the resting state and leads back to it — through the tricks
 * a cue sets off, the states that run out and the chemistry —, circling a pet one way or the other changes its resting
 * state, and every pet has the clips of the learner's hand and of the gear it owns. The chemistry implements the rules
 * of the brief it can express, each reaction naming its rule, with sides and effects that exist and can happen, at
 * distances and periods that make sense, and gives every scene something to react to.
 * @see ../../../🐾️pets/🔣️.json — the ensemble under test
 * @see ../../../🐾️pets/🟦️.ts — the menagerie the site hands to the quiz
 * @see ../../../🐾️pets/README.md — the states, tricks, gear and chemistry in words
 * @see ../../🔣️.json — the catalog whose quizzes ground the species
 * @see ../../../../../🧰️framework/🛍️products/🐾️pets/🧬️schema/🔣️.json — the contract ajv validates against */
import { assembleMenagerie, ensembleIssues, menagerieIssues, speciesIssues, stateAfterTrick, type Cast, type Ensemble, type Reaction, type Species, type Trait, type Trick } from "@semio-tech/pets";
import Ajv from "ajv";
import { readFileSync, readdirSync, statSync } from "node:fs";
import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import { describe, expect, it } from "vitest";
import * as architecturePets from "../../../🐾️pets/🟦️.ts";

const siteRoot = resolve(dirname(fileURLToPath(import.meta.url)), "../..");
const petsRoot = resolve(siteRoot, "../🐾️pets");
const ensemblePath = resolve(petsRoot, "🔣️.json");
const catalogPath = resolve(siteRoot, "🔣️.json");
const schemaPath = resolve(siteRoot, "../../../🧰️framework/🛍️products/🐾️pets/🧬️schema/🔣️.json");
const read = (path: string): unknown => JSON.parse(readFileSync(path, "utf8"));

const ensemble = read(ensemblePath) as Ensemble;
const documents = ensemble.species.map((path) => ({ path, species: read(resolve(petsRoot, path)) as Species }));
const menagerie = assembleMenagerie(
  ensemble,
  documents.map(({ species }) => species),
);
const ids = menagerie.species.map((species) => species.id);

const catalog = read(catalogPath) as { readonly quizzes: readonly string[] };
const quizzes = catalog.quizzes.map((path) => read(resolve(siteRoot, path)) as { readonly id: string; readonly tasks: readonly { readonly id: string; readonly items: readonly { readonly id: string }[] }[] });
const groundable = new Set(quizzes.flatMap((quiz) => [quiz.id, ...quiz.tasks.flatMap((task) => [`${quiz.id}/${task.id}`, ...task.items.map((item) => `${quiz.id}/${task.id}/${item.id}`)])]));

const schema = read(schemaPath) as { readonly $id: string };
const ajv = new Ajv({ strict: false, allErrors: true });
ajv.addSchema(schema);
const validSpecies = ajv.compile({ $ref: `${schema.$id}#/$defs/Species` });
const validEnsemble = ajv.compile({ $ref: `${schema.$id}#/$defs/Ensemble` });
const validMenagerie = ajv.compile({ $ref: `${schema.$id}#/$defs/Menagerie` });

/** 🌱️ The nine pets the owner named, in the owner's order: the roster starts with them and the home screen shows them
 * first. */
const SEEDS = ["sunny", "cloudy", "housy", "solary", "radiatory", "pumpy", "windowy", "waly", "battery"];

/** 🏡️ The scene of every screen that belongs to no quiz. */
const HOME = "home";

describe("architecture menagerie documents", () => {
  it("is the ensemble of the architecture pets and names twenty species, the owner's nine first", () => {
    expect(ensemble.id).toBe("architecture");
    expect(ids).toHaveLength(20);
    expect(new Set(ids).size).toBe(ids.length);
    expect(ids.slice(0, SEEDS.length)).toEqual(SEEDS);
  });

  it("names every species directory beside it, each called after its species", () => {
    const directories = readdirSync(petsRoot).filter((name) => statSync(resolve(petsRoot, name)).isDirectory());
    expect([...ensemble.species].sort()).toEqual(directories.map((name) => `${name}/🔣️.json`).sort());
    for (const { path, species } of documents) expect(dirname(path).endsWith(species.id), `${path} holds ${species.id}`).toBe(true);
  });

  it("is accepted by the draft-07 contract (ajv)", () => {
    expect(validEnsemble(ensemble) ? [] : validEnsemble.errors).toEqual([]);
  });

  it("has no issue in the pets product", () => {
    expect(ensembleIssues(ensemble)).toEqual([]);
  });

  for (const { path, species } of documents) {
    it(`${path} is accepted by the draft-07 contract (ajv)`, () => {
      expect(validSpecies(species) ? [] : validSpecies.errors).toEqual([]);
    });

    it(`${path} has no issue in the pets product`, () => {
      expect(speciesIssues(species)).toEqual([]);
    });
  }

  it("is rejected by both validators once a species is broken, so neither is vacuous", () => {
    const broken = { ...documents[0]!.species, bones: [] };
    expect(validSpecies(broken)).toBe(false);
    expect(speciesIssues(broken)).not.toEqual([]);
  });
});

describe("architecture menagerie", () => {
  it("assembles without an issue and is accepted by the draft-07 contract (ajv)", () => {
    expect(menagerieIssues(menagerie)).toEqual([]);
    expect(validMenagerie(menagerie) ? [] : validMenagerie.errors).toEqual([]);
  });

  it("is what the site ships: the module's static imports equal the documents on disk, in the ensemble's order, and it exports nothing else", () => {
    expect(architecturePets.ARCHITECTURE_MENAGERIE).toEqual(menagerie);
    expect(Object.keys(architecturePets)).toEqual(["ARCHITECTURE_MENAGERIE"]);
  });

  it("joins existing, different species with every bond, each pair once", () => {
    expect(menagerie.bonds.length).toBeGreaterThan(0);
    for (const bond of menagerie.bonds) {
      expect(ids, `bond ${bond.between.join(" – ")}`).toEqual(expect.arrayContaining([...bond.between]));
      expect(bond.between[0]).not.toBe(bond.between[1]);
      expect(Math.abs(bond.affinity)).toBeLessThanOrEqual(1);
    }
    const pairs = menagerie.bonds.map((bond) => [...bond.between].sort().join(" "));
    expect(new Set(pairs).size).toBe(pairs.length);
  });

  it("gives every species at least one bond, so nobody is a stranger", () => {
    const bonded = new Set(menagerie.bonds.flatMap((bond) => bond.between));
    expect(ids.filter((id) => !bonded.has(id))).toEqual([]);
  });
});

describe("architecture menagerie grounds", () => {
  for (const species of menagerie.species) {
    it(`${species.id} is grounded in the quiz files`, () => {
      expect(species.grounds.length).toBeGreaterThan(0);
      expect(species.grounds.filter((ground) => !groundable.has(ground))).toEqual([]);
    });
  }
});

describe("architecture menagerie casts", () => {
  const scenes = menagerie.casts.map((cast) => cast.scene);
  const members = (scene: string): readonly string[] => menagerie.casts.filter((cast) => cast.scene === scene).flatMap((cast) => [...cast.core, ...cast.rotation]);

  it("has a cast for the home screen and one for every quiz of the catalog, and for nothing else", () => {
    expect(quizzes.length).toBeGreaterThan(0);
    expect([...scenes].sort()).toEqual([HOME, ...quizzes.map((quiz) => quiz.id)].sort());
  });

  it("shows the owner's nine at home and lets every other species take turns there", () => {
    const home = menagerie.casts.find((cast) => cast.scene === HOME)!;
    expect(home.core).toEqual(SEEDS);
    expect([...home.core, ...home.rotation].sort()).toEqual([...ids].sort());
  });

  it("casts existing species, each at most once per scene", () => {
    for (const scene of scenes) {
      expect(ids, scene).toEqual(expect.arrayContaining([...members(scene)]));
      expect(new Set(members(scene)).size, scene).toBe(members(scene).length);
    }
  });

  for (const quiz of quizzes) {
    it(`casts for ${quiz.id} only species grounded in ${quiz.id}`, () => {
      const cast = members(quiz.id);
      expect(cast.length).toBeGreaterThan(0);
      const ungrounded = cast.filter((id) => !menagerie.species.find((species) => species.id === id)!.grounds.some((ground) => ground === quiz.id || ground.startsWith(`${quiz.id}/`)));
      expect(ungrounded).toEqual([]);
    });

    it(`gives the core of ${quiz.id} someone to like or to bicker with among themselves`, () => {
      const core = new Set(menagerie.casts.find((cast) => cast.scene === quiz.id)!.core);
      expect(menagerie.bonds.some((bond) => core.has(bond.between[0]) && core.has(bond.between[1]))).toBe(true);
    });
  }
});

/** 🎒️ The activities a gear brings, each of which needs a clip. */
const GEAR_ACTIVITIES: Readonly<Record<string, readonly string[]>> = { climb: ["climb", "mantle", "slide"], ladder: ["carry", "climb"], grapple: ["aim", "reel"], parachute: ["glide"] };

/** 🤲️ The activities of the learner's hand and of a crowded perch every pet plays: held, thrown, dizzy after a shake, shrugging when it has had enough, scooting aside, pushing a fixture. */
const HAND_ACTIVITIES = ["hang", "tumble", "dizzy", "shrug", "scoot", "push"];

/** 📜️ The rules of the brief (`📓️explore2-species-content.md` §7 of ticket `2026/10/02/QUIZ-PETS`) the chemistry implements: every physical rule but R06 — solary's peak that turns hot is species data, and "alone" has no reaction — and every mood rule but G2, which the stage's contagion already is. */
const RULES = [...Array.from({ length: 64 }, (_, index) => `r${String(index + 1).padStart(2, "0")}`).filter((rule) => rule !== "r06"), "g1", "g3", "g4", "g5", "g6"];

/** 📐️ The farthest a reaction may look, in pixels: about a third of a desktop stage. */
const FARTHEST = 400;

/** ⏳️ The longest a reaction may rest between its turns, in seconds. */
const LONGEST_REST = 600;

/** 🧭️ Where a species can go from each of its states: to the state a state gives way to when it runs out (`lasts`, `then`, the resting state when `then` names none), to where every trick a cue sets off leaves it from each state it is on offer in, and to every state the chemistry puts it in or a trick the chemistry has it play leaves it in. */
function moves(species: Species, chemistry: readonly Reaction[]): ReadonlyMap<string, ReadonlySet<string>> {
  const states = species.states.map((state) => state.id);
  const rest = states[0]!;
  const edges = new Map(states.map((state) => [state, new Set<string>()]));
  const offered = (trick: Trick, state: string): boolean => trick.from === undefined || trick.from.includes(state);
  for (const state of species.states) if (state.lasts !== undefined) edges.get(state.id)!.add(state.then !== undefined && states.includes(state.then) ? state.then : rest);
  for (const trick of species.tricks) if (trick.cues.length > 0) for (const state of states) if (offered(trick, state)) edges.get(state)!.add(stateAfterTrick(species, state, trick));
  for (const reaction of chemistry) {
    for (const effect of reaction.then) {
      const side = effect.on === "when" ? reaction.when : reaction.near;
      if (side.species !== undefined && side.species !== species.id) continue;
      const trick = species.tricks.find((candidate) => candidate.id === effect.trick);
      for (const state of side.state === undefined ? states : states.filter((candidate) => candidate === side.state)) {
        const entered = effect.state !== undefined && states.includes(effect.state) ? effect.state : state;
        edges.get(state)!.add(entered);
        if (trick !== undefined && offered(trick, state)) edges.get(state)!.add(stateAfterTrick(species, entered, trick));
      }
    }
  }
  return edges;
}

/** 🗺️ The states `edges` lead to from `start`, `start` included. */
function reachable(edges: ReadonlyMap<string, ReadonlySet<string>>, start: string): ReadonlySet<string> {
  const seen = new Set([start]);
  const queue = [start];
  while (queue.length > 0) {
    for (const next of edges.get(queue.shift()!) ?? []) {
      if (seen.has(next)) continue;
      seen.add(next);
      queue.push(next);
    }
  }
  return seen;
}

describe("architecture menagerie states, tricks and gear", () => {
  for (const species of menagerie.species) {
    const clips = new Set(species.clips.map((clip) => clip.id));
    const emitters = new Set(species.emitters.map((emitter) => emitter.id));
    const states = species.states.map((state) => state.id);
    const rest = states[0]!;

    it(`${species.id} shows only what it has: every state, trick and its purr name clips, particles and states of its own`, () => {
      const named: (readonly [owner: string, what: "clip" | "emitter" | "state", id: string | undefined])[] = [["purr", "clip", species.purr.clip], ["purr", "emitter", species.purr.emitter]];
      for (const entry of species.states) named.push([`state ${entry.id}`, "clip", entry.clip], [`state ${entry.id}`, "emitter", entry.emitter], [`state ${entry.id}`, "state", entry.then]);
      for (const trick of species.tricks) named.push([`trick ${trick.id}`, "clip", trick.clip], [`trick ${trick.id}`, "emitter", trick.emitter], [`trick ${trick.id}`, "state", trick.to], ...(trick.from ?? []).map((from) => [`trick ${trick.id}`, "state", from] as const));
      const known = { clip: clips, emitter: emitters, state: new Set(states) };
      expect(named.filter(([, what, id]) => id !== undefined && !known[what].has(id)).map(([owner, what, id]) => `${owner} names no ${what} ${id}`)).toEqual([]);
    });

    it(`${species.id} can reach every state from its resting state and come back to it, through tricks, states that run out and the chemistry`, () => {
      const edges = moves(species, menagerie.chemistry);
      const fromRest = reachable(edges, rest);
      expect(states.filter((state) => !fromRest.has(state))).toEqual([]);
      expect(states.filter((state) => !reachable(edges, state).has(rest))).toEqual([]);
    });

    it(`${species.id} changes its resting state when the pointer circles it one way or the other`, () => {
      const circled = (["circle", "countercircle"] as const).flatMap((cue) => species.tricks.filter((trick) => trick.cues.includes(cue) && (trick.from === undefined || trick.from.includes(rest))).slice(0, 1));
      expect(circled.map((trick) => stateAfterTrick(species, rest, trick)).filter((state) => state !== rest)).not.toEqual([]);
    });

    it(`${species.id} has a clip for the learner's hand and for everything its gear brings (${species.gear.join(", ") || "no gear"})`, () => {
      const needed = [...HAND_ACTIVITIES, ...species.gear.flatMap((gear) => GEAR_ACTIVITIES[gear] ?? [`unknown gear ${gear}`])];
      const repertoire = species.repertoire as Readonly<Record<string, readonly string[] | undefined>>;
      expect(needed.filter((activity) => (repertoire[activity] ?? []).length === 0)).toEqual([]);
      expect(Object.entries(repertoire).flatMap(([activity, named]) => (named ?? []).filter((id) => !clips.has(id)).map((id) => `${activity}: ${id}`))).toEqual([]);
      if (species.locomotion.gait === "float") expect(species.gear.filter((gear) => gear !== "parachute")).toEqual([]);
    });
  }
});

describe("architecture menagerie chemistry", () => {
  const reactions = menagerie.chemistry;
  const kinds = new Map(menagerie.species.map((species) => [species.id, species]));
  const reachOf = (trait: Trait): number => (trait.species === undefined ? Math.max(...menagerie.species.map((species) => species.reach)) : (kinds.get(trait.species)?.reach ?? 0));

  it("implements the rules of the brief it can express — R01 to R64 but R06, G1 and G3 to G6 —, each reaction named after its rule", () => {
    expect(reactions.filter((reaction) => !/^(?:r\d\d|g\d)(?:-[a-z0-9]+)*$/u.test(reaction.id)).map((reaction) => reaction.id)).toEqual([]);
    expect([...new Set(reactions.map((reaction) => reaction.id.split("-")[0]!))].sort()).toEqual([...RULES].sort());
    expect(new Set(reactions.map((reaction) => reaction.id)).size).toBe(reactions.length);
  });

  it("leaves R06 to solary: its peak gives way to hot after 40 s, hot to generating after 20 s", () => {
    const states = kinds.get("solary")!.states;
    expect(states.find((state) => state.id === "peak")).toMatchObject({ lasts: 40, then: "hot" });
    expect(states.find((state) => state.id === "hot")).toMatchObject({ lasts: 20, then: "generating" });
  });

  it("names on every side species, states and tricks that exist, and a state held no longer than it lasts", () => {
    const wrong: string[] = [];
    for (const reaction of reactions) {
      for (const [party, trait] of [["when", reaction.when], ["near", reaction.near], ["unless", reaction.unless]] as const) {
        if (trait === undefined) continue;
        const kind = trait.species === undefined ? undefined : kinds.get(trait.species);
        const pool = kind === undefined ? menagerie.species : [kind];
        if (trait.species !== undefined && kind === undefined) wrong.push(`${reaction.id} ${party}: no species ${trait.species}`);
        if (trait.state !== undefined && !pool.some((species) => species.states.some((state) => state.id === trait.state))) wrong.push(`${reaction.id} ${party}: no state ${trait.state}`);
        if (trait.trick !== undefined && !pool.some((species) => species.tricks.some((trick) => trick.id === trait.trick))) wrong.push(`${reaction.id} ${party}: no trick ${trait.trick}`);
        const lasts = kind?.states.find((state) => state.id === trait.state)?.lasts;
        if (trait.held !== undefined && lasts !== undefined && trait.held >= lasts) wrong.push(`${reaction.id} ${party}: ${trait.state} held ${trait.held} s but lasts ${lasts} s`);
      }
    }
    expect(wrong).toEqual([]);
  });

  it("lets every effect do something its side can: a state or trick of the side's species, the trick on offer in the state the side asks for, an amount only with a mood", () => {
    const wrong: string[] = [];
    for (const reaction of reactions) {
      for (const [index, effect] of reaction.then.entries()) {
        const where = `${reaction.id} then ${index}`;
        const side = effect.on === "when" ? reaction.when : reaction.near;
        const kind = side.species === undefined ? undefined : kinds.get(side.species);
        if ([effect.state, effect.mood, effect.rapport, effect.encounter, effect.trick, effect.activity].every((part) => part === undefined)) wrong.push(`${where} does nothing`);
        if (effect.amount !== undefined && effect.mood === undefined) wrong.push(`${where} has an amount without a mood`);
        if ((effect.state !== undefined || effect.trick !== undefined) && kind === undefined) wrong.push(`${where} changes a side of no named species`);
        if (effect.state !== undefined && kind !== undefined && !kind.states.some((state) => state.id === effect.state)) wrong.push(`${where}: ${kind.id} has no state ${effect.state}`);
        const trick = kind?.tricks.find((candidate) => candidate.id === effect.trick);
        if (effect.trick !== undefined && kind !== undefined && trick === undefined) wrong.push(`${where}: ${kind.id} has no trick ${effect.trick}`);
        if (trick?.from !== undefined && side.state !== undefined && !trick.from.includes(side.state)) wrong.push(`${where}: ${trick.id} is not on offer in ${side.state}`);
      }
    }
    expect(wrong).toEqual([]);
  });

  it("looks no nearer than the reach of both species together and no farther than 400 px, rests 1 to 600 s and takes a chance in (0, 1]", () => {
    const wrong: string[] = [];
    for (const reaction of reactions) {
      const reach = reachOf(reaction.when) + reachOf(reaction.near);
      if (reaction.within < reach || reaction.within > FARTHEST) wrong.push(`${reaction.id} looks ${reaction.within} px (reach ${reach} px)`);
      if (reaction.every < 1 || reaction.every > LONGEST_REST) wrong.push(`${reaction.id} rests ${reaction.every} s`);
      if (reaction.chance !== undefined && !(reaction.chance > 0 && reaction.chance <= 1)) wrong.push(`${reaction.id} takes a chance of ${reaction.chance}`);
      if (reaction.affinity !== undefined && reaction.affinity[0] > reaction.affinity[1]) wrong.push(`${reaction.id} asks for an empty affinity`);
    }
    expect(wrong).toEqual([]);
  });

  it("gives every scene a reaction whose two sides can stand on its stage together", () => {
    const together = (cast: Cast, one: string, other: string): boolean => {
      const members = [...cast.core, ...cast.rotation];
      return one !== other && members.includes(one) && members.includes(other) && (cast.scene !== HOME || cast.core.includes(one) || cast.core.includes(other));
    };
    const sides = (trait: Trait): readonly string[] => (trait.species === undefined ? ids : [trait.species]);
    const reacting = menagerie.casts.filter((cast) => reactions.some((reaction) => reaction.when.species !== undefined && reaction.near.species !== undefined && sides(reaction.when).some((one) => sides(reaction.near).some((other) => together(cast, one, other)))));
    expect(reacting.map((cast) => cast.scene).sort()).toEqual(menagerie.casts.map((cast) => cast.scene).sort());
  });
});
