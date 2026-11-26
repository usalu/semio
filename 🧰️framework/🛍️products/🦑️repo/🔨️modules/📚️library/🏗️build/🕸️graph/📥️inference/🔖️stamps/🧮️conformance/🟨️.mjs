import { createHash } from "node:crypto";
import { existsSync, mkdirSync, readFileSync, renameSync, rmSync, statSync, writeFileSync } from "node:fs";
import { dirname, join, relative, resolve } from "node:path";
import schema from "./🧬️schema/🔣️.json" with { type: "json" };
import corpus from "./🔣️.json" with { type: "json" };

/** 🗣️ The texts of the conformance stamps in every language: `{name}` placeholders, English first, German second. */
export const CONFORMANCE_TEXTS_V1 = schema.definitions.TextCatalogue.const;

/** 📜️ The admitted corpus manifest: per derived inferrer its entries, schemas, fixtures and runner. */
export const CONFORMANCE_CORPUS_V1 = Object.freeze(structuredClone(corpus));

const sorted = (value) => (Array.isArray(value) ? value.map(sorted) : value !== null && typeof value === "object" ? Object.fromEntries(Object.keys(value).sort().map((key) => [key, sorted(value[key])])) : value);
const SPEC = /(?:^|[\s;{}(])(?:import|export)\b[^"'`;]*?\bfrom\s*["']([^"']+)["']|(?:^|[\s;{}(])import\s*["']([^"']+)["']|\bimport\s*\(\s*["']([^"']+)["']\s*\)|\brequire\s*\(\s*["']([^"']+)["']\s*\)/gu;
const CANDIDATES = ["", ".ts", ".mjs", ".cjs", ".json", "/🟦️.ts", "/🟨️.mjs"];
const isFile = (path) => { try { return statSync(path).isFile(); } catch { return false; } };

/** 🧾️ Sorted keys at every depth and no whitespace: the form every language reproduces. */
export const canonicalJsonV1 = (value) => JSON.stringify(sorted(value));

/** 🔑️ The sha256 hex digest of bytes or text. */
export const sha256V1 = (data) => createHash("sha256").update(data).digest("hex");

/** 🧮️ Renders a text of the catalogue in one language. */
export function conformanceTextV1(key, language, fields = {}) {
  const row = CONFORMANCE_TEXTS_V1.texts[key]?.[language];
  if (row === undefined) throw Error("Conformance text required: " + key + " " + language);
  return row.replace(/\{(\w+)\}/gu, (_, name) => String(fields[name] ?? ""));
}

/** 🔬️ The sorted module specifiers one source file imports directly (static, dynamic, re-export and require). */
export function directImportsV1(text) {
  const out = new Set();
  for (const match of text.matchAll(SPEC)) out.add(match[1] ?? match[2] ?? match[3] ?? match[4]);
  return [...out].sort();
}

/** 🕸️ The transitive closure of relative imports below the entries, as sorted paths relative to `root`; bare and node specifiers are not followed (the import digest names them). */
export function closureFilesV1(entries, root, read = (path) => readFileSync(path, "utf8")) {
  const seen = new Set(), pending = entries.map((entry) => resolve(root, entry));
  while (pending.length) {
    const file = pending.pop();
    if (seen.has(file) || !isFile(file)) continue;
    seen.add(file);
    if (file.endsWith(".json")) continue;
    for (const specifier of directImportsV1(read(file))) {
      if (!specifier.startsWith(".")) continue;
      const base = resolve(dirname(file), specifier), hit = CANDIDATES.map((suffix) => base + suffix).find(isFile);
      if (hit) pending.push(hit);
    }
  }
  return [...seen].map((file) => relative(root, file)).sort();
}

/** 🏷️ The key a stored record lives under: the content digest of the closure, the schemas, the fixtures, the corpus manifest and the corpus code. */
export function closureKeyV1(id, files, shaOf) {
  return sha256V1(canonicalJsonV1({ id, files: files.map((file) => [file, shaOf(file)]) }));
}

/** 🪪️ The stamp a digest of corpus results makes of the declared stamp of an inferrer. */
export const stampOfV1 = (declared, parts) => declared + "+" + sha256V1(canonicalJsonV1(parts)).slice(0, 32);

/** 🧬️ The digest of the declared schemas of a provider: canonical content, so a formatting edit counts for nothing and a structural one counts. */
export function schemaDigestV1(schemas, root, read = (path) => readFileSync(path, "utf8")) {
  return sha256V1(canonicalJsonV1(schemas.map((path) => [path, JSON.parse(read(resolve(root, path)))])));
}

/** 📥️ The digest of the direct import lists of the entry files of a provider, so an added, removed or redirected import always counts. */
export function importsDigestV1(entries, root, read = (path) => readFileSync(path, "utf8")) {
  return sha256V1(canonicalJsonV1(entries.map((entry) => [entry, directImportsV1(read(resolve(root, entry)))])));
}

/** 🧪️ Evaluates the corpus of one provider with its runner: one digest per case, the schema and import digests, and what the cases covered. */
export async function evaluateProviderV1({ id, root, runners, provider = CONFORMANCE_CORPUS_V1.providers[id], read = (path) => readFileSync(path, "utf8") }) {
  const run = runners[provider.runner];
  if (typeof run !== "function") throw Error("Corpus runner required: " + provider.runner);
  const fixtures = Object.fromEntries(Object.entries(provider.fixtures).map(([name, path]) => [name, JSON.parse(read(resolve(root, path)))]));
  const result = await run(fixtures);
  return { cases: Object.fromEntries(Object.entries(result.cases).sort(([a], [b]) => (a < b ? -1 : a > b ? 1 : 0)).map(([name, output]) => [name, sha256V1(canonicalJsonV1(output === undefined ? null : output))])), covered: result.covered, schema: schemaDigestV1(provider.schemas, root, read), imports: importsDigestV1(provider.entries, root, read) };
}

/** 🛡️ True when a stored record is well formed, belongs to the key and its stamp is the one its parts make: a record that does not check is never trusted. */
export function recordHoldsV1(record, { id, key, declared }) {
  try {
    const parts = record?.parts;
    return record.version === 1 && record.id === id && record.key === key && parts !== null && typeof parts === "object" && typeof parts.schema === "string" && typeof parts.imports === "string" && parts.cases !== null && typeof parts.cases === "object" && Object.keys(parts.cases).length > 0 && Object.values(parts.cases).every((digest) => /^[0-9a-f]{64}$/u.test(digest)) && record.stamp === stampOfV1(declared, parts);
  } catch { return false; }
}

const recordPathV1 = (directory, id, key) => join(directory, id, key + ".json");

/** 💾️ Reads the local record of a key, or undefined when it is absent, damaged or does not hold. */
export function readRecordV1(directory, id, key, declared) {
  try {
    const record = JSON.parse(readFileSync(recordPathV1(directory, id, key), "utf8"));
    return recordHoldsV1(record, { id, key, declared }) ? record : undefined;
  } catch { return undefined; }
}

/** 📝️ Writes the record of a key atomically; a failing write only costs the next process its evaluation. */
export function writeRecordV1(directory, record) {
  const path = recordPathV1(directory, record.id, record.key), temporary = path + "." + process.pid + ".tmp";
  try {
    mkdirSync(dirname(path), { recursive: true });
    writeFileSync(temporary, JSON.stringify(sorted(record)), { flag: "w" });
    renameSync(temporary, path);
    return true;
  } catch { try { rmSync(temporary, { force: true }); } catch { return false; } return false; }
}

/** 🛰️ Resolves the stamp of a derived inferrer: the local record of its closure key, else the fleet's (re-checked here), else one evaluation of the corpus within `timeoutMilliseconds`; unavailable (never an error) when none of them works, which makes the files of the inferrer count as content again.
 * @param {{ id: string, declared: string, root: string, directory: string, runners: any, fleet?: any, provider?: any, timeoutMilliseconds?: number, signal?: AbortSignal, now?: () => number, read?: (path: string) => string, shaOf?: (path: string) => string, codeFiles?: readonly string[] }} options */
export async function resolveStampV1({ id, declared, root, directory, runners, fleet, provider = CONFORMANCE_CORPUS_V1.providers[id], timeoutMilliseconds = 20000, signal, now = Date.now, read = (path) => readFileSync(path, "utf8"), shaOf = (path) => sha256V1(readFileSync(resolve(root, path))), codeFiles = [] }) {
  try {
    const files = [...new Set([...closureFilesV1(provider.entries, root, read), ...provider.schemas, ...Object.values(provider.fixtures), ...codeFiles])].sort(), key = closureKeyV1(id, files, shaOf);
    const stored = readRecordV1(directory, id, key, declared);
    if (stored) return { status: "stored", id, key, stamp: stored.stamp, record: stored };
    const shared = await Promise.resolve(fleet?.get?.(id, key)).catch(() => undefined);
    if (shared && recordHoldsV1(shared, { id, key, declared })) { writeRecordV1(directory, shared); return { status: "fleet", id, key, stamp: shared.stamp, record: shared }; }
    const started = now();
    let timer;
    const guard = new Promise((_, reject) => { timer = setTimeout(() => reject(Error("evaluation exceeded " + timeoutMilliseconds + " ms")), timeoutMilliseconds); if (signal?.aborted) reject(Error("cancelled")); else signal?.addEventListener("abort", () => reject(Error("cancelled")), { once: true }); });
    const parts = await Promise.race([evaluateProviderV1({ id, root, runners, provider, read }), guard]).finally(() => clearTimeout(timer));
    const record = { version: 1, id, key, stamp: stampOfV1(declared, { cases: parts.cases, schema: parts.schema, imports: parts.imports }), parts: { cases: parts.cases, schema: parts.schema, imports: parts.imports }, covered: parts.covered, milliseconds: now() - started };
    writeRecordV1(directory, record);
    Promise.resolve(fleet?.put?.(id, key, record)).catch(() => undefined);
    return { status: "computed", id, key, stamp: record.stamp, record };
  } catch (error) { return { status: "unavailable", id, reason: String(error?.message ?? error) }; }
}

/** 🧭️ The kinds a provider declares, read from its own schema or table: what its corpus has to cover. */
export const DECLARED_KINDS_V1 = Object.freeze({
  "input-precision": (root) => {
    const document = JSON.parse(readFileSync(resolve(root, "🏗️build/📥️inputs/🎯️precision/🧬️schema/🔣️.json"), "utf8")), definitions = document.definitions;
    return { read: definitions.Read.oneOf.map((entry) => definitions[entry.$ref.split("/").pop()].required[0]).sort() };
  },
  "package-managers": (root) => {
    const base = "⚡️caching/🚀️bootstrap/📦️dependencies/🔌️managers/", catalog = JSON.parse(readFileSync(resolve(root, base + "🔣️catalog.json"), "utf8")), policy = JSON.parse(readFileSync(resolve(root, base + "🔣️policy.json"), "utf8"));
    return { manager: Object.keys(catalog.adapters).sort(), ecosystem: Object.keys(policy.selection).sort() };
  },
});

/** 📐️ The declared kinds a corpus left unexercised, as `category:name`: the coverage law, so a provider that gains a kind without a fixture fails a test instead of passing unnoticed. */
export function coverageViolationsV1(declared, covered) {
  return Object.entries(declared).flatMap(([category, names]) => names.filter((name) => !(covered[category] ?? []).includes(name)).map((name) => category + ":" + name));
}

/** 🎲️ A deterministic sample of `count` names: the names with the smallest digest of `seed` and the name, so the same seed samples the same projects and a new seed another set. */
export const sampleNamesV1 = (names, count, seed) => [...names].map((name) => [sha256V1(seed + "\n" + name), name]).sort(([a], [b]) => (a < b ? -1 : a > b ? 1 : 0)).slice(0, Math.max(0, count)).map(([, name]) => name);

/** 🔍️ The periodic full audit: infers the sampled projects completely again and compares the result with the stored record, so a behaviour change on an input shape the corpus lacks is found by the fleet's capable machine instead of by a developer. `infer(project)` and `stored(project)` answer JSON; progress and cancellation are honoured between projects.
 * @param {{ projects: readonly string[], infer: (project: string) => Promise<unknown>, stored: (project: string) => Promise<unknown>, signal?: AbortSignal, progress?: (row: { checked: number, total: number }) => void }} options */
export async function runConformanceAuditV1({ projects, infer, stored, signal, progress }) {
  const differing = [], failed = [];
  let same = 0, checked = 0;
  for (const project of projects) {
    signal?.throwIfAborted();
    try {
      const fresh = sha256V1(canonicalJsonV1(await infer(project))), held = sha256V1(canonicalJsonV1(await stored(project)));
      if (fresh === held) same++; else differing.push({ project, fresh, stored: held });
    } catch (error) { failed.push({ project, message: String(error?.message ?? error) }); }
    checked++;
    progress?.({ checked, total: projects.length });
  }
  return { version: 1, checked, same, differing, failed };
}

/** 🗒️ The one line a finished audit reports in a language. */
export function auditReportTextV1(report, language) {
  return report.differing.length || report.failed.length ? conformanceTextV1("audit.differs", language, { differing: report.differing.length, checked: report.checked, failed: report.failed.length }) : conformanceTextV1("audit.clean", language, { checked: report.checked });
}
