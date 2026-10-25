/** 🧲️ Artifact routers: ownership of a mutation or inference route, conflict-checked registration, and the mutation and inference routers.
 *
 * 🧱️ Child module of `🎠️kernel`; importers import this module directly, and the kernel package entry {@link ../📦️packages/🟦️typescript/🟦️.ts} lists its public names.
 */

import { type ContributedInferenceMetadata, type ContributedMutationMetadata } from "../🔖️artifact-contribution/🟦️.ts";

// #region 🎠️Kernel
//#region 🔖️ArtifactRouters
/** 🧯 Thrown by both routers' `registerContributed` when the same `(artifactKind, key)` is claimed
 * twice with non-identical metadata — contract freeze §4 rule 3's conflict rule. */
export class ArtifactRouterConflictError extends Error {
  readonly code = "artifact-router.conflict" as const;
  constructor(artifactKind: string, key: string) {
    super(`router conflict: ${artifactKind}#${key} already registered with different metadata`);
    this.name = "ArtifactRouterConflictError";
  }
}

/** ❌️ Thrown when a contributor registers onto an artifact kind whose owning plugin is not a direct
 * entry in the contributor's declared `dependencies` — contract freeze §4 rule 1. Carries the frozen
 * transaction rejection code since an unpermitted contribution is exactly what that code names. */
export class ArtifactContributionNotPermittedError extends Error {
  readonly code = "transaction.contribution-not-permitted" as const;
  constructor(contributorPluginId: string, ownerPluginId: string) {
    super(`"${contributorPluginId}" may not contribute onto "${ownerPluginId}"'s artifact kind — not a direct dependency`);
    this.name = "ArtifactContributionNotPermittedError";
  }
}

/** 🔢️ Deterministic deep stringify (sorted object keys) — the "byte-identical metadata" idempotence
 * check contract freeze §4 rule 3 asks for, without requiring an actual byte-level codec on the TS
 * side (mirrors the same rule Rust's `ArtifactInferenceRouter::register_plugin` already enforces). */
function stableStringify(value: unknown): string {
  if (value === null || typeof value !== "object") return JSON.stringify(value);
  if (Array.isArray(value)) return `[${value.map(stableStringify).join(",")}]`;
  const record = value as Record<string, unknown>;
  const keys = Object.keys(record).sort();
  return `{${keys.map((key) => `${JSON.stringify(key)}:${stableStringify(record[key])}`).join(",")}}`;
}

export type ArtifactRouterOwnership = { readonly kind: "owner" } | { readonly kind: "contributed"; readonly pluginId: string };

/** 🗂️ Shared conflict-checked `(artifactKind, key) -> ownership` table both routers below build on —
 * contract freeze §4 rule 3's conflict rule in one place instead of duplicated per router. */
class ConflictCheckedRegistry {
  private readonly entries = new Map<string, { readonly ownership: ArtifactRouterOwnership; readonly fingerprint: string }>();

  register(artifactKind: string, key: string, ownership: ArtifactRouterOwnership, metadata: unknown): void {
    const compositeKey = `${artifactKind} ${key}`;
    const fingerprint = stableStringify(metadata);
    const existing = this.entries.get(compositeKey);
    if (existing && existing.fingerprint !== fingerprint) throw new ArtifactRouterConflictError(artifactKind, key);
    this.entries.set(compositeKey, { ownership, fingerprint });
  }

  resolve(artifactKind: string, key: string): ArtifactRouterOwnership | undefined {
    return this.entries.get(`${artifactKind} ${key}`)?.ownership;
  }
}

/** 🏺️ `(artifactKind, mutationId) -> Owner | Contributed{pluginId}` — the transaction coordinator's
 * `ArtifactMutationRouter` lookup (contract freeze §5.3). */
export class ArtifactMutationRouter {
  private readonly registry = new ConflictCheckedRegistry();

  registerOwner(artifactKind: string, mutationId: string): void {
    this.registry.register(artifactKind, mutationId, { kind: "owner" }, { kind: "owner", artifactKind, mutationId });
  }

