import { createRequire } from "node:module";
import { existsSync, readFileSync, realpathSync } from "node:fs";
import { dirname, join, relative } from "node:path";
import { fileURLToPath } from "node:url";
import schema from "./🧬️schema/🔣️.json" with { type: "json" };

const DECLARATION = schema.definitions.Declaration.const, API = schema.definitions.Api.const, TEXTS = schema.definitions.Texts.const;
const own = createRequire(import.meta.url), here = dirname(fileURLToPath(import.meta.url)), loaded = new Map();

const readJson = (file) => { try { return JSON.parse(readFileSync(file, "utf8")); } catch { return null; } };
const rootOf = (start) => { for (let at = start; ; at = dirname(at)) { if (existsSync(join(at, DECLARATION.rootMarker))) return at; if (dirname(at) === at) return start; } };
const anchors = (options) => {
  const defaultRoot = rootOf(here), defaultParser = join(here, ...DECLARATION.manifest.slice(0, -1)), root = options?.root ?? defaultRoot;
  return { root, parserDirectory: options?.parserDirectory ?? (options?.root === undefined ? defaultParser : join(root, relative(defaultRoot, defaultParser))) };
};
const refuse = (code, fields) => {
  const fill = (language) => TEXTS[language][code].replace(/\{(\w+)\}/gu, (_all, name) => String(name === "missing" ? fields.missing.join(", ") : fields[name] ?? "-"));
  const refusal = Object.freeze({ code, retry: "never", wanted: fields.wanted, resolved: fields.resolved, found: fields.found, missing: fields.missing, tried: fields.tried, en: fill("en"), de: fill("de") });
  return refusal;
};

/** 🚫️ The typed refusal of the tooling compiler: `refusal` is the closed record of the schema, `retry` is always `never`, and the message carries English and German. @see ./🧬️schema/🔣️.json */
export class TypescriptCompilerRefusalV1 extends Error {
  constructor(refusal) { super(refusal.en + " / " + refusal.de); this.name = "TypescriptCompilerRefusalV1"; this.code = refusal.code; this.retry = "never"; this.refusal = refusal; }
}

const candidates = (anchor) => DECLARATION.candidates.map((name) => join(name === "parser" ? anchor.parserDirectory : anchor.root, "node_modules", DECLARATION.package));
const describe = (directory) => {
  const manifest = readJson(join(directory, "package.json"));
  return { path: existsSync(directory) ? realpathSync(directory) : directory, found: manifest?.name === DECLARATION.package && typeof manifest.version === "string" ? manifest.version : null, main: typeof manifest?.main === "string" ? manifest.main : "index.js", missing: [] };
};
const declared = (anchor) => {
  const manifest = readJson(join(anchor.parserDirectory, DECLARATION.manifest.at(-1))), value = manifest?.[DECLARATION.field]?.[DECLARATION.package];
  return { path: join(anchor.parserDirectory, DECLARATION.manifest.at(-1)), value: typeof value === "string" && new RegExp(DECLARATION.exactVersion, "u").test(value) ? value : null, raw: typeof value === "string" ? value : null };
};
const plan = (anchor) => {
  const wanted = declared(anchor);
  if (wanted.value === null) return { refusal: refuse("typescript-compiler-declaration", { wanted: null, resolved: wanted.path, found: wanted.raw, missing: [], tried: [] }) };
  const rows = candidates(anchor).map(describe), tried = rows.map(({ path, found, missing }) => ({ path, found, missing })), match = rows.filter((row) => row.found === wanted.value);
  if (match.length > 0) return { wanted: wanted.value, match, tried };
  const first = rows.find((row) => row.found !== null);
  return { refusal: first === undefined ? refuse("typescript-compiler-absent", { wanted: wanted.value, resolved: rows[0].path, found: null, missing: [], tried }) : refuse("typescript-compiler-version", { wanted: wanted.value, resolved: first.path, found: first.found, missing: [], tried }) };
};
const missingApi = (compiler) => [...API.functions.filter((name) => typeof compiler?.[name] !== "function"), ...API.objects.filter((name) => compiler?.[name] === null || typeof compiler?.[name] !== "object")];

/** 🔎️ Reads the two package manifests and answers whether the declared compiler is installed where the tooling loads it, without loading the compiler: `{ok:true, path, version, wanted}` or `{ok:false, refusal}`. */
export function inspectTypescriptCompilerV1(options) {
  const result = plan(anchors(options));
  return result.refusal === undefined ? { ok: true, path: result.match[0].path, version: result.match[0].found, wanted: result.wanted } : { ok: false, refusal: result.refusal };
}

/** 📍️ The path of the main file of the declared compiler, checked by its manifests alone (version and name), without loading it; throws `TypescriptCompilerRefusalV1` when the declared compiler is not installed. */
export function typescriptCompilerFileV1(options) {
  const result = plan(anchors(options));
  if (result.refusal !== undefined) throw new TypescriptCompilerRefusalV1(result.refusal);
  return join(result.match[0].path, result.match[0].main);
}

/** 🔷️ Loads the TypeScript compiler API of the tooling: the exact version the parser package declares, from the install slot of the parser package or the direct dependency of the workspace root, never by the bare name; checks the version and the API the tooling uses and otherwise throws `TypescriptCompilerRefusalV1`. */
export function typescriptCompilerV1(options) {
  const result = plan(anchors(options));
  if (result.refusal !== undefined) throw new TypescriptCompilerRefusalV1(result.refusal);
  let first;
  for (const row of result.match) {
    const key = row.path + "\0" + result.wanted;
    if (loaded.has(key)) return loaded.get(key);
    let compiler;
    try { compiler = own(join(row.path, row.main)); } catch { compiler = undefined; }
    const missing = compiler === undefined ? [...API.functions, ...API.objects] : missingApi(compiler);
    if (missing.length === 0 && compiler.version === result.wanted) { loaded.set(key, compiler); return compiler; }
    first ??= refuse("typescript-compiler-api", { wanted: result.wanted, resolved: row.path, found: compiler?.version ?? row.found, missing, tried: result.tried });
  }
  throw new TypescriptCompilerRefusalV1(first);
}
