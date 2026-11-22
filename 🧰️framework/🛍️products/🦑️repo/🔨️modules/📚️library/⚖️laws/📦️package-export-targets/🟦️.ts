import { existsSync, readFileSync } from "node:fs";
import { dirname, join } from "node:path";
import { gitSyncV1 } from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧰️tools/🟦️.ts";

const GENERATED = new Set(["dist", "build", "out", "pkg", "🕸️bindings"]);

/** 📦️ Every relative file a package.json `exports` map points at, except patterns (`*`) and generated output (`dist`, `build`, `out`, wasm-bindgen `pkg` and `🕸️bindings`), in document order. */
export function packageExportTargetsV1(value: unknown): readonly string[] {
  if (typeof value === "string") return value.startsWith("./") && !value.includes("*") && !value.split("/").some((part) => GENERATED.has(part)) ? [value] : [];
  if (Array.isArray(value)) return value.flatMap(packageExportTargetsV1);
  if (value !== null && typeof value === "object") return Object.values(value).flatMap(packageExportTargetsV1);
  return [];
}

/** 🚩️ The export targets of one package.json text whose file is missing in `directory`; an unparseable file reports none (other laws own syntax). */
export function missingPackageExportTargetsV1(text: string, exists: (target: string) => boolean): readonly string[] {
  try {
    return packageExportTargetsV1((JSON.parse(text) as { exports?: unknown }).exports).filter((target) => !exists(target));
  } catch {
    return [];
  }
}

/** 🗂️ The tracked package.json files of the repository (git), or an empty list without git. */
export function listPackageJsonFilesV1(root: string): readonly string[] {
  if (!existsSync(join(root, ".git"))) return [];
  const listed = gitSyncV1(["-c", "core.quotepath=false", "ls-files", "-z", "--", "package.json", "*/package.json"], { cwd: root, maxBuffer: 512 * 1024 * 1024 });
  if (listed.status !== 0) throw Error(`git-ls-files-failed:${listed.stderr.trim()}`);
  return listed.stdout.split("\0").filter((path) => path !== "" && existsSync(join(root, path)));
}

/** 🧾️ Package.json path -> missing export targets, for every listed file that has some. */
export function packageExportReportV1(root: string, files: readonly string[] = listPackageJsonFilesV1(root)): ReadonlyMap<string, readonly string[]> {
  const report = new Map<string, readonly string[]>();
  for (const path of files) {
    const missing = missingPackageExportTargetsV1(readFileSync(join(root, path), "utf8"), (target) => existsSync(join(root, dirname(path), target)));
    if (missing.length > 0) report.set(path, missing);
  }
  return report;
}

/** 🚨️ One `path: target` line per missing export target of the repository, the lines `repo verify` prints and counts. */
export function packageExportBreachesV1(root: string, files: readonly string[] = listPackageJsonFilesV1(root)): readonly string[] {
  return [...packageExportReportV1(root, files)].flatMap(([path, missing]) => missing.map((target) => `${path}: ${target}`));
}
