/** ⌨️ Explicit keyboard gestures share the hand's state, mood and eligibility rules. */
import { readFileSync } from "node:fs";
import { describe, expect, it } from "vitest";
import type { Actor, Deed, Menagerie, Stage } from "../../../../🧬️schema/🟦️.ts";
import { atRest } from "../../../💗️feeling/🟦️.ts";
import { advance, openStage } from "../../../🎪️stage/🟦️.ts";

const menagerie = JSON.parse(readFileSync(new URL("../../../../🧪️tests/🧫️fixtures/⚗️chemistry-rules/🔣️.json", import.meta.url), "utf8")).menagerie as Menagerie;
const fixture = JSON.parse(readFileSync(new URL("../../../../🧪️tests/🧫️fixtures/⌨️keyboard-gestures/🔣️.json", import.meta.url), "utf8")) as { deeds: Deed[]; vectors: { species: string; state: string; deed: Deed; trick: string | null; after: string }[]; route: { species: string; state: string; deeds: Deed[]; states: string[] }; blocked: Partial<Actor>[] };

/** 🪑️ One willing or unwilling actor at rest, with the hand permitted. */
function seated(species: string, state: string, grumpy = false): Stage {
  const stage = advance(menagerie, openStage(7), [{ kind: "surveyed", width: 1280, height: 720, surfaces: [{ id: "floor", x0: 0, x1: 1280, y: 720 }], keepouts: [], walls: [], fixtures: [] }, { kind: "summoned", species: [species] }, { kind: "permitted", play: true, mischief: false }]);
  return { ...stage, actors: stage.actors.map(actor => ({ ...actor, state, stateSince: 0, activity: "idle", since: 0, until: 100000, partner: null, leaving: false, feeling: grumpy ? { mood: "grumpy", intensity: 0.9, since: 0 } : atRest("content", 0) })) };
}

/** 🎬️ One keyboard gesture and the end of its clip, when it has one. */
function gesture(stage: Stage, species: string, deed: Deed): Stage {
  const asked = advance(menagerie, stage, [{ kind: "played", species, deed }]);
  const actor = asked.actors[0]!;
  return actor.activity === "trick" ? advance(menagerie, asked, [{ kind: "ticked", ticks: actor.until - asked.tick }]) : asked;
}

describe("explicit keyboard gestures", () => {
  for (const vector of fixture.vectors) for (const grumpy of [false, true]) it(`${vector.species}/${vector.state}/${vector.deed}, unwilling=${grumpy}`, () => {
    const stage = seated(vector.species, vector.state, grumpy);
    const asked = advance(menagerie, stage, [{ kind: "played", species: vector.species, deed: vector.deed }]);
    expect(asked.actors[0]!.trick).toBe(vector.trick);
    expect(gesture(stage, vector.species, vector.deed).actors[0]!.state).toBe(vector.after);
    expect(asked.actors[0]!.warmth).toEqual(stage.actors[0]!.warmth);
  });
  it("chooses a sequence of states from the resting state without shared-counter starvation", () => {
    let stage = seated(fixture.route.species, fixture.route.state);
    for (const [index, deed] of fixture.route.deeds.entries()) {
      stage = gesture(stage, fixture.route.species, deed);
      expect(stage.actors[0]!.state).toBe(fixture.route.states[index]);
    }
  });
  it("preserves stage permissions and the free-actor boundary for every gesture", () => {
    const stage = seated("glow", "bright");
    for (const deed of fixture.deeds) {
      for (const changes of fixture.blocked) {
        const blocked = { ...stage, actors: [{ ...stage.actors[0]!, ...changes }] };
        expect(advance(menagerie, blocked, [{ kind: "played", species: "glow", deed }])).toEqual(blocked);
      }
      for (const blocked of [{ ...stage, play: false }, { ...stage, mode: "still" as const }]) expect(advance(menagerie, blocked, [{ kind: "played", species: "glow", deed }])).toEqual(blocked);
    }
  });
  it("keeps purring under a stroke even when the species offers a stroke trick", () => {
    const stage = seated("mist", "fluffy");
    const purring = { ...stage, actors: [{ ...stage.actors[0]!, activity: "purr" as const }] };
    const asked = advance(menagerie, purring, [{ kind: "played", species: "mist", deed: "stroke" }]);
    expect(asked.actors[0]!.activity).toBe("purr");
    expect(asked.actors[0]!.trick).toBeNull();
  });
});
