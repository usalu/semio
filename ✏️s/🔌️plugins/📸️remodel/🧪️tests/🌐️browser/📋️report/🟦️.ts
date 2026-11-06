import { resolve, relative, isAbsolute, sep } from "node:path";
import scenarios from "../🧫️fixtures/🔣️.json" with { type: "json" };

/** 📋️ Records only browser-observed evidence, leaving every unperformed scenario explicit. */
export type RemodelBrowserReportV1 = {
  host: string;
  scenarios: { id: string; description: string; status: "unexecuted" | "passed" | "failed" | "unsupported"; evidence: string[] }[];
  faults: string[];
};

/** 🧾️ Starts the full acceptance checklist without treating planned work as successful. */
export function initialRemodelBrowserReportV1(host: string): RemodelBrowserReportV1 {
  return { host, scenarios: scenarios.map(row => ({ ...row, status: "unexecuted", evidence: [] })), faults: [] };
}


/** 🪪️ Finds the editor chip without claiming a relationship to any window body. */
export function remodelEditorPresentV1(observed: readonly { app: string | null; role: string | null }[]): boolean {
  return observed.some(item => item.app === "s.remodel.remodeling@1/*#editor" && item.role === "editor");
}

/** 📤️ Serializes a complete report as a standalone JSON document. */
export function serializeRemodelBrowserReportV1(report: RemodelBrowserReportV1): string {
  return JSON.stringify(report, null, 2) + "\n";
}

/** 🎛️ Admits one explicit host, output and locale before any browser effects. */
export function parseRemodelBrowserArgumentsV1(segments: readonly string[]): { host: string; output: string; locale: "en" | "de" } {
  if (segments.length !== 6) throw Error("Exactly --host, --out and --locale are required");
  const values = new Map<string, string>();
  for (let index = 0; index < segments.length; index += 2) {
    const key = segments[index]!, value = segments[index + 1]!;
    if (!["--host", "--out", "--locale"].includes(key) || values.has(key) || !value || value.startsWith("--")) throw Error("Invalid browser acceptance arguments");
    values.set(key, value);
  }
  const host = new URL(values.get("--host")!), output = resolve(values.get("--out")!), locale = values.get("--locale");
  if (!["http:", "https:"].includes(host.protocol) || host.username || host.password || output.length > 256 || (locale !== "en" && locale !== "de")) throw Error("Invalid browser acceptance host, output or locale");
  return { host: host.href, output, locale };
}

/** ✅️ Accepts only the complete authored checklist with nonempty observed evidence and no faults. */
export function remodelAcceptanceSatisfiedV1(report: RemodelBrowserReportV1): boolean {
  const expected = new Set(scenarios.map(row => row.id));
  return report.faults.length === 0 && report.scenarios.length === expected.size &&
    report.scenarios.every(row => expected.delete(row.id) && row.status === "passed" && row.evidence.some(item => item.trim().length > 0)) && expected.size === 0;
}


/** 🖼️ Resolves the authored image inputs in place under the same repository. */
export function remodelImageInputsV1(repoRoot: string): string[] {
  const root = resolve(repoRoot), row = scenarios.find(item => item.id === "import-images");
  if (!row?.assets || row.assets.length !== 3) throw Error("Three authored image inputs are required");
  return row.assets.map(asset => {
    const path = resolve(root, asset), inside = relative(root, path);
    if (!inside || inside === ".." || inside.startsWith(".." + sep) || isAbsolute(inside) || path.length > 256) throw Error("Image input must stay inside the repository");
    return path;
  });
}


/** 🚦️ Admits UI interactions only after the host publishes its ready beacon. */
export function remodelHostReadyV1(beacon: string | null): boolean {
  return beacon !== null && beacon.startsWith("ready:");
}
