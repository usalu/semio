import { loadTaxonomy, type BreachRecord } from "../../../🟦️.ts";
import { policyListArtifactDialectDirs } from "../../../🔍️discovery/🗿️artifact/🗣️dialects/🟦️.ts";
import { policyListPluginArtifactDirs } from "../../../🔍️discovery/🗿️artifact/🏠️roots/🟦️.ts";
import { policySurfaceRoots } from "../../../🔍️discovery/🗺️surface/🟦️.ts";
import { POLICY_SKIP_DIRS, POLICY_SOURCE_OPERATIONS, policySourceDirectory, policySourceText, type PolicySourceOperations } from "../../../🔍️discovery/📖️source-access/🟦️.ts";
import { policyExtractRustSchemaFields } from "../../../🧬️schema/🔍️field-discovery/🦀️rust/🟦️.ts";
import { POLICY_APP_SCHEMA_FACET } from "../../../🧬️schema/🗺️surface/🧱️contract/🟦️.ts";
import { policyDiscoverAppSchemaOwners } from "../../../🧬️schema/🗺️surface/🔍️owner-discovery/🟦️.ts";
import { policyLoadAppSchemaFacetLeaves } from "../../../🧬️schema/🗺️surface/📚️facet-leaves/🟦️.ts";
import { POLICY_RS_COMPONENT_LEAF_NAME, policyStripEmoji } from "../../../🧹️normalization/🧬️mutation/🪪️identity/🟦️.ts";
import { abstractionOwnershipSchema, abstractionOwnershipViolations, type AbstractionOwnership } from "../🧱️contract/🟦️.ts";
import { abstractionOwnershipRustCommands, abstractionOwnershipSchemaFields } from "../🔍️source-projection/🟦️.ts";

