/** 🗝️ The implemented keys of the LaTeX visualization packages: the scanned l3keys declarations, the key paths a helper reaches and the keys each family implements. */
import { readFileSync, readdirSync } from "node:fs";
import { join } from "node:path";
import { VIZ_LATEX_DIR } from "../🟦️.ts";

//#region 🔖️Coverage
//#region 🔖️ImplementedKeys
/** ✂️ The balanced `{…}` group opening at `open`, with TeX comments skipped inside it. */
export function texGroup(text: string, open: number): { body: string; end: number } {
  let depth = 0;
  for (let index = open; index < text.length; index += 1) {
    const char = text[index]!;
    if (char === "\\") {
      index += 1;
      continue;
    }
    if (char === "%") {
      while (index < text.length && text[index] !== "\n") index += 1;
      continue;
    }
    if (char === "{") depth += 1;
    else if (char === "}") {
      depth -= 1;
      if (depth === 0) return { body: text.slice(open + 1, index), end: index + 1 };
    }
  }
  throw new Error("unbalanced TeX group while reading an l3keys block");
}

/** ⏭️ Index of the next non-space character at or after `from`. */
export function texSkipSpace(text: string, from: number): number {
  let index = from;
  while (index < text.length && /\s/.test(text[index]!)) index += 1;
  return index;
}

