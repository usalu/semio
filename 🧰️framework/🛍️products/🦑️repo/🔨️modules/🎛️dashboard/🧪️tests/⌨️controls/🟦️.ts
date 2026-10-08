import { expect, test } from "bun:test";
import Ajv from "ajv";
import { readFileSync } from "node:fs";

const read = (path: string) => JSON.parse(readFileSync(new URL(path, import.meta.url), "utf8"));
const keymap = read("../../⚙️preferences/⌨️keymap/🔣️.json");
const keymapSchema = read("../../🧬️schema/⚙️preferences/⌨️keymap/🔣️.json");
const preferenceSchema = read("../../🧬️schema/⚙️preferences/🔣️.json");
const vectors = read("../../🧫️fixtures/⌨️keymap/🔣️.json");

type Pressed = { name?: string; sequence: string; ctrl: boolean; meta: boolean; shift: boolean };
type Binding = { scope: string; action: string; keys: string[] };

const NAMES: Record<string, string> = { return: "enter", escape: "esc", backspace: "backspace", delete: "delete", insert: "insert", up: "up", down: "down", left: "left", right: "right", home: "home", end: "end", pageup: "pageup", pagedown: "pagedown", tab: "tab" };

const spellOut = (key: Pressed): string => {
  const named = key.name === "tab" && key.shift ? "backtab" : key.name && (NAMES[key.name] ?? (/^f\d+$/.test(key.name) ? key.name : undefined));
  const base = named ?? (key.ctrl ? (key.sequence === "\u0000" ? "space" : key.name!) : key.sequence.slice(-1));
  return `${key.ctrl ? "ctrl+" : ""}${key.meta ? "alt+" : ""}${base}`;
};

const MODIFIERS = new Set(["ctrl", "alt", "shift"]);
const KEY_NAMES = new Set(["enter", "esc", "tab", "backtab", "backspace", "delete", "insert", "up", "down", "left", "right", "home", "end", "pageup", "pagedown", "space"]);

const parse = (text: string): string | undefined => {
  const parts = text.split("+");
  const mods = new Set<string>();
  while (parts.length > 1 && MODIFIERS.has(parts[0]!) && parts.slice(1).join("+") !== "") mods.add(parts.shift()!);
  let key = parts.join("+");
  if (key.length === 0 || (parts.length > 1 && key !== "+")) return undefined;
  if ([...key].length > 1) {
    const fn = /^f(\d+)$/.exec(key);
    if (!KEY_NAMES.has(key) && !(fn && Number(fn[1]) >= 1 && Number(fn[1]) <= 24 && String(Number(fn[1])) === fn[1])) return undefined;
  } else if (/\s/.test(key)) return undefined;
  if (key === "tab" && mods.has("shift")) { key = "backtab"; mods.delete("shift"); }
  if (key === "backtab") mods.delete("shift");
  if ([...key].length === 1) {
    if (mods.has("shift")) { if (mods.has("ctrl") || !/[a-z]/i.test(key)) return undefined; key = key.toUpperCase(); mods.delete("shift"); }
    if (mods.has("ctrl")) { if (!/[a-z]/i.test(key)) return undefined; key = key.toLowerCase(); if ("ijmh".includes(key)) return undefined; }
  }
  return `${mods.has("ctrl") ? "ctrl+" : ""}${mods.has("alt") ? "alt+" : ""}${mods.has("shift") ? "shift+" : ""}${key}`;
};

const prefixOf = (text: string): string | undefined => {
  const spec = parse(text);
  if (spec === undefined) return undefined;
  return /^(ctrl|alt)\+|\+(ctrl|alt)\+/.test(spec) || /^(shift\+)?([a-z]{2,}\d*|f\d+)$/.test(spec) || spec === "ctrl+space" ? spec : undefined;
};

test("Ajv independently validates the shipped keymap and every preference vector that binds keys", () => {
  const validateKeymap = new Ajv({ strict: false }).compile(keymapSchema);
  expect(validateKeymap(keymap), JSON.stringify(validateKeymap.errors)).toBe(true);
  const validateChange = new Ajv({ strict: false }).compile(preferenceSchema);
  for (const spelling of vectors.spellings) expect(validateChange({ bindings: { "prefix.split-right": [spelling.spec] } }), spelling.spec).toBe(true);
  expect(validateChange({ prefix: "ctrl+a" })).toBe(true);
  expect(validateChange({ prefix: "x" })).toBe(false);
  expect(validateChange({ bindings: { "nothing.split": ["v"] } })).toBe(false);
  expect(new Set(keymap.bindings.map((row: Binding) => `${row.scope}.${row.action}`)).size).toBe(keymap.bindings.length);
});

test("an independent spelling of keys folds every vector into the same canonical form", () => {
  for (const row of vectors.spellings) expect(parse(row.spec), row.spec).toBe(row.canonical);
  for (const text of vectors.rejected) expect(parse(text), JSON.stringify(text)).toBeUndefined();
});

