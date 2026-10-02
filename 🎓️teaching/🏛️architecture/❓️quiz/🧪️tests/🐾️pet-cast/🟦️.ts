/** 🐾️ The pets of the site fit its quizzes. Every species document and the ensemble of the architecture menagerie are
 * accepted by the draft-07 contract of the pets product through a third-party validator (ajv) and by the product's own
 * validators, and the menagerie the site ships is the one those documents assemble to, without an issue. Every ground a
 * species names exists in the quiz files, every species in the cast of a quiz is grounded in that quiz, every quiz of
 * the catalog has a cast, the home screen has one that knows every species, and every bond joins two of them.
 * @see ../../../🐾️pets/🔣️.json — the ensemble under test
 * @see ../../../🐾️pets/🟦️.ts — the menagerie the site hands to the quiz
 * @see ../../🔣️.json — the catalog whose quizzes ground the species
 * @see ../../../../../🧰️framework/🛍️products/🐾️pets/🧬️schema/🔣️.json — the contract ajv validates against */
import { assembleMenagerie, ensembleIssues, menagerieIssues, speciesIssues, type Ensemble, type Species } from "@semio-tech/pets";
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
