/** ✅️ Unit suite of the validation module: the owned validator against ajv on the normative schema, the committed rule vectors, a sweep of broken copies of the sample menagerie, and the assembly of a menagerie.
 *
 * @see ../../🟦️.ts — the implementation under test
 * @see ../../../../🧬️schema/🔣️.json — the contract ajv compiles
 * @see ../../../../🧫️fixtures/🧬️schema-conformance/🔣️.json — the shared vectors
 * @see https://ajv.js.org/ — the JavaScript oracle of the structural part
 */
import Ajv, { type ErrorObject, type ValidateFunction } from "ajv";
import { readFileSync } from "node:fs";
import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import { describe, expect, it } from "vitest";
import { ACTIVITIES, CUES, ENSEMBLE_SCHEMA, GEARS, MENAGERIE_SCHEMA, type Ensemble, type Menagerie, type Species } from "../../../../🧬️schema/🟦️.ts";
import { sampled } from "../../../../🧪️tests/🎚️config/🟦️.ts";
import { assembleMenagerie, ensembleIssues, menagerieIssues, speciesIssues, type Issue } from "../../🟦️.ts";

type Definition = "Species" | "Menagerie" | "Ensemble";
type Violation = { readonly keyword: string; readonly at: string };
type Vector = { readonly id: string; readonly definition: Definition; readonly document?: unknown; readonly pointer?: string; readonly issues?: readonly Issue[]; readonly violates?: Violation | null };
type Vectors = {
  readonly menagerie: Menagerie;
  readonly ensemble: Ensemble;
  readonly species: readonly { readonly path: string; readonly document: Species }[];
  readonly bases: { readonly species: Species; readonly menagerie: Menagerie; readonly ensemble: Ensemble };
  readonly accepted: readonly Vector[];
  readonly structural: readonly Vector[];
  readonly rules: readonly Vector[];
};
type Node = Record<string, unknown> | unknown[];

const HERE = dirname(fileURLToPath(import.meta.url));
const SCHEMA = JSON.parse(readFileSync(resolve(HERE, "../../../../🧬️schema/🔣️.json"), "utf8")) as { readonly $id: string; readonly $defs: Record<string, unknown> };
const VECTORS = JSON.parse(readFileSync(resolve(HERE, "../../../../🧫️fixtures/🧬️schema-conformance/🔣️.json"), "utf8")) as Vectors;
const AJV = new Ajv({ strict: false, allErrors: true });
AJV.addSchema(SCHEMA);
const JUDGES: Record<Definition, ValidateFunction> = { Species: AJV.getSchema(`${SCHEMA.$id}#/$defs/Species`)!, Menagerie: AJV.getSchema(`${SCHEMA.$id}#/$defs/Menagerie`)!, Ensemble: AJV.getSchema(`${SCHEMA.$id}#/$defs/Ensemble`)! };

/** 🧱️ The codes that restate the type-level structure of the schema; every other code is a value-level or rule finding. */
const STRUCTURAL = new Set(["type-invalid", "required", "property-unknown", "value-invalid"]);

/** 📜️ The document rules of the design (§3 of the first round, §19 and §21 of the second); every one of them needs a committed vector. */
const RULES = ["duplicate-id", "unknown-reference", "bone-order", "key-order", "loop-seam", "ease-range", "out-of-range", "self-bond", "duplicate-bond", "missing-gait-clip", "float-hover", "empty-cast", "duplicate-scene", "duplicate-entry", "lasts-then", "missing-gear-clip", "missing-activity-clip", "floater-gear"];

/** ⚖️ The findings of the owned validator of a definition. */
function issuesOf(definition: Definition, document: unknown): Issue[] {
  return definition === "Species" ? speciesIssues(document) : definition === "Ensemble" ? ensembleIssues(document) : menagerieIssues(document);
}

/** 🧷️ A JSON pointer one step below `base`. */
function below(base: string, key: string | number): string {
  return `${base}/${String(key).replaceAll("~", "~0").replaceAll("/", "~1")}`;
}

