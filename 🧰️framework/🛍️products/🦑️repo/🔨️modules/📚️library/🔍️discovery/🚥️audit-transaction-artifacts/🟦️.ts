import type { TaxonomyAudit } from "../🪣️taxonomy-audit-context/🟦️.ts";

/** 🚥️ Audits the permanent taxonomy CLI artifact directory kinds, the exact transaction scoped file kinds and the preparation and lease grammar they enable. */
export function auditTransactionArtifacts(audit: TaxonomyAudit): void {
  const { taxonomy, problems, exactKeys, semanticDirectoryKindId, taxonomyCliAttemptPreparationsProblems, taxonomyCliLeaseDirectoryProblems } = audit;
  const taxonomyCliArtifactDirectoryKinds: Readonly<Record<string, Readonly<{ name: string; emoji: string; slugPattern: string; parentKindIds?: readonly string[] }>>> = {
    "taxonomy-transaction": { name: "🧾️taxonomy-transaction", emoji: "🧾️", slugPattern: "^taxonomy-transaction$" },
    "transaction-digest": { name: `🔖️${"0".repeat(64)}`, emoji: "🔖️", slugPattern: "^[a-f0-9]{64}$", parentKindIds: ["taxonomy-transaction"] },
    "transaction-attempts": { name: "🔂️attempts", emoji: "🔂️", slugPattern: "^attempts$", parentKindIds: ["transaction-digest"] },
    "transaction-attempt": { name: "🔢️000001", emoji: "🔢️", slugPattern: "^[0-9]{6}$", parentKindIds: ["transaction-attempts"] },
    "transaction-stage": { name: "🚧️stage", emoji: "🚧️", slugPattern: "^stage$", parentKindIds: ["transaction-attempt", "transaction-attempt-preparation"] },
    "transaction-backup": { name: "💾️backup", emoji: "💾️", slugPattern: "^backup$", parentKindIds: ["transaction-attempt", "transaction-attempt-preparation"] },
    "transaction-lease": { name: "🔒️lease", emoji: "🔒️", slugPattern: "^lease$", parentKindIds: ["transaction-attempt", "transaction-attempt-preparation"] },
    "transaction-attempt-preparation": { name: "🚧️prepare-000001-42-123e4567-e89b-42d3-a456-426614174000", emoji: "🚧️", slugPattern: "^prepare-[0-9]{6}-[1-9][0-9]*-[0-9a-f]{8}-[0-9a-f]{4}-4[0-9a-f]{3}-[89ab][0-9a-f]{3}-[0-9a-f]{12}$", parentKindIds: ["transaction-attempts"] },
    "transaction-edit-preparation": { name: "🚧️edit-0123456789abcdef01234567-42-123e4567-e89b-42d3-a456-426614174000", emoji: "🚧️", slugPattern: "^edit-[0-9a-f]{24}-[1-9][0-9]*-[0-9a-f]{8}-[0-9a-f]{4}-4[0-9a-f]{3}-[89ab][0-9a-f]{3}-[0-9a-f]{12}$", parentKindIds: ["transaction-stage"] },
    "transaction-edit-write-preparation": { name: "🚧️write-42-123e4567-e89b-42d3-a456-426614174000", emoji: "🚧️", slugPattern: "^write-[1-9][0-9]*-[0-9a-f]{8}-[0-9a-f]{4}-4[0-9a-f]{3}-[89ab][0-9a-f]{3}-[0-9a-f]{12}$", parentKindIds: ["transaction-edit-preparation"] },
    "transaction-journal-write": { name: "🚧️journal", emoji: "🚧️", slugPattern: "^journal$", parentKindIds: ["transaction-stage"] },
    "transaction-json-write-preparation": { name: "🚧️write-42-123e4567-e89b-42d3-a456-426614174000", emoji: "🚧️", slugPattern: "^write-[1-9][0-9]*-[0-9a-f]{8}-[0-9a-f]{4}-4[0-9a-f]{3}-[89ab][0-9a-f]{3}-[0-9a-f]{12}$", parentKindIds: ["transaction-journal-write", "transaction-lease-preparation", "transaction-lease"] },
    "transaction-lease-preparation": { name: "🚧️lease-42-123e4567-e89b-42d3-a456-426614174000-preparing", emoji: "🚧️", slugPattern: "^lease-[1-9][0-9]*-[0-9a-f]{8}-[0-9a-f]{4}-4[0-9a-f]{3}-[89ab][0-9a-f]{3}-[0-9a-f]{12}-(preparing|stale)$", parentKindIds: ["transaction-backup"] },
    "transaction-backup-preparation": { name: "🚧️backup-0123456789abcdef01234567-42-123e4567-e89b-42d3-a456-426614174000", emoji: "🚧️", slugPattern: "^backup-[0-9a-f]{24}-[1-9][0-9]*-[0-9a-f]{8}-[0-9a-f]{4}-4[0-9a-f]{3}-[89ab][0-9a-f]{3}-[0-9a-f]{12}$", parentKindIds: ["transaction-backup"] },
    "transaction-backup-write-preparation": { name: "🚧️write-42-123e4567-e89b-42d3-a456-426614174000", emoji: "🚧️", slugPattern: "^write-[1-9][0-9]*-[0-9a-f]{8}-[0-9a-f]{4}-4[0-9a-f]{3}-[89ab][0-9a-f]{3}-[0-9a-f]{12}$", parentKindIds: ["transaction-backup-preparation"] },
    "transaction-restore-preparation": { name: "🚧️restore-0123456789abcdef01234567-42-123e4567-e89b-42d3-a456-426614174000", emoji: "🚧️", slugPattern: "^restore-[0-9a-f]{24}-[1-9][0-9]*-[0-9a-f]{8}-[0-9a-f]{4}-4[0-9a-f]{3}-[89ab][0-9a-f]{3}-[0-9a-f]{12}$", parentKindIds: ["transaction-backup"] },
    "taxonomy-inventory-data": { name: "📊️taxonomy-inventory", emoji: "📊️", slugPattern: "^taxonomy-inventory$" },
    "taxonomy-plan-data": { name: "📊️taxonomy-plan", emoji: "📊️", slugPattern: "^taxonomy-plan$" },
    "taxonomy-apply-data": { name: "📊️taxonomy-apply", emoji: "📊️", slugPattern: "^taxonomy-apply$" },
    "taxonomy-verification-data": { name: "📊️taxonomy-verification", emoji: "📊️", slugPattern: "^taxonomy-verification$" },
    "taxonomy-inventory-summary": { name: "📓️taxonomy-inventory", emoji: "📓️", slugPattern: "^taxonomy-inventory$" },
    "taxonomy-plan-summary": { name: "📓️taxonomy-plan", emoji: "📓️", slugPattern: "^taxonomy-plan$" },
    "taxonomy-apply-summary": { name: "📓️taxonomy-apply", emoji: "📓️", slugPattern: "^taxonomy-apply$" },
    "taxonomy-verification-summary": { name: "📓️taxonomy-verification", emoji: "📓️", slugPattern: "^taxonomy-verification$" },
    "taxonomy-inventory-shards": { name: "📊️shards", emoji: "📊️", slugPattern: "^shards$", parentKindIds: ["taxonomy-inventory-data"] },
    "taxonomy-inventory-shard-digest": { name: `🔖️${"0".repeat(64)}`, emoji: "🔖️", slugPattern: "^[a-f0-9]{64}$", parentKindIds: ["taxonomy-inventory-shards"] },
  };
  for (const [id, expected] of Object.entries(taxonomyCliArtifactDirectoryKinds)) {
    const spec = taxonomy.semanticDirectoryKinds[id];
    if (!spec) {
      problems.push(`semanticDirectoryKinds[${JSON.stringify(id)}] must declare the permanent taxonomy CLI artifact directory.`);
      continue;
    }
    exactKeys(spec, expected.parentKindIds ? ["emoji", "slugPattern", "allowEmojiOnly", "parentKindIds"] : ["emoji", "slugPattern", "allowEmojiOnly"], `semanticDirectoryKinds[${JSON.stringify(id)}]`);
    if (spec.emoji !== expected.emoji || spec.slugPattern !== expected.slugPattern || spec.allowEmojiOnly || JSON.stringify(spec.parentKindIds) !== JSON.stringify(expected.parentKindIds)) problems.push(`semanticDirectoryKinds[${JSON.stringify(id)}] must remain the exact taxonomy CLI artifact contract.`);
    if (semanticDirectoryKindId(expected.name, taxonomy, { parentKindId: expected.parentKindIds?.[0] }) !== id) problems.push(`semanticDirectoryKinds[${JSON.stringify(id)}] does not resolve its canonical taxonomy CLI artifact directory uniquely.`);
  }
  const editCandidate = taxonomy.scopedFileKinds["transaction-edit-candidate"];
  if (!editCandidate || editCandidate.pathPattern !== "**/🚧️stage/🚧️edit-*/*.edit" || editCandidate.parentDirectoryKindId !== "transaction-edit-preparation" || editCandidate.extensionChains.join("\0") !== ".edit" || editCandidate.sourceFilenamePattern !== "^[0-9a-f]{24}\\.edit$") problems.push("scopedFileKinds.transaction-edit-candidate must remain the exact transaction edit candidate authority.");
  const editPreimage = taxonomy.scopedFileKinds["transaction-edit-preimage"];
  if (!editPreimage || editPreimage.pathPattern !== "**/🚧️stage/🚧️edit-*/*.pre" || editPreimage.parentDirectoryKindId !== "transaction-edit-preparation" || editPreimage.extensionChains.join("\0") !== ".pre" || editPreimage.sourceFilenamePattern !== "^[0-9a-f]{24}\\.pre$") problems.push("scopedFileKinds.transaction-edit-preimage must remain the exact transaction edit displaced-preimage authority.");
  const editWriteCandidate = taxonomy.scopedFileKinds["transaction-edit-write-candidate"];
  if (!editWriteCandidate || editWriteCandidate.pathPattern !== "**/🚧️stage/🚧️edit-*/🚧️write-*/🚧️.edit" || editWriteCandidate.parentDirectoryKindId !== "transaction-edit-write-preparation" || editWriteCandidate.extensionChains.join("\0") !== ".edit" || editWriteCandidate.sourceFilenamePattern !== "^🚧️\\.edit$") problems.push("scopedFileKinds.transaction-edit-write-candidate must remain the exact unpublished edit-writer authority.");
  const backupWriteCandidate = taxonomy.scopedFileKinds["transaction-backup-write-candidate"];
  if (!backupWriteCandidate || backupWriteCandidate.pathPattern !== "**/💾️backup/🚧️backup-*/🚧️write-*/🚧️.backup" || backupWriteCandidate.parentDirectoryKindId !== "transaction-backup-write-preparation" || backupWriteCandidate.extensionChains.join("\0") !== ".backup" || backupWriteCandidate.sourceFilenamePattern !== "^🚧️\\.backup$") problems.push("scopedFileKinds.transaction-backup-write-candidate must remain the exact unpublished backup-writer authority.");
  const jsonPrevious = taxonomy.scopedFileKinds["transaction-json-previous"];
  if (!jsonPrevious || jsonPrevious.pathPattern !== "**/🚧️write-*/⏮️.json" || jsonPrevious.parentDirectoryKindId !== "transaction-json-write-preparation" || jsonPrevious.extensionChains.join("\0") !== ".json" || jsonPrevious.sourceFilenamePattern !== "^⏮️\\.json$") problems.push("scopedFileKinds.transaction-json-previous must remain the exact displaced canonical JSON authority.");
  const backupCandidate = taxonomy.scopedFileKinds["transaction-backup-candidate"];
  if (!backupCandidate || backupCandidate.pathPattern !== "**/💾️backup/🚧️restore-*/*.backup" || backupCandidate.parentDirectoryKindId !== "transaction-restore-preparation" || backupCandidate.extensionChains.join("\0") !== ".backup" || backupCandidate.sourceFilenamePattern !== "^[0-9a-f]{24}\\.backup$") problems.push("scopedFileKinds.transaction-backup-candidate must remain the exact transaction restore preimage authority.");
  const postimageCandidate = taxonomy.scopedFileKinds["transaction-postimage-candidate"];
  if (!postimageCandidate || postimageCandidate.pathPattern !== "**/💾️backup/🚧️restore-*/*.post" || postimageCandidate.parentDirectoryKindId !== "transaction-restore-preparation" || postimageCandidate.extensionChains.join("\0") !== ".post" || postimageCandidate.sourceFilenamePattern !== "^[0-9a-f]{24}\\.post$") problems.push("scopedFileKinds.transaction-postimage-candidate must remain the exact transaction restore postimage authority.");
  if (semanticDirectoryKindId("🚧️write-42-123e4567-e89b-42d3-a456-426614174000", taxonomy, { parentKindId: "transaction-lease-preparation" }) !== "transaction-json-write-preparation" || semanticDirectoryKindId("🚧️write-42-123e4567-e89b-42d3-a456-426614174000", taxonomy, { parentKindId: "transaction-lease" }) !== "transaction-json-write-preparation") problems.push("semanticDirectoryKinds.transaction-json-write-preparation must resolve below all three exact parent kinds.");
  for (const [name, expectedKindId] of [["🚧️stage", "transaction-stage"], ["💾️backup", "transaction-backup"], ["🔒️lease", "transaction-lease"]] as const) if (semanticDirectoryKindId(name, taxonomy, { parentKindId: "transaction-attempt-preparation" }) !== expectedKindId) problems.push(`semanticDirectoryKinds.${expectedKindId} must resolve below transaction-attempt-preparation.`);
  const attemptPreparation = { parentKindId: "transaction-attempts", directoryName: "🚧️prepare-000001-42-123e4567-e89b-42d3-a456-426614174000", children: [{ name: "🚧️stage", nodeKind: "directory" }, { name: "💾️backup", nodeKind: "directory" }, { name: "🔒️lease", nodeKind: "directory" }, { name: "🔣️.json", nodeKind: "file" }] } as const;
  if (taxonomyCliAttemptPreparationsProblems([attemptPreparation], taxonomy).length > 0 || taxonomyCliAttemptPreparationsProblems([attemptPreparation, attemptPreparation], taxonomy).length === 0) problems.push("transaction-attempt-preparation must retain its validate-all exact child and duplicate-sibling authority.");
  const leasePreparation = { parentKindId: "transaction-backup", directoryName: "🚧️lease-42-123e4567-e89b-42d3-a456-426614174000-preparing", leafNames: ["🔣️.json"], writePreparations: [] } as const;
  if (taxonomyCliLeaseDirectoryProblems(leasePreparation, taxonomy).length > 0) problems.push("transaction-lease-preparation must admit one complete canonical JSON lease before publication.");
}
