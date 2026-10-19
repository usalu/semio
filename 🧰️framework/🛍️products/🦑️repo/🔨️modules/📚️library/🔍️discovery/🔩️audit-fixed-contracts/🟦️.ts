import type { FixedContractScope } from "../📜️taxonomy-contracts/🟦️.ts";
import { fixedContractFilename, parseFixedDirectoryContractSetScope, parseNamedFixedDirectoryContractSetScope } from "../🎯️taxonomy-path-matching/🟦️.ts";
import type { TaxonomyAudit } from "../🪣️taxonomy-audit-context/🟦️.ts";

/** 🔩️ Audits the fixed directory contract sets, the fixed filename and directory contracts, the exact Cargo, Nx and Bun scopes and the filename rejection contracts. */
export function auditFixedContracts(audit: TaxonomyAudit): void {
  const { taxonomy, problems, record, pathPattern, exactKeys } = audit;
  if (taxonomy.fixedDirectoryContractSets !== undefined && !(taxonomy.fixedDirectoryContractSets && typeof taxonomy.fixedDirectoryContractSets === "object" && !Array.isArray(taxonomy.fixedDirectoryContractSets))) problems.push("fixedDirectoryContractSets must be an object.");
  else for (const [id, values] of Object.entries(taxonomy.fixedDirectoryContractSets ?? {})) {
    if (!/^[a-z][a-z0-9]*(?:-[a-z0-9]+)*$/u.test(id)) problems.push(`fixedDirectoryContractSets[${JSON.stringify(id)}] must use a literal set ID.`);
    try { parseFixedDirectoryContractSetScope({ kind: "fixed-directory-contract-set", fixedDirectoryContractIds: values }, taxonomy.fixedDirectoryContracts); } catch (error) { problems.push(`fixedDirectoryContractSets[${JSON.stringify(id)}]: ${error instanceof Error ? error.message : String(error)}`); }
  }
  const fixedScope = (scope: FixedContractScope, contractPathPattern: string, key: string, filenameContract: boolean): void => {
    if (!(typeof scope === "object" && scope !== null && !Array.isArray(scope)) || typeof scope.kind !== "string") {
      problems.push(`${key} must be a tagged fixed-contract scope.`);
      return;
    }
    if (scope.kind === "exact-path") {
      exactKeys(scope, ["kind", "path"], key);
      pathPattern(scope.path, `${key}.path`);
      if (scope.path !== contractPathPattern || /[*?\[\]{}]/u.test(scope.path)) problems.push(`${key}.path must equal the exact wildcard-free contract path.`);
    } else if (scope.kind === "repository-root") {
      exactKeys(scope, ["kind"], key);
      if (contractPathPattern.includes("/") || /[*?\[\]{}]/u.test(contractPathPattern)) problems.push(`${key} repository-root contract must be one exact basename.`);
    } else if (scope.kind === "package-root") {
      exactKeys(scope, ["kind", "ecosystemId"], key);
      if (!filenameContract) problems.push(`${key} cannot use package-root scope.`);
      if (!scope.ecosystemId || !taxonomy.ecosystems[scope.ecosystemId]) problems.push(`${key}.ecosystemId must reference an ecosystem.`);
    } else if (scope.kind === "directory-kind") {
      exactKeys(scope, ["kind", "directoryKindId"], key);
      if (!scope.directoryKindId || !taxonomy.semanticDirectoryKinds[scope.directoryKindId]) problems.push(`${key}.directoryKindId must reference a semantic directory kind.`);
    } else if (scope.kind === "fixed-directory-contract") {
      exactKeys(scope, ["kind", "fixedDirectoryContractId"], key);
      if (!filenameContract) problems.push(`${key} cannot use fixed-directory-contract scope.`);
      if (!scope.fixedDirectoryContractId || !taxonomy.fixedDirectoryContracts[scope.fixedDirectoryContractId]) problems.push(`${key}.fixedDirectoryContractId must reference a fixed directory contract.`);
    } else if (scope.kind === "fixed-directory-contract-set") {
      if (!filenameContract) problems.push(`${key} cannot use fixed-directory-contract-set scope.`);
      try { parseFixedDirectoryContractSetScope(scope, taxonomy.fixedDirectoryContracts); } catch (error) { problems.push(`${key}: ${error instanceof Error ? error.message : String(error)}`); }
    } else if (scope.kind === "named-fixed-directory-contract-set") {
      if (!filenameContract) problems.push(`${key} cannot use named-fixed-directory-contract-set scope.`);
      try { parseNamedFixedDirectoryContractSetScope(scope, taxonomy.fixedDirectoryContracts, taxonomy.fixedDirectoryContractSets ?? {}); } catch (error) { problems.push(`${key}: ${error instanceof Error ? error.message : String(error)}`); }
    } else if (scope.kind === "sibling-fixed-filename-contract") {
      exactKeys(scope, ["kind", "fixedFilenameContractId"], key);
      if (!filenameContract) problems.push(`${key} cannot use sibling-fixed-filename-contract scope.`);
      if (!scope.fixedFilenameContractId || !taxonomy.fixedFilenameContracts[scope.fixedFilenameContractId]) problems.push(`${key}.fixedFilenameContractId must reference a fixed filename contract.`);
    } else if (scope.kind === "path-pattern") exactKeys(scope, ["kind"], key);
    else problems.push(`${key}.kind is invalid.`);
  };

  if (record(taxonomy.fixedFilenameContracts, "fixedFilenameContracts")) for (const [id, contract] of Object.entries(taxonomy.fixedFilenameContracts)) {
    pathPattern(contract.pathPattern, `fixedFilenameContracts[${JSON.stringify(id)}].pathPattern`);
    if (typeof contract.pathPattern === "string" && /[*?\[\]{}]/u.test(fixedContractFilename(contract))) problems.push(`fixedFilenameContracts[${JSON.stringify(id)}].pathPattern must end in one exact literal basename.`);
    if (!contract.authority || !contract.reason || !contract.verification) problems.push(`fixedFilenameContracts[${JSON.stringify(id)}] must declare authority, reason, and verification.`);
    if (contract.configurability !== "unconfigurable") problems.push(`fixedFilenameContracts[${JSON.stringify(id)}].configurability must be unconfigurable.`);
    fixedScope(contract.scope, contract.pathPattern, `fixedFilenameContracts[${JSON.stringify(id)}].scope`, true);
    if (!(contract.expires === null || /^\d{4}-\d{2}-\d{2}$/u.test(contract.expires))) problems.push(`fixedFilenameContracts[${JSON.stringify(id)}].expires must be null or YYYY-MM-DD.`);
  }
  const cargoTargetEvidence = taxonomy.semanticDirectoryKinds["ticket-cargo-target-evidence"];
  if (!cargoTargetEvidence || cargoTargetEvidence.emoji !== "🧪️" || cargoTargetEvidence.slugPattern !== "^target-[a-z0-9]+(?:-[a-z0-9]+)*$" || cargoTargetEvidence.allowEmojiOnly || cargoTargetEvidence.parentKindIds !== undefined) problems.push("semanticDirectoryKinds.ticket-cargo-target-evidence must remain the exact ticket-local Cargo target authority.");
  const cargoCacheTag = taxonomy.fixedFilenameContracts["cargo-cache-tag"];
  if (!cargoCacheTag || cargoCacheTag.pathPattern !== "**/.🧬semio/🦑️repo/🎫️tickets/🎆️[0-9][0-9]/🌙️[0-9][0-9]/☀️[0-9][0-9]/*/**/CACHEDIR.TAG" || cargoCacheTag.authority !== "Cargo" || cargoCacheTag.scope.kind !== "directory-kind" || cargoCacheTag.scope.directoryKindId !== "ticket-cargo-target-evidence") problems.push("fixedFilenameContracts.cargo-cache-tag must remain conjunctively scoped to a governed ticket path and the ticket-cargo-target-evidence directory kind.");
  const ticketCargoPattern = "**/.🧬semio/🦑️repo/🎫️tickets/🎆️[0-9][0-9]/🌙️[0-9][0-9]/☀️[0-9][0-9]/*/**/";
  for (const triple of ["wasm32-unknown-unknown", "wasm32-wasip2"] as const) {
    const directoryId = `cargo-target-triple-${triple}`;
    const directory = taxonomy.fixedDirectoryContracts[directoryId];
    if (!directory || directory.pathPattern !== `${ticketCargoPattern}${triple}` || directory.authority !== "Cargo" || directory.scope.kind !== "directory-kind" || directory.scope.directoryKindId !== "ticket-cargo-target-evidence") problems.push(`fixedDirectoryContracts.${directoryId} must remain the exact governed ticket Cargo target-triple authority.`);
    const cacheId = `cargo-cache-tag-${triple}`;
    const cache = taxonomy.fixedFilenameContracts[cacheId];
    if (!cache || cache.pathPattern !== `${ticketCargoPattern}${triple}/CACHEDIR.TAG` || cache.authority !== "Cargo" || cache.scope.kind !== "fixed-directory-contract" || cache.scope.fixedDirectoryContractId !== directoryId) problems.push(`fixedFilenameContracts.${cacheId} must remain conjunctively scoped through its exact Cargo target-triple contract.`);
  }
  const nxManifestScopes = [
    ["build-owned-node-package-manifest", "**/package.json", "Build Tools and Node.js"],
    ["build-owned-typescript-config", "**/tsconfig.json", "Build Tools and TypeScript"],
  ] as const;
  for (const [id, pattern, authority] of nxManifestScopes) {
    const contract = taxonomy.fixedFilenameContracts[id];
    if (!contract || contract.pathPattern !== pattern || contract.authority !== authority || contract.scope.kind !== "sibling-fixed-filename-contract" || contract.scope.fixedFilenameContractId !== "build-project-manifest") problems.push(`fixedFilenameContracts.${id} must remain conjunctively scoped through an adjacent exact build project manifest.`);
  }
  const bunPublicationScopes = [
    ["bun-package-readme", "**/README.md"],
    ["bun-package-license", "**/LICENSE.md"],
  ] as const;
  for (const [id, pattern] of bunPublicationScopes) {
    const contract = taxonomy.fixedFilenameContracts[id];
    if (!contract || contract.pathPattern !== pattern || contract.authority !== "Bun package publisher" || contract.scope.kind !== "package-root" || contract.scope.ecosystemId !== "🟦️typescript" || contract.verification !== "bun pm pack --dry-run --ignore-scripts") problems.push(`fixedFilenameContracts.${id} must remain scoped to an exact publishable Bun package root.`);
  }
  const ticketCargoManifest = taxonomy.fixedFilenameContracts["ticket-cargo-manifest"];
  if (!ticketCargoManifest || ticketCargoManifest.pathPattern !== `${ticketCargoPattern}Cargo.toml` || ticketCargoManifest.authority !== "Cargo" || ticketCargoManifest.scope.kind !== "path-pattern") problems.push("fixedFilenameContracts.ticket-cargo-manifest must remain scoped to governed canonical or embedded ticket paths.");
  const ticketCargoLock = taxonomy.fixedFilenameContracts["ticket-cargo-lock"];
  if (!ticketCargoLock || ticketCargoLock.pathPattern !== `${ticketCargoPattern}Cargo.lock` || ticketCargoLock.authority !== "Cargo" || ticketCargoLock.scope.kind !== "path-pattern") problems.push("fixedFilenameContracts.ticket-cargo-lock must remain scoped to governed canonical or embedded ticket paths.");
  const rootCargoLock = taxonomy.fixedFilenameContracts["root-cargo-lock"];
  if (!rootCargoLock || rootCargoLock.pathPattern !== "Cargo.lock" || rootCargoLock.authority !== "Cargo" || rootCargoLock.scope.kind !== "repository-root") problems.push("fixedFilenameContracts.root-cargo-lock must remain the exact repository-root Cargo lock authority.");

  if (record(taxonomy.fixedFilenameRejectionContracts, "fixedFilenameRejectionContracts")) {
    const identities = new Map<string, string>();
    for (const [id, contract] of Object.entries(taxonomy.fixedFilenameRejectionContracts)) {
      exactKeys(contract, ["sourcePathIdentities", "disposition", "reason"], `fixedFilenameRejectionContracts[${JSON.stringify(id)}]`);
      if (!Array.isArray(contract.sourcePathIdentities) || contract.sourcePathIdentities.length === 0) problems.push(`fixedFilenameRejectionContracts[${JSON.stringify(id)}].sourcePathIdentities must be non-empty.`);
      for (const identity of contract.sourcePathIdentities ?? []) {
        pathPattern(identity, `fixedFilenameRejectionContracts[${JSON.stringify(id)}].sourcePathIdentities`);
        if (/[*?\[]/u.test(identity)) problems.push(`fixedFilenameRejectionContracts[${JSON.stringify(id)}] identities must be exact paths.`);
        if (identities.has(identity)) problems.push(`Fixed filename rejection identity ${JSON.stringify(identity)} is duplicated by ${JSON.stringify(identities.get(identity))} and ${JSON.stringify(id)}.`);
        identities.set(identity, id);
      }
      if (!["normalize", "relocate"].includes(contract.disposition)) problems.push(`fixedFilenameRejectionContracts[${JSON.stringify(id)}].disposition is invalid.`);
      if (!contract.reason) problems.push(`fixedFilenameRejectionContracts[${JSON.stringify(id)}].reason must be non-empty.`);
    }
  }

  if (record(taxonomy.fixedDirectoryContracts, "fixedDirectoryContracts")) for (const [id, contract] of Object.entries(taxonomy.fixedDirectoryContracts)) {
    pathPattern(contract.pathPattern, `fixedDirectoryContracts[${JSON.stringify(id)}].pathPattern`);
    if (!contract.authority || !contract.reason || !contract.verification) problems.push(`fixedDirectoryContracts[${JSON.stringify(id)}] must declare authority, reason, and verification.`);
    if (contract.configurability !== "unconfigurable") problems.push(`fixedDirectoryContracts[${JSON.stringify(id)}].configurability must be unconfigurable.`);
    if (contract.descendants !== undefined && contract.descendants !== "reserved") problems.push(`fixedDirectoryContracts[${JSON.stringify(id)}].descendants must be reserved when present.`);
    fixedScope(contract.scope, contract.pathPattern, `fixedDirectoryContracts[${JSON.stringify(id)}].scope`, false);
    if (!(contract.expires === null || /^\d{4}-\d{2}-\d{2}$/u.test(contract.expires))) problems.push(`fixedDirectoryContracts[${JSON.stringify(id)}].expires must be null or YYYY-MM-DD.`);
  }
}
