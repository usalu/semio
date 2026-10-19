import { parseCurrentSourceApprovals, parseSemanticOwnedDocumentCorrections } from "../✅️owned-file-approvals/🟦️.ts";
import { exampleAssetPath } from "../🎁️package-projection/🟦️.ts";
import type { TaxonomyAudit } from "../🪣️taxonomy-audit-context/🟦️.ts";

/** 🔑️ Audits the semantic owned file projection contracts. */
export function auditOwnedFileProjection(audit: TaxonomyAudit): void {
  const { taxonomy, problems, record, exactKeys, kebabId, semanticDirectoryKindId, fileKindIdForSourcePath, canonicalFilenameForKind } = audit;
  if (record(taxonomy.semanticOwnedFileProjectionContracts, "semanticOwnedFileProjectionContracts")) {
    const ownerTuples = new Set<string>();
    for (const [id, contract] of Object.entries(taxonomy.semanticOwnedFileProjectionContracts)) {
      const scope = `semanticOwnedFileProjectionContracts[${JSON.stringify(id)}]`;
      kebabId(id, `${scope} id`);
      if (!contract || typeof contract !== "object" || Array.isArray(contract)) {
        problems.push(`${scope} must be an object.`);
        continue;
      }
      if (contract.contractKind === "exact-owner-path-catalog") {
        exactKeys(contract, ["contractKind", "authorityCatalogPath", "authorityCatalogSha256", "sourceFileKindId", "sourceBasenames", "destinationDirectoryKinds", "allowedDispositions", "ownerEvidenceKinds", "referenceOwnerIds", "generatorOwnerIds", "authoredDocumentCorrections", "rationaleRule", ...(Object.hasOwn(contract, "currentSourceRevisions") ? ["currentSourceRevisions"] : [])], scope);
        if (Object.hasOwn(contract, "currentSourceRevisions")) {
          try {
            const revisions = parseCurrentSourceApprovals(contract.currentSourceRevisions);
            for (const [revisionId, revision] of Object.entries(revisions)) if (!exampleAssetPath(revision.approvalPath, taxonomy)) problems.push(scope + ".currentSourceRevisions." + revisionId + ".approvalPath must be beneath " + taxonomy.exampleAssetsDirName + ".");
          } catch (error) { problems.push(scope + ".currentSourceRevisions: " + (error instanceof Error ? error.message : String(error))); }
        }
        if (id !== "readme-license-owner-leaves-v1" || contract.rationaleRule !== "readme-license-owner-projection-v1") problems.push(scope + " must be the exact README/LICENSE owner projection contract.");
        if (!exampleAssetPath(contract.authorityCatalogPath, taxonomy)) problems.push(scope + ".authorityCatalogPath must be one repository-local path beneath " + taxonomy.exampleAssetsDirName + ".");
        if (typeof contract.authorityCatalogSha256 !== "string" || !/^[a-f0-9]{64}$/u.test(contract.authorityCatalogSha256)) problems.push(scope + ".authorityCatalogSha256 must be one SHA-256 digest.");
        if (!taxonomy.fileKinds[contract.sourceFileKindId] || contract.sourceFileKindId !== "markdown") problems.push(scope + ".sourceFileKindId must be markdown.");
        if (!Array.isArray(contract.sourceBasenames) || contract.sourceBasenames.join("\0") !== "LICENSE.md\0README.md") problems.push(scope + ".sourceBasenames must be exactly LICENSE.md and README.md.");
        if (!contract.destinationDirectoryKinds || typeof contract.destinationDirectoryKinds !== "object" || Array.isArray(contract.destinationDirectoryKinds)) problems.push(scope + ".destinationDirectoryKinds must be an object.");
        else {
          exactKeys(contract.destinationDirectoryKinds, ["license", "readme"], scope + ".destinationDirectoryKinds");
          const expectedDestinations = {
            license: { directoryKindId: "owner-license", directoryName: "⚖️license", filename: "📝️.md" },
            readme: { directoryKindId: "owner-readme", directoryName: "📃️readme", filename: "📝️.md" },
          } as const;
          for (const kind of ["license", "readme"] as const) {
            const destination = contract.destinationDirectoryKinds[kind];
            exactKeys(destination, ["directoryKindId", "directoryName", "filename"], scope + ".destinationDirectoryKinds." + kind);
            if (JSON.stringify(destination) !== JSON.stringify(expectedDestinations[kind])) problems.push(scope + ".destinationDirectoryKinds." + kind + " is not canonical.");
            const directory = taxonomy.semanticDirectoryKinds[destination.directoryKindId];
            if (!directory || directory.projectionOnly !== true || semanticDirectoryKindId(destination.directoryName, taxonomy) !== destination.directoryKindId || fileKindIdForSourcePath(destination.filename, taxonomy) !== contract.sourceFileKindId) problems.push(scope + ".destinationDirectoryKinds." + kind + " does not resolve through the registered projection-only directory and Markdown leaf kinds.");
          }
        }
        const exactArrays = [
          [contract.allowedDispositions, "attribution-relocate\0configurable-owner-license-relocate\0fixed\0generated-evidence-relocate\0owner-documentation-relocate", "allowedDispositions"],
          [contract.ownerEvidenceKinds, "configurable-owner-license\0ordinary-owner-doc\0package-publication\0third-party-attribution\0ticket-evidence\0ticket-scratch", "ownerEvidenceKinds"],
        ] as const;
        for (const [values, expected, field] of exactArrays) if (!Array.isArray(values) || values.join("\0") !== expected) problems.push(scope + "." + field + " is not the declared domain vocabulary.");
        for (const [values, field] of [[contract.referenceOwnerIds, "referenceOwnerIds"], [contract.generatorOwnerIds, "generatorOwnerIds"]] as const) if (!Array.isArray(values) || new Set(values).size !== values.length || values.some(id => typeof id !== "string" || !/^[a-z][a-z0-9-]*$/u.test(id))) problems.push(scope + "." + field + " must declare unique domain registry identities.");
        if (contract.generatorOwnerIds.some(id => !taxonomy.generatorContracts[id])) problems.push(scope + " has an unregistered generator owner.");
        try { parseSemanticOwnedDocumentCorrections(contract.authoredDocumentCorrections); } catch (error) { problems.push(scope + ".authoredDocumentCorrections: " + (error instanceof Error ? error.message : String(error))); }
        if (taxonomy.fixedFilenameContracts["root-script"]?.pathPattern !== "**/📜️script.ts") problems.push(scope + ".authoredDocumentCorrections requires the canonical root-script filename contract.");
        continue;
      }
      if (contract.contractKind === "semantic-facet-primary-file") {
        exactKeys(contract, ["contractKind", "sourceRoot", "sourceFilename", "fileKindAuthority", "sourceDisposition", "directoryCaptures", "ownerPathPatterns", "authoringCommand", "referenceConsumer", "rationaleRule"], scope);
        if (id !== "artifact-empty-facet-primary-markdown-v1" || contract.rationaleRule !== id || contract.sourceRoot !== "✏️s/🔌️plugins" || contract.sourceFilename !== "📌️.empty.md" || contract.fileKindAuthority !== "windowEmptyFacetFileKindId" || contract.sourceDisposition !== "authored" || taxonomy.windowEmptyFacetFileKindId !== "markdown") problems.push(`${scope} must use the exact authored empty-facet primary-leaf grammar.`);
        if (record(contract.directoryCaptures, `${scope}.directoryCaptures`)) for (const [capture, spec] of Object.entries(contract.directoryCaptures)) {
          if (!record(spec, `${scope}.directoryCaptures.${capture}`)) continue;
          exactKeys(spec, ["kindIds", ...(spec.names ? ["names"] : [])], `${scope}.directoryCaptures.${capture}`);
          if (!/^[a-zA-Z]+$/u.test(capture) || !Array.isArray(spec.kindIds) || spec.kindIds.length === 0 || new Set(spec.kindIds).size !== spec.kindIds.length || spec.kindIds.some((kindId) => !taxonomy.semanticDirectoryKinds[kindId] && !taxonomy.semanticDirectoryMemberKinds[kindId])) problems.push(`${scope}.directoryCaptures.${capture} must resolve registered semantic directory kinds.`);
          if (spec.names && (!Array.isArray(spec.names) || spec.names.length === 0 || new Set(spec.names).size !== spec.names.length || spec.names.some((name) => typeof name !== "string" || name !== name.normalize("NFC") || /[\\/]/u.test(name)))) problems.push(`${scope}.directoryCaptures.${capture}.names must be exact NFC directory names.`);
        }
        if (record(contract.ownerPathPatterns, `${scope}.ownerPathPatterns`)) {
          if (Object.keys(contract.ownerPathPatterns).join("\0") !== "plugin-commands\0artifact-surface\0artifact-mode\0artifact-window\0engine-mode\0engine-window\0extension-window") problems.push(`${scope}.ownerPathPatterns must contain the seven semantic owner contexts.`);
          const patterns = new Set<string>();
          for (const [form, pattern] of Object.entries(contract.ownerPathPatterns)) {
            if (typeof pattern !== "string" || pattern !== pattern.normalize("NFC") || !pattern.startsWith("{plugin}/") || patterns.has(pattern)) { problems.push(`${scope}.ownerPathPatterns.${form} is not a unique rooted owner grammar.`); continue; }
            patterns.add(pattern);
            for (const segment of pattern.split("/")) {
              const capture = /^\{([a-zA-Z]+)\}$/u.exec(segment)?.[1];
              if (capture ? !contract.directoryCaptures?.[capture] : !segment || /[\\*?{}\0]/u.test(segment) || !semanticDirectoryKindId(segment, taxonomy)) problems.push(`${scope}.ownerPathPatterns.${form} has an unregistered directory segment.`);
            }
          }
        }
        if (record(contract.authoringCommand, `${scope}.authoringCommand`)) {
          exactKeys(contract.authoringCommand, ["scriptPath", "command", "writeDisposition"], `${scope}.authoringCommand`);
          if (contract.authoringCommand.scriptPath !== "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/📜️script.ts" || !Array.isArray(contract.authoringCommand.command) || contract.authoringCommand.command.join("\0") !== "new\0surface" || contract.authoringCommand.writeDisposition !== "create-if-absent") problems.push(`${scope}.authoringCommand must be the create-if-absent surface author.`);
        }
        if (record(contract.referenceConsumer, `${scope}.referenceConsumer`)) {
          exactKeys(contract.referenceConsumer, ["path", "ownerRoot", "adapter", "region", "lineTemplate"], `${scope}.referenceConsumer`);
          if (contract.referenceConsumer.path !== "🌎️hub/🧩️compositions/🔋️energy/📦️packages/🦀️rust/🦀️.rs" || contract.referenceConsumer.ownerRoot !== "✏️s/🔌️plugins/🔋️energy" || contract.referenceConsumer.adapter !== "rust" || contract.referenceConsumer.region !== "✏️👁️Surfaces" || contract.referenceConsumer.lineTemplate !== "// `{filename}` (`🎚️config`/`🎮️commands`/`👥️presence`/`🫧️transient` at every surface/mode level) need") problems.push(`${scope}.referenceConsumer must be the exact energy surface-region prose form.`);
        }
        continue;
      }
      if (contract.contractKind === "owner-primary-file") {
        exactKeys(contract, ["contractKind", "ownerFixedDirectoryContractId", "sourceFileKindId", "sourceFilename", "destinationFilename", "rationaleRule"], scope);
        if (id !== "ticket-document-primary-markdown-v1" || contract.ownerFixedDirectoryContractId !== "ticket-slug" || contract.sourceFileKindId !== "markdown" || contract.sourceFilename !== "ticket.md" || contract.rationaleRule !== id) problems.push(`${scope} must use the exact ticket document primary-leaf grammar.`);
        if (!taxonomy.fixedDirectoryContracts[contract.ownerFixedDirectoryContractId]) problems.push(`${scope}.ownerFixedDirectoryContractId is missing.`);
        if (!taxonomy.fileKinds[contract.sourceFileKindId] || fileKindIdForSourcePath(contract.sourceFilename, taxonomy) !== contract.sourceFileKindId || contract.destinationFilename !== canonicalFilenameForKind(contract.sourceFileKindId, taxonomy)) problems.push(`${scope}.destinationFilename must be the registered primary file-kind leaf.`);
        const tuple = [contract.ownerFixedDirectoryContractId, contract.sourceFilename, contract.contractKind].join("\0");
        if (ownerTuples.has(tuple)) problems.push(`${scope} overlaps another owned-file projection contract.`);
        ownerTuples.add(tuple);
        continue;
      }
      if (contract.contractKind === "owner-sibling-manifest-file") {
        exactKeys(contract, ["contractKind", "ownerFixedDirectoryContractId", "requiredSiblingFixedFilenameContractId", "manifestAdapter", "manifestStatusLocation", "allowedStatuses", "sourceFileKindId", "sourceFilename", "destinationDirectoryKindId", "destinationDirectoryName", "destinationFilename", "emptyContentRule", "statusDispositions", "rationaleRule"], scope);
        if (contract.manifestAdapter !== "json" || contract.manifestStatusLocation !== "status" || contract.emptyContentRule !== "zero-byte" || contract.rationaleRule !== "ticket-important-markdown-projection-v1") problems.push(`${scope} must use the exact active ticket important owner-file grammar.`);
        if (!Array.isArray(contract.allowedStatuses) || contract.allowedStatuses.join("\0") !== "closed\0open") problems.push(`${scope}.allowedStatuses must be exactly closed, open.`);
        if (contract.statusDispositions && typeof contract.statusDispositions === "object" && !Array.isArray(contract.statusDispositions)) exactKeys(contract.statusDispositions, ["open", "closed-empty", "closed-nonempty", "invalid"], `${scope}.statusDispositions`);
        else problems.push(`${scope}.statusDispositions must be an object.`);
        if (JSON.stringify(contract.statusDispositions) !== JSON.stringify({ open: "project", "closed-empty": "remove", "closed-nonempty": "problem", invalid: "problem" })) problems.push(`${scope}.statusDispositions must be the exact lifecycle mapping.`);
        if (!taxonomy.fixedFilenameContracts[contract.requiredSiblingFixedFilenameContractId]) problems.push(`${scope}.requiredSiblingFixedFilenameContractId is missing.`);
      } else if (contract.contractKind === "owner-optional-sibling-manifest-file") {
        exactKeys(contract, ["contractKind", "ownerFixedDirectoryContractId", "optionalSiblingFixedFilenameContractId", "manifestAdapter", "manifestStatusLocation", "sourceFileKindId", "sourceFilename", "destinationDirectoryKindId", "destinationDirectoryName", "destinationFilename", "admittedDispositions", "rationaleRule"], scope);
        if (contract.manifestAdapter !== "json" || contract.manifestStatusLocation !== "status" || contract.rationaleRule !== "ticket-important-history-markdown-v1") problems.push(`${scope} must use the exact ticket important history grammar.`);
        if (!Array.isArray(contract.admittedDispositions) || contract.admittedDispositions.join("\0") !== "closed-nonzero\0invalid-manifest\0missing-manifest") problems.push(`${scope}.admittedDispositions must be the exact history mapping.`);
        if (!taxonomy.fixedFilenameContracts[contract.optionalSiblingFixedFilenameContractId]) problems.push(`${scope}.optionalSiblingFixedFilenameContractId is missing.`);
      } else problems.push(`${scope}.contractKind is invalid.`);
      if (!taxonomy.fixedDirectoryContracts[contract.ownerFixedDirectoryContractId]) problems.push(`${scope}.ownerFixedDirectoryContractId is missing.`);
      if (typeof contract.sourceFilename !== "string" || !taxonomy.fileKinds[contract.sourceFileKindId] || fileKindIdForSourcePath(contract.sourceFilename, taxonomy) !== contract.sourceFileKindId) problems.push(`${scope}.sourceFilename must resolve to sourceFileKindId.`);
      const destinationKind = taxonomy.semanticDirectoryKinds[contract.destinationDirectoryKindId];
      if (!destinationKind || destinationKind.projectionOnly !== true || typeof contract.destinationDirectoryName !== "string" || semanticDirectoryKindId(contract.destinationDirectoryName, taxonomy, { parentKindId: "ticket-slug" }) !== contract.destinationDirectoryKindId) problems.push(`${scope}.destination directory must resolve to one ticket-scoped projectionOnly kind.`);
      if (contract.destinationFilename !== "📝️.md" || typeof contract.destinationFilename !== "string" || fileKindIdForSourcePath(contract.destinationFilename, taxonomy) !== contract.sourceFileKindId) problems.push(`${scope}.destinationFilename must be the exact Markdown physical leaf.`);
      for (const [field, value] of [["sourceFilename", contract.sourceFilename], ["destinationDirectoryName", contract.destinationDirectoryName], ["destinationFilename", contract.destinationFilename]] as const) if (typeof value !== "string" || !value || value !== value.normalize("NFC") || /[\\/]/u.test(value)) problems.push(`${scope}.${field} must be one non-empty NFC name.`);
      const siblingContractId = contract.contractKind === "owner-sibling-manifest-file" ? contract.requiredSiblingFixedFilenameContractId : contract.optionalSiblingFixedFilenameContractId;
      const tuple = [contract.ownerFixedDirectoryContractId, siblingContractId, contract.sourceFilename, contract.contractKind].join("\0");
      if (ownerTuples.has(tuple)) problems.push(`${scope} overlaps another owned-file projection contract.`);
      ownerTuples.add(tuple);
    }
    if (Object.keys(taxonomy.semanticOwnedFileProjectionContracts).join("\0") !== "artifact-empty-facet-primary-markdown-v1\0readme-license-owner-leaves-v1\0ticket-document-primary-markdown-v1\0ticket-important-history-markdown-v1\0ticket-important-markdown-v1") problems.push("semanticOwnedFileProjectionContracts must contain the exact artifact-facet, README/LICENSE, ticket-document, active ticket-important, and history ticket-important contracts.");
  }
}
