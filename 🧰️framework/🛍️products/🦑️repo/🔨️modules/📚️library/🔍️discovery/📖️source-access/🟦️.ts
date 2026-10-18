import { lstatSync, readFileSync, readdirSync } from "node:fs";
import { isAbsolute, join, relative, resolve, sep } from "node:path";
import { listModuleTree, readModuleTreeSource, type ModuleTreeOptions } from "../../../../../../🔨️modules/📚️compiler/📖️syntax/🌳️module-tree/🟦️.ts";

export type PolicySourceEntry = Readonly<{ name: string; isFile: boolean; isDirectory: boolean; isSymbolicLink: boolean }>;

export type PolicySourceOperations = Readonly<{
  lstat: (path: string) => Readonly<{ isFile: boolean; isDirectory: boolean; isSymbolicLink: boolean }>;
  readFile: (path: string) => string;
  readdir: (path: string) => readonly PolicySourceEntry[];
}>;

export type PolicySourceDirectoryResult = Readonly<{ state: "directory"; entries: readonly PolicySourceEntry[] }> | Readonly<{ state: "missing" | "unreadable" | "symlink" | "not-directory"; entries: readonly [] }>;

export type PolicySourceTextResult = Readonly<{ state: "file"; text: string }> | Readonly<{ state: "missing" | "unreadable" | "symlink" | "not-file"; text: "" }>;

export const POLICY_SKIP_DIRS = new Set(["compose", "node_modules", ".git", ".🧬semio", "target", "dist", "build", "coverage", "🤖️generated", ".claude", "vendor", ".venv", ".turbo", ".nx", ".storybook", "storybook-static"]);

export const POLICY_SOURCE_OPERATIONS: PolicySourceOperations = {
  lstat: (path) => {
    const stat = lstatSync(path);
    return { isFile: stat.isFile(), isDirectory: stat.isDirectory(), isSymbolicLink: stat.isSymbolicLink() };
  },
  readFile: (path) => readFileSync(path, "utf8"),
  readdir: (path) => readdirSync(path, { withFileTypes: true }).map((entry) => ({ name: entry.name, isFile: entry.isFile(), isDirectory: entry.isDirectory(), isSymbolicLink: entry.isSymbolicLink() })),
};

export function policySourceUnavailableState(error: unknown): "missing" | "unreadable" {
  const code = error && typeof error === "object" && "code" in error ? String(error.code) : "";
  return code === "ENOENT" ? "missing" : "unreadable";
}

export function policySourceAncestry(repoRoot: string, relPath: string, operations: PolicySourceOperations = POLICY_SOURCE_OPERATIONS): "clear" | "missing" | "unreadable" | "symlink" | "not-directory" {
  const root = resolve(repoRoot),
    target = resolve(root, relPath),
    locator = relative(root, target);
  if (locator === ".." || locator.startsWith(`..${sep}`) || isAbsolute(locator)) return "unreadable";
  const segments = locator ? locator.split(sep) : [],
    ancestors = segments.length ? [root, ...segments.slice(0, -1).map((_, index) => join(root, ...segments.slice(0, index + 1)))] : [];
  for (const ancestor of ancestors) {
    let stat: ReturnType<PolicySourceOperations["lstat"]>;
    try {
      stat = operations.lstat(ancestor);
    } catch (error) {
      return policySourceUnavailableState(error);
    }
    if (stat.isSymbolicLink) return "symlink";
    if (!stat.isDirectory) return "not-directory";
  }
  return "clear";
}

export function policySourceDirectory(repoRoot: string, relDir: string, operations: PolicySourceOperations = POLICY_SOURCE_OPERATIONS): PolicySourceDirectoryResult {
  const path = join(repoRoot, relDir);
  const ancestry = policySourceAncestry(repoRoot, relDir, operations);
  if (ancestry !== "clear") return { state: ancestry, entries: [] };
  let stat: ReturnType<PolicySourceOperations["lstat"]>;
  try {
    stat = operations.lstat(path);
  } catch (error) {
    return { state: policySourceUnavailableState(error), entries: [] };
  }
  if (stat.isSymbolicLink) return { state: "symlink", entries: [] };
  if (!stat.isDirectory) return { state: "not-directory", entries: [] };
  try {
    return { state: "directory", entries: operations.readdir(path) };
  } catch (error) {
    return { state: policySourceUnavailableState(error), entries: [] };
  }
}

