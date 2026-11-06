import { existsSync } from "node:fs";
import { resolve } from "node:path";
import { expect, test } from "vitest";
import Ajv from "ajv";
import schema from "../🧬️schema/🔣️.json";
import scenarios from "../🧫️fixtures/🔣️.json";
import { initialRemodelBrowserReportV1, remodelEditorPresentV1, remodelAcceptanceSatisfiedV1, serializeRemodelBrowserReportV1, parseRemodelBrowserArgumentsV1, remodelImageInputsV1, remodelHostReadyV1 } from "../📋️report/🟦️.ts";

test("every Remodel browser scenario remains unexecuted until real UI evidence exists", () => {
  const oracle = new Ajv({ strict: true });
  expect(oracle.validate(schema, scenarios)).toBe(true);
  expect(scenarios.find(row => row.id === "import-images")?.assets).toHaveLength(3);
  for (const id of ["import-jpeg", "import-corrupt", "import-append"]) expect(scenarios.some(row => row.id === id)).toBe(true);
  expect(scenarios.find(row => row.id === "import-video")?.description).toContain("45");
  expect(oracle.validate(schema, scenarios.map(row => row.id === "import-images" ? { id: row.id, description: row.description } : row))).toBe(false);
  expect(oracle.validate(schema, scenarios.slice(1))).toBe(false);
  expect(oracle.validate(schema, [...scenarios.slice(0, -1), { ...scenarios[0]!, description: "different" }])).toBe(false);
  const report = initialRemodelBrowserReportV1("http://localhost:6063");
  expect(report.scenarios.map(row => row.id)).toEqual(scenarios.map(row => row.id));
  expect(report.scenarios.every(row => row.status === "unexecuted")).toBe(true);
  expect(report.scenarios.every(row => row.evidence.length === 0)).toBe(true);
});


test("acceptance refuses wrong identity, missing evidence and incomplete checklists", () => {
  expect(remodelEditorPresentV1([{ app: "s.remodel.remodeling@1/*#viewer", role: "viewer" }])).toBe(false);
  expect(remodelEditorPresentV1([{ app: "s.remodel.remodeling@1/*#editor", role: "editor" }])).toBe(true);
  const report = initialRemodelBrowserReportV1("http://localhost:6063");
  report.scenarios[0]!.evidence.push(JSON.stringify({ editorChip: true, unrelatedWindowBody: true }));
  expect(report.scenarios[0]!.status).toBe("unexecuted");
  expect(remodelAcceptanceSatisfiedV1(report)).toBe(false);
  expect(JSON.parse(serializeRemodelBrowserReportV1(report))).toEqual(report);
  for (const row of report.scenarios) { row.status = "passed"; row.evidence.push("observed"); }
  expect(remodelAcceptanceSatisfiedV1(report)).toBe(true);
  report.scenarios[0]!.evidence.length = 0;
  expect(remodelAcceptanceSatisfiedV1(report)).toBe(false);
  report.scenarios[0]!.evidence.push("observed");
  report.scenarios.pop();
  expect(remodelAcceptanceSatisfiedV1(report)).toBe(false);
  report.faults.push("pageerror");
  expect(remodelAcceptanceSatisfiedV1(report)).toBe(false);
});


test("browser acceptance requires exact arguments and an explicit locale before side effects", () => {
  const good = ["--host", "http://localhost:6063", "--out", "report.json", "--locale", "en"];
  expect(parseRemodelBrowserArgumentsV1(good).locale).toBe("en");
  for (const bad of [
    good.slice(0, -2), [...good, "--extra", "x"], [...good, "--host", "http://localhost:1"],
    ["--host", "--out", "--locale", "en", "--out", "x"], [...good.slice(0, -1), "fr"],
    ["--host", "http://user:pass@localhost", "--out", "x", "--locale", "de"],
    ["--host", "http://localhost", "--out", "x".repeat(257), "--locale", "en"]
  ]) expect(() => parseRemodelBrowserArgumentsV1(bad)).toThrow();
});


test("the import input plan uses the three existing repository images in place", () => {
  const root = resolve(import.meta.dirname, "../../../../../..");
  const inputs = remodelImageInputsV1(root);
  expect(inputs).toHaveLength(3);
  expect(inputs.every(path => path.startsWith(root) && path.length <= 256 && existsSync(path))).toBe(true);
  expect(inputs.map(path => path.split(/[\\/]/u).at(-1))).toEqual(["🎞️frame-00.png", "🎞️frame-01.png", "🎞️frame-02.png"]);
});


test("only an actual ready beacon admits host interactions", () => {
  expect(remodelHostReadyV1(null)).toBe(false);
  expect(remodelHostReadyV1("error:broken")).toBe(false);
  expect(remodelHostReadyV1("not-found:missing")).toBe(false);
  expect(remodelHostReadyV1("ready:yes")).toBe(true);
});