/** 🔎️ Checks authored surface schema and command sources plus artifact state boundaries. */
export function policyAbstractionOwnershipBreaches(repoRoot: string, operations: PolicySourceOperations = POLICY_SOURCE_OPERATIONS): BreachRecord[] {
  const schema = abstractionOwnershipSchema(repoRoot, operations),
    taxonomy = loadTaxonomy(),
    breaches = new Map<string, BreachRecord>(),
    report = (path: string, declaration: AbstractionOwnership): void => {
      for (const violation of abstractionOwnershipViolations(declaration, schema)) {
        const key = `${path}:${violation}`;
        breaches.set(key, {
          id: `abstraction-ownership-${key}`,
          summary: `${violation} belongs to the OS or host window state`,
          kind: "app-schema/abstraction-ownership",
          scope: path,
          priority: "high",
          reason: "A surface must consume OS preferences and host control state without owning duplicate config or command contracts.",
          solution: "Remove the surface declaration and consume the shared OS preference or host window context.",
        });
      }
    },
    unavailable = (path: string, state: string): void => {
      breaches.set(`source:${path}`, {
        id: `abstraction-ownership-source-${path}`,
        summary: `"${path}" is ${state}`,
        kind: "app-schema/source-unreadable",
        scope: path,
        priority: "high",
        reason: "Abstraction ownership requires admitted readable source and never follows symbolic links.",
        solution: `Restore a readable regular source at ${path}.`,
      });
    },
    directoryEntries = (path: string) => {
      const source = policySourceDirectory(repoRoot, path, operations);
      if (source.state === "directory") return source.entries;
      if (source.state !== "missing") unavailable(path, source.state);
      return [];
    },
    sourceText = (path: string): string | null => {
      const source = policySourceText(repoRoot, path, operations);
      if (source.state === "file") return source.text;
      if (source.state !== "missing") unavailable(path, source.state);
      return null;
    },
    scanCommands = (root: string): void => {
      for (const entry of directoryEntries(root)) {
        const path = `${root}/${entry.name}`;
        if (entry.isSymbolicLink) unavailable(path, "symlink");
        else if (entry.isDirectory) report(path, { owner: "surface", fields: [], commands: [policyStripEmoji(entry.name)] });
      }
    },
    scanSurfaceSchemas = (root: string): void => {
      for (const entry of directoryEntries(root)) {
        const path = `${root}/${entry.name}`;
        if (entry.isSymbolicLink) {
          unavailable(path, "symlink");
          continue;
        }
        if (entry.isDirectory) {
          if (!POLICY_SKIP_DIRS.has(entry.name) && !["🧪️tests", "🧫️fixtures", "📚️examples", "📦️packages"].includes(entry.name)) scanSurfaceSchemas(path);
          continue;
        }
        if (entry.name === "🔣️.json") {
          const text = sourceText(path);
          if (text === null) continue;
          const authored = JSON.parse(text) as Record<string, unknown>;
          if (authored.$schema) report(path, { owner: "surface", fields: abstractionOwnershipSchemaFields(authored), commands: [] });
        } else if (entry.name === "🦀️.rs") {
          const source = sourceText(path);
          if (source && /\benum\s+\w*(?:Command|Mutation)\b/.test(source)) report(path, { owner: "surface", fields: [], commands: abstractionOwnershipRustCommands(source) });
        }
      }
    };
  for (const owner of policyDiscoverAppSchemaOwners(repoRoot, operations)) {
    for (const leaf of policyLoadAppSchemaFacetLeaves(repoRoot, `${owner.ownerRel}/${POLICY_APP_SCHEMA_FACET}`, owner.configType, operations)) {
      if (leaf.sourceState !== "file" && leaf.sourceState !== "missing") unavailable(leaf.relPath, leaf.sourceState);
      if (leaf.extract) report(leaf.relPath, { owner: "surface", fields: leaf.extract.fields.map((field) => field.name), commands: [] });
    }
    const path = `${owner.ownerRel}/${POLICY_RS_COMPONENT_LEAF_NAME}`,
      source = sourceText(path);
    if (source !== null) report(path, { owner: "surface", fields: policyExtractRustSchemaFields(source, owner.configType).fields.map((field) => field.name), commands: [] });
    scanCommands(`${owner.ownerRel}/${POLICY_APP_SCHEMA_FACET}/🧬️mutations`);
  }
  for (const plugin of directoryEntries("✏️s/🔌️plugins")) {
    const pluginPath = `✏️s/🔌️plugins/${plugin.name}`;
    if (plugin.isSymbolicLink) {
      unavailable(pluginPath, "symlink");
      continue;
    }
    if (!plugin.isDirectory) continue;
    for (const surface of policySurfaceRoots(repoRoot, pluginPath, taxonomy, operations)) {
      scanCommands(`${surface}/🎮️commands`);
      scanSurfaceSchemas(surface);
    }
  }
  const artifactRoots = new Set([...policyListPluginArtifactDirs(repoRoot, operations), ...policyListArtifactDialectDirs(repoRoot, operations).map((dialect) => dialect.subsetRel)]);
  for (const root of artifactRoots) {
    for (const facet of ["🧬️schema", "🧬️schema/🔺️diff"]) {
      const path = `${root}/${facet}/🔣️.json`,
        text = sourceText(path);
      if (text === null) continue;
      const artifact = JSON.parse(text) as { properties?: Record<string, { "x-semio-state"?: string }> };
      report(path, { owner: "artifact", fields: Object.keys(artifact.properties ?? {}), commands: [] });
      for (const [field, definition] of Object.entries(artifact.properties ?? {})) {
        const state = definition["x-semio-state"];
        if (!state || state === "artifact") continue;
        const key = `${path}:state:${field}`;
        breaches.set(key, {
          id: `abstraction-ownership-${key}`,
          summary: `${field} belongs to ${state} state`,
          kind: "artifact-schema/abstraction-ownership",
          scope: path,
          priority: "high",
          reason: "An artifact contract and its document diffs must not duplicate app or window state.",
          solution: `Keep ${field} in its ${state} owner and remove the duplicate artifact declaration and diff member.`,
        });
      }
    }
  }
  return [...breaches.values()].sort((left, right) => left.scope.localeCompare(right.scope) || left.id.localeCompare(right.id));
}
