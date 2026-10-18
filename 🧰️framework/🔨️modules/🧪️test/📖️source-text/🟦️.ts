/** 📖️ Test-side read access to framework sources: one validated resolver from a framework-relative path to the absolute path, the file URL and the text or JSON a source-text law reads, so a test names `🔨️modules/<owner>/<file>` instead of climbing relative ladders out of its own tree.
 * Pure resolution plus `node:fs`; never imports the sources it reads.
 * @see ./🧪️tests/🟦️.ts
 * @see https://nodejs.org/api/path.html#pathrelativefrom-to */
import { existsSync, readdirSync, readFileSync } from "node:fs";
import { dirname, isAbsolute, join, relative, resolve, sep } from "node:path";
import { fileURLToPath, pathToFileURL } from "node:url";

const FRAMEWORK_ROOT = resolve(dirname(fileURLToPath(import.meta.url)), "../../..");

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

/** 🧫️ The file URL of a fixture beside `base` (pass `import.meta.url`), resolved by path arithmetic into a native `URL` so that bundlers which rewrite a literal `new URL(path, import.meta.url)` into an asset URL (vite under jsdom) and jsdom's own `URL` class never reach `node:fs`. */
export function fixtureUrl(base: string, path: string): URL {
  return pathToFileURL(resolve(dirname(fileURLToPath(base)), path));
}

const STORE_ROOT = "🛍️products/💻️os/🔨️modules/🏪️store";
const STORE_CARVED_DIRECTORIES = ["⚙️engine", "🏗️initialization", "🧰️test-support", "🧩️composition", "🧾️document", "🎚️config", "📜️space-history/🏠️space", "📦️codec/🏭️factory"] as const;
const STORE_EXCLUDED_SEGMENTS = new Set(["🧪️tests", "🧪️testing", "📦️packages", "node_modules"]);
const STORE_SOURCE_FILE = "🦀️.rs";

/** 🏪️ The OS store as one Rust source text: the store facade root, then the module files of every crate carved out of it (`⚙️engine`, `🏗️initialization`, `🧩️composition`, `🧾️document`, `🎚️config`, `🧰️test-support`, `📜️space-history/🏠️space`, `📦️codec/🏭️factory`, and every `📥️lifted` directory of the store tree) in byte order of their paths, tests and packaging folders left out. A source-text law that pins an item of the store reads this text and keeps working while the item moves between the crates of the store family.
 * @see ./🧪️tests/🟦️.ts */
export function readStoreSourceText(): string {
  const base = frameworkSourcePath(STORE_ROOT);
  const root = join(base, STORE_SOURCE_FILE);
  const carvedRoots = STORE_CARVED_DIRECTORIES.map(directory => join(base, directory) + sep);
  const found: string[] = [];
  const walk = (directory: string): void => {
    for (const entry of readdirSync(directory, { withFileTypes: true })) {
      const path = join(directory, entry.name);
      if (entry.isDirectory()) {
        if (!STORE_EXCLUDED_SEGMENTS.has(entry.name)) walk(path);
      } else if (entry.name === STORE_SOURCE_FILE && path !== root && (carvedRoots.some(carved => path.startsWith(carved)) || path.includes(sep + "📥️lifted" + sep))) found.push(path);
    }
  };
  walk(base);
  found.sort((left, right) => Buffer.compare(Buffer.from(left), Buffer.from(right)));
  return [root, ...found].map(file => readFileSync(file, "utf8")).join("\n");
}

/** 🪞️ The resolved targets of a fixture facade entry: a TypeScript file inside a `🧫️fixtures` folder whose every statement is an `export … from "<relative>"` re-export, so it carries no data of its own and only names fixtures an owner keeps under its `🧪️tests` tree. `undefined` for any other file (data, code, or a re-export of something that is not a file).
 * @see ./🧪️tests/🟦️.ts */
export function fixtureFacadeTargets(path: string): readonly string[] | undefined {
  if (!/[\\/]🧫️fixtures(?:[\\/][^\\/]+)*[\\/]🟦️\.ts$/u.test(path)) return undefined;
  const code = readFileSync(path, "utf8").replace(/\/\*[\s\S]*?\*\//gu, "").replace(/(^|[^:])\/\/[^\n]*/gu, "$1");
  const statements = code.split(";").map(statement => statement.trim()).filter(statement => statement.length > 0);
  const targets: string[] = [];
  for (const statement of statements) {
    const match = /^export\s+(?:type\s+)?(?:\*|\{[^}]*\})\s*from\s*"([^"]+)"(?:\s+with\s*\{[^}]*\})?$/u.exec(statement);
    if (match === null) return undefined;
    targets.push(resolve(dirname(path), match[1]!));
  }
  return targets.length > 0 && targets.every(target => existsSync(target)) ? targets : undefined;
}
