import ts from "typescript";
import { Database } from "bun:sqlite";
import "../../📂️workspace/🖥️view/🧪️tests/🟦️.ts";
import "../../🖥️terminal/📣️feedback/🧪️tests/🟦️.ts";
import "../../🖥️terminal/⚙️settings/⌨️keymap/🧪️tests/🟦️.ts";
import { expect, test } from "bun:test";
import Ajv from "ajv";
import { readFileSync } from "node:fs";
import { readModuleTreeSource } from "@semio-tech/framework-compiler-ts/syntax/module-tree";

const read = (path: string) => JSON.parse(readFileSync(new URL(path, import.meta.url), "utf8"));
const keymap = read("../../⚙️preferences/⌨️keymap/🔣️.json");
const keymapSchema = read("../../🧬️schema/⚙️preferences/⌨️keymap/🔣️.json");
const preferenceSchema = read("../../🧬️schema/⚙️preferences/🔣️.json");
const vectors = read("../🧫️fixtures/⌨️keymap/🔣️.json");

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
  const rust = ["../../⚙️preferences/⌨️keymap/🧪️tests/🔬️unit/🦀️.rs", "../../🖥️terminal/🧪️tests/🔬️unit/🦀️.rs"].map((path) => readModuleTreeSource(new URL(path, import.meta.url))).join("\n");
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


