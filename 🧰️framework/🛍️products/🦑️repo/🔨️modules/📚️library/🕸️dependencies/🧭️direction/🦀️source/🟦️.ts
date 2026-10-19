import { posix } from "node:path";
import { type RustModuleContext } from "../../../🔍️discovery/🌐️rust-module-graph/🟦️.ts";
import { type RustCompileReference } from "@semio-tech/compiler-syntax-rust-ts";
import { dependencyDirectionPathMatches as matches, type DependencyDirectionRule } from "../🟦️.ts";


export type RustSourceDirectionEdge = Readonly<{ rule: string; from: string; to: string; kind: RustCompileReference["kind"]; line: number; expansion?: RustCompileReference["expansion"] }>;
export type RustSourceOwnership = Readonly<{ contexts?: readonly RustModuleContext[]; manifestPaths?: readonly string[] }>;
export type RustSourceTarget = Readonly<{ to: string; reference: RustCompileReference; directories: readonly string[] }>;
export type RustSourceInputNode = Readonly<{ path: string; kind: "file" | "directory" | "symlink" }>;
export type RustSourceInputInventory = ReadonlyMap<string, RustSourceInputNode["kind"]>;
export type RustSourceInputProblem = "linked-input" | "missing-input" | "non-directory-ancestor" | "unexpected-input-kind" | "uncensused-source";

/** 🔒️ Requires every physical authored input and ancestor to be visible without following links. */
export function rustSourceTargetProblem(target: string, directory: boolean, inventory: RustSourceInputInventory, sources: ReadonlySet<string>): RustSourceInputProblem | null {
  const parts = target.split("/");
  for (let index = 0; index < parts.length; index++) {
    const path = parts.slice(0, index + 1).join("/"), kind = inventory.get(path), leaf = index === parts.length - 1;
    if (kind === undefined) return "missing-input";
    if (kind === "symlink") return "linked-input";
    if (!leaf && kind !== "directory") return "non-directory-ancestor";
    if (leaf && kind !== (directory ? "directory" : "file")) return "unexpected-input-kind";
  }
  return !directory && target.endsWith(".rs") && !sources.has(target) ? "uncensused-source" : null;
}



/** 🧭️ Resolves each authored input using Rust module and Cargo manifest provenance. */
export function rustSourceTargets(from: string, references: readonly RustCompileReference[], ownership: RustSourceOwnership = {}): readonly RustSourceTarget[] {
  const targets: RustSourceTarget[] = [];
  for (const reference of references) {
    if (reference.inlineBase !== undefined) {
      if (reference.kind !== "path" || !reference.modulePath?.length || reference.base || !reference.inlineBase || posix.isAbsolute(reference.inlineBase) || /^[A-Za-z]:/u.test(reference.inlineBase) || /[\\\0]/u.test(reference.inlineBase)) throw new Error(`Rust source dependency requires a portable scoped inline base: ${from}:${reference.line}`);
      const base = posix.normalize(`${posix.dirname(from)}/${reference.inlineBase}`);
      if ((ownership.contexts ?? []).some((context) => context.sourceScope.join("::") === reference.modulePath!.join("::") && posix.normalize(context.moduleBase) !== base)) throw new Error(`Rust source dependency has contradictory inline base provenance: ${from}:${reference.line}`);
    }
    if (reference.base === "generated") continue;
    const path = reference.path.replaceAll("\\", "/");
    if ((!reference.base && posix.isAbsolute(path)) || /^[A-Za-z]:/u.test(path)) throw new Error(`Rust source dependency escapes the authored workspace: ${from} → ${reference.path}`);
    let bases = [posix.dirname(from)];
    if (reference.base === "manifest") {
      const manifests = [...new Set([...(ownership.contexts ?? []).flatMap((context) => context.manifestPath ? [context.manifestPath] : []), ...(ownership.manifestPaths ?? [])])];
      if (!manifests.length) throw new Error(`Rust source dependency requires manifest provenance: ${from}:${reference.line}`);
      bases = manifests.map((manifest) => posix.dirname(manifest));
    } else if (reference.kind === "path" && reference.modulePath?.length) {
      bases = reference.inlineBase !== undefined ? [`${posix.dirname(from)}/${reference.inlineBase}`] : [...new Set((ownership.contexts ?? []).filter((context) => context.sourceScope.join("::") === reference.modulePath!.join("::")).map((context) => context.moduleBase))];
      if (!bases.length) throw new Error(`Rust source dependency requires module provenance: ${from}:${reference.line}; module=${reference.modulePath.join("::")}`);
    }
    for (const base of bases) {
      const segments = `${base}/${reference.base ? path.slice(1) : path}`.split("/"), current: string[] = [], directories: string[] = [];
      for (let index = 0; index < segments.length; index++) {
        const segment = segments[index]!;
        if (!segment || segment === ".") continue;
        if (segment === "..") {
          if (!current.length) throw new Error(`Rust source dependency escapes the authored workspace: ${from} → ${reference.path}`);
          current.pop();
        } else {
          current.push(segment);
          if (index < segments.length - 1 || reference.directory) directories.push(current.join("/"));
        }
      }
      targets.push({ to: current.join("/") || ".", reference, directories: [...new Set(directories)] });
    }
  }
  return distinctRustRows(targets);
}

/** 🏛️ Enforces the authored boundary policy on compile-time file inputs, including fixture and test edges. */
export function rustSourceDirectionEdges(from: string, references: readonly RustCompileReference[], rules: readonly DependencyDirectionRule[], ownership: RustSourceOwnership = {}): readonly RustSourceDirectionEdge[] {
  const targets=rustSourceTargets(from,references,ownership);
  if(!targets.length)return [];
  const applicable = rules.filter(rule=>rule.severity==="error"&&matches(rule.from.path,from)&&!matches(rule.from.pathNot??[],from));
  return distinctRustRows(targets.flatMap(({ to, reference }) => {
    const targetMatches = (patterns: readonly string[]): boolean => matches(patterns, to) || reference.directory === true && matches(patterns, `${to}/`);
    return applicable.filter((rule) => targetMatches(rule.to.path) && !targetMatches(rule.to.pathNot ?? [])).map((rule) => ({ rule: rule.name, from, to, kind: reference.kind, line: reference.line, ...(reference.expansion ? { expansion: reference.expansion } : {}) }));
  }));
}

/** 🧮️ Retains the first identical owned fact with one serialization per row. */
function distinctRustRows<T>(rows: readonly T[]): readonly T[] {
  const seen = new Set<string | undefined>();
  return rows.filter((row) => {
    const key = JSON.stringify(row);
    if (seen.has(key)) return false;
    seen.add(key);
    return true;
  });
}
