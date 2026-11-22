import { spawnSync } from "node:child_process";
import { existsSync, readdirSync, readFileSync } from "node:fs";
import { dirname, join, resolve, sep } from "node:path";

type ProjectTargetsV1 = Readonly<{ name?: string; targets?: Readonly<Record<string, Readonly<{ outputs?: readonly unknown[] }>>> }>;

const SKIPPED_DIRECTORIES_V1 = new Set(["node_modules", ".git", ".cache", "target", ".🧬semio", "🗑️generated"]);
const GENERATED_SEGMENT_V1 = "🤖️generated";
const MOUNT_PATTERN_V1 = /#\[\s*path\s*=\s*"([^"]+)"\s*\]\s*(?:pub(?:\([^)]*\))?\s+)?mod\s+[A-Za-z_][A-Za-z0-9_]*\s*;/gu;

const projectFilesV1 = (directory: string, depth: number): string[] => {
  if (!existsSync(directory)) return [];
  const found: string[] = [], manifest = join(directory, "📋️project.json");
  if (existsSync(manifest)) found.push(manifest);
  if (depth > 0) for (const entry of readdirSync(directory, { withFileTypes: true })) if (entry.isDirectory() && !SKIPPED_DIRECTORIES_V1.has(entry.name)) found.push(...projectFilesV1(join(directory, entry.name), depth - 1));
  return found;
};

const ignoredByGitV1 = (root: string, path: string): boolean => spawnSync("git", ["check-ignore", "-q", "--", path], { cwd: root, stdio: "ignore", shell: false }).status === 0;

/** 🏭️ Names the `project:target` pairs that produce a missing Rust module: a target declaring the file among its outputs, or else the `generate` target of a project near it, when the file is untracked generated output.
 * @see https://doc.rust-lang.org/reference/items/modules.html#the-path-attribute */
export function generatedModuleOwnersV1(root: string, missing: string): readonly string[] {
  const absolute = resolve(missing), generated = absolute.split(sep).includes(GENERATED_SEGMENT_V1) || ignoredByGitV1(root, absolute);
  const owners = new Set<string>(), fallback = new Set<string>();
  let directory = dirname(absolute);
  for (let level = 0; level < 6 && directory.startsWith(root) && directory !== root && owners.size === 0 && fallback.size === 0; level++, directory = dirname(directory)) {
    for (const manifest of projectFilesV1(directory, 4)) {
      let project: ProjectTargetsV1;
      try { project = JSON.parse(readFileSync(manifest, "utf8")) as ProjectTargetsV1; } catch { continue; }
      if (project.name === undefined || project.targets === undefined) continue;
      for (const [target, declaration] of Object.entries(project.targets)) {
        const produces = (declaration.outputs ?? []).some(output => typeof output === "string" && (() => {
          const base = resolve(output.replace("{projectRoot}", dirname(manifest)).replace("{workspaceRoot}", root));
          return absolute === base || absolute.startsWith(base + sep);
        })());
        if (produces) owners.add(`${project.name}:${target}`);
        else if (generated && target === "generate") fallback.add(`${project.name}:${target}`);
      }
    }
  }
  const chosen = owners.size > 0 ? owners : fallback;
  return [...chosen].sort();
}

const rustFilesV1 = (directory: string): string[] => readdirSync(directory, { withFileTypes: true }).flatMap(entry =>
  entry.isDirectory() ? SKIPPED_DIRECTORIES_V1.has(entry.name) ? [] : rustFilesV1(join(directory, entry.name)) : entry.name.endsWith(".rs") ? [join(directory, entry.name)] : []);

const mountingFilesV1 = (root: string): string[] => {
  const listed = spawnSync("git", ["grep", "-l", "-z", "-F", "--untracked", "#[path", "--", "*.rs", ":(exclude).🧬semio"], { cwd: root, encoding: "utf8", shell: false, maxBuffer: 1 << 28 });
  return listed.status === 0 || listed.status === 1 ? listed.stdout.split("\0").filter(Boolean).map(file => join(root, file)) : rustFilesV1(root);
};

/** 🔎️ Lists the files that Rust `#[path]` module mounts under `root` point to and that do not exist yet. */
export function missingMountedModulesV1(root: string): readonly string[] {
  const missing = new Set<string>();
  for (const file of mountingFilesV1(root)) {
    const source = readFileSync(file, "utf8");
    for (const match of source.matchAll(MOUNT_PATTERN_V1)) {
      const target = resolve(dirname(file), match[1]!);
      if (!existsSync(target)) missing.add(target);
    }
  }
  return [...missing].sort();
}

/** 🧱️ Neutral `run` requests, one per owning `project:target`, that must succeed before any Rust source closure of the workspace is read. */
export function generatedPrerequisiteArgumentsV1(root: string, missing: readonly string[] = missingMountedModulesV1(root)): readonly (readonly string[])[] {
  const targets = new Set(missing.flatMap(path => generatedModuleOwnersV1(root, path)));
  return [...targets].sort().map(target => Object.freeze(["run", target]));
}