/** 💥️ What ajv reports about a document as `keyword pointer` lines: `required` and `additionalProperties` at the property concerned, every other keyword at the value it judges. */
function complaints(definition: Definition, document: unknown): string[] {
  const judge = JUDGES[definition];
  if (judge(document)) return [];
  return (judge.errors ?? []).map((error: ErrorObject) => {
    if (error.keyword === "required") return `required ${below(error.instancePath, error.params.missingProperty as string)}`;
    if (error.keyword === "additionalProperties") return `additionalProperties ${below(error.instancePath, error.params.additionalProperty as string)}`;
    return `${error.keyword} ${error.instancePath}`;
  });
}

/** 🔎️ The document of a vector: inline, or the value its JSON pointer reaches in the vectors. */
function documentOf(vector: Vector): unknown {
  if (vector.pointer === undefined) return vector.document;
  return vector.pointer
    .split("/")
    .slice(1)
    .reduce<unknown>((value, part) => (value as Record<string, unknown>)[part], VECTORS);
}

/** 🌳️ Every object and array of a document with its JSON pointer, the document itself first. */
function nodes(value: unknown, path = ""): [string, Node][] {
  if (typeof value !== "object" || value === null) return [];
  const entries: [string | number, unknown][] = Array.isArray(value) ? value.map((entry, index) => [index, entry]) : Object.entries(value);
  return [[path, value as Node], ...entries.flatMap(([key, entry]) => nodes(entry, below(path, key)))];
}

/** 🧊️ A deeply frozen copy, so a validator that writes to its input throws. */
function frozen<T>(value: T): T {
  const copy = structuredClone(value);
  for (const [, node] of nodes(copy)) Object.freeze(node);
  return copy;
}

/** 🔬️ The sample documents the sweeps break: every species, the base menagerie and the sample ensemble, and at the exhaustive level the whole sample menagerie as well. */
const SWEPT: readonly [string, Definition, unknown][] = [...VECTORS.species.map((member): [string, Definition, unknown] => [member.document.id, "Species", member.document]), ["base-menagerie", "Menagerie", VECTORS.bases.menagerie], ["sample-ensemble", "Ensemble", VECTORS.ensemble], ...sampled<[string, Definition, unknown][]>([], [], [["sample-menagerie", "Menagerie", VECTORS.menagerie]])];

/** 🧵️ How many nodes of a document a sweep breaks at the level of the run. */
const REACH = sampled(24, Number.POSITIVE_INFINITY, Number.POSITIVE_INFINITY);

/** 🪡️ The nodes of a document a sweep breaks: all of them above the fundamental level, at it about two dozen, evenly spread from the root on (a small document is swept whole at every level). */
function swept(document: unknown): [string, Node][] {
  const all = nodes(document);
  const stride = Math.max(Math.ceil(all.length / REACH), 1);
  return all.filter((_, index) => index % stride === 0);
}

