import type { RustPathLiteral } from "../../../../../../../../🔨️modules/📚️compiler/📖️syntax/🦀️rust/📁️paths/🟦️.ts";
import type { DependencyDirectionRule } from "../../🟦️.ts";

/** 🪪️ A syntactic owner boundary reference with exact argument provenance. */
export type RustRuntimePathDirectionEdge = Readonly<{
  rule: string; from: string; to: string; line: number; call: string;
  context: RustPathLiteral["context"]; start: number; end: number;
}>;

/** 🧱️ Applies declared strict layer selectors to decoded owner-qualified path arguments. */
export function rustRuntimePathDirectionEdges(from: string, references: readonly RustPathLiteral[], rules: readonly DependencyDirectionRule[]): readonly RustRuntimePathDirectionEdge[] {
  const matches = (patterns: readonly string[], path: string): boolean => patterns.some(pattern => new RegExp(pattern, "u").test(path));
  return references.flatMap(reference => rules.filter(rule =>
    rule.severity === "error" && matches(rule.from.path, from) && !matches(rule.from.pathNot ?? [], from)
    && (matches(rule.to.path, reference.path) || matches(rule.to.path, reference.path + "/"))
    && !matches(rule.to.pathNot ?? [], reference.path) && !matches(rule.to.pathNot ?? [], reference.path + "/")
  ).map(rule => ({ rule: rule.name, from, to: reference.path, line: reference.line, call: reference.call, context: reference.context, start: reference.start, end: reference.end })));
}

