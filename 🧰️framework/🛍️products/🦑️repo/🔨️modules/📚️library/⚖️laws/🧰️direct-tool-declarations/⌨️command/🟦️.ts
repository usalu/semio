import { toolLanguageV1 } from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧰️tools/🟦️.ts";
import { listDirectToolFilesV1, scanDirectToolDeclarationsV1, type DirectToolFindingV1 } from "../🟦️.ts";
import {writeResultLineV1} from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🏗️build/📤️output/🟦️.ts";

const TEXTS = {
  passed: { en: "[verify tool-declarations] passed: {scanned} files read, {allowed} allowed.", de: "[verify tool-declarations] bestanden: {scanned} Dateien gelesen, {allowed} erlaubt." },
  failed: { en: "[verify tool-declarations] {count} direct tool invocation(s): call `repo ...` instead.", de: "[verify tool-declarations] {count} direkte(r) Werkzeugaufruf(e): rufe stattdessen `repo ...` auf." },
  finding: { en: "{path}:{line}: {tool} ({kind}, {context}) {command}", de: "{path}:{line}: {tool} ({kind}, {context}) {command}" }
} as const;

const fill = (text: string, values: Readonly<Record<string, string | number>>): string => text.replace(/\{(\w+)\}/gu, (_, key: string) => String(values[key]));

/** 🚪️ Runs `repo verify tool-declarations [--json] [--report]`: reads every tracked and untracked file of the declarative kinds, reports each banned tool started by name outside the allowed paths and answers the exit code (0 clean, 1 findings). */
export function runDirectToolDeclarationLawV1(root: string, args: readonly string[], write: (line: string) => void = writeResultLineV1, environment: NodeJS.ProcessEnv = process.env): number {
  const language = toolLanguageV1(environment), scan = scanDirectToolDeclarationsV1(root, listDirectToolFilesV1(root));
  const live = scan.findings.filter((finding: DirectToolFindingV1) => finding.allowed === null), allowed = scan.findings.length - live.length;
  if (args.includes("--json")) { write(JSON.stringify({ scanned: scan.scanned, findings: live, allowed }, null, 1)); return live.length > 0 ? 1 : 0; }
  for (const finding of live) write(fill(TEXTS.finding[language], { path: finding.path, line: finding.line, tool: finding.tool, kind: finding.kind, context: finding.context, command: finding.command }));
  if (live.length > 0) { write(fill(TEXTS.failed[language], { count: live.length })); return args.includes("--report") ? 0 : 1; }
  write(fill(TEXTS.passed[language], { scanned: scan.scanned, allowed }));
  return 0;
}