test("registered window native selections retain source feedback settings and semantic owner coverage", () => {
  const fixture = read("../../🧫️fixtures/🪟️windows/🎟️coverage/🔣️.json");
  const validate = new Ajv({ strict: true }).compile(read("../../🧬️schema/🪟️windows/🎟️coverage/🔣️.json"));
  expect(validate(fixture)).toBe(true);
  expect(validate({ ...fixture, unknown: true })).toBe(false);
  const missing = structuredClone(fixture); missing.native[0].required.pop();
  expect(validate(missing)).toBe(false);
  const source = readFileSync(new URL("../../📦️packages/🦀️rust/📜️script.ts", import.meta.url), "utf8");
  const parsed = ts.createSourceFile("dashboard.ts", source, ts.ScriptTarget.Latest, true, ts.ScriptKind.TS);
  expect((parsed as ts.SourceFile & { parseDiagnostics: readonly ts.Diagnostic[] }).parseDiagnostics).toEqual([]);
  const literals = new Map<string, string>();
  const strings = (node: ts.Node | undefined): string[] => {
    if (!node || !ts.isArrayLiteralExpression(node)) throw Error("Expected authored literal selection");
    return node.elements.map((item) => {
      if (ts.isStringLiteral(item)) return item.text;
      if (ts.isIdentifier(item) && literals.has(item.text)) return literals.get(item.text)!;
      throw Error("Expected local immutable string selection");
    });
  };
  const units: string[][] = [], selections: { packages: string[]; extraArgs: string[] }[][] = [];
  const visit = (node: ts.Node, withinWindows = false): void => {
    const windowBranch = ts.isIfStatement(node) && ts.isBinaryExpression(node.expression) && node.expression.operatorToken.kind === ts.SyntaxKind.EqualsEqualsEqualsToken && ts.isStringLiteral(node.expression.right) && node.expression.right.text === "windows";
    if (windowBranch) { visit(node.thenStatement, true); return; }
    if (ts.isVariableDeclaration(node) && ts.isIdentifier(node.name)) {
      if (node.initializer && ts.isStringLiteral(node.initializer) && ts.isVariableDeclarationList(node.parent) && (node.parent.flags & ts.NodeFlags.Const)) literals.set(node.name.text, node.initializer.text);
      if (node.name.text === "crates") units.push(strings(node.initializer));
      if (withinWindows && node.name.text === "selections") {
        if (!node.initializer || !ts.isArrayLiteralExpression(node.initializer)) throw Error("Expected native child selections");
        selections.push(node.initializer.elements.map((item) => {
          if (!ts.isObjectLiteralExpression(item)) throw Error("Expected native child object");
          const fields = new Map(item.properties.map((property) => {
            if (!ts.isPropertyAssignment(property) || !ts.isIdentifier(property.name)) throw Error("Expected native child field");
            return [property.name.text, property.initializer] as const;
          }));
          return { packages: strings(fields.get("packages")), extraArgs: strings(fields.get("extraArgs")) };
        }));
      }
    }
    ts.forEachChild(node, (child) => visit(child, withinWindows));
  };
  visit(parsed);
  expect(units).toHaveLength(1); expect(selections).toHaveLength(1); expect(selections[0]).toHaveLength(2);
  const first = selections[0]![0]!;
  expect(first.extraArgs.slice(0, 2)).toEqual(["--lib", "-E"]);
  expect(first.extraArgs).toHaveLength(3);
  const expression = first.extraArgs[2]!;
  expect(expression).toMatch(/^test\([a-z0-9_]+\)(?:\s*\|\s*test\([a-z0-9_]+\))*$/);
  const filters = [...expression.matchAll(/test\(([a-z0-9_]+)\)/g)].map((match) => match[1]!);
  const own = ts.createSourceFile("controls.ts", readFileSync(new URL(import.meta.url), "utf8"), ts.ScriptTarget.Latest, true, ts.ScriptKind.TS);
  const imports = own.statements.filter(ts.isImportDeclaration).map((node) => (node.moduleSpecifier as ts.StringLiteral).text);
  const roster = fixture.native.flatMap((owner: { package: string; source: string; required: string[] }) => [...readModuleTreeSource(new URL(owner.source, import.meta.url)).matchAll(/#\[test\]\s*fn\s+([a-z0-9_]+)\s*\(/g)].map((match) => ({ package: owner.package, name: match[1]! })));
  const db = new Database(":memory:");
  try {
    const admit = (unitPackages: string[], packages: string[], tokens: string[], entries: string[]): boolean => {
      if (!fixture.unitPackages.every((name: string) => unitPackages.includes(name)) || !fixture.native.every((row: { package: string }) => packages.includes(row.package)) || !fixture.retainedFilters.every((token: string) => tokens.includes(token)) || !entries.includes(fixture.neutralEntry)) return false;
      const actual = roster.filter((row: { package: string; name: string }) => packages.includes(row.package) && tokens.some((token) => row.name.includes(token))).map((row: { package: string; name: string }) => row.package + "::" + row.name).sort();
      const oracle = db.query("select json_extract(r.value,'$.package') || '::' || json_extract(r.value,'$.name') identity from json_each(?1) r where exists(select 1 from json_each(?2) p where p.value=json_extract(r.value,'$.package')) and exists(select 1 from json_each(?3) f where instr(json_extract(r.value,'$.name'),f.value)>0) order by identity").all(JSON.stringify(roster), JSON.stringify(packages), JSON.stringify(tokens)) as { identity: string }[];
      expect(actual).toEqual(oracle.map((row) => row.identity));
      return fixture.native.every((owner: { package: string; required: string[] }) => owner.required.every((name) => actual.includes(owner.package + "::" + name)));
    };
    expect(admit(units[0]!, first.packages, filters, imports)).toBe(true);
    for (const name of fixture.unitPackages) expect(admit(units[0]!.filter((value) => value !== name), first.packages, filters, imports)).toBe(false);
    for (const owner of fixture.native) expect(admit(units[0]!, first.packages.filter((value) => value !== owner.package), filters, imports)).toBe(false);
    for (const token of ["source_", ...fixture.native[0].required.slice(9)]) expect(admit(units[0]!, first.packages, filters.filter((value) => value !== token), imports)).toBe(false);
    expect(admit(units[0]!, first.packages, filters, imports.filter((value) => value !== fixture.neutralEntry))).toBe(false);
    const settlement = read("../../📂️workspace/🖥️view/🧪️tests/🧫️fixtures/🔣️.json");
    expect(settlement.cases.map((row: { name: string }) => row.name)).toEqual(fixture.settlementCases);
  } finally { db.close(); }
});