describe("accepted documents", () => {
  it.each(VECTORS.accepted.map((vector) => [vector.id, vector] as const))("%s conforms for ajv and for the owned validator", (_, vector) => {
    const document = documentOf(vector);
    expect(complaints(vector.definition, document)).toEqual([]);
    expect(issuesOf(vector.definition, document)).toEqual([]);
  });

  it("the sample menagerie covers every shape kind, every gait, every activity and both scenes", () => {
    const species = VECTORS.menagerie.species;
    expect(new Set(species.flatMap((member) => member.parts.map((part) => part.shape.kind)))).toEqual(new Set(["path", "ellipse", "rect", "line"]));
    expect(species.map((member) => member.locomotion.gait)).toEqual(["walk", "hop", "float"]);
    expect(Object.keys(species[0]!.repertoire).sort()).toEqual([...ACTIVITIES].sort());
    expect(VECTORS.menagerie.casts.map((cast) => cast.scene)).toEqual(["home", "meadow"]);
    expect(VECTORS.menagerie.bonds).toHaveLength(3);
  });

  it("the sample menagerie exercises the second round: a state ladder, tricks for every cue, emitters of several motions, purrs, every gear, a canopy, moods and chemistry", () => {
    const [walker, hopper, floater] = VECTORS.menagerie.species;
    expect(walker!.states.map((state) => state.id)).toEqual(["resting", "glowing", "radiant"]);
    expect(walker!.states.slice(1).map((state) => state.then)).toEqual(["resting", "glowing"]);
    for (const [cue, from, to] of [["circle", "resting", "glowing"], ["circle", "glowing", "radiant"], ["countercircle", "radiant", "glowing"], ["countercircle", "glowing", "resting"]] as const) {
      expect(walker!.tricks.filter((trick) => trick.cues.includes(cue) && trick.from?.includes(from) && trick.to === to), `${cue} ${from}`).toHaveLength(1);
    }
    expect(new Set(VECTORS.menagerie.species.flatMap((member) => member.tricks.flatMap((trick) => trick.cues)))).toEqual(new Set(CUES));
    expect(walker!.emitters.map((emitter) => emitter.motion)).toEqual(["burst", "rise"]);
    expect([walker!.purr, hopper!.purr, floater!.purr]).toEqual([{ clip: "nuzzle", emitter: "hum" }, { clip: "snuggle" }, { clip: "glow" }]);
    expect([walker!.gear, hopper!.gear, floater!.gear]).toEqual([[...GEARS], ["grapple", "parachute"], ["parachute"]]);
    expect([walker!.canopy?.kind, hopper!.canopy, floater!.canopy]).toEqual(["path", undefined, undefined]);
    expect(VECTORS.menagerie.species.map((member) => member.mood)).toEqual(["content", "playful", "sleepy"]);
    for (const member of VECTORS.menagerie.species) {
      expect(member.grip).toBeLessThanOrEqual(member.size.height);
      expect(member.reach).toBeLessThanOrEqual(member.size.width);
    }
    expect(VECTORS.menagerie.chemistry).toHaveLength(6);
    expect(VECTORS.menagerie.chemistry.flatMap((reaction) => reaction.then.map((effect) => effect.on)).sort()).toEqual(["near", "near", "near", "near", "near", "near", "near", "near", "when", "when", "when"]);
    const [, , , friends, entertainer, lullaby] = VECTORS.menagerie.chemistry;
    expect([friends!.when.held, friends!.near, friends!.affinity, entertainer!.when.trick, entertainer!.unless, lullaby!.when.species, lullaby!.then[0]!.activity]).toEqual([4, {}, [0.4, 1], "boing", { species: "floaty", mood: "scared" }, undefined, "sleep"]);
    expect(VECTORS.ensemble.chemistry).toEqual(VECTORS.menagerie.chemistry);
  });
});

describe("structural rejections", () => {
  it.each(VECTORS.structural.map((vector) => [vector.id, vector] as const))("%s: ajv names the keyword, the owned validator the same pointer", (_, vector) => {
    expect(complaints(vector.definition, vector.document)).toContain(`${vector.violates!.keyword} ${vector.violates!.at}`);
    expect(issuesOf(vector.definition, vector.document)).toEqual(vector.issues);
  });
});

