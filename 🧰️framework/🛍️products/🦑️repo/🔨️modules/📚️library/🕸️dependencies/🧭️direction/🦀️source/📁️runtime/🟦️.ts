import type { RustPathLiteral } from "@semio-tech/compiler-syntax-rust-ts/paths";
import { dependencyDirectionPathMatches as matches, type DependencyDirectionRule } from "../../🟦️.ts";

/** 🪪️ A syntactic owner boundary reference with exact argument provenance. */
export type RustRuntimePathDirectionEdge = Readonly<{
  rule: string; from: string; to: string; line: number; call: string;
  context: RustPathLiteral["context"]; start: number; end: number;
}>;

/** 🧱️ Applies declared strict layer selectors to decoded owner-qualified path arguments. */
export function rustRuntimePathDirectionEdges(from: string, references: readonly RustPathLiteral[], rules: readonly DependencyDirectionRule[]): readonly RustRuntimePathDirectionEdge[] {
  if(!references.length)return [];
  const applicable = rules.filter(rule=>rule.severity==="error"&&matches(rule.from.path,from)&&!matches(rule.from.pathNot??[],from));
  return references.flatMap(reference => applicable.filter(rule =>
    (matches(rule.to.path, reference.path) || matches(rule.to.path, reference.path + "/"))
    && !matches(rule.to.pathNot ?? [], reference.path) && !matches(rule.to.pathNot ?? [], reference.path + "/")
  ).map(rule => ({ rule: rule.name, from, to: reference.path, line: reference.line, call: reference.call, context: reference.context, start: reference.start, end: reference.end })));
}

