/** 🧩️ Concrete rebuild owners contribute stages through the neutral schema. */
import { expect, test } from "bun:test";
import Ajv from "ajv";
import { parse as parseJsonc, type ParseError } from "jsonc-parser";
import { readFileSync } from "node:fs";
import { readSRebuildPlanV1 } from "../🟦️.ts";
import { printRebuildPlanV1 } from "../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🔁️rebuild/🟦️.ts";
import schema from "../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🔁️rebuild/🧬️schema/🔣️.json";
import fixtures from "./🧫️fixtures/🔣️.json";

test("S and Hub own their complete rebuild contributions", () => {
  const plan = readSRebuildPlanV1();
  const validate = new Ajv({ strict: true, strictTuples: false, allErrors: true }).compile(schema);
  expect(validate(plan)).toBe(true);
  expect(plan.steps.map(step => step.id)).toEqual(fixtures.steps);
  expect(plan.guestFrameworkChecks.flatMap(check => check.packages).filter(name => name.startsWith("semio-s-"))).toEqual(fixtures.guestPackages);
  expect(plan.steps.filter(step => step.stage === "catalog").map(step => step.command[2])).toEqual(fixtures.catalogTargets);
  expect(plan.steps.find(step => step.id === "verify-s")?.command).toEqual(["repo", "run", "@semio-tech/framework-os-dev:verify-staged", "--variant", "s", "--catalog", "✏️s/🧑‍💻dev/🧩️catalog/🤖️generated"]);
});

test("neutral registry routes no longer own development convergence", () => {
  const root = new URL("../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/", import.meta.url);
  expect(readFileSync(new URL("📜️script.ts", root), "utf8")).not.toContain('registerLazy("verify-staged"');
  expect(JSON.parse(readFileSync(new URL("📋️project.json", root), "utf8")).targets).not.toHaveProperty("verify-staged");
});

/** 📣️ Confirms owner composition emits actual runtime progress rows. */
test("observes S deployment composition through real runtime output", () => {
  printRebuildPlanV1(readSRebuildPlanV1());
});

/** 🧭️ Authored launch registration survives regeneration of the visible launch catalog. */
test("launch seed and generated launch retain every canonical rebuild route", () => {
  const root = new URL("../../../../", import.meta.url);
  const read = (path: string): Record<string, unknown>[] => {
    const errors: ParseError[] = [];
    const result = parseJsonc(readFileSync(new URL(path, root), "utf8"), errors, { allowTrailingComma: true });
    expect(errors).toEqual([]);
    return result.configurations;
  };
  const seed = read(".vscode/🧩️launch.seed.jsonc"), launch = read(".vscode/launch.json");
  const names = ["🔁️rebuild-all✏️s", "🔁️rebuild-general🔌️plugin-registry", "🧊️check-guest-framework✏️s", "🔬️test-rebuild-contract📇️registry", "🔬️test-rebuild-contract✏️s", "📋️check-rebuild-plan✏️s", "📋️check-rebuild-plan📇️registry", "⚖️verify-staged🧑‍💻dev"];
  for (const name of names) {
    const authored = seed.filter(row => row.name === name), generated = launch.filter(row => row.name === name);
    expect(authored.length).toBe(1);
    expect(generated).toEqual(authored);
  }
  expect(seed.some(row => typeof row.command === "string" && row.command.includes("plugin-registry:verify-staged"))).toBe(false);
});
