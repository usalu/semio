import { posix } from "node:path";
import { inspectRustCompileReferences, type RustCompileReference, type RustModuleContext } from "../../../🔍️discovery/🟦️.ts";
import type { DependencyDirectionRule } from "../🟦️.ts";

export type RustSourceReference = RustCompileReference;
export type RustSourceDirectionEdge = Readonly<{ rule: string; from: string; to: string; kind: RustSourceReference["kind"]; line: number }>;

/** 📎️ Uses the shared Rust source scanner as the compile-time dependency contract. */
export function rustSourceReferences(source: string): readonly RustSourceReference[] {
  return inspectRustCompileReferences(source);
}

/** 🏛️ Enforces the authored boundary policy on compile-time file inputs, including fixture and test edges. */
export function rustSourceDirectionEdges(from: string, references: readonly RustSourceReference[], rules: readonly DependencyDirectionRule[], ownership: Readonly<{ contexts?: readonly RustModuleContext[]; manifestPaths?: readonly string[] }> = {}): readonly RustSourceDirectionEdge[] {
  const edges: RustSourceDirectionEdge[] = [];
  const matches = (patterns: readonly string[], path: string): boolean => patterns.some((pattern) => new RegExp(pattern, "u").test(path));
  for (const reference of references) {
    if (reference.base === "generated") continue;
    const path = reference.path.replaceAll("\\", "/");
    if ((!reference.base && posix.isAbsolute(path)) || /^[A-Za-z]:/u.test(path)) throw new Error(`Rust source dependency escapes the authored workspace: ${from} → ${reference.path}`);
    let bases = [posix.dirname(from)];
    if (reference.base === "manifest") {
      const manifests = [...new Set([...(ownership.contexts ?? []).flatMap((context) => context.manifestPath ? [context.manifestPath] : []), ...(ownership.manifestPaths ?? [])])];
      if (!manifests.length) throw new Error(`Rust source dependency requires manifest provenance: ${from}:${reference.line}`);
      bases = manifests.map((manifest) => posix.dirname(manifest));
    } else if (reference.kind === "path" && reference.modulePath?.length) {
      bases = [...new Set((ownership.contexts ?? []).filter((context) => context.sourceScope.join("::") === reference.modulePath!.join("::")).map((context) => context.moduleBase))];
      if (!bases.length) throw new Error(`Rust source dependency requires module provenance: ${from}:${reference.line}; module=${reference.modulePath.join("::")}`);
    }
    for (const base of bases) {
      const to = posix.normalize(posix.join(base, reference.base ? path.slice(1) : path));
      if (to === ".." || to.startsWith("..")) throw new Error(`Rust source dependency escapes the authored workspace: ${from} → ${reference.path}`);
      const targetMatches = (patterns: readonly string[]): boolean => matches(patterns, to) || reference.directory === true && matches(patterns, `${to}/`);
      for (const rule of rules) if (rule.severity === "error" && matches(rule.from.path, from) && !matches(rule.from.pathNot ?? [], from) && targetMatches(rule.to.path) && !targetMatches(rule.to.pathNot ?? [])) edges.push({ rule: rule.name, from, to, kind: reference.kind, line: reference.line });
    }
  }
  return edges.filter((edge, index) => edges.findIndex((prior) => JSON.stringify(prior) === JSON.stringify(edge)) === index);
}