/** 🔑️ A stored unknown-result option is a control; forwarding and empty unknown handlers are not. */
export function vizDeclaredKeyNames(body: string): string[] {
  const names = new Set<string>();
  const code = body.replace(/(^|[^\\])%[^\n]*/g, "$1");
  for (const match of code.matchAll(/(^|,)\s*([A-Za-z][A-Za-z0-9-]*)\s*\./gm)) names.add(match[2]!);
  const unknown = /(?:^|,)\s*unknown\s*\.([A-Za-z_:]+)\s*=/gm;
  let stored = false;
  for (let match = unknown.exec(code); match !== null; match = unknown.exec(code)) {
    if (match[1] !== "code:n") { stored = true; continue; }
    const open = texSkipSpace(code, unknown.lastIndex);
    if (code[open] === "{" && /\\(?:(?:tl|fp|int|bool|clist)_(?:g)?set:N[nxV]\s+\\[A-Za-z_]+\s*\{[^}]*#1|prop_(?:g)?put:Nnn\s+\\[A-Za-z_]+\s*\{\s*unknown\s*\}\s*\{[^}]*#1)/.test(texGroup(code, open).body)) stored = true;
  }
  if (!stored) names.delete("unknown");
  return [...names];
}

type KeyPath = string;
/** 🧭️ Expl3 ignores whitespace within a key namespace. */
export function vizKeyPath(value: string): KeyPath { return value.trim().replace(/\s*\/\s*/g, " / "); }
type PathTemplate = { readonly template: string; readonly keys: readonly string[] };
type KeysUnit = { templates: PathTemplate[]; literals: { path: KeyPath; keys: string[] }[]; setPaths: Set<KeyPath>; calls: { macro: string; argument: string }[] };
export type KeysModel = {
  literal: Map<KeyPath, Set<string>>;
  helpers: Map<string, { templates: PathTemplate[]; forwards: string[] }>;
  installs: { argument: string; macro: string }[];
  families: Map<string, string>;
  macros: Map<string, string>;
};

/** 🔍️ Every l3keys declaration, application and helper call one stretch of expl3 code carries.
 * A path written in terms of the enclosing macro's `#1` stays a template, so the call site decides
 * which family it lands on; an `unknown` handler that forwards to another path is kept as a marker
 * and resolved once the whole model is built. */
export function scanKeys(text: string): KeysUnit {
  const found: KeysUnit = { templates: [], literals: [], setPaths: new Set(), calls: [] };
  const define = /\\keys_define:nn\s*\{/g;
  for (let match = define.exec(text); match !== null; match = define.exec(text)) {
    const head = texGroup(text, match.index + match[0].length - 1);
    const argument = vizKeyPath(head.body);
    const bodyOpen = texSkipSpace(text, head.end);
    if (text[bodyOpen] !== "{") continue;
    const block = texGroup(text, bodyOpen).body;
    const keys = vizDeclaredKeyNames(block);
    for (const handler of block.matchAll(/unknown\s*\.code:n\s*=\s*\{/g)) {
      const body = texGroup(block, handler.index! + handler[0].length - 1).body;
      for (const forward of body.matchAll(/\\keys_set:nn\s*\{([^}]*)\}/g)) keys.push(`@forward:${vizKeyPath(forward[1]!)}`);
    }
    if (argument.includes("#1")) found.templates.push({ template: argument, keys });
    else found.literals.push({ path: argument, keys });
    define.lastIndex = bodyOpen;
  }
  const routes = /\\keys_set_known:n/.test(text);
  const apply = /\\keys_set(?:_known)?:n[nVx]N?\s*\{/g;
  for (let match = apply.exec(text); match !== null; match = apply.exec(text)) {
    const head = texGroup(text, match.index + match[0].length - 1);
    const listOpen = texSkipSpace(text, head.end);
    const list = text[listOpen] === "{" ? texGroup(text, listOpen).body : "\\";
    if (routes || /#\d/.test(list)) found.setPaths.add(vizKeyPath(head.body));
    apply.lastIndex = head.end;
  }
  const indirect = /\\exp_args:NV\s*\\keys_define:nn\s*\\[A-Za-z_@]+\s*\{/g;
  for (let match = indirect.exec(text); match !== null; match = indirect.exec(text)) {
    const open = match.index + match[0].length - 1;
    found.templates.push({ template: "#1", keys: vizDeclaredKeyNames(texGroup(text, open).body) });
    indirect.lastIndex = open;
  }
  const call = /\\(semio_viz_[a-z0-9_]+):([a-zA-Z]*)/g;
  for (let match = call.exec(text); match !== null; match = call.exec(text)) {
    const after = texSkipSpace(text, match.index + match[0].length);
    const takesGroup = match[2] === "n" && text[after] === "{";
    found.calls.push({ macro: match[1]!, argument: takesGroup ? texGroup(text, after).body.trim() : "" });
  }
  const lists = new Map<string, readonly string[]>();
  for (const match of text.matchAll(/\\clist_const:Nn\s*\\([A-Za-z_]+)\s*\{/g)) {
    const values = texGroup(text, match.index! + match[0].length - 1).body;
    if (!values.includes("#")) lists.set(match[1]!, values.split(",").map(value => value.trim()).filter(Boolean));
  }
  for (const match of text.matchAll(/\\clist_map_inline:Nn\s*\\([A-Za-z_]+)\s*\{/g)) {
    const body = texGroup(text, match.index! + match[0].length - 1).body;
    for (const value of lists.get(match[1]!) ?? []) {
      const installed = scanKeys(body.replace(/(?<!#)#1\b/g, value));
      found.literals.push(...installed.literals);
      found.templates.push(...installed.templates);
    }
  }
  return found;
}

/** 🧱 Blanks out the given ranges so the file-level scan does not see a macro body twice. */
function maskRanges(text: string, ranges: readonly { start: number; end: number }[]): string {
  let masked = text;
  for (const range of ranges) masked = `${masked.slice(0, range.start)}${" ".repeat(range.end - range.start)}${masked.slice(range.end)}`;
  return masked;
}

/** 🗺️ The whole l3keys picture of `🖋️latex`: which path carries which keys, which helper installs
 * keys on a path it is handed, and what each `\SemioVizFamily` body does with its option list. */
export function vizKeysModel(sources?: readonly string[]): KeysModel {
  const literal = new Map<KeyPath, Set<string>>();
  const helpers = new Map<string, { templates: PathTemplate[]; forwards: string[] }>();
  const installs: { argument: string; macro: string }[] = [];
  const families = new Map<string, string>();
  const macros = new Map<string, string>();

  for (const text of sources ?? readdirSync(VIZ_LATEX_DIR).filter(file => file.endsWith(".sty")).map(file => readFileSync(join(VIZ_LATEX_DIR, file), "utf8"))) {
    const definition = /\\cs_(?:new|set|new_protected|set_protected|generate_variant)[a-z_]*:Npn\s+\\([A-Za-z@_]+:[A-Za-z]*)\s*((?:#\d\s*)*)\{/g;
    const bodies: { name: string; body: string; start: number; end: number }[] = [];
    for (let match = definition.exec(text); match !== null; match = definition.exec(text)) {
      const open = match.index + match[0].length - 1;
      const found = texGroup(text, open);
      bodies.push({ name: match[1]!, body: found.body, start: match.index, end: found.end });
      definition.lastIndex = found.end;
    }
    for (const entry of bodies) macros.set(entry.name, entry.body);
    for (const entry of bodies) {
      const scanned = scanKeys(entry.body);
      const base = entry.name.replace(/:.*$/, "");
      const helper = helpers.get(base) ?? { templates: [], forwards: [] };
      helper.templates.push(...scanned.templates);
      for (const call of scanned.calls) if (call.argument === "#1" || call.argument === "") helper.forwards.push(call.macro);
      helpers.set(base, helper);
    }

    const scanned = scanKeys(maskRanges(text, bodies));
    for (const entry of scanned.literals) {
      const bag = literal.get(entry.path) ?? new Set<string>();
      for (const key of entry.keys) bag.add(key);
      literal.set(entry.path, bag);
    }
    for (const call of [...scanned.calls, ...bodies.flatMap((entry) => scanKeys(entry.body).calls)]) {
      if (call.argument.length > 0 && !call.argument.includes("#")) installs.push({ argument: call.argument, macro: call.macro });
    }

    const family = /\\(?:SemioVizFamily|semio_viz_family_define:nn)\s*\{\s*([a-z0-9-]+)\s*\}\s*\{/g;
    for (let match = family.exec(text); match !== null; match = family.exec(text)) {
      const open = match.index + match[0].length - 1;
      const found = texGroup(text, open);
      family.lastIndex = found.end;
      if (text.slice(text.lastIndexOf("\n", match.index) + 1, match.index).includes("%")) continue;
      families.set(match[1]!, found.body);
    }
  }
  return { literal, helpers, installs, families, macros };
}

/** 🪝️ The keys a helper installs on `path` when it is called with `argument`. */
export function helperKeysOn(model: KeysModel, macro: string, argument: string, path: KeyPath, seen: Set<string>): Set<string> {
  if (seen.has(macro)) return new Set();
  seen.add(macro);
  const helper = model.helpers.get(macro);
  const keys = new Set<string>();
  if (helper === undefined) return keys;
  for (const entry of helper.templates) {
    if (vizKeyPath(entry.template.replaceAll("#1", argument)) !== vizKeyPath(path)) continue;
    for (const key of entry.keys) keys.add(key);
  }
  for (const forward of helper.forwards) for (const key of helperKeysOn(model, forward, argument, path, seen)) keys.add(key);
  return keys;
}

/** 🧾️ A helper's documented vocabulary includes the declarations of helpers it calls. */
export function helperDeclaredKeyNames(model: KeysModel, macro: string, seen: Set<string>): Set<string> {
  if (seen.has(macro)) return new Set();
  seen.add(macro);
  const helper = model.helpers.get(macro), keys = new Set(helper?.templates.flatMap(template => [...template.keys]) ?? []);
  for (const forward of helper?.forwards ?? []) for (const key of helperDeclaredKeyNames(model, forward, seen)) keys.add(key);
  for (const key of [...keys]) if (key.startsWith("@forward:")) {
    keys.delete(key);
    for (const forwarded of keysOfPath(model, key.slice(9), new Set())) keys.add(forwarded);
  }
  return keys;
}

/** 📂️ Every key one l3keys path accepts: its own declarations, the ones a helper installed on it,
 * and the ones its `unknown` handler forwards to another path. */
export function keysOfPath(model: KeysModel, path: KeyPath, seen: Set<KeyPath>): Set<string> {
  path = vizKeyPath(path);
  if (seen.has(path)) return new Set();
  seen.add(path);
  const keys = new Set(model.literal.get(path) ?? []);
  for (const install of model.installs) for (const key of helperKeysOn(model, install.macro, install.argument, path, new Set())) keys.add(key);
  for (const key of [...keys]) {
    if (!key.startsWith("@forward:")) continue;
    keys.delete(key);
    for (const forwarded of keysOfPath(model, key.slice("@forward:".length), seen)) keys.add(forwarded);
  }
  return keys;
}

/** 🧵️ Follows authored option parameters and residual lists through native calls in execution order. */
export function pathsReachedBy(model: KeysModel, body: string, depth: number, seen: Set<string>, parameters = new Set([1]), aliases = new Set<string>()): Set<KeyPath> {
  const paths = new Set<KeyPath>();
  const authored = (value: string): boolean => [...value.matchAll(/(?<!#)#([1-9])/g)].some(match => parameters.has(Number(match[1]))) || [...aliases].some(alias => value.includes(alias));
  const argument = (position: number): { value: string; end: number } => {
    const start = texSkipSpace(body, position);
    if (body[start] === "{") { const group = texGroup(body, start); return { value: group.body, end: group.end }; }
    const token = /^\\[A-Za-z_@:]+/.exec(body.slice(start));
    return { value: token?.[0] ?? body[start] ?? "", end: start + (token?.[0].length ?? 1) };
  };
  const events = [...body.matchAll(/\\keys_set(?:_known)?:n[nVx]N?\s*\{|\\tl_(?:g)?(?:set(?:_eq)?|put_(?:left|right)):N[nxNV]\s*\\[A-Za-z_@:]+|\\tl_(?:g)?clear:N\s*\\[A-Za-z_@:]+|\\semio_viz_[a-z0-9_]+:[a-zA-Z]*/g)];
  for (const event of events) {
    const position = event.index! + event[0].length;
    if (event[0].startsWith("\\keys_set")) {
      const head = texGroup(body, position - 1), list = argument(head.end), origin = authored(list.value);
      if (origin) paths.add(vizKeyPath(head.body));
      if (event[0].startsWith("\\keys_set_known:")) {
        const residual = argument(list.end).value;
        if (origin) aliases.add(residual); else aliases.delete(residual);
      }
    } else if (event[0].startsWith("\\tl_")) {
      const variable = /\\[A-Za-z_@:]+$/.exec(event[0])![0], append = event[0].includes("put_");
      if (!event[0].includes("clear:") && authored(argument(position).value)) aliases.add(variable);
      else if (!append) aliases.delete(variable);
    } else {
      const name = event[0].slice(1), [base, signature] = name.split(":") as [string, string], forwarded = new Set<number>();
      let cursor = position;
      for (let index = 0; index < signature.length; index++) {
        const value = argument(cursor);
        if (authored(value.value)) forwarded.add(index + 1);
        cursor = value.end;
      }
      const definition = model.macros.has(name) ? name : [...model.macros.keys()].find(key => key.startsWith(base + ":") && key.split(":")[1]!.length === signature.length);
      if (!definition) continue;
      const macro = model.macros.get(definition)!;
      if (forwarded.size === 0 && ![...aliases].some(alias => macro.includes(alias))) continue;
      const identity = definition + "|" + [...forwarded].join(",") + "|" + [...aliases].sort().join(",");
      if (seen.has(identity)) continue;
      seen.add(identity);
      for (const path of pathsReachedBy(model, macro, depth + 1, seen, forwarded, aliases)) paths.add(path);
    }
  }
  return paths;
}

/** 🛤️ Resolves one exact native owner path, including its installed and forwarded controls. */
export function vizImplementedKeyPathNames(path: string): readonly string[] {
  return [...keysOfPath(vizKeysModel(), path, new Set())].sort();
}

/** 🎛️ Family names retain only their source-reachable controls and the registry variant. */
export function vizImplementedFamilyKeys(): Readonly<Record<string, readonly string[]>> {
  const model = vizKeysModel();
  const result: Record<string, readonly string[]> = {};
  for (const [name, body] of model.families) {
    const own = `semio / viz / family / ${name}`;
    const keys = new Set(keysOfPath(model, own, new Set()));
    for (const path of pathsReachedBy(model, body, 0, new Set())) {
      if (path === own || path.includes("#")) continue;
      for (const key of keysOfPath(model, path, new Set())) keys.add(key);
    }
    keys.add("variant");
    result[name] = [...keys].sort();
  }
  return result;
}

/** 🔗️ Only source-reachable key paths and installed helper vocabularies may supply inherited documentation. */
export function vizImplementedFamilyKeyScopes(sources?: readonly string[]): Readonly<Record<string, Readonly<Record<string, readonly string[]>>>> {
  const model = vizKeysModel(sources);
  const result: Record<string, Record<string, readonly string[]>> = {};
  for (const [name, body] of model.families) {
    const own = `semio / viz / family / ${name}`;
    const scopes: Record<string, readonly string[]> = {};
    const paths = new Set([own, ...pathsReachedBy(model, body, 0, new Set())]);
    for (const path of paths) {
      const declared = [...(model.literal.get(path) ?? []), ...model.installs.flatMap(install => [...helperKeysOn(model, install.macro, install.argument, path, new Set())])];
      for (const key of declared) if (key.startsWith("@forward:")) paths.add(key.slice(9));
    }
    for (const path of paths) {
      if (path.includes("#")) continue;
      const keys = [...keysOfPath(model, path, new Set())].sort();
      if (keys.length > 0) scopes[path] = keys;
    }
    for (const install of model.installs) {
      const keys = [...new Set([...paths].flatMap(path => [...helperKeysOn(model, install.macro, install.argument, path, new Set())]))].sort();
      if (keys.length > 0) scopes[`helper:${install.macro}`] = keys;
    }
    result[name] = scopes;
  }
  return result;
}
//#endregion 🔖️ImplementedKeys
//#endregion 🔖️Coverage