test("an independent keymap builder resolves every customization vector like the native one", () => {
  const defaults: Binding[] = keymap.bindings;
  for (const row of [vectors.defaults, ...vectors.cases]) {
    const problems: string[] = [];
    const table = new Map<string, string[]>(defaults.map((binding) => [`${binding.scope}.${binding.action}`, binding.keys.map((key) => parse(key)!)]));
    const base = new Map(table);
    let prefix = parse(keymap.prefix)!;
    const changed = new Set<string>();
    const wanted = prefixOf(row.prefix);
    if (wanted === undefined) problems.push("prefix");
    else if (wanted !== prefix) { prefix = wanted; changed.add("prefix"); }
    for (const [id, keys] of Object.entries<string[]>(row.overrides ?? {})) {
      const scope = id.split(".")[0];
      const parsed = keys.map(parse);
      const typing = parsed.some((key) => key !== undefined && scope !== "prefix" && [...key].length === 1);
      if (!table.has(id) || keys.length === 0 || parsed.includes(undefined) || typing || new Set(parsed).size !== parsed.length) { problems.push(id); continue; }
      table.set(id, parsed as string[]);
      changed.add(id);
    }
    for (;;) {
      let clash: string[] | undefined;
      for (const scope of ["prefix", "window", "view"]) {
        const holders = new Map<string, string[]>([[prefix, ["prefix"]]]);
        for (const [id, keys] of table) if (id.startsWith(`${scope}.`)) for (const key of keys) holders.set(key, [...(holders.get(key) ?? []), id]);
        clash = [...holders.values()].find((ids) => ids.length > 1);
        if (clash) break;
      }
      if (!clash) break;
      const culprits = clash.filter((id) => changed.has(id));
      if (culprits.length === 0) break;
      for (const id of culprits) {
        changed.delete(id);
        if (id === "prefix") prefix = parse(keymap.prefix)!;
        else table.set(id, base.get(id)!);
      }
      problems.push(culprits.join(","));
    }
    for (const probe of row.resolve) {
      const key = parse(probe.key)!;
      const hit = probe.scope === "prefix" && key === prefix ? "send-prefix" : [...table].find(([id, keys]) => id.startsWith(`${probe.scope}.`) && keys.includes(key))?.[0].split(".")[1] ?? null;
      expect(hit, JSON.stringify(probe)).toBe(probe.action);
    }
    expect(problems.length, JSON.stringify(row.name ?? "defaults")).toBe(row.problems ?? 0);
  }
});

test("Node readline independently decodes terminal bytes into the keymap's canonical spellings", async () => {
  const source = "const {PassThrough}=require('node:stream'),{emitKeypressEvents}=require('node:readline'),rows=JSON.parse(process.argv[1]),out=[];(async()=>{for(const row of rows){const input=new PassThrough(),events=[];emitKeypressEvents(input);input.on('keypress',(_,key)=>events.push({name:key.name,sequence:key.sequence,ctrl:Boolean(key.ctrl),meta:Boolean(key.meta),shift:Boolean(key.shift)}));input.write(Buffer.from(row.bytes));await new Promise((resolve)=>setImmediate(resolve));input.end();out.push(events)}console.log(JSON.stringify(out))})();";
  const child = Bun.spawn(["node", "-e", source, JSON.stringify(vectors.terminal)], { stdout: "pipe", stderr: "pipe" });
  const output = await new Response(child.stdout).text();
  expect(await child.exited, await new Response(child.stderr).text()).toBe(0);
  const decoded: Pressed[][] = JSON.parse(output);
  vectors.terminal.forEach((row: { spec: string }, index: number) => {
    expect(decoded[index]!.length, row.spec).toBe(1);
    expect(spellOut(decoded[index]![0]!), row.spec).toBe(row.spec);
  });
});

test("every scenario of the keymap feature is proved by a test of this file and by the named Rust tests", () => {
  const feature = readFileSync(new URL("../⌨️keymap/🥒️.feature", import.meta.url), "utf8");
  const own = readFileSync(new URL(import.meta.url), "utf8");
  const rust = ["../../⚙️preferences/⌨️keymap/🧪️tests/🔬️unit/🦀️.rs", "../../🖥️terminal/🧪️tests/🔬️unit/🦀️.rs"].map((path) => readFileSync(new URL(path, import.meta.url), "utf8")).join("\n");
  const proofs: [string, string[], string[]][] = [
    ["Key spellings fold into one canonical key", ["an independent spelling of keys folds every vector into the same canonical form"], ["spellings_fold_into_one_canonical_key_with_a_readable_label"]],
    ["The keymap is data with action ids", ["Ajv independently validates the shipped keymap and every preference vector that binds keys"], ["shipped_defaults_resolve_per_the_shared_vectors_and_never_collide"]],
    ["Customizations fold over the defaults without losing a key", ["an independent keymap builder resolves every customization vector like the native one"], ["customizations_fold_over_the_defaults_and_collisions_are_named"]],
    ["Every action stays reachable by keyboard under any customization", [], ["every_action_stays_reachable_by_keyboard_under_any_customization"]],
    ["Terminal bytes become the canonical key spellings of the keymap", ["Node readline independently decodes terminal bytes into the keymap's canonical spellings"], ["terminal_bytes_become_the_keymaps_canonical_spellings"]],
  ];
  expect([...feature.matchAll(/^\s*Scenario:\s*(.+?)\s*$/gm)].map((match) => match[1])).toEqual(proofs.map(([title]) => title));
  for (const [title, bun, native] of proofs) {
    expect(bun.length + native.length, title).toBeGreaterThan(0);
    for (const name of bun) expect(own.includes(`test(${JSON.stringify(name)}`), `${title}: ${name}`).toBe(true);
    for (const name of native) expect(rust.slice(0, rust.indexOf(`fn ${name}(`)).trimEnd().endsWith("#[test]"), `${title}: ${name}`).toBe(true);
  }
});
