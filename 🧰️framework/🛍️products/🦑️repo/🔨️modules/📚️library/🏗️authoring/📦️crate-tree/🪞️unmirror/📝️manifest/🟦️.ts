/** 📝️ Manifest edits of an unmirror run: path dependencies on the crates that now own a re-pointed item are added, dependencies nothing references any more are removed. Plain line edits keep the hand-formatted manifests intact. */

/** ➕️ A dependency to add: the package name, the path from the manifest directory and the section. */
export type DependencyAddition = Readonly<{ name: string; path: string; section: "dependencies" | "dev-dependencies" }>;

const sectionHeader = /^\[([^\]]+)\]\s*$/u;
const dependencyKey = /^([A-Za-z0-9_-]+)\s*=/u;

/** 📋️ The package names a manifest declares as dependencies, per section header (target tables included). */
export function declaredDependencies(manifest: string): Map<string, Set<string>> {
  const declared = new Map<string, Set<string>>();
  let section = "";
  for (const line of manifest.split("\n")) {
    const header = sectionHeader.exec(line);
    if (header) { section = header[1]!; continue; }
    if (!/(?:^|\.)(?:dev-|build-)?dependencies$/u.test(section)) continue;
    const key = dependencyKey.exec(line)?.[1];
    if (key) declared.set(key, new Set([...(declared.get(key) ?? []), section]));
  }
  return declared;
}

/** ➕️ Inserts dependency lines into the matching section in byte order, creating the section when it is missing. */
export function addDependencyLines(manifest: string, additions: readonly DependencyAddition[]): string {
  let lines = manifest.split("\n");
  for (const addition of additions) {
    const line = `${addition.name} = { path = ${JSON.stringify(addition.path)} }`;
    const header = lines.findIndex((entry) => entry.trim() === `[${addition.section}]`);
    if (header < 0) {
      while (lines.length && lines.at(-1)!.trim() === "") lines.pop();
      lines = [...lines, "", `[${addition.section}]`, line, ""];
      continue;
    }
    let end = header + 1;
    let insert = -1;
    let last = header;
    while (end < lines.length && !sectionHeader.test(lines[end]!)) {
      const key = dependencyKey.exec(lines[end]!)?.[1];
      if (key && insert < 0 && key > addition.name) insert = end;
      if (lines[end]!.trim() !== "") last = end;
      end += 1;
    }
    if (insert < 0) insert = last + 1;
    lines.splice(insert, 0, line);
  }
  return lines.join("\n");
}

/** ➖️ Removes the single-line dependency entries of the given package names from every dependency section. */
export function removeDependencyLines(manifest: string, names: ReadonlySet<string>): string {
  let section = "";
  return manifest.split("\n").filter((line) => {
    const header = sectionHeader.exec(line);
    if (header) { section = header[1]!; return true; }
    if (!/(?:^|\.)(?:dev-)?dependencies$/u.test(section)) return true;
    const key = dependencyKey.exec(line)?.[1];
    return !(key && names.has(key) && !line.includes("[") && (line.trim().endsWith("}") || !line.includes("{")));
  }).join("\n");
}
