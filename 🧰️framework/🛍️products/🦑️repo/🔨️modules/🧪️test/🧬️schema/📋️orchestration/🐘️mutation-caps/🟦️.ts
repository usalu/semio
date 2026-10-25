/** 🐘️ Mutation cap gate
 *
 * The structural cap rule: every unbounded leaf of a generic aggregate declares a cap law.
 */

import { readFileSync } from "node:fs";
import { join } from "node:path";
import { isRecord, readJsonObject } from "../📚️schema-documents/🟦️.ts";
import { mutationArtifactScope, mutationTree, sharedSegments } from "../🌳️mutation-tree/🟦️.ts";

//#region 🐘️MutationCaps
/** 🍃️ One mutation leaf whose payload schema caps its inverse rows (`x-semio-inverse-rows.bounded`): its directory, the aggregate
 * variant its descriptor names and the cap. */
export type MutationCapLeaf = Readonly<{ directory: string; variant: string; bounded: number }>;

/** 🎩️ One `#[derive(Mutations)]` aggregate as the cap rule reads it: its source path, name, variants and whether it is generic. The
 * derive emits the `#[cfg(test)]` payload law (`semio_payload_law_<aggregate>`: every operation answers its schema-declared inverse
 * rows, every fixture case's inverse fits them, and a leaf past the store ceiling is refused `mutation.too-large`) only for a
 * non-generic aggregate. */
export type MutationCapAggregate = Readonly<{ path: string; name: string; variants: readonly string[]; generic: boolean }>;

/** 🪤️ A bounded leaf the derived cap law never runs over. */
export type MutationCapFinding = Readonly<{ code: "capLawMissing"; directory: string; detail: string }>;

/** 🧢️ The cap `x-semio-inverse-rows.bounded` of a leaf payload schema, or `null` when it declares none. */
export function mutationLeafBound(schema: Record<string, unknown>): number | null {
  const rows = schema["x-semio-inverse-rows"];
  return isRecord(rows) && typeof rows.bounded === "number" && Number.isInteger(rows.bounded) && rows.bounded > 0 ? rows.bounded : null;
}

/** 🧗️ Whether the enum `name` of a Rust source declares generic parameters. */
export function mutationAggregateGeneric(source: string, name: string): boolean {
  return new RegExp(`\\benum\\s+${name}\\s*<`, "u").test(source);
}

/** 🏔️ The structural cap rule (audit F3, coordinator decision 2026-10-05): every `bounded` leaf is wrapped by a `#[derive(Mutations)]`
 * aggregate that is not generic — the nearest one of its own artifact that names its variant, never a foreign artifact's — so the
 * derived payload law holds its declared rows. The per-leaf oracle is that Rust law; this rule only proves the law exists. */
export function mutationCapFindings(leaves: readonly MutationCapLeaf[], aggregates: readonly MutationCapAggregate[]): MutationCapFinding[] {
  return leaves.flatMap((leaf): MutationCapFinding[] => {
    const scope = mutationArtifactScope(leaf.directory);
    const scored = aggregates.filter((aggregate) => aggregate.variants.includes(leaf.variant) && mutationArtifactScope(aggregate.path) === scope).map((aggregate) => [sharedSegments(aggregate.path, leaf.directory), aggregate] as const);
    const best = Math.max(0, ...scored.map(([score]) => score));
    const nearest = scored.filter(([score]) => score === best && best > 0).map(([, aggregate]) => aggregate);
    if (nearest.length === 0) return [{ code: "capLawMissing", directory: leaf.directory, detail: `the leaf caps its inverse at ${leaf.bounded} row(s) but no #[derive(Mutations)] aggregate of its artifact wraps ${leaf.variant}: a hand-written aggregate runs no derived cap law` }];
    return nearest.every((aggregate) => aggregate.generic) ? [{ code: "capLawMissing", directory: leaf.directory, detail: `the leaf caps its inverse at ${leaf.bounded} row(s) but its aggregate ${nearest[0]!.name} is generic: the derive emits no payload law for a generic aggregate` }] : [];
  });
}

/** 📐️ The cap rule over every mutation leaf under `under`: the bounded leaves and those without the derived cap law. */
export function mutationCapReport(repoRoot: string, under = ""): { readonly leaves: readonly MutationCapLeaf[]; readonly findings: readonly MutationCapFinding[] } {
  const tree = mutationTree(repoRoot);
  const leaves = tree.leaves.filter((leaf) => leaf.directory.startsWith(under)).flatMap((leaf) => {
    const schema = readJsonObject(repoRoot, leaf.schemaPath);
    const bounded = schema === null ? null : mutationLeafBound(schema);
    return bounded === null ? [] : [{ directory: leaf.directory, variant: leaf.variant, bounded }];
  });
  const sources = new Map<string, string>();
  const aggregates = tree.aggregates.map((aggregate) => {
    if (!sources.has(aggregate.path)) sources.set(aggregate.path, readFileSync(join(repoRoot, aggregate.path), "utf8"));
    return { path: aggregate.path, name: aggregate.name, variants: [...aggregate.variants.keys()], generic: mutationAggregateGeneric(sources.get(aggregate.path)!, aggregate.name) };
  });
  return { leaves, findings: mutationCapFindings(leaves, aggregates) };
}
//#endregion 🐘️MutationCaps
