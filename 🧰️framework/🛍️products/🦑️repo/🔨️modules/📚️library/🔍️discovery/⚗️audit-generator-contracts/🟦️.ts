import { parseGeneratorPreviewProgressPolicyV1 } from "../../🏭️generator/👁️preview/📈️progress/🟦️.ts";
import { exactOwnerPath, parseCurrentJcoPackageDestination } from "../📦️package-projection-contracts/🟦️.ts";
import { generatorPreviewResourceLimits, generatorPreviewScriptArguments } from "../🛠️generator-commands/🟦️.ts";
import { parseSemanticPackageBrowserProfile } from "../🎁️package-projection/🟦️.ts";
import type { TaxonomyAudit } from "../🪣️taxonomy-audit-context/🟦️.ts";

/** ⚗️ Audits every generator contract with its ownership, targets, inputs, outputs and preview declarations. */
export function auditGeneratorContracts(audit: TaxonomyAudit): void {
  const { taxonomy, problems, pathMatcher, record, pathPattern, workspacePath, exactKeys } = audit;
  if (record(taxonomy.generatorContracts, "generatorContracts")) {
    const contractIds = Object.keys(taxonomy.generatorContracts);
    if (contractIds.join("\0") !== [...contractIds].sort().join("\0")) problems.push("generatorContracts ids must be lexically ordered.");
    const opaqueRoots = Object.values(taxonomy.pathExclusions ?? {}).map((entry) => entry.path.replace(/\/$/u, ""));
    const exactTouchesOpaque = (value: string): boolean => opaqueRoots.some((opaque) => value === opaque || value.startsWith(`${opaque}/`) || opaque.startsWith(`${value}/`));
    // 🚧️ A pattern crosses an opaque boundary when it is lexically under (or over) an opaque root, or
    // when a wildcard sits at or above that root's own depth and could therefore expand into it. An
    // exact literal path that merely SHARES a first segment with a deeper opaque root — `♻️mit-bestand/
    // 📋️bericht/…` beside the `♻️mit-bestand/🔎️recherche/` gitlink — reaches nothing opaque and is
    // admitted; anything glob-shaped above the root still is not.
    const patternTouchesOpaque = (value: string): boolean => {
      if (exactTouchesOpaque(value)) return true;
      const segments = value.split("/");
      return opaqueRoots.some((opaque) => segments.slice(0, opaque.split("/").length).some((segment) => /[*?\[]/u.test(segment)));
    };
    const nxTarget = (value: unknown, key: string): value is string => {
      const valid = typeof value === "string" && /^(?:@[a-z0-9][a-z0-9._-]*\/)?[a-z0-9][a-z0-9._-]*:[a-z0-9][a-z0-9._-]*$/u.test(value);
      if (!valid) problems.push(`${key} must be one exact build project:target identity.`);
      return valid;
    };
    const outputOwners: { id: string; path: string }[] = [];
    const targets = new Map<string, string>();
    for (const [id, contract] of Object.entries(taxonomy.generatorContracts)) {
      if (!/^[a-z0-9]+(?:-[a-z0-9]+)*$/u.test(id)) problems.push(`generatorContracts id ${JSON.stringify(id)} must be kebab-case.`);
      const allowedKeys = new Set(["ownership", "ownerPath", "target", "previewTarget", "previewArguments", "previewProgress", "previewLimits", "checkTarget", "nativeConsumers", "inputPatterns", "inputDiscovery", "compilerInputManifest", "packageGeneration", "currentPackageDestination", "projectionActivation", "outputRoots", "reason"]);
      for (const key of Object.keys(contract)) if (!allowedKeys.has(key)) problems.push(`generatorContracts[${JSON.stringify(id)}].${key} is forbidden.`);
      const ownership = contract.ownership as string;
      if (!["owned", "external"].includes(ownership)) problems.push(`generatorContracts[${JSON.stringify(id)}].ownership must be owned or external.`);
      if (!contract.reason) problems.push(`generatorContracts[${JSON.stringify(id)}].reason must be non-empty.`);
      const runnable = ownership === "owned";
      if (contract.nativeConsumers !== undefined) {
        if (!runnable || !Array.isArray(contract.nativeConsumers) || contract.nativeConsumers.length === 0 || new Set(contract.nativeConsumers).size !== contract.nativeConsumers.length) problems.push(`generatorContracts[${JSON.stringify(id)}].nativeConsumers requires distinct Cargo project roots on an owned generator.`);
        else for (const consumer of contract.nativeConsumers) if (workspacePath(consumer, `generatorContracts[${JSON.stringify(id)}].nativeConsumers`) && exactTouchesOpaque(consumer)) problems.push(`generatorContracts[${JSON.stringify(id)}].nativeConsumers crosses an opaque boundary.`);
      }
      const targetKnown = runnable;
      if (targetKnown) {
        if (workspacePath(contract.ownerPath, `generatorContracts[${JSON.stringify(id)}].ownerPath`) && exactTouchesOpaque(contract.ownerPath)) problems.push(`generatorContracts[${JSON.stringify(id)}].ownerPath crosses an opaque boundary.`);
        if (nxTarget(contract.target, `generatorContracts[${JSON.stringify(id)}].target`)) {
          const prior = targets.get(contract.target);
          if (prior) problems.push(`generatorContracts ${JSON.stringify(prior)} and ${JSON.stringify(id)} duplicate target ${JSON.stringify(contract.target)}.`);
          targets.set(contract.target, id);
        }
      } else if (contract.ownerPath !== null || contract.target !== null) problems.push(`generatorContracts[${JSON.stringify(id)}] unowned/external classifications must have null ownerPath and target.`);
      if (runnable) {
        if (contract.previewTarget === undefined) problems.push(`generatorContracts[${JSON.stringify(id)}].previewTarget is required for owned contracts.`);
        else if (nxTarget(contract.previewTarget, `generatorContracts[${JSON.stringify(id)}].previewTarget`) && typeof contract.target === "string") {
          try { generatorPreviewScriptArguments(contract); }
          catch (error) { problems.push(`generatorContracts[${JSON.stringify(id)}]: ${error instanceof Error ? error.message : String(error)}`); }
          const prior = targets.get(contract.previewTarget);
          if (prior) problems.push(`generatorContracts ${JSON.stringify(prior)} and ${JSON.stringify(id)} duplicate target ${JSON.stringify(contract.previewTarget)}.`);
          targets.set(contract.previewTarget, id);
        }
      } else if (contract.previewTarget !== undefined) problems.push(`generatorContracts[${JSON.stringify(id)}].previewTarget is forbidden for external contracts.`);
      if (contract.previewProgress !== undefined) { try { if (!runnable || contract.previewTarget === undefined) throw new Error("Declared progress requires owned preview authority"); parseGeneratorPreviewProgressPolicyV1(contract.previewProgress); } catch (error) { problems.push(`generatorContracts[${JSON.stringify(id)}].previewProgress: ${(error as Error).message}`); } }
      if (contract.previewArguments !== undefined && (!runnable || contract.previewTarget === undefined)) problems.push(`generatorContracts[${JSON.stringify(id)}].previewArguments requires an owned preview target.`);
      try { generatorPreviewResourceLimits(contract); } catch (error) { problems.push(`generatorContracts[${JSON.stringify(id)}].previewLimits: ${(error as Error).message}`); }
      if (contract.compilerInputManifest !== undefined) {
        const authority = contract.compilerInputManifest;
        if (!runnable || !authority || typeof authority !== "object" || Array.isArray(authority) || JSON.stringify(Object.keys(authority).sort()) !== JSON.stringify(["kind", "manifestOutputPath", "manifestSchemaPath", "maxFiles", "staticAuthorityPath"])) problems.push(`generatorContracts[${JSON.stringify(id)}].compilerInputManifest requires one closed owned authority.`);
        else {
          const literal = (path: unknown): path is string => typeof path === "string" && path === path.normalize("NFC") && path.length > 0 && !path.startsWith("/") && !/[\\*?\[\]\u0000-\u001f]/u.test(path) && !path.split("/").some(part => !part || part === "." || part === "..");
          if (authority.kind !== "compiler-input-manifest-v1" || !literal(authority.manifestOutputPath) || !literal(authority.manifestSchemaPath) || !literal(authority.staticAuthorityPath) || !Number.isSafeInteger(authority.maxFiles) || authority.maxFiles < 1 || authority.maxFiles > 10000) problems.push(`generatorContracts[${JSON.stringify(id)}].compilerInputManifest has invalid bounded fields.`);
          if (!contract.outputRoots?.some(root => authority.manifestOutputPath === root.path || authority.manifestOutputPath.startsWith(`${root.path}/`))) problems.push(`generatorContracts[${JSON.stringify(id)}].compilerInputManifest manifest is outside its output roots.`);
          for (const path of [authority.manifestSchemaPath, authority.staticAuthorityPath]) if (!contract.inputPatterns?.includes(path)) problems.push(`generatorContracts[${JSON.stringify(id)}].compilerInputManifest authority is not one exact input: ${JSON.stringify(path)}.`);
        }
      }
      if (contract.checkTarget !== undefined) {
        if (!targetKnown || !nxTarget(contract.checkTarget, `generatorContracts[${JSON.stringify(id)}].checkTarget`)) problems.push(`generatorContracts[${JSON.stringify(id)}].checkTarget requires a known target.`);
        else if (typeof contract.target === "string" && contract.checkTarget.slice(0, contract.checkTarget.lastIndexOf(":")) !== contract.target.slice(0, contract.target.lastIndexOf(":"))) problems.push(`generatorContracts[${JSON.stringify(id)}].checkTarget must belong to the target project.`);
      }
      if (id === "jco-package-adapter" && contract.currentPackageDestination === undefined) problems.push(`generatorContracts.${id} requires explicit current package destination authority.`);
      if (["jco-package-adapter", "wgpu-frame-worker"].includes(id) && contract.projectionActivation !== undefined) problems.push(`generatorContracts.${id} must use its current catalog instead of historical projection activation.`);
      if (contract.currentPackageDestination !== undefined) {
        try {
          const current = parseCurrentJcoPackageDestination(contract.currentPackageDestination);
          if (!runnable || id !== "jco-package-adapter" || contract.outputRoots.length !== 1 || contract.outputRoots[0]?.path !== current.adapterPath || contract.outputRoots[0]?.inclusion !== "tracked" || [current.cargoManifestPath, current.cargoLockPath, current.componentPath, current.witPath].some((path) => !contract.inputPatterns.includes(path))) problems.push(`generatorContracts.${id} current JCO ownership or inputs disagree.`);
        } catch (error) { problems.push(`generatorContracts.${id}.currentPackageDestination: ${error instanceof Error ? error.message : String(error)}`); }
      }
      if (id === "wgpu-frame-worker" && contract.packageGeneration === undefined) problems.push(`generatorContracts.${id} requires explicit package generation authority.`);
      if (contract.projectionActivation !== undefined) {
        const activation = contract.projectionActivation;
        exactKeys(activation, ["kind", "projectionContractId", "packageId", "sourceManifestPath", "destinationManifestPath"], `generatorContracts.${id}.projectionActivation`);
        if (!runnable || !["jco-package-adapter", "wgpu-frame-worker"].includes(id) || activation.kind !== "canonical-or-planned-package" || activation.projectionContractId !== "nested-cargo-packages-v1" || activation.packageId !== (id === "jco-package-adapter" ? "jcoprobe-guest" : "wgpu-renderer") || !exactOwnerPath(activation.sourceManifestPath) || !exactOwnerPath(activation.destinationManifestPath) || activation.sourceManifestPath === activation.destinationManifestPath) problems.push(`generatorContracts.${id}.projectionActivation has no exact package authority.`);
      }
      if (contract.packageGeneration !== undefined) {
        const generation = contract.packageGeneration;
        exactKeys(generation, ["kind", "previewInput", "catalogPath", "catalogSha256", "browserProfile"], `generatorContracts.${id}.packageGeneration`);
        const protocol = generation.previewInput;
        if (!runnable || id !== "wgpu-frame-worker" || contract.inputDiscovery || generation.kind !== "wgpu-package-artifacts" || !protocol || Object.keys(protocol).sort().join("\0") !== "maxBytes\0maxOperations\0protocol" || protocol.protocol !== "package-projected-inputs-v1" || protocol.maxBytes !== 67108864 || protocol.maxOperations !== 200000) problems.push(`generatorContracts.${id}.packageGeneration requires its exact bounded package protocol.`);
        try {
          const profile = parseSemanticPackageBrowserProfile(generation.browserProfile, taxonomy.pathEmojiPolicy.genericEmojiIdentities);
          if (generation.catalogPath !== profile.ownerPath + "/🪪️package-catalog.json" || !/^[0-9a-f]{64}$/u.test(generation.catalogSha256)) problems.push(`generatorContracts.${id}.packageGeneration requires its exact current catalog and digest.`);
          if (profile.entries.some((entry) => !contract.outputRoots.some((output) => output.path === `${profile.ownerPath}/${entry.outputRelativePath}` && output.inclusion === entry.inclusion))) problems.push(`generatorContracts.${id}.packageGeneration browser outputs disagree with ownership.`);
        } catch (error) { problems.push(`generatorContracts.${id}.packageGeneration: ${error instanceof Error ? error.message : String(error)}`); }
      }
      if (contract.inputDiscovery !== undefined) {
        const input = contract.inputDiscovery;
        exactKeys(input, ["kind", "previewInput", "descriptorRelativePath", "exampleDirectoryName", "exampleFileKindId", "implementationEntryPaths", "workspaceImports"], `generatorContracts[${JSON.stringify(id)}].inputDiscovery`);
        if (!runnable || id !== "plugin-registry" || input.kind !== "registry-catalog") problems.push(`generatorContracts[${JSON.stringify(id)}].inputDiscovery requires the owned registry-catalog authority.`);
        if (!input.previewInput || JSON.stringify(Object.keys(input.previewInput).sort()) !== JSON.stringify(["maxBytes", "maxOperations", "protocol"]) || input.previewInput.protocol !== "registry-projected-inputs-v1" || input.previewInput.maxBytes !== 67108864 || input.previewInput.maxOperations !== 200000) problems.push(`generatorContracts[${JSON.stringify(id)}].inputDiscovery requires the bounded registry-projected-inputs-v1 preview protocol.`);
        if (input.descriptorRelativePath !== "../../🔣️.json" || input.exampleDirectoryName !== "📚️examples" || !taxonomy.artifactChildDirs.includes(input.exampleDirectoryName) || input.exampleFileKindId !== taxonomy.exampleFileKinds["🦀️rust"]) problems.push(`generatorContracts[${JSON.stringify(id)}].inputDiscovery must preserve the exact descriptor and example vocabulary.`);
        if (!Array.isArray(input.implementationEntryPaths) || input.implementationEntryPaths.length === 0 || input.implementationEntryPaths.join("\0") !== [...new Set(input.implementationEntryPaths)].sort().join("\0")) problems.push(`generatorContracts[${JSON.stringify(id)}].inputDiscovery implementation entries must be nonempty, unique and ordered.`);
        else for (const path of input.implementationEntryPaths) if (!workspacePath(path, `generatorContracts[${JSON.stringify(id)}].inputDiscovery entry`) || exactTouchesOpaque(path)) problems.push(`generatorContracts[${JSON.stringify(id)}].inputDiscovery entry crosses an opaque boundary.`);
        if (!input.workspaceImports || typeof input.workspaceImports !== "object" || Array.isArray(input.workspaceImports)) problems.push(`generatorContracts[${JSON.stringify(id)}].inputDiscovery.workspaceImports must be an object.`);
        else for (const [name, binding] of Object.entries(input.workspaceImports)) {
          exactKeys(binding, ["manifestPath", "entryPath"], `generatorContracts[${JSON.stringify(id)}].inputDiscovery.workspaceImports[${JSON.stringify(name)}]`);
          if (!/^@[a-z0-9-]+\/[a-z0-9-]+$/u.test(name)) problems.push(`generatorContracts[${JSON.stringify(id)}].inputDiscovery workspace import must be an exact package name.`);
          for (const path of [binding.manifestPath, binding.entryPath]) if (!workspacePath(path, `generatorContracts[${JSON.stringify(id)}].inputDiscovery workspace binding`) || exactTouchesOpaque(path)) problems.push(`generatorContracts[${JSON.stringify(id)}].inputDiscovery workspace binding crosses an opaque boundary.`);
        }
      }
      if (!Array.isArray(contract.inputPatterns) || (runnable && contract.inputPatterns.length === 0) || (!runnable && contract.inputPatterns.length > 0)) problems.push(`generatorContracts[${JSON.stringify(id)}].inputPatterns must be non-empty only for owned contracts.`);
      if (Array.isArray(contract.inputPatterns)) {
        if (contract.inputPatterns.join("\0") !== [...contract.inputPatterns].sort().join("\0") || new Set(contract.inputPatterns).size !== contract.inputPatterns.length) problems.push(`generatorContracts[${JSON.stringify(id)}].inputPatterns must be unique and lexically ordered.`);
        for (const [index, input] of contract.inputPatterns.entries()) {
          pathPattern(input, `generatorContracts[${JSON.stringify(id)}].inputPatterns[${index}]`);
          if (typeof input === "string" && patternTouchesOpaque(input)) problems.push(`generatorContracts[${JSON.stringify(id)}].inputPatterns[${index}] can cross an opaque boundary.`);
        }
      }
      if (!Array.isArray(contract.outputRoots) || contract.outputRoots.length === 0) problems.push(`generatorContracts[${JSON.stringify(id)}].outputRoots must be non-empty.`);
      if (Array.isArray(contract.outputRoots)) {
        const outputPaths = contract.outputRoots.map((output) => output.path);
        if (outputPaths.join("\0") !== [...outputPaths].sort().join("\0") || new Set(outputPaths).size !== outputPaths.length) problems.push(`generatorContracts[${JSON.stringify(id)}].outputRoots must be unique and lexically ordered.`);
        for (const [index, output] of contract.outputRoots.entries()) {
          const key = `generatorContracts[${JSON.stringify(id)}].outputRoots[${index}]`;
          if (workspacePath(output.path, `${key}.path`) && exactTouchesOpaque(output.path)) problems.push(`${key}.path crosses an opaque boundary.`);
          if (!["tracked", "ignored"].includes(output.inclusion)) problems.push(`${key}.inclusion must be tracked or ignored.`);
          if (output.inclusion === "tracked" && typeof output.path === "string" && output.path.split("/").some((part: string) => part.replace("\uFE0F", "") === "🤖generated")) problems.push(`${key} declares a tracked output inside 🤖️generated, which .gitignore ignores and the indexed-generated-output law never indexes: no clone has it.`);
          if (contract.ownership === "external" && output.inclusion !== "tracked") problems.push(`${key} is an external input that is not tracked: nothing produces it, so no clone has it.`);
          if (output.producer !== undefined) {
            exactKeys(output.producer, ["ownerPath", "target"], `${key}.producer`);
            if (!runnable) problems.push(`${key}.producer requires an owned generator.`);
            if (workspacePath(output.producer?.ownerPath, `${key}.producer.ownerPath`) && exactTouchesOpaque(output.producer.ownerPath)) problems.push(`${key}.producer crosses an opaque boundary.`);
            if (nxTarget(output.producer?.target, `${key}.producer.target`)) {
              const prior = targets.get(output.producer.target);
              if (prior && prior !== id) problems.push(`${key}.producer belongs to another generator contract.`);
              targets.set(output.producer.target, id);
              if (output.producer.target === contract.target && output.producer.ownerPath !== contract.ownerPath) problems.push(`${key}.producer disagrees with the generator owner.`);
            }
          }
          if (runnable && contract.inputPatterns.some((input) => pathMatcher.matches(output.path, input))) problems.push(`${key}.path is also declared as an input.`);
          outputOwners.push({ id, path: output.path });
        }
      }
    }
    for (let left = 0; left < outputOwners.length; left += 1) for (let right = left + 1; right < outputOwners.length; right += 1) {
      const a = outputOwners[left]!;
      const b = outputOwners[right]!;
      if (a.path === b.path || a.path.startsWith(`${b.path}/`) || b.path.startsWith(`${a.path}/`)) problems.push(`generatorContracts ${JSON.stringify(a.id)} and ${JSON.stringify(b.id)} have overlapping output roots ${JSON.stringify(a.path)} and ${JSON.stringify(b.path)}.`);
    }
    const unsettled = Object.entries(taxonomy.generatorContracts).filter(([, contract]) => !["owned", "external"].includes(contract.ownership as string)).map(([id]) => id);
    if (unsettled.length > 0) problems.push(`generatorContracts must contain zero unknown or unsafe contracts; found ${unsettled.join(", ")}.`);
    for (const removed of ["ownerless-ui-icons", "root-layering-declarations"]) if (taxonomy.generatorContracts[removed]) problems.push(`generatorContracts.${removed} is false ownership and must remain absent.`);
    const foreignMetadataRoots = Object.entries(taxonomy.fixedDirectoryContracts ?? {}).filter(([, contract]) => contract.scope.kind === "repository-root" && contract.pathPattern.startsWith(".") && !contract.pathPattern.includes("/")).map(([id, contract]) => [id, contract.pathPattern] as const);
    const ralphFileContracts: Readonly<Record<string, string>> = {
      "ralph-config": ".ralph-tui/config.toml",
      "ralph-lock": ".ralph-tui/ralph.lock",
      "ralph-prd-json": ".ralph-tui/prd/*/prd.json",
      "ralph-prd-markdown": ".ralph-tui/prd/*/prd.md",
      "ralph-progress": ".ralph-tui/progress.md",
      "ralph-session-meta": ".ralph-tui/session-meta.json",
      "ralph-session": ".ralph-tui/session.json",
    };
    for (const [id, expected] of Object.entries(ralphFileContracts)) {
      const contract = taxonomy.fixedFilenameContracts[id];
      if (!contract || contract.pathPattern !== expected || contract.authority !== "Ralph TUI" || contract.scope.kind !== "path-pattern") problems.push(`fixedFilenameContracts.${id} must be the exact Ralph-owned path contract ${JSON.stringify(expected)}.`);
    }
    const ralphDirectoryContracts: Readonly<Record<string, string>> = { "ralph-metadata": ".ralph-tui", "ralph-prd-root": ".ralph-tui/prd", "ralph-prd-identifier": ".ralph-tui/prd/*" };
    for (const [id, expected] of Object.entries(ralphDirectoryContracts)) {
      const contract = taxonomy.fixedDirectoryContracts[id];
      if (!contract || contract.pathPattern !== expected || contract.authority !== "Ralph TUI") problems.push(`fixedDirectoryContracts.${id} must be the exact Ralph-owned path contract ${JSON.stringify(expected)}.`);
    }
    for (const [id, contract] of [...Object.entries(taxonomy.fixedFilenameContracts), ...Object.entries(taxonomy.fixedDirectoryContracts)]) for (const [, root] of foreignMetadataRoots) if (contract.pathPattern.startsWith(`${root}/`) && contract.pathPattern.includes("**")) problems.push(`Foreign metadata contract ${JSON.stringify(id)} must not use a recursive wildcard.`);
    for (const output of outputOwners) for (const [contractId, root] of foreignMetadataRoots) if (output.path === root || output.path.startsWith(`${root}/`)) problems.push(`generatorContracts ${JSON.stringify(output.id)} declares output ${JSON.stringify(output.path)} inside the foreign metadata directory ${JSON.stringify(root)} owned by ${JSON.stringify(contractId)}; a foreign tool's own state is never a repository output.`);
    const fixedRootManifests: Readonly<Record<string, string>> = { "root-package": "package.json", "root-cargo": "Cargo.toml", "root-go-work": "go.work" };
    for (const [id, expected] of Object.entries(fixedRootManifests)) if (taxonomy.fixedFilenameContracts[id]?.pathPattern !== expected) problems.push(`fixedFilenameContracts.${id} must remain the authored root manifest contract ${JSON.stringify(expected)}.`);
    const generatedRootManifests = outputOwners.filter((output) => ["package.json", "Cargo.toml", "go.work"].includes(output.path));
    if (generatedRootManifests.length > 0) problems.push("Root Bun, Cargo, and Go manifests are authored fixed contracts, not generator outputs.");
  }
}
