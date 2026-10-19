import { mutationCatalogSourceOwnersProblems, mutationDomainOwnersProblems } from "../👥️mutation-owner-rules/🟦️.ts";
import type { TaxonomyAudit } from "../🪣️taxonomy-audit-context/🟦️.ts";

/** 💉️ Audits the mutation directory pattern, domain owners, facet directories, forbidden region markers, file kinds and payload schema declarations. */
export function auditMutationVocabulary(audit: TaxonomyAudit): void {
  const { taxonomy, problems, record, fullPattern, semanticDirectoryKindId } = audit;
  fullPattern(taxonomy.mutationDirectoryPattern, "mutationDirectoryPattern");
  problems.push(...mutationDomainOwnersProblems(taxonomy.mutationDomainOwners, taxonomy.pathEmojiPolicy.genericEmojiIdentities));
  problems.push(...mutationCatalogSourceOwnersProblems(taxonomy));
  const mutationBehaviorFacetDirs = ["🦠️mutation", "🔺️diff", "↩️inverse"];
  if (taxonomy.mutationBehaviorFacetDirs?.join("\0") !== mutationBehaviorFacetDirs.join("\0")) problems.push(`mutationBehaviorFacetDirs must contain exactly ${mutationBehaviorFacetDirs.join(", ")} in canonical order.`);
  const mutationOrganizationalFacetDirs = ["🧩️plan", "📝️text", "💾️binary", "🧬️schema"];
  if (taxonomy.mutationOrganizationalFacetDirs?.join("\0") !== mutationOrganizationalFacetDirs.join("\0")) problems.push(`mutationOrganizationalFacetDirs must contain exactly ${mutationOrganizationalFacetDirs.join(", ")} in canonical order.`);
  for (const dir of [...(taxonomy.mutationBehaviorFacetDirs ?? []), ...(taxonomy.mutationOrganizationalFacetDirs ?? [])]) if (!taxonomy.taxonomyLeafParentDirs.includes(dir)) problems.push(`taxonomyLeafParentDirs must include mutation facet ${JSON.stringify(dir)}.`);
  for (const dir of taxonomy.mutationBehaviorFacetDirs ?? []) if (dir !== "🦠️mutation" && !taxonomy.mutationDirectLeafForbiddenRegionMarkers?.[dir]) problems.push(`mutationDirectLeafForbiddenRegionMarkers must name the forbidden inline region marker for behavior facet ${JSON.stringify(dir)}.`);
  if (taxonomy.mutationDirectLeafForbiddenRegionMarkers?.["🔺️diff"] !== "🔖️Diff" || taxonomy.mutationDirectLeafForbiddenRegionMarkers?.["↩️inverse"] !== "🔖️Inverse") problems.push('mutationDirectLeafForbiddenRegionMarkers must map "🔺️diff" to "🔖️Diff" and "↩️inverse" to "🔖️Inverse".');
  if (taxonomy.mutationComponentFileKindId !== "rust-source") problems.push('mutationComponentFileKindId must select the direct owner "rust-source" kind.');
  if (taxonomy.mutationDescriptorFileKindId !== "json") problems.push('mutationDescriptorFileKindId must select the language-neutral "json" kind.');
  if (record(taxonomy.mutationPayloadSchemaLocation, "mutationPayloadSchemaLocation")) {
    const location = taxonomy.mutationPayloadSchemaLocation;
    if (Object.keys(location).sort().join("\0") !== "directoryKindId\0directoryName\0fileKindId" || location.directoryKindId !== "schema" || location.directoryName !== "🧬️schema" || location.fileKindId !== "json" || semanticDirectoryKindId(location.directoryName, taxonomy) !== location.directoryKindId) problems.push("mutationPayloadSchemaLocation must select the registered schema directory and JSON physical leaf.");
  }

  if (record(taxonomy.mutationPayloadSchemaAuthority, "mutationPayloadSchemaAuthority")) {
    const expected = { contractKind: "descriptor-linked-mutation-payload-schema", ownerAuthority: "mutationOwnerIdentity", descriptorFileKindId: "json", descriptorField: "payloadSchema", descriptorSchemaVersion: 1, descriptorCardinality: "one-canonical-no-competing-descriptor", descriptorOwnerField: "owner", descriptorIdentityField: "semanticKind", jsonSchemaDialect: "http://json-schema.org/draft-07/schema#", targetAuthority: "owner-relative-regular-json-schema" };
    const authority = taxonomy.mutationPayloadSchemaAuthority;
    if (Object.keys(authority).length !== Object.keys(expected).length || Object.entries(expected).some(([key, value]) => authority[key as keyof typeof authority] !== value)) problems.push("mutationPayloadSchemaAuthority must bind exact mutation descriptors to regular owner-contained JSON Schema files.");
  }
}