  /** 🧬️ Contract freeze §4 rule 1 registration gate: `contributorDependsOnOwner` must already be true —
   * callers derive it from a {@link PluginGraph} lookup (the contributor's declared `dependencies`
   * directly naming `ownerPluginId`) before calling this. */
  registerContributed(artifactKind: string, contributorPluginId: string, ownerPluginId: string, metadata: ContributedMutationMetadata, contributorDependsOnOwner: boolean): void {
    if (!contributorDependsOnOwner) throw new ArtifactContributionNotPermittedError(contributorPluginId, ownerPluginId);
    this.registry.register(artifactKind, metadata.mutationId, { kind: "contributed", pluginId: contributorPluginId }, metadata);
  }

  resolve(artifactKind: string, mutationId: string): ArtifactRouterOwnership | undefined {
    return this.registry.resolve(artifactKind, mutationId);
  }
}

/** 💡️ `(artifactKind, inferenceSchema) -> Owner | Contributed{pluginId}`, plus the contributed
 * `dependsOn` DAG (contract freeze §5.3/§6's `dependencies` list on `artifact-inference-request`) —
 * the transaction coordinator's contributor-aware `ArtifactInferenceRouter`. */
export class ArtifactInferenceRouter {
  private readonly registry = new ConflictCheckedRegistry();
  private readonly dependsOn = new Map<string, readonly string[]>();

  registerOwner(artifactKind: string, inferenceSchema: string): void {
    this.registry.register(artifactKind, inferenceSchema, { kind: "owner" }, { kind: "owner", artifactKind, inferenceSchema });
  }

  /** 📐️ Contract freeze §4 rules 1+4: the contributor must directly depend on the owner, and the
   * metadata's own `owner`/`contributor` must match each other and `artifactKind` must match the
   * target. */
  registerContributed(artifactKind: string, metadata: ContributedInferenceMetadata, contributorDependsOnOwner: boolean): void {
    if (metadata.owner !== metadata.contributor) {
      throw new Error(`contributed inference owner/contributor mismatch: ${metadata.owner} !== ${metadata.contributor}`);
    }
    if (metadata.artifactKind !== artifactKind) {
      throw new Error(`contributed inference artifactKind mismatch: ${metadata.artifactKind} !== ${artifactKind}`);
    }
    if (!contributorDependsOnOwner) throw new ArtifactContributionNotPermittedError(metadata.contributor, artifactKind);
    this.registry.register(artifactKind, metadata.inferenceSchema, { kind: "contributed", pluginId: metadata.contributor }, metadata);
    this.dependsOn.set(`${artifactKind} ${metadata.inferenceSchema}`, metadata.dependsOn ?? []);
  }

  resolve(artifactKind: string, inferenceSchema: string): ArtifactRouterOwnership | undefined {
    return this.registry.resolve(artifactKind, inferenceSchema);
  }

  /** 🔗️ Topological order of every registered contributed inference's `(artifactKind,
   * inferenceSchema)` key honoring the `dependsOn` DAG (an inference that itself needs another
   * contributed inference's output runs after it) — Kahn toposort over `dependsOn` edges, same
   * lexicographic tie-break as {@link resolvePluginLoadOrder}. Throws {@link Error} naming the
   * cyclic keys on a cycle (never silently drops an entry). */
  dependencyOrder(): readonly string[] {
    const keys = [...this.dependsOn.keys()];
    const indegree = new Map<string, number>(keys.map((key) => [key, 0]));
    const dependents = new Map<string, string[]>();
    for (const key of keys) {
      for (const dependency of this.dependsOn.get(key) ?? []) {
        if (!indegree.has(dependency)) continue;
        indegree.set(key, (indegree.get(key) ?? 0) + 1);
        const list = dependents.get(dependency) ?? [];
        list.push(key);
        dependents.set(dependency, list);
      }
    }
    const order: string[] = [];
    const remaining = new Map(indegree);
    const queue = keys.filter((key) => (indegree.get(key) ?? 0) === 0);
    while (queue.length > 0) {
      queue.sort();
      const key = queue.shift()!;
      order.push(key);
      for (const dependent of dependents.get(key) ?? []) {
        const next = (remaining.get(dependent) ?? 0) - 1;
        remaining.set(dependent, next);
        if (next === 0) queue.push(dependent);
      }
    }
    if (order.length !== keys.length) {
      const leftover = keys.filter((key) => !order.includes(key)).sort();
      throw new Error(`ArtifactInferenceRouter.dependencyOrder: cycle among ${leftover.join(", ")}`);
    }
    return order;
  }
}
//#endregion 🔖️ArtifactRouters
// #endregion 🎠️Kernel
