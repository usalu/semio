/** 🔌️ The run-time source text of the whole plugin family: every Rust source file below the plugin folder that is not a canonical `🧪️tests` file or a companion test crate, in path order. Source-text laws whose anchors lived in the old plugin monolith read it instead of the facade file the cut left; it does not depend on which crate mounts a file, so a carve that moves a mount between crates never moves an anchor out of reach. */
import { readdirSync, readFileSync } from "node:fs";
import { join, resolve } from "node:path";

const plugin = resolve(import.meta.dir, "../../..");
const modules = resolve(plugin, "..");
const SKIPPED = new Set(["🧷️companions", "🧪️tests", "node_modules", "target", "🗑️generated"]);
let memo: string | undefined;
const sibling = new Map<string, string>();
const subtrees = new Map<string, string>();

/** 🌳️ Every non-test `.rs` file below `directory`, depth first in name order. */
function sources(directory: string, files: string[] = []): string[] {
  for (const entry of readdirSync(directory, { withFileTypes: true }).sort((left, right) => (left.name < right.name ? -1 : 1))) {
    if (entry.isDirectory()) {
      if (!SKIPPED.has(entry.name)) sources(join(directory, entry.name), files);
    } else if (entry.name.endsWith(".rs")) files.push(join(directory, entry.name));
  }
  return files;
}

/** 🔌️ The text of every plugin source file, read once per process; each file ends in a newline. */
export function pluginFamilyText(): string {
  memo ??= sources(plugin).map(path => readFileSync(path, "utf8")).map(text => (text.endsWith("\n") ? text : text + "\n")).join("");
  return memo;
}

/** 🔌️ The text of every non-test Rust source file of a sibling module of the plugin (for example `🏪️store`), read once per process, with the same rules as the plugin family. */
export function moduleFamilyText(name: string): string {
  let text = sibling.get(name);
  if (text === undefined) {
    text = sources(join(modules, name)).map(path => readFileSync(path, "utf8")).map(source => (source.endsWith("\n") ? source : source + "\n")).join("");
    sibling.set(name, text);
  }
  return text;
}

/** 🔌️ The text of every non-test Rust source file below one subfolder of the plugin (for example `⏪️time-travel`), read once per process: the narrowest honest scope for a law about one plugin domain, because a negative assertion over the whole family would see unrelated code. */
export function pluginSubtreeText(...segments: string[]): string {
  const key = segments.join("/");
  let text = subtrees.get(key);
  if (text === undefined) {
    text = sources(join(plugin, ...segments)).map(path => readFileSync(path, "utf8")).map(source => (source.endsWith("\n") ? source : source + "\n")).join("");
    subtrees.set(key, text);
  }
  return text;
}

/** 🔌️ The text of every non-test Rust source file below a folder of a sibling module of the plugin (for example `📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell`), read once per process, for laws about one owner folder whose crate root was cut into staged files. */
export function moduleSubtreeText(...segments: string[]): string {
  const key = "module/" + segments.join("/");
  let text = subtrees.get(key);
  if (text === undefined) {
    text = sources(join(modules, ...segments)).map(path => readFileSync(path, "utf8")).map(source => (source.endsWith("\n") ? source : source + "\n")).join("");
    subtrees.set(key, text);
  }
  return text;
}

/** 🧪️ The text of every Rust file below one folder of the plugin, test folders included (read once per process): for laws about a Rust test whose file the cut split into topic folders next to it. */
export function pluginTestTreeText(...segments: string[]): string {
  const key = "tests/" + segments.join("/");
  let text = subtrees.get(key);
  if (text === undefined) {
    const files: string[] = [];
    const walk = (directory: string): void => {
      for (const entry of readdirSync(directory, { withFileTypes: true }).sort((left, right) => (left.name < right.name ? -1 : 1))) {
        if (entry.isDirectory()) { if (entry.name !== "node_modules" && entry.name !== "target") walk(join(directory, entry.name)); }
        else if (entry.name.endsWith(".rs")) files.push(join(directory, entry.name));
      }
    };
    walk(join(plugin, ...segments));
    text = files.map(path => readFileSync(path, "utf8")).map(source => (source.endsWith("\n") ? source : source + "\n")).join("");
    subtrees.set(key, text);
  }
  return text;
}
