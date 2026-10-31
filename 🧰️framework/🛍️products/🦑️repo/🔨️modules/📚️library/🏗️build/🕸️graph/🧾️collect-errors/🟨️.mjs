/** 🧾️ Diagnostic mode of the graph builder: infers every project, collects every per-project failure (instead of stopping at the first), writes one report (JSON + EN/DE summary) and never lets a graph be published. Enabled by `SEMIO_GRAPH_COLLECT_ERRORS=<directory>` (`1` = the system temporary directory); the scheduling, pinning, capacity, progress and cancellation of the ordinary build are untouched. */
import { mkdirSync, renameSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { join, relative, resolve } from "node:path";
import schema from "./🧬️schema/🔣️.json" with { type: "json" };

/** 🗣️ The summary texts of the closed schema, one entry per supported language (EN first, DE second; no default language). */
export const collectTextsV1 = Object.freeze(schema["x-semio-texts"]);

/** 🏷️ Names this component as its roster entry does. */
export const component = "collect-errors";

/** 🚦️ The error codes whose meaning is "stop now": a restart is demanded or the diagnostic run itself is over. */
const FINAL_CODES = new Set(["BUILD_AUTHORITY_CHANGED", "BUILD_COLLECTED_ERRORS"]);

/** 📁️ The report directory the environment selects, or `undefined` when the diagnostic mode is off. */
export function collectErrorsDirectoryV1(env = process.env) {
  const value = env.SEMIO_GRAPH_COLLECT_ERRORS;
  if (value === undefined || value === "" || value === "0" || value === "off") return undefined;
  return value === "1" || value === "on" ? join(tmpdir(), "semio-collect-errors") : resolve(value);
}

/** 🔑️ The cause of a failure: its message without the parts that differ per project (paths, hashes, numbers, package names), so equal causes group. */
export function causeKeyV1(message) {
  return String(message ?? "")
    .replace(/@[a-z0-9._-]+\/[a-z0-9._-]+/giu, "<project>")
    .replace(/[A-Za-z]:\\[^\s'"`)]+|\/[^\s'"`)]+/gu, "<path>")
    .replace(/\b[a-f0-9]{12,}\b/gu, "<hash>")
    .replace(/\d+/gu, "N")
    .replace(/\s+/gu, " ")
    .trim()
    .slice(0, 200);
}

/** 🧱️ The first frames of a stack, relative to the workspace, so the throwing owner is visible without the whole trace. */
export function stackTopV1(error, workspaceRoot, count = 3) {
  const lines = typeof error?.stack === "string" ? error.stack.split("\n").filter((line) => /^\s*at\s/u.test(line)) : [];
  return lines.slice(0, count).map((line) => line.trim().split(workspaceRoot).join("<workspace>").slice(0, 300));
}

/** 🗣️ The summary lines of a report in one language; the texts are the schema's, never a default language. */
export function renderCollectSummaryV1(report, texts, language) {
  const template = texts[language];
  if (!template) throw Error("Known summary language required");
  const fill = (text, fields) => text.replace(/\{(\w+)\}/gu, (_, key) => String(fields[key]));
  const lines = [fill(template.headline, { failed: report.projectsFailed, total: report.projectsTotal, causes: report.groups.length, directory: report.directory })];
  for (const group of report.groups) lines.push(fill(template.group, { count: group.count, cause: group.cause, example: group.projects[0]?.project ?? "", top: group.stackTop[0] ?? "" }));
  return lines.join("\n") + "\n";
}

/** 🧪️ Creates the collector of one inference: `guard` runs one project and records its failure, `finish` writes the report and ends the run with a final error. */
export function createErrorCollectorV1({ directory, workspaceRoot, texts, signal, now = () => new Date(), write = (text) => process.stderr.write(text) }) {
  const rows = [], startedAt = now().toISOString();
  const isFinal = (error) => signal?.aborted || error?.name === "AbortError" || FINAL_CODES.has(error?.code) || /restart required/iu.test(String(error?.message));
  return Object.freeze({
    async guard(meta, run) {
      try { await run(); return true; }
      catch (error) {
        if (isFinal(error)) throw error;
        rows.push({ project: meta.project, root: meta.root, phase: meta.phase, ...(error?.target ? { target: String(error.target) } : {}), ...(error?.code ? { code: String(error.code) } : {}), error: String(error?.message ?? error).slice(0, 2048), cause: causeKeyV1(error?.message ?? error), stackTop: stackTopV1(error, workspaceRoot) });
        return false;
      }
    },
    rows: () => rows.slice(),
    async finish({ projects }) {
      const grouped = new Map();
      for (const row of rows) {
        const group = grouped.get(row.cause) ?? { cause: row.cause, count: 0, code: row.code ?? null, message: row.error, stackTop: row.stackTop, projects: [] };
        group.count++; group.projects.push({ project: row.project, root: row.root, phase: row.phase, ...(row.target ? { target: row.target } : {}) });
        grouped.set(row.cause, group);
      }
      const groups = [...grouped.values()].sort((a, b) => b.count - a.count || (a.cause < b.cause ? -1 : 1));
      const report = { version: 1, startedAt, finishedAt: now().toISOString(), workspace: relative(process.cwd(), workspaceRoot) || ".", directory, projectsTotal: projects, projectsFailed: rows.length, groups, rows };
      mkdirSync(directory, { recursive: true });
      const publish = (name, text) => { const temporary = join(directory, `.${name}.${process.pid}.tmp`); writeFileSync(temporary, text); renameSync(temporary, join(directory, name)); };
      publish("report.json", JSON.stringify(report, null, 1) + "\n");
      for (const language of Object.keys(texts)) publish(`summary.${language}.txt`, renderCollectSummaryV1(report, texts, language));
      for (const language of Object.keys(texts)) write(`[collect-errors] ${renderCollectSummaryV1(report, texts, language).split("\n")[0]}\n`);
      throw Object.assign(Error(`${texts.en.final.replace("{failed}", String(rows.length)).replace("{total}", String(projects)).replace("{directory}", directory)} / ${texts.de.final.replace("{failed}", String(rows.length)).replace("{total}", String(projects)).replace("{directory}", directory)}`), { code: "BUILD_COLLECTED_ERRORS" });
    },
  });
}

/** 🔌️ The collector the environment asks for, or `undefined` when the diagnostic mode is off. */
export function createErrorCollectorFromEnvV1({ workspaceRoot, signal, env = process.env }) {
  const directory = collectErrorsDirectoryV1(env);
  return directory === undefined ? undefined : createErrorCollectorV1({ directory, workspaceRoot, texts: collectTextsV1, signal });
}