describe("rule violations", () => {
  it.each(VECTORS.rules.map((vector) => [vector.id, vector] as const))("%s yields the committed findings", (_, vector) => {
    expect(issuesOf(vector.definition, vector.document)).toEqual(vector.issues);
    const reported = complaints(vector.definition, vector.document);
    if (vector.violates === null) expect(reported).toEqual([]);
    else expect(reported).toContain(`${vector.violates!.keyword} ${vector.violates!.at}`);
  });

  it("every rule of the design has a vector", () => {
    const covered = new Set(VECTORS.rules.flatMap((vector) => vector.issues!.map((issue) => issue.code)));
    expect(RULES.filter((code) => !covered.has(code))).toEqual([]);
  });

  it("a looping rotation closes on the same angle modulo 360° and on nothing else", () => {
    const spin = (channel: string, from: number, to: number, loop: boolean) => {
      const species = structuredClone(VECTORS.bases.species) as unknown as { clips: { loop: boolean; tracks: unknown[] }[] };
      species.clips[1]!.loop = loop;
      species.clips[1]!.tracks = [{ bone: "body", channel, keys: [{ at: 0, value: from }, { at: 1, value: to }] }];
      return speciesIssues(species).map((issue) => issue.code);
    };
    for (const [from, to] of [[0, 360], [0, -360], [-180, 180], [45, 765], [90, 90]] as const) expect(spin("rotation", from, to, true)).toEqual([]);
    for (const [from, to] of [[0, 180], [0, 359], [0, 360.5], [10, -10]] as const) expect(spin("rotation", from, to, true)).toEqual(["loop-seam"]);
    for (const channel of ["x", "y", "scaleX", "scaleY"]) expect(spin(channel, 0, 360, true)).toEqual(["loop-seam"]);
    expect(spin("rotation", 0, 180, false)).toEqual([]);
    expect(spin("x", 0, 12, false)).toEqual([]);
  });

  it("findings are sorted by pointer, then code, without repeats, and the input is left untouched", () => {
    for (const vector of [...VECTORS.structural, ...VECTORS.rules]) {
      const found = issuesOf(vector.definition, frozen(vector.document));
      const keys = found.map((issue) => `${issue.path}\u0000${issue.code}`);
      expect(new Set(keys).size).toBe(keys.length);
      expect([...found].sort((left, right) => (left.path < right.path ? -1 : left.path > right.path ? 1 : left.code < right.code ? -1 : left.code > right.code ? 1 : 0))).toEqual(found);
    }
  });

  it("a document of another kind is refused at its root or its schema identifier", () => {
    for (const value of [null, 7, "pets", true, []]) for (const definition of ["Species", "Menagerie", "Ensemble"] as const) expect(issuesOf(definition, value)).toEqual([{ path: "", code: "type-invalid" }]);
    expect(menagerieIssues(VECTORS.ensemble).some((issue) => issue.path === "/schema" && issue.code === "value-invalid")).toBe(true);
    expect(ensembleIssues(VECTORS.menagerie).some((issue) => issue.path === "/schema" && issue.code === "value-invalid")).toBe(true);
  });
});

describe("sweep against ajv", () => {
  it.each(SWEPT)("%s: a removed property is judged like ajv judges it", (_, definition, source) => {
    const document = structuredClone(source);
    let checked = 0;
    for (const [path, node] of swept(document)) {
      if (Array.isArray(node)) continue;
      for (const key of Object.keys(node)) {
        const kept = node[key];
        const order = Object.keys(node);
        delete node[key];
        const reported = complaints(definition, document);
        const found = issuesOf(definition, document);
        const missing = { path: below(path, key), code: "required" };
        expect(found.some((issue) => issue.path === missing.path && issue.code === missing.code), `${missing.path} required`).toBe(reported.includes(`required ${missing.path}`));
        if (reported.length > 0) expect(found.length, `${missing.path} rejected by ajv`).toBeGreaterThan(0);
        else expect(found.filter((issue) => STRUCTURAL.has(issue.code)), `${missing.path} accepted by ajv`).toEqual([]);
        node[key] = kept;
        for (const name of order.slice(order.indexOf(key) + 1)) {
          const value = node[name];
          delete node[name];
          node[name] = value;
        }
        checked += 1;
      }
    }
    expect(checked).toBeGreaterThan(10);
    expect(document).toEqual(source);
  });

  it.each(SWEPT)("%s: an undeclared property is refused wherever it is added", (_, definition, source) => {
    const document = structuredClone(source);
    for (const [path, node] of swept(document)) {
      if (Array.isArray(node)) continue;
      node["zz-undeclared"] = 1;
      const pointer = below(path, "zz-undeclared");
      expect(complaints(definition, document), pointer).toContain(`additionalProperties ${pointer}`);
      expect(issuesOf(definition, document), pointer).toContainEqual({ path: pointer, code: "property-unknown" });
      delete node["zz-undeclared"];
    }
    expect(document).toEqual(source);
  });

  it.each(SWEPT)("%s: a value of the wrong JSON type is refused at its own pointer", (_, definition, source) => {
    const document = structuredClone(source);
    let checked = 0;
    for (const [path, node] of swept(document)) {
      for (const key of Object.keys(node)) {
        const kept = (node as Record<string, unknown>)[key];
        if (typeof kept === "object" && kept !== null) continue;
        const pointer = below(path, key);
        (node as Record<string, unknown>)[key] = typeof kept === "string" ? 7 : "seven";
        expect(complaints(definition, document).some((line) => line.endsWith(` ${pointer}`)), pointer).toBe(true);
        expect(issuesOf(definition, document).some((issue) => issue.path === pointer && STRUCTURAL.has(issue.code)), pointer).toBe(true);
        (node as Record<string, unknown>)[key] = kept;
        checked += 1;
      }
    }
    expect(checked).toBeGreaterThan(10);
    expect(document).toEqual(source);
  });

  it.each(SWEPT)("%s: an object or list replaced by a number is refused at its own pointer", (_, definition, source) => {
    const document = structuredClone(source);
    for (const [path, node] of swept(document)) {
      for (const key of Object.keys(node)) {
        const kept = (node as Record<string, unknown>)[key];
        if (typeof kept !== "object" || kept === null) continue;
        const pointer = below(path, key);
        (node as Record<string, unknown>)[key] = 7;
        expect(complaints(definition, document).some((line) => line.endsWith(` ${pointer}`)), pointer).toBe(true);
        expect(issuesOf(definition, document), pointer).toContainEqual({ path: pointer, code: "type-invalid" });
        (node as Record<string, unknown>)[key] = kept;
      }
    }
    expect(document).toEqual(source);
  });
});

