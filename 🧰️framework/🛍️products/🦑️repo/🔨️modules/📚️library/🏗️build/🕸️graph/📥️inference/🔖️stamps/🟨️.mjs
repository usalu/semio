/** 🔖️ Declares the stamp every inferrer of the graph carries and classifies the loaded implementation by it, so a record or a generation fact names the protocol version of the code that took part in its output, never the bytes of that code. */
import { registerInferenceStampsV1 } from "../../../../🏗️build/👁️observation/🟨️.mjs";
import schema from "./🧬️schema/🔣️.json" with { type: "json" };
import { declareFormatStampV1 } from "../../../🧷️format-stamps/🟨️.mjs";

declareFormatStampV1("graph.inferrers", schema);

/** 🏷️ Names this component as its roster entry does. */
export const component = "stamps";
const STAMP = /^semio\.graph-inferrer\.[a-z][a-z0-9]*(?:-[a-z0-9]+)*\/[1-9][0-9]*$/u;
const ID = /^[a-z][a-z0-9]*(?:-[a-z0-9]+)*$/u;
const EXECUTABLE = /\.(?:mjs|cjs|js|ts|mts|cts|tsx)$/u;
let protocol;

/** 🧾️ Admits the declared protocol: closed keys, well formed stamps, known scopes and fragments no two groups share, so every path has at most one meaning. */
export function admitInferenceProtocolV1(value) {
  const closed = (row, keys, what) => { if (!row || typeof row !== "object" || Array.isArray(row) || Object.keys(row).sort().join() !== [...keys].sort().join()) throw Error("Closed inference " + what + " required"); };
  closed(value, ["version", "scopes", "inferrers", "none"], "protocol");
  if (value.version !== 1) throw Error("Inference protocol version 1 required");
  if (!Array.isArray(value.scopes) || new Set(value.scopes).size !== value.scopes.length) throw Error("Distinct inference scopes required");
  const seen = new Map(), claim = (fragment, owner) => { if (typeof fragment !== "string" || fragment.length < 2) throw Error("Inference source fragment required: " + owner); if (seen.has(fragment)) throw Error("Inference source fragment is claimed twice: " + fragment + " (" + seen.get(fragment) + ", " + owner + ")"); seen.set(fragment, owner); };
  for (const fragment of value.none) claim(fragment, "none");
  const ids = Object.keys(value.inferrers);
  if (!ids.length) throw Error("Inferrers required");
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
  registerInferenceStampsV1(inferenceStampEntriesV1(protocol), inferenceFragmentsV1(protocol));
}

/** 🧭️ Every path fragment the protocol covers, by stamp or as infrastructure. */
export function inferenceFragmentsV1(document) {
  return [...document.none, ...Object.values(document.inferrers).flatMap((group) => group.sources)];
}

/** 🗂️ The registry entries of a protocol: one `inferrer:<id>` stamp per inferrer. */
export function inferenceStampEntriesV1(document) {
  return Object.entries(document.inferrers).map(([id, row]) => ["inferrer:" + id, row.stamp]);
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
  return Object.entries(protocol.inferrers).filter(([, row]) => row.scope === "project" || row.scope === "all" || native && row.scope === "native").map(([id]) => inferrerStampPathV1(id));
}

/** 🔬️ True when a workspace relative path lies below one of the fragments. */
export const matchesFragmentsV1 = (path, fragments) => fragments.some((fragment) => path.includes(fragment));

/** 🧱️ Turns the digest rows of the loaded implementation into the facts a generation names: one stamp row per inferrer (carrying its scope), the content row of every data file that no group covers, and the content row of unclassified code, which stays strict until a group claims it. Infrastructure is dropped. */
export function classifyImplementationRowsV1(rows, document = protocol) {
  const out = [], none = document.none, groups = Object.entries(document.inferrers);
  for (const row of rows) {
    if (matchesFragmentsV1(row.path, none) || groups.some(([, group]) => matchesFragmentsV1(row.path, group.sources))) continue;
    out.push(row);
  }
  for (const [id, group] of groups) out.push({ kind: "stamp", path: inferrerStampPathV1(id), fact: group.stamp, scope: group.scope });
  return out;
}

/** 🚧️ Names the executable files of the loaded implementation that no group and no infrastructure fragment claims, the files whose edits would still change every record. */
export function unclassifiedCodeV1(rows, document = protocol) {
  return rows.filter((row) => EXECUTABLE.test(row.path) && !matchesFragmentsV1(row.path, document.none) && !Object.values(document.inferrers).some((group) => matchesFragmentsV1(row.path, group.sources))).map((row) => row.path);
}