export function policySourceText(repoRoot: string, relPath: string, operations: PolicySourceOperations = POLICY_SOURCE_OPERATIONS): PolicySourceTextResult {
  const path = join(repoRoot, relPath);
  const ancestry = policySourceAncestry(repoRoot, relPath, operations);
  if (ancestry !== "clear") return { state: ancestry === "not-directory" ? "not-file" : ancestry, text: "" };
  let stat: ReturnType<PolicySourceOperations["lstat"]>;
  try {
    stat = operations.lstat(path);
  } catch (error) {
    return { state: policySourceUnavailableState(error), text: "" };
  }
  if (stat.isSymbolicLink) return { state: "symlink", text: "" };
  if (!stat.isFile) return { state: "not-file", text: "" };
  try {
    return { state: "file", text: operations.readFile(path) };
  } catch (error) {
    return { state: policySourceUnavailableState(error), text: "" };
  }
}

export function policyReaddirSafe(repoRoot: string, relDir: string): { name: string; isDirectory: boolean }[] {
  const source = policySourceDirectory(repoRoot, relDir);
  if (source.state === "missing") return [];
  if (source.state !== "directory") throw new Error(`Policy source directory ${join(repoRoot, relDir)} is ${source.state}.`);
  return source.entries.filter((entry) => !POLICY_SKIP_DIRS.has(entry.name)).map((entry) => ({ name: entry.name, isDirectory: entry.isDirectory }));
}

export function policyReadFileSafe(repoRoot: string, ...parts: string[]): string {
  const relPath = join(...parts),
    source = policySourceText(repoRoot, relPath);
  if (source.state === "missing") return "";
  if (source.state !== "file") throw new Error(`Policy source file ${join(repoRoot, relPath)} is ${source.state}.`);
  return source.text;
}

/** 📜️ Reads a WIT package directory as the one text its files declare: every direct `.wit` file in name order, through the symlink-refusing policy read. */
export function policyReadWitPackageSource(repoRoot: string, relDir: string): string {
  return policyReaddirSafe(repoRoot, relDir).filter((entry) => !entry.isDirectory && entry.name.endsWith(".wit")).map((entry) => entry.name).sort().map((name) => policyReadFileSafe(repoRoot, relDir, name)).join("\n");
}

const OS_ENGINE = "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine";
const OS_INFINITE_DIRECTED = "🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed";