describe("assembly", () => {
  it("assembles the sample menagerie from its ensemble and species documents", () => {
    const assembled = assembleMenagerie(VECTORS.ensemble, VECTORS.species.map((member) => member.document));
    expect(assembled).toEqual(VECTORS.menagerie);
    expect(menagerieIssues(assembled)).toEqual([]);
    expect(complaints("Menagerie", assembled)).toEqual([]);
  });

  it("carries the menagerie identifier, drops the schema hints and keeps the given order", () => {
    const [walker, hopper, floater] = VECTORS.species.map((member) => member.document);
    expect(VECTORS.ensemble.$schema).toBeTypeOf("string");
    expect(walker!.$schema).toBeTypeOf("string");
    const assembled = assembleMenagerie(frozen(VECTORS.ensemble), frozen([floater!, walker!, hopper!]));
    expect(assembled.schema).toBe(MENAGERIE_SCHEMA);
    expect(VECTORS.ensemble.schema).toBe(ENSEMBLE_SCHEMA);
    expect(Object.hasOwn(assembled, "$schema")).toBe(false);
    expect(assembled.species.some((member) => Object.hasOwn(member, "$schema"))).toBe(false);
    expect(assembled.species.map((member) => member.id)).toEqual(["floaty", "blobby", "hoppy"]);
    expect(assembled.bonds).toEqual(VECTORS.ensemble.bonds);
    expect(assembled.casts).toEqual(VECTORS.ensemble.casts);
  });

  it("an ensemble whose bonds and casts name species it does not assemble is caught in the menagerie", () => {
    const assembled = assembleMenagerie(VECTORS.ensemble, [VECTORS.species[0]!.document]);
    expect(ensembleIssues(VECTORS.ensemble)).toEqual([]);
    expect(new Set(menagerieIssues(assembled).map((issue) => issue.code))).toEqual(new Set(["unknown-reference"]));
  });
});

describe("contract twins", () => {
  it("the schema declares one definition per exported type of the TypeScript twin", () => {
    const source = readFileSync(resolve(HERE, "../../../../🧬️schema/🟦️.ts"), "utf8");
    const exported = [...source.matchAll(/^export type (\w+)/gmu)].map((match) => match[1]!);
    expect(Object.keys(SCHEMA.$defs).sort()).toEqual([...exported].sort());
  });
});
