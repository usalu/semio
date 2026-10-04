import { test, expect } from "bun:test";
import { readFileSync, mkdirSync, writeFileSync } from "node:fs";
import { resolve } from "node:path";
import { createRequire } from "node:module";
import Ajv from "ajv";
import { createAdmittedWorkerCell, createWorkerCell } from "../🟦️.ts";
const require = createRequire(import.meta.url);
const memoize: (factory: (owner: string) => State) => ((owner: string) => State) = require("lodash/memoize");
type State = { locale: string; retained: string[]; constructions: number };
const fixture = JSON.parse(readFileSync(resolve(import.meta.dir, "../🧫️fixtures/🔣️.json"), "utf8"));
const interpreter = resolve(import.meta.dir, "../../🎯️targets/🧊️wgpu/🦀️.rs");

test("closed independent lifecycle corpus retains exact admission and thread identities", () => {
  const validate = new Ajv({ strict: true }).compile(JSON.parse(readFileSync(resolve(import.meta.dir, "../🧬️schema/🔣️.json"), "utf8")));
  expect(validate(fixture)).toBe(true);
  expect(validate({ ...fixture, schemaVersion: 2 })).toBe(false);
  const actual = [];
  for (const row of fixture.cases) {
    const cell = createAdmittedWorkerCell<State>();
    const ready = new Set<string>();
    const reference = memoize(() => ({ locale: "", retained: [], constructions: 1 }));
    const observations = [], oracle = [];
    for (const operation of row.operations) {
      if (operation.action === "admit") {
        cell.admit(operation.owner, () => ({ locale: operation.locale, retained: [], constructions: 1 }), state => { state.locale = operation.locale; });
        ready.add(operation.owner); reference(operation.owner).locale = operation.locale;
      } else if (operation.action === "retain") {
        cell.read(operation.owner)!.retained.push(operation.value); reference(operation.owner).retained.push(operation.value);
      }
      const state = cell.read(operation.owner), expected = ready.has(operation.owner) ? reference(operation.owner) : undefined;
      observations.push(state ? { owner: operation.owner, state: "ready", ...structuredClone(state) } : { owner: operation.owner, state: "unadmitted" });
      oracle.push(expected ? { owner: operation.owner, state: "ready", ...structuredClone(expected) } : { owner: operation.owner, state: "unadmitted" });
    }
    expect(observations).toEqual(row.expected); expect(oracle).toEqual(row.expected); actual.push({ id: row.id, observations });
  }
  const output = process.env.SEMIO_TEST_ARTIFACT_DIR;
  if (!output) throw new Error("Caller-owned SEMIO_TEST_ARTIFACT_DIR is required");
  mkdirSync(output, { recursive: true }); writeFileSync(resolve(output, "worker-cell-lifecycle.json"), JSON.stringify(actual, null, 2));
});

test("nonDefault construction is lazy and scoped rather than implicit", () => {
  let calls = 0; const cell = createWorkerCell(() => ({ secret: ++calls }));
  expect(calls).toBe(0); expect(cell.read("a")).toBe(cell.read("a"));
  expect(cell.read("b").secret).toBe(2); expect(cell.read("a").secret).toBe(1);
});

test("actual Rust worker source requires owned factories and explicit locale admission", () => {
  const source = readFileSync(interpreter, "utf8");
  expect(source).not.toMatch(/impl<T: Default|test_worker_cell<T: Default/);
  expect(source.includes("initialize: fn() -> T")).toBe(true);
  expect(source.includes("get_or_init(|| Mutex::new((self.initialize)()))")).toBe(true);
  expect(source.includes("test_worker_cell::<T>(std::ptr::from_ref(self).addr(), self.initialize)")).toBe(true);
  expect(source.includes("UiEngineCell(WorkerCell<Option<ui_wgpu::wgpu::Ui>>)")).toBe(true);
  expect(source.match(/fn initial_ui_engine\(\)[\s\S]*?\n}/)?.[0]).toMatch(/#\[cfg\(test\)\]\s+return Some\(ui_wgpu::wgpu::Ui::new\(TEST_UI_ENGINE_LOCALE\)\);\s+#\[cfg\(not\(test\)\)\]\s+None/);
  expect(source.includes("self.0.as_ref().expect(UI_ENGINE_LOCALE_UNRESOLVED)")).toBe(true);
  expect(source.includes("self.0.as_mut().expect(UI_ENGINE_LOCALE_UNRESOLVED)")).toBe(true);
  const admission = source.match(/pub\(crate\) fn install_ui_engine_locale\([^)]*\)[\s\S]*?\n}/)?.[0];
  expect(admission).toMatch(/Some\(ui\) => ui.set_locale\(locale\),/);
  expect(admission).toMatch(/None => \*engine = Some\(ui_wgpu::wgpu::Ui::new\(locale\)\),/);
  expect(source.includes('#[path = "../../🧵️worker-cell/🧪️tests/🦀️.rs"]')).toBe(true);
});

test("actual Shell and Ui locale updates retain the existing engine and Scenes cell contract", () => {
  const shell = readFileSync(resolve(import.meta.dir, "../../../🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs"), "utf8");
  expect(shell).toMatch(/pub fn new\(plugins: Vec<ProgramBridgeEntry>, plugin_filter: String, locale: Locale, terminology: Terminology\) -> Self \{\s+crate::interpreter::install_ui_engine_locale\(locale\);/);
  expect(shell.match(/crate::interpreter::install_ui_engine_locale\(self.active_locale\(\)\);/g)?.length).toBe(3);
  const engine = readFileSync(resolve(import.meta.dir, "../../../../../../../../../🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/⚙️engine/🦀️.rs"), "utf8");
  expect(engine.match(/pub fn set_locale\([^)]*\)[\s\S]*?\n    }/)?.[0]).toMatch(/\{\s+self.shell.set_locale\(locale\);\s+}/);
  const scenes = readFileSync(resolve(import.meta.dir, "../../../🎞️Scenes/🎯️targets/🧊️wgpu/🦀️.rs"), "utf8");
  expect(scenes.includes("OnceLock<Mutex<RefCell<T>>>")).toBe(true);
  expect(scenes.includes("std::sync::PoisonError::into_inner")).toBe(true);
  expect(scenes.includes("|| RefCell::new(T::default())")).toBe(true);
});


test("the authored worker launch supplies its explicit caller-owned lifecycle output", () => {
  const source = readFileSync(resolve(import.meta.dir, "../../../../../../../../../../.vscode/🧩️launch.seed.jsonc"), "utf8");
  const seed = Bun.JSONC.parse(source);
  expect(seed).toEqual(require("jsonc-parser").parse(source));
  if (seed === null || typeof seed !== "object" || !("configurations" in seed) || !Array.isArray(seed.configurations)) throw new Error("launch seed configurations are an explicit array");
  const launches = seed.configurations.filter((row: { command?: string }) => row.command === "bun nx run @semio-tech/framework-renderer-wgpu:test-worker-cell --skip-nx-cache");
  expect(launches).toHaveLength(1);
  expect(launches[0].presentation.order).toBe(900.05816);
  expect(launches[0].env.SEMIO_TEST_ARTIFACT_DIR).toBe("${workspaceFolder}/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🗑️generated/current-native-worker/portable-launch-artifacts");
});