/** 🪓️ Sources that were split into mounted child modules (`#[path]` in Rust, relative imports in TSX): policy evidence reads each of them as the one text the unsplit file held. Every other source keeps its own file as its evidence, because its mounted modules are audited in their own right. */
export const POLICY_SPLIT_SOURCE_ROOTS: readonly string[] = [
  `${OS_ENGINE}/🎯️targets/🧊️wgpu/🧊️renderer/🦀️.rs`,
  `${OS_ENGINE}/🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/🦀️.rs`,
  `${OS_ENGINE}/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs`,
  `${OS_ENGINE}/🧱️elements/⚙️EngineCanvas/🎯️targets/🧊️wgpu/🦀️.rs`,
  `${OS_ENGINE}/🧱️elements/🗣️Interpreter/🎯️targets/🧊️wgpu/🦀️.rs`,
  "🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🌍️world/🗃️model/🦀️.rs",
  "🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🌍️world/🧩️components/🦀️.rs",
  "🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🌍️world/🧲️cursors/🦀️.rs",
  "🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🌍️world/🧱️scene/🦀️.rs",
  "🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🌍️world/🌉️bridge/🦀️.rs",
  "🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🌍️world/🧵️interaction/🦀️.rs",
  "🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🌍️world/🎛️host/🦀️.rs",
  `${OS_INFINITE_DIRECTED}/➕️normal/🗃️model/🦀️.rs`,
  `${OS_INFINITE_DIRECTED}/➕️normal/🖼️scene/🦀️.rs`,
  `${OS_INFINITE_DIRECTED}/➕️normal/🎯️focus/🦀️.rs`,
  `${OS_INFINITE_DIRECTED}/➕️normal/🕹️gestures/🦀️.rs`,
  `${OS_INFINITE_DIRECTED}/➕️normal/🪣️fill/🦀️.rs`,
  `${OS_INFINITE_DIRECTED}/➕️normal/🧭️host/🦀️.rs`,
  `${OS_INFINITE_DIRECTED}/🕸️dag/🗃️model/🦀️.rs`,
  `${OS_INFINITE_DIRECTED}/🕸️dag/🕸️graph/🦀️.rs`,
  `${OS_INFINITE_DIRECTED}/🕸️dag/🧭️host/🦀️.rs`,
  "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧵️instance/🦀️.rs",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🦀️.rs",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🏃️runtime/🦀️.rs",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🔄️turn/🦀️.rs",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🏃️run/🦀️.rs",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🌿️vcs/🦀️.rs",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🚪️io/🦀️.rs",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🧠️neural/⚙️engine/🧱️model/🦀️.rs",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🦀️.rs",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧵️shard/🦀️.rs",
  "🧰️framework/🛍️products/💻️os/🖥️host/🦀️.rs",
  "🧰️framework/🔨️modules/⏳️async/🦀️.rs",
  "🧰️framework/🔨️modules/🎒️pack/📐️format/🦀️.rs",
  "🧰️framework/🔨️modules/📡️replication/📡️wire/🦀️.rs",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🛎️services/🦀️.rs",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🛎️services/🧪️tests/🔬️component-unit/🦀️.rs",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🚚️transport/🦀️.rs",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🏠️workspace/🔗️remote/🦀️.rs",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🧵️bridge/🦀️.rs",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🧵️bridge/🪢️session/🦀️.rs",
  "🧰️framework/🔨️modules/🖱️ui/🧠️runtime/♻️reconcile/🦀️.rs",
  "🧰️framework/🔨️modules/🖱️ui/🧠️runtime/⏭️cursor/🦀️.rs",
  "🧰️framework/🔨️modules/🖱️ui/🧠️runtime/🧱️primitives/🦀️.rs",
  "🧰️framework/🔨️modules/🖱️ui/🌳️retained/🌳️tree/🍃️nodes/🦀️.rs",
  "🧰️framework/🔨️modules/🖱️ui/🖌️render/🎬️scene/🦀️.rs",
  "🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧩️component/🦀️.rs",
  "🧰️framework/🔨️modules/🖱️ui/🧬️contract/🛡️limits/🦀️.rs",
  "🧰️framework/🔨️modules/🖱️ui/🧬️contract/🏗️builder/🦀️.rs",
  "🧰️framework/🔨️modules/🗺️surface/🗺️tiled-map/🦀️.rs",
  "🌎️hub/🏗️bootstrap/🦀️.rs",
  "🌎️hub/🏗️bootstrap/⚙️runtime/🦀️.rs",
  "🌎️hub/🏗️bootstrap/📒️ledgers/🦀️.rs",
  "🌎️hub/🏗️bootstrap/🧱️models/🦀️.rs",
  "🌎️hub/🏗️bootstrap/🧠️state/🦀️.rs",
  "🌎️hub/🏗️bootstrap/🛰️session-routes/🦀️.rs",
  "🌎️hub/🏗️bootstrap/📨️document-routes/🦀️.rs",
  "🌎️hub/🏗️bootstrap/🛣️routes/🦀️.rs",
  `${OS_ENGINE}/🧱️elements/🌐️World3dHost/🟦️.tsx`,
  `${OS_ENGINE}/🧱️elements/🏛️ShellHost/🟦️.tsx`,
  "✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs",
  "✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs",
  "✏️s/🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs",
  "✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs",
  "✏️s/🔌️plugins/🏭️process/🗿️artifacts/🧊️process3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs",
  "✏️s/🔌️plugins/🎥️shooting/🗿️artifacts/🎥️shooting/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs",
  "✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🦀️.rs",
  "✏️s/🔌️plugins/🏙️bim/🗿️artifacts/🏢️model/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs",
  "✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🧵️session/🦀️.rs",
  "✏️s/🔌️plugins/🏙️bim/🗿️artifacts/🏢️model/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🗣️terminology/🦀️.rs",
  "✏️s/🔌️plugins/🏙️bim/🗿️artifacts/🏢️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/🩹️patches/🧱️building-elements/🦀️.rs",
  "✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/♻️rewriting/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs",
  "✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs",
  "✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs",
  "✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/✏️editor/🪆️1-any/🎮️commands/🖱️canvas-pointer-down/🦀️.rs",
  "✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs",
  "✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs",
  "✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⏳️precompute/🦀️.rs",
  "✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⏳️precompute/📐️geometry/🦀️.rs",
  "✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⏳️precompute/🪣️fill/🦀️.rs",
  "✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎟️session/🦀️.rs",
  "✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️unit/🦀️.rs",
  "✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧵️retained/🎬️action/🦀️.rs",
  "✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧵️retained/🗂️document/🦀️.rs",
  "✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🦀️.rs",
  "✏️s/🔌️plugins/🧱️block/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs",
  "✏️s/🔌️plugins/🪵️sourcing/🗿️artifacts/🗂️curation/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs",
  "✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs",
  "✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs",
  "✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs",
  "✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/📤️export/🦀️.rs",
  "✏️s/🔌️plugins/📕️norm/📇️registry/🧬️contract/🖥️app-surface/🦀️.rs",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/⚙️engine/🦀️.rs",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/⚙️engine/📖️catalog-read/🦀️.rs",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/⚙️engine/🔭️capability-open/🦀️.rs",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/⚙️engine/🥾️catalog-bootstrap/🦀️.rs",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🔄️sync/🦀️.rs",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🔄️sync/🧪️tests/🔬️unit/🦀️.rs",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🔢️index/🦀️.rs",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/📸️snapshot/🦀️.rs",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗄️storage/🦀️.rs",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗄️storage/🧱️io-core/🦀️.rs",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗄️storage/📁️fs/🦀️.rs",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗄️storage/🗃️memory/🦀️.rs",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗄️storage/🪶️sqlite/🦀️.rs",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗄️storage/🐘️postgres/🦀️.rs",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗿️artifact/🧬️contract/🦀️.rs",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗿️artifact/🧪️tests/🔬️unit/🦀️.rs",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗿️artifact/⚙️engine/🧪️tests/🔬️unit/🦀️.rs",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/📝️wal/🧪️tests/🔬️unit/🦀️.rs",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/⌨️cli/🦀️.rs",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🔍️query/🦀️.rs",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🔘️state/🦀️.rs",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗄️storage/🌐️neo4j/🦀️.rs",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗄️storage/📇️io-registry/🦀️.rs",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗄️storage/🚦️io-driver/🦀️.rs",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗄️storage/🚦️io-driver/🧪️tests/🔬️db-io-retained-fixtures/🦀️.rs",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗿️artifact/🦀️.rs",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗿️artifact/📜️history/🦀️.rs",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗿️artifact/⚙️engine/🦀️.rs",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/⚙️engine/🧪️tests/🔬️unit/🦀️.rs",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/⚙️engine/🎫️handle/🦀️.rs",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/⚙️engine/🏗️catalog-create/🦀️.rs",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/📝️wal/🧾️log/🦀️.rs",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗜️compact/🦀️.rs",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔄️sync/🦀️.rs",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔗️read/🦀️.rs",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️tests/🌅️bin-05/🔬️unit/🦀️.rs",
];

/** 🌳️ Lists the repo-relative files of a split source's module tree (the root first, then its mounted children) through the symlink-refusing policy read; the crates of the renderer partition are not followed unless `crates: true` is passed, because the canonical test-mount rule of the evidence reader does not know the crate roots' `🧪️tests/🦀️.rs` stubs. */
export function policyModuleTreeFiles(repoRoot: string, relPath: string, options: Omit<ModuleTreeOptions, "read"> = {}): string[] {
  return listModuleTree(join(repoRoot, relPath), { crates: false, ...options, read: (path) => policyReadFileSafe(repoRoot, relative(repoRoot, path)) }).map((path) => relative(repoRoot, path).split(sep).join("/"));
}

/** 📖️ Reads a split source as the one text the unsplit file held: the root and every mounted child, through the symlink-refusing policy read. */
export function policyReadModuleTreeSource(repoRoot: string, relPath: string, options: Omit<ModuleTreeOptions, "read"> = {}): string {
  return readModuleTreeSource(join(repoRoot, relPath), { crates: false, ...options, read: (path) => policyReadFileSafe(repoRoot, relative(repoRoot, path)) });
}
