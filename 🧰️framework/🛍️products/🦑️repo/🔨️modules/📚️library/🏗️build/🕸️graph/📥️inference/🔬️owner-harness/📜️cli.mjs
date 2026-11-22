import { openOwnerHarnessV1 } from "./🟨️.mjs";

/** 🔬️ Infers owners one after the other in one session and prints one JSON line per owner: `bun 📜️cli.mjs <workspaceRoot> [--selected build,test] [--analysis-store <directory>] [--late] [--trim <megabytes>] <ownerRoot>...`; the first owner pays the cold walk, the next ones show the steady state. */
const [workspaceRoot, ...rest] = process.argv.slice(2), roots = [], options = {};
for (let at = 0; at < rest.length; at++) { if (rest[at] === "--selected") options.selected = { targets: (rest[++at] ?? "").split(",").filter(Boolean), inputs: [] }; else if (rest[at] === "--analysis-store") options.analysisStore = rest[++at]; else if (rest[at] === "--late") options.late = true; else if (rest[at] === "--trim") options.trim = Number(rest[++at]) * 1048576; else roots.push(rest[at]); }
const harness = await openOwnerHarnessV1({ workspaceRoot, late: options.late === true, analysisStore: options.analysisStore });
try { for (const root of roots) { const result = await harness.infer(root, options.selected === undefined ? {} : { selected: options.selected }); process.stdout.write(JSON.stringify({ ...result, trimmed: options.trim === undefined ? false : harness.trim(options.trim) }) + "\n"); } } finally { harness.close(); }
