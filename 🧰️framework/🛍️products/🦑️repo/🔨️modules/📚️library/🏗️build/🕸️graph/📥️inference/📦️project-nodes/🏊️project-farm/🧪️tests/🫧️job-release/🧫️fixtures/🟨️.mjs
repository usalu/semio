import { pathToFileURL } from "node:url";

const [, , nodes, mode] = process.argv, calls = [], sentinel = { name: "scripts" };
const deep = () => new Proxy(function () {}, { get: (_target, key) => key === "then" ? undefined : deep(), apply: () => undefined });
const owned = {
  releaseAfterTakeLookV1: (scripts, options) => { calls.push({ scripts: scripts === sentinel, options }); },
  withClosurePassV1: async () => { if (mode === "throws") throw Error("controlled"); },
};
const groups = { coordinates: { workspaceDataDirectory: () => "/farm-job-release" }, scriptInputs: new Proxy(owned, { get: (target, key) => target[key] ?? deep() }) };
const module = await import(pathToFileURL(nodes).href);
module.wire(new Proxy({}, { get: (_target, group) => groups[group] ?? deep() }));
let failed = false;
try { await module.produceProjectRecordV1({ workspaceRoot: "/farm-job-release", scripts: sentinel }, { cacheKey: "k", head: [], root: "r", json: {}, native: false, pipeline: false, selectedInputs: [] }); } catch { failed = true; }
console.log(JSON.stringify({ calls, failed }));
