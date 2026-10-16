/** 📖️ Test-side read access to framework sources: one validated resolver from a framework-relative path to the absolute path, the file URL and the text or JSON a source-text law reads, so a test names `🔨️modules/<owner>/<file>` instead of climbing relative ladders out of its own tree.
 * Pure resolution plus `node:fs`; never imports the sources it reads.
 * @see ./🧪️tests/🟦️.ts
 * @see https://nodejs.org/api/path.html#pathrelativefrom-to */
import { readFileSync } from "node:fs";
import { isAbsolute, relative, resolve, sep } from "node:path";
import { fileURLToPath, pathToFileURL } from "node:url";

const FRAMEWORK_ROOT = resolve(fileURLToPath(new URL("../../..", import.meta.url)));

/** 🧭️ Resolves a framework-relative path (`🔨️modules/…`, `🛍️products/…`, `📦️packages/…`) to the absolute path inside the framework tree; refuses absolute paths, empty paths, `.` or `..` segments, backslashes and NUL. */
export function frameworkSourcePath(path: string): string {
  if (typeof path !== "string" || path.length === 0 || isAbsolute(path) || path.includes("\\") || path.includes("\0") || path.split("/").some(segment => segment === "" || segment === "." || segment === "..")) throw new Error(`framework source path must be a clean framework-relative path: ${JSON.stringify(path)}`);
  const absolute = resolve(FRAMEWORK_ROOT, path);
  const inside = relative(FRAMEWORK_ROOT, absolute);
  if (inside === "" || inside.startsWith(".." + sep) || inside === ".." || isAbsolute(inside)) throw new Error(`framework source path leaves the framework tree: ${JSON.stringify(path)}`);
  return absolute;
}

/** 🔗️ The file URL of {@link frameworkSourcePath}, for readers that take a `URL`. */
export function frameworkSourceUrl(path: string): URL {
  return pathToFileURL(frameworkSourcePath(path));
}

/** 📜️ Reads one framework source as UTF-8 text. */
export function readFrameworkSourceText(path: string): string {
  return readFileSync(frameworkSourcePath(path), "utf8");
}

/** 🧾️ Reads one framework source as JSON. */
export function readFrameworkSourceJson<Value = unknown>(path: string): Value {
  return JSON.parse(readFrameworkSourceText(path)) as Value;
}
