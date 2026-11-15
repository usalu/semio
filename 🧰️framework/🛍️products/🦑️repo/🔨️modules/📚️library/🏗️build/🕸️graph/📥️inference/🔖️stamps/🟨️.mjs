/** 🔖️ Declares the stamp every inferrer of the graph carries and classifies the loaded implementation by it, so a record or a generation fact names the protocol version of the code that took part in its output, never the bytes of that code. */
import { registerInferenceStampsV1 } from "../../../../🏗️build/👁️observation/🟨️.mjs";
import schema from "./🧬️schema/🔣️.json" with { type: "json" };
import { declareFormatStampV1 } from "../../../🧷️format-stamps/🟨️.mjs";
import { conformanceTextV1, resolveStampV1 } from "./🧮️conformance/🟨️.mjs";
import { createBoundedCacheV1 } from "../../../🧊️bounded/🟨️.mjs";
import { homedir } from "node:os";
import { join } from "node:path";
import { fileURLToPath } from "node:url";

declareFormatStampV1("graph.inferrers", schema);

/** 🏷️ Names this component as its roster entry does. */
export const component = "stamps";
const STAMP = /^semio\.graph-inferrer\.[a-z][a-z0-9]*(?:-[a-z0-9]+)*\/[1-9][0-9]*$/u;
const ID = /^[a-z][a-z0-9]*(?:-[a-z0-9]+)*$/u;
const EXECUTABLE = /\.(?:mjs|cjs|js|ts|mts|cts|tsx)$/u;
let protocol;
const effective = createBoundedCacheV1({ name: "inference:effective-stamps", maxEntries: 256, maxBytes: 256 });
let fleetPort;

/** 🧾️ Admits the declared protocol: closed keys, well formed stamps, known scopes and fragments no two groups share, so every path has at most one meaning. */
export function admitInferenceProtocolV1(value) {
  const closed = (row, keys, what) => { if (!row || typeof row !== "object" || Array.isArray(row) || Object.keys(row).sort().join() !== [...keys].sort().join()) throw Error("Closed inference " + what + " required"); };
  closed(value, ["version", "scopes", "inferrers", "derived", "none"], "protocol");
  if (value.version !== 1) throw Error("Inference protocol version 1 required");
  if (!Array.isArray(value.scopes) || new Set(value.scopes).size !== value.scopes.length) throw Error("Distinct inference scopes required");
  const seen = new Map(), claim = (fragment, owner) => { if (typeof fragment !== "string" || fragment.length < 2) throw Error("Inference source fragment required: " + owner); if (seen.has(fragment)) throw Error("Inference source fragment is claimed twice: " + fragment + " (" + seen.get(fragment) + ", " + owner + ")"); seen.set(fragment, owner); };
  for (const fragment of value.none) claim(fragment, "none");
  const ids = Object.keys(value.inferrers);
  if (!ids.length) throw Error("Inferrers required");
  if (!Array.isArray(value.derived) || new Set(value.derived).size !== value.derived.length || value.derived.some((id) => !ids.includes(id))) throw Error("Distinct derived inferrers of the protocol required");
  for (const id of ids) {
    const row = value.inferrers[id];
    closed(row, ["stamp", "scope", "sources"], "inferrer " + id);
    if (!ID.test(id) || !STAMP.test(row.stamp) || row.stamp.split("/")[0] !== "semio.graph-inferrer." + id) throw Error("Stamp named after its inferrer required: " + id);
    if (!value.scopes.includes(row.scope) || !Array.isArray(row.sources) || !row.sources.length) throw Error("Known scope and sources required: " + id);
    for (const fragment of row.sources) claim(fragment, id);
  }
  return Object.freeze(value);
}

/** 🔌️ Binds the protocol this module loaded and registers its stamps with the observation, which verifies every recorded stamp against them. */
export function wire() {
  protocol = admitInferenceProtocolV1(schema.definitions.Protocol.const);
  effective.clear();
  registerInferenceStampsV1(inferenceStampEntriesV1(protocol), inferenceFragmentsV1(protocol));
}

/** 🎚️ True while an inferrer carries a stamp: every declared one, and a derived one only once its conformance stamp exists; without it its sources count as content facts again. */
export const inferrerActiveV1 = (id, stamps = effective, document = protocol) => !document?.derived.includes(id) || stamps.has(id);

/** 🧭️ Every path fragment the protocol covers, by stamp or as infrastructure. */
export function inferenceFragmentsV1(document, stamps = effective) {
  return [...document.none, ...Object.entries(document.inferrers).filter(([id]) => inferrerActiveV1(id, stamps, document)).flatMap(([, group]) => group.sources)];
}

/** 🗂️ The registry entries of a protocol: one `inferrer:<id>` stamp per active inferrer, the effective stamp of a derived one. */
export function inferenceStampEntriesV1(document, stamps = effective) {
  return Object.entries(document.inferrers).filter(([id]) => inferrerActiveV1(id, stamps, document)).map(([id, row]) => ["inferrer:" + id, stamps.get(id) ?? row.stamp]);
}

/** 📜️ The admitted protocol this revision loaded. */
export function inferenceProtocolV1() { return protocol; }

/** 🔑️ The observation path of the stamp of one inferrer. */
export const inferrerStampPathV1 = (id) => "inferrer:" + id;

