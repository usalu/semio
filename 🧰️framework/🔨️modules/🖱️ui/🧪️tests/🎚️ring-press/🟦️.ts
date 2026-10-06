import { expect, test } from "bun:test";
import { readFileSync, mkdirSync, writeFileSync } from "node:fs";
import { join, resolve } from "node:path";
import Ajv from "ajv";
import ts from "typescript";
import React from "react";
import { createContinuousGestureLane } from "../../🎬️scene/🟦️.ts";

const ui = resolve(import.meta.dir, "../.."), root = resolve(ui, "../..");
const renderer = join(root, "🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine");
const read = (path: string): string => readFileSync(path, "utf8");
const fixture = JSON.parse(read(join(ui, "🧫️fixtures/🎛️retained-control-commit/🔣️.json"))) as { cases: Case[] };
type Case = { name: string; node: { kind: string }; expected: { press?: "open" | "released" } | null };
type Lifecycle = { id: string; values: number[]; terminal: "pointerup" | "pointercancel"; expected: (number | string)[][] };
const lifecycle = JSON.parse(read(join(ui, "🧫️fixtures/🎚️ring-press/🔣️.json"))) as { cases: Lifecycle[] };
const validateLifecycle = new Ajv({ strict: true }).compile(JSON.parse(read(join(ui, "🧬️schema/🎚️ring-press/🔣️.json"))));

function declaration(source: string, name: string): string {
  const file = ts.createSourceFile("reference.tsx", source, ts.ScriptTarget.Latest, true, ts.ScriptKind.TSX);
  const matches = file.statements.filter(node => ts.isFunctionDeclaration(node) && node.name?.text === name || ts.isVariableStatement(node) && node.declarationList.declarations.some(entry => ts.isIdentifier(entry.name) && entry.name.text === name));
  if (matches.length !== 1) throw Error("Expected one actual reference declaration " + name);
  return matches[0]!.getText(file);
}

function compile(source: string): string {
  const result = ts.transpileModule(source, { compilerOptions: { target: ts.ScriptTarget.ES2022, module: ts.ModuleKind.CommonJS, jsx: ts.JsxEmit.React }, reportDiagnostics: true });
  expect(result.diagnostics?.filter(row => row.category === ts.DiagnosticCategory.Error)).toEqual([]);
  return result.outputText;
}

test("the complete shared fixture pins Ring to the continuous contract", () => {
  expect(fixture.cases.map(row => row.name)).toEqual(["input-change-per-keystroke", "input-number-commits-a-number", "input-blur-policy-is-silent-while-typing", "input-blur-policy-commits-on-enter", "input-blur-policy-commits-on-blur", "input-unbound-is-silent", "toggle-commits-the-flip", "toggle-already-on-commits-false", "slider-reads-its-own-track", "slider-clamps-past-its-end", "stepper-increment-falls-back-to-absolute", "stepper-decrement-prefers-the-delta-binding", "stepper-value-segment-commits-nothing", "ring-reads-its-own-circle", "icon-select-edits-its-icon-string"]);
  const hostile = structuredClone(fixture);
  delete hostile.cases.find(row => row.node.kind === "ring")!.expected!.press;
  expect(validateLifecycle(lifecycle)).toBe(true);
  const substituted = structuredClone(lifecycle);
  substituted.cases[0]!.expected.pop();
  expect(validateLifecycle(substituted)).toBe(false);
});

test("the retained TypeScript reference independently agrees for all fifteen controls", () => {
  const source = read(join(renderer, "🧪️tests/🎛️retained-control-commit/🟦️.ts"));
  const body = declaration(source, "inputCommitsOnBlur") + "\n" + declaration(source, "expectedPress");
  const reference = new Function(compile(body) + "\nreturn expectedPress;")() as (row: Case) => "open" | "released" | undefined;
  for (const row of fixture.cases) if (row.expected !== null) expect(reference(row), row.name).toBe(row.expected.press);
  expect(reference(fixture.cases.find(row => row.node.kind === "ring")!)).toBe("released");
});

for (const row of lifecycle.cases) test("actual React RingView and owned lane: " + row.id, async () => {
  const observations: (number | string)[][] = [], listeners = new Map<string, () => void>(), cleanup: (() => void)[] = [];
  const lane = createContinuousGestureLane<number>({ send: (value, phase) => { observations.push([value, phase]); }, abort: reason => { observations.push([reason, "abort"]); } });
  const source = read(join(renderer, "🧱️elements/🗣️Interpreter/🟦️.tsx"));
  const factory = new Function("React", "Ring", "useContinuousTriggerLane", "useEffect", "window", "nodeDomId", "toUiValue", compile(declaration(source, "RingView")) + "\nreturn RingView;");
  const view = factory(React, "owned-ring", () => lane, (effect: () => () => void) => { cleanup.push(effect()); }, { addEventListener: (name: string, listener: () => void) => listeners.set(name, listener), removeEventListener: (name: string) => listeners.delete(name) }, () => "fixture-ring", (value: number) => value) as (props: unknown) => React.ReactElement<{ onOrbChange: (id: string, before: number, after: number) => void }>;
  const element = view({ record: { component: { orbId: "sun", t: 0 }, disabled: false }, context: {} });
  expect(React.isValidElement(element)).toBe(true);
  for (const value of row.values) element.props.onOrbChange("sun", 0, value);
  listeners.get(row.terminal)!();
  await Promise.resolve();
  expect(observations).toEqual(row.expected);
  expect(lane.open()).toBe(false);
  for (const release of cleanup) release();
  expect(listeners.size).toBe(0);
  const artifacts = process.env.SEMIO_TEST_ARTIFACT_DIR;
  if (!artifacts) throw Error("SEMIO_TEST_ARTIFACT_DIR must name caller-owned ticket output");
  mkdirSync(artifacts, { recursive: true });
  writeFileSync(join(artifacts, "ring-reference-" + row.id + ".json"), JSON.stringify({ id: row.id, observations }));
});
