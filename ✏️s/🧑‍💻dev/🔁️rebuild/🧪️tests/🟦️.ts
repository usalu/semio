/** 🧩️ Concrete rebuild owners contribute stages through the neutral schema. */
import { expect, test } from "bun:test";
import Ajv from "ajv";
import { parse as parseJsonc, type ParseError } from "jsonc-parser";
import { readFileSync } from "node:fs";
import { readSRebuildPlanV1 } from "../🟦️.ts";
import { printRebuildPlanV1 } from "@semio-tech/plugin-registry/rebuild";
import schema from "@semio-tech/plugin-registry/rebuild/schema.json";
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

/** 🧭️ Neutral rebuild registration is read directly from the owning project manifests. */
test("canonical rebuild routes remain owned by their dashboard projects",()=>{const root=new URL("../../../../",import.meta.url);for(const [file,targets] of [["✏️s/🧑‍💻dev/🔁️rebuild/📋️project.json",["rebuild-all","guest-framework-check","plan-check","test"]],["🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/📋️project.json",["rebuild-all","test-rebuild-contract"]]] as const){const text=readFileSync(new URL(file,root),"utf8"),parsed=parseJsonc(text),manifest=JSON.parse(text);expect(parsed).toEqual(manifest);for(const target of targets)expect(manifest.targets[target].options.command).toContain("bun ./📜️script.ts ");}const dashboard=JSON.parse(readFileSync(new URL("📋️project.json",root),"utf8")).metadata.semio.dashboard;expect(dashboard.tools.find((row:any)=>row.id==="build-task").command).toEqual(["bun","repo","run","{task}","--output=stream"]);});