/** 🏷️ The stamp one inferrer of the loaded protocol carries. */
export function inferrerStampV1(id) {
  const row = protocol.inferrers[id];
  if (row === undefined) throw Error("Declared inferrer required: " + id);
  return row.stamp;
}

/** 🎯️ The stamps one valid project record takes part in: every inferrer of scope project or all, and the native ones when the project has a native target. */
export function projectStampPathsV1({ native }) {
  return Object.entries(protocol.inferrers).filter(([id]) => inferrerActiveV1(id)).filter(([, row]) => row.scope === "project" || row.scope === "all" || native && row.scope === "native").map(([id]) => inferrerStampPathV1(id));
}

/** 🛰️ Binds the port that shares conformance records through the fleet cache (`get(id, key)`, `put(id, key, record)`); a record from the fleet is used only after it checks against its key and its parts. */
export function bindConformanceFleetPortV1(port) { fleetPort = port; }

/** 📁️ The per user directory of the conformance records: a record is named by the content digest of its closure, so one directory serves every checkout of the machine. */
export function conformanceDirectoryV1(environment = process.env, platform = process.platform, home = homedir()) {
  const base = platform === "darwin" ? join(home, "Library", "Caches") : platform === "win32" ? environment.LOCALAPPDATA ?? join(home, "AppData", "Local") : environment.XDG_CACHE_HOME?.startsWith("/") ? environment.XDG_CACHE_HOME : join(home, ".cache");
  return join(base, "semio", "inference-conformance");
}

/** 🧮️ Settles the conformance stamp of every derived inferrer once per process: the local record of its closure, the fleet's, or one evaluation of the corpus; an inferrer without a stamp keeps the content facts of its files. Never fails and never waits longer than the evaluation limit. */
export async function settleConformanceStampsV1({ libraryRoot, directory = conformanceDirectoryV1(), runners, fleet = fleetPort, signal, say = (process.env.SEMIO_GRAPH_PHASES === "1" ? (text) => process.stderr.write("[conformance] " + text + "\n") : undefined), language = process.env.SEMIO_LANGUAGE === "de" ? "de" : "en", timeoutMilliseconds } = {}) {
  const results = [];
  try {
    const loaded = runners ?? (await import("./🧮️conformance/🏃️runners/🟨️.mjs")).RUNNERS_V1;
    const own = fileURLToPath(new URL("./🧮️conformance/", import.meta.url)), codeFiles = ["🟨️.mjs", "🔣️.json", "🏃️runners/🟨️.mjs", "🧬️schema/🔣️.json"].map((name) => join(own, name).slice(libraryRoot.length + 1));
    for (const id of protocol.derived) {
      const result = await resolveStampV1({ id, declared: protocol.inferrers[id].stamp, root: libraryRoot, directory, runners: loaded, fleet, signal, codeFiles, ...(timeoutMilliseconds === undefined ? {} : { timeoutMilliseconds }) });
      if (result.stamp) effective.set(id, result.stamp); else effective.delete(id);
      say?.(conformanceTextV1("stamp." + result.status, language, { id, cases: Object.keys(result.record?.parts.cases ?? {}).length, milliseconds: Math.round(result.record?.milliseconds ?? 0), reason: result.reason ?? "" }));
      results.push(result);
    }
  } catch (error) {
    for (const id of protocol.derived) { effective.delete(id); results.push({ status: "unavailable", id, reason: String(error?.message ?? error) }); }
  }
  registerInferenceStampsV1([...inferenceStampEntriesV1(protocol), ...protocol.derived.filter((id) => !effective.has(id)).map((id) => ["inferrer:" + id, "!unavailable"])], inferenceFragmentsV1(protocol));
  return results;
}

/** 🔬️ True when a workspace relative path lies below one of the fragments. */
export const matchesFragmentsV1 = (path, fragments) => fragments.some((fragment) => path.includes(fragment));

/** 🧱️ Turns the digest rows of the loaded implementation into the facts a generation names: one stamp row per inferrer (carrying its scope), the content row of every data file that no group covers, and the content row of unclassified code, which stays strict until a group claims it. Infrastructure is dropped. */
export function classifyImplementationRowsV1(rows, document = protocol, stamps = effective) {
  const out = [], none = document.none, groups = Object.entries(document.inferrers).filter(([id]) => inferrerActiveV1(id, stamps, document));
  for (const row of rows) {
    if (matchesFragmentsV1(row.path, none) || groups.some(([, group]) => matchesFragmentsV1(row.path, group.sources))) continue;
    out.push(row);
  }
  for (const [id, group] of groups) out.push({ kind: "stamp", path: inferrerStampPathV1(id), fact: stamps.get(id) ?? group.stamp, scope: group.scope });
  return out;
}

/** 🚧️ Names the executable files of the loaded implementation that no group and no infrastructure fragment claims, the files whose edits would still change every record. */
export function unclassifiedCodeV1(rows, document = protocol) {
  return rows.filter((row) => EXECUTABLE.test(row.path) && !matchesFragmentsV1(row.path, document.none) && !Object.values(document.inferrers).some((group) => matchesFragmentsV1(row.path, group.sources))).map((row) => row.path);
}
