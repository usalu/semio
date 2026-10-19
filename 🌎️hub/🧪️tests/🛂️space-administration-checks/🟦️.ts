/** 🧭️ Space administration, public boundary, presence lease and presence normalization gates.
 * @see ../../📦️packages/🦀️rust/📜️script.ts
 */
import { scriptCompilerAdmissionControlV1 as exactCargoCallerControlV1 } from "@semio-tech/process/compiler-capacity";
import { unsignedLebOracleV1 } from "@semio-tech/framework-pack-ts/format/testing/oracles/leb";
import { configuredExactCargoLawPolicyV1 } from "@semio-tech/process/cargo-laws";
import { buildBudgetMs } from "@semio-tech/process/budget";
import { readModuleTreeSource } from "@semio-tech/framework-compiler-ts/syntax/module-tree";
import { createHash } from "node:crypto";
import { readFileSync } from "node:fs";
import { join } from "node:path";
import { decodePresencePeer, encodePresencePeer, PRESENCE_PRINCIPAL_KINDS, type ArtifactPresencePeer, type ArtifactPresencePrincipalKind } from "@semio-tech/framework-replication";
import { runCargo, runCmd, orchestratorBudgetOpts, runProbe } from "@semio-tech/process/execution";
import { runAdmittedExactCargoLawsV1 } from "@semio-tech/process/cargo-test";
import { BundleScript } from "@semio-tech/process/routing";
import { hubSchemaExport } from "../../📦️packages/🟦️typescript/🟦️.ts";
import { exactCargoStageEnvironments } from "../../🏗️build/🛂staging-environment/🟦️.ts";
import { assertObservationExpectation, type ObservationExpectation } from "../../🧪️testing/🛂️expectation/🟦️.ts";
import { hubLawGroups, readDirectoryBackendSource, readHubBinSource, readNeo4jDirectorySource } from "../../🧪️testing/📖️source-readers/🟦️.ts";

type SpacePublicBoundaryFixture = {
  readonly positives: Readonly<Record<"anonymous" | "publicNonmember" | "member" | "author", Record<string, unknown>>>;
  readonly forbiddenPublicKeys: readonly string[];
  readonly hostileMutations: readonly ({ readonly name: string; readonly path: readonly (string | number)[]; readonly value: unknown } & ObservationExpectation)[];
  readonly rawPublicEvent: Record<string, unknown>;
};

function mutateSpacePublicProjection(source: Record<string, unknown>, path: readonly (string | number)[], value: unknown): Record<string, unknown> {
  const candidate = structuredClone(source) as Record<string, unknown>;
  let cursor: any = candidate;
  for (const segment of path.slice(0, -1)) cursor = cursor[segment];
  cursor[path[path.length - 1]!] = value;
  return candidate;
}

async function proveSpacePublicBoundaryFixture(repoRoot: string): Promise<void> {
  const root = join(repoRoot, "🌎️hub/📇️directory/🧪️tests/🧫️fixtures/🏛️public-space-detail-v1");
  const fixture = JSON.parse(readFileSync(join(root, "🔣️.json"), "utf8")) as SpacePublicBoundaryFixture;
  const validatePublic = hubSchemaExport(repoRoot, "schema://hub.directory/PublicSpaceDetailV1");
  const validateMember = hubSchemaExport(repoRoot, "schema://hub.directory/MemberSpaceDetailV1");
  const validateAuthor = hubSchemaExport(repoRoot, "schema://hub.directory/AuthorSpaceDetailV1");
  if (Object.keys(fixture).sort().join(",") !== "forbiddenPublicKeys,hostileMutations,positives,rawPublicEvent,schema") throw new Error("space public boundary fixture envelope drift");
  if (Object.keys(fixture.positives).sort().join(",") !== "anonymous,author,member,publicNonmember") throw new Error("space public boundary access inventory drift");
  if (fixture.forbiddenPublicKeys.length !== 21 || new Set(fixture.forbiddenPublicKeys).size !== 21 || fixture.hostileMutations.length !== 21 || new Set(fixture.hostileMutations.map(({ name }) => name)).size !== 21)
    throw new Error("space public boundary hostile/forbidden inventory drift");
  if (!validatePublic(fixture.positives.anonymous) || !validatePublic(fixture.positives.publicNonmember) || !validateMember(fixture.positives.member) || !validateAuthor(fixture.positives.author))
    throw new Error("space public boundary positive projection drift");
  const publicBytes = JSON.stringify([fixture.positives.anonymous, fixture.positives.publicNonmember]);
  for (const key of fixture.forbiddenPublicKeys) if (publicBytes.includes(`\"${key}\"`)) throw new Error(`space public boundary positive leaked ${key}`);
  for (const vector of fixture.hostileMutations) {
    const candidate = mutateSpacePublicProjection(fixture.positives.anonymous, vector.path, vector.value);
    assertObservationExpectation(`space public boundary hostile ${vector.name}`, vector, validatePublic(candidate));
  }
  if (validatePublic(fixture.rawPublicEvent)) throw new Error("space public boundary admitted a raw DirectoryEvent as public projection");
  const raw = JSON.stringify(fixture.rawPublicEvent);
  if (!raw.includes('"actor"') || !raw.includes('"hlc"') || !raw.includes('"userId"')) throw new Error("space public boundary raw event hostile vector lost identity fields");
  console.log(`space-public-boundary-oracle: hub.directory-exports=3 positives=4 hostile=${fixture.hostileMutations.length} forbidden=${fixture.forbiddenPublicKeys.length} raw-event=denied passed`);
}

export class SpacePublicBoundaryCheckScript extends BundleScript {
  async run(): Promise<void> {
    await proveSpacePublicBoundaryFixture(this.repoRoot);
    const target = ["test", "--manifest-path", "Cargo.toml", "--all-features", "-p", "semio-hub-directory-route-laws", "--lib"];
    const suffixes = [
      "space_public_boundary_real_routes_emit_discriminated_public_member_author_and_private_404",
      "space_public_boundary_public_event_route_denies_raw_directory_events",
      "space_public_boundary_real_socket_denies_public_raw_events_and_member_telemetry",
    ];
    const listed = runProbe("cargo", [...target, "--", "--list"], { cwd: this.root, ...orchestratorBudgetOpts() });
    const discovered = listed.stdout
      .split("\n")
      .filter((line) => line.endsWith(": test"))
      .map((line) => line.slice(0, -": test".length));
    const laws = suffixes.map((suffix) => {
      const matches = discovered.filter((name) => name.endsWith(suffix));
      if (listed.status !== 0 || matches.length !== 1) throw new Error(`space-public-boundary-check expected exactly one ${suffix}, selected ${matches.length}; status=${listed.status}; diagnostic=${listed.stderr.trim().slice(-4_000) || "<none>"}`);
      return matches[0]!;
    });
    console.log(`space-public-boundary-laws: exact=${laws.length} laws=${laws.join(",")}`);
    for (const law of laws) runCargo([...target, law, "--", "--exact", "--test-threads=1"], this.root);
    runCmd("bun", ["repo", "run", "@semio-tech/framework-os:test", "--skip-cache", "--", "quick", "--run", "-t", "DirectoryClient space public boundary"], { cwd: this.repoRoot, ...orchestratorBudgetOpts() });
    runCargo(["check", "--manifest-path", "Cargo.toml", "--all-features", "--bin", "os-hub"], this.root);
    console.log("space-public-boundary-check: hostile neutral projection oracle, exact real REST/socket laws, TypeScript discriminator/unknown-field law, and hub all-feature check passed");
  }
}

type PresenceLeaseOperation =
  | { readonly kind: "install" | "tick" | "close"; readonly scope: string; readonly actor: string; readonly liveId: string; readonly nowMs: number }
  | { readonly kind: "refresh"; readonly scope: string; readonly actor: string; readonly liveId: string; readonly nowMs: number; readonly peerTag: string; readonly peerBytes: number }
  | { readonly kind: "fill"; readonly scope: string; readonly count: number; readonly peerBytes: number; readonly nowMs: number }
  | { readonly kind: "restart" };

type PresenceLeaseFixture = {
  readonly schema: "semio.hub.presence-lease/v1";
  readonly limits: { readonly ttlMs: 15000; readonly maximumItems: 64; readonly maximumEntryBytes: 4096; readonly maximumBytes: 262144 };
  readonly vectors: readonly {
    readonly name: string;
    readonly operations: readonly PresenceLeaseOperation[];
    readonly expected: {
      readonly outcomes: readonly string[];
      readonly fanoutCount: number;
      readonly final: readonly { readonly scope: string; readonly count: number; readonly bytes: number; readonly actors: readonly string[] }[];
      readonly socketStillLive: boolean;
      readonly durableWrites: 0;
      readonly directoryRecipients: readonly "member"[];
    };
  }[];
};

/** 🧯️ Structural fence: the worker's one terminal transition must erase EVERY retained
 * administration field — the page bytes, the receipt, and the invite capability — before it settles
 * a phase. Written against the extracted function body rather than a contiguous literal so a sibling
 * lane may add retained fields (the invite transfer epoch did) without silently disarming the law. */
function directoryAdministrationTerminateErases(browser: string): boolean {
  const start = browser.indexOf("function terminateDirectoryAdministration(");
  if (start < 0) return false;
  const body = browser.slice(start, browser.indexOf("\n}", start));
  const erased = ["canonicalJson", "receiptSha256", "outcome", "inviteToken", "requestId"];
  return (
    erased.every((field) => body.includes(`operation.${field} = null;`)) &&
    body.includes("operation.phase = phase;") &&
    body.includes("operation.abort.abort(") &&
    body.indexOf("operation.phase = phase;") > body.indexOf("operation.inviteToken = null;")
  );
}

/** 🏛️ Replays the bounded space-administration page contract without using the Rust implementation. */
type DirectorySpaceAdministrationFixture = {
  readonly schema: "semio.hub.directory-space-administration-page/v1";
  readonly limits: { readonly windowRows: 64; readonly pageBytes: 49152; readonly cursorBytes: 1024; readonly safeInteger: 9007199254740991 };
  readonly session: { readonly sessionId: string; readonly userId: string; readonly authorizationGeneration: number; readonly expiresAt: number; readonly spaceId: string; readonly bindingSha256: string };
  readonly space: Record<string, unknown>;
  readonly members: readonly { readonly userId: string; readonly email: string; readonly displayName: string; readonly role: string; readonly owner: boolean }[];
  readonly invites: readonly { readonly inviteId: string; readonly role: string; readonly createdAtMs: number; readonly expiresAtMs: number; readonly revoked: boolean; readonly accepted: boolean }[];
  readonly vectors: readonly {
    readonly name: string;
    readonly access: "author" | "member" | "public";
    readonly expected: { readonly hasMembers: boolean; readonly hasInvites: boolean; readonly hasCapabilities: boolean; readonly memberRows: number; readonly inviteRows: number };
  }[];
  readonly cursorCases: readonly { readonly query: string; readonly status: number; readonly reads: number }[];
  readonly hostiles: readonly string[];
};

async function proveDirectorySpaceAdministrationPageV1(repoRoot: string): Promise<number> {
  const root = join(repoRoot, "🌎️hub/📇️directory/🧪️tests/🧫️fixtures/🏘️space-administration-page-v1");
  const fixture = JSON.parse(readFileSync(join(root, "🔣️.json"), "utf8")) as DirectorySpaceAdministrationFixture;
  if (fixture.schema !== "semio.hub.directory-space-administration-page/v1") throw new Error("space administration fixture schema drift");
  if (Object.keys(fixture).sort().join(",") !== "cursorCases,hostiles,invites,limits,members,schema,session,space,vectors") throw new Error("space administration fixture envelope drift");
  if (fixture.limits.windowRows !== 64 || fixture.limits.pageBytes !== 49152 || fixture.limits.cursorBytes !== 1024 || fixture.limits.safeInteger !== 9007199254740991) throw new Error("space administration bound drift");
  if (!hubSchemaExport(repoRoot, "schema://hub.directory/DirectorySpaceSessionBindingV1")(fixture.session)) throw new Error("space administration session is not the owned space session-binding contract");
  if (!hubSchemaExport(repoRoot, "schema://hub.directory/SpaceAdministrationSpaceV1")(fixture.space)) throw new Error("space administration space is not the owned administration space contract");
  if (fixture.session.spaceId !== (fixture.space.id as string)) throw new Error("space administration session is not bound to its own space");
  const administrationMember = hubSchemaExport(repoRoot, "schema://hub.directory/SpaceAdministrationMemberRowV1");
  const administrationInvite = hubSchemaExport(repoRoot, "schema://hub.directory/SpaceAdministrationInviteRowV1");
  if (fixture.members.length < 2 || fixture.members.length > fixture.limits.windowRows || fixture.invites.length < 2 || fixture.invites.length > fixture.limits.windowRows) throw new Error("space administration window inventory drift");
  for (const member of fixture.members) if (!administrationMember(member)) throw new Error(`space administration member is not the owned member-row contract: ${member.userId}`);
  for (const invite of fixture.invites) if (!administrationInvite(invite)) throw new Error(`space administration invite is not the owned invite-row contract: ${invite.inviteId}`);
  if (administrationInvite({ ...fixture.invites[0]!, inviteSecret: "leak" })) throw new Error("space administration invite-row contract admitted a secret field");
  if (fixture.vectors.map((vector) => vector.name).join(",") !== "author,member,public,author-window-boundary") throw new Error("space administration vector inventory drift");
  const administrationHostiles = ["receipt-substituted", "trailing-whitespace", "unknown-field", "space-mismatch", "member-order-reversed", "invite-order-reversed", "window-row-max-plus-one", "page-byte-max-plus-one", "invite-secret-field"];
  if (fixture.hostiles.join(",") !== administrationHostiles.join(",")) throw new Error("space administration hostile inventory drift");
  if (fixture.cursorCases.length !== 8 || fixture.cursorCases.some((row) => ![200, 400].includes(row.status) || ![0, 1].includes(row.reads))) throw new Error("space administration cursor inventory drift");
  const u32be = (value: number): Buffer => {
    const bytes = Buffer.alloc(4);
    bytes.writeUInt32BE(value);
    return bytes;
  };
  const u64be = (value: number): Buffer => {
    const bytes = Buffer.alloc(8);
    bytes.writeBigUInt64BE(BigInt(value));
    return bytes;
  };
  const i64be = (value: number): Buffer => {
    const bytes = Buffer.alloc(8);
    bytes.writeBigInt64BE(BigInt(value));
    return bytes;
  };
  const sessionId = Buffer.from(fixture.session.sessionId, "utf8");
  const userId = Buffer.from(fixture.session.userId, "utf8");
  const spaceId = Buffer.from(fixture.session.spaceId, "utf8");
  const binding = createHash("sha256")
    .update(
      Buffer.concat([
        Buffer.from("semio/hub/directory-space-administration/session-binding/v1\0"),
        u32be(sessionId.length),
        sessionId,
        u32be(userId.length),
        userId,
        u64be(fixture.session.authorizationGeneration),
        i64be(fixture.session.expiresAt),
        u32be(spaceId.length),
        spaceId,
      ]),
    )
    .digest("hex");
  if (binding !== fixture.session.bindingSha256) throw new Error("space administration session binding is not byte-exact");
  const alternateSession = Buffer.from("session-admin-0", "utf8");
  const alternateUser = Buffer.from("1user-admin-01", "utf8");
  if (Buffer.concat([sessionId, userId]).compare(Buffer.concat([alternateSession, alternateUser])) !== 0) throw new Error("binding ambiguity fixture drifted");
  const alternateBinding = createHash("sha256")
    .update(
      Buffer.concat([
        Buffer.from("semio/hub/directory-space-administration/session-binding/v1\0"),
        u32be(alternateSession.length),
        alternateSession,
        u32be(alternateUser.length),
        alternateUser,
        u64be(fixture.session.authorizationGeneration),
        i64be(fixture.session.expiresAt),
        u32be(spaceId.length),
        spaceId,
      ]),
    )
    .digest("hex");
  if (alternateBinding === binding) throw new Error("length-prefixed session binding aliased concatenated identities");

  const capabilities = { renameSpace: true, setVisibility: true, deleteSpace: true, upsertMember: true, removeMember: true, createInvite: true, revokeInvite: true };
  const memberRow = (index: number) =>
    index < fixture.members.length ? fixture.members[index]! : { userId: `user-${String(index).padStart(4, "0")}`, email: `u${index}@example.invalid`, displayName: `U${index}`, role: "spectator" as const, owner: false };
  const inviteRow = (index: number) =>
    index < fixture.invites.length ? fixture.invites[index]! : { inviteId: `invite-${String(1000 - index).padStart(4, "0")}`, role: "spectator" as const, createdAtMs: 1000 - index, expiresAtMs: 900000, revoked: false, accepted: false };
  const memberRows = (count: number) => Array.from({ length: count }, (_, index) => memberRow(index)).sort((left, right) => (left.userId < right.userId ? -1 : left.userId > right.userId ? 1 : 0));
  const inviteRows = (count: number) => Array.from({ length: count }, (_, index) => inviteRow(index)).sort((left, right) => right.createdAtMs - left.createdAtMs || (left.inviteId < right.inviteId ? 1 : -1));

  const seal = (access: "author" | "member" | "public", members: number, invites: number): { canonical: string; page: Record<string, unknown> } => {
    const publicSpace = {
      id: fixture.space.id,
      name: fixture.space.name,
      kind: fixture.space.kind,
      visibility: fixture.space.visibility,
      memberCount: fixture.space.memberCount,
      documentCount: fixture.space.documentCount,
      createdAtMs: fixture.space.createdAtMs,
      updatedAtMs: fixture.space.updatedAtMs,
    };
    const memberSpace = { ...fixture.space, role: access === "author" ? "author" : "spectator" };
    const base = {
      access,
      schema: "semio.directory.space-administration-page.v1" as const,
      sessionBindingSha256: access === "public" ? "0".repeat(64) : binding,
      authorizationGeneration: access === "public" ? 0 : fixture.session.authorizationGeneration,
      spaceId: fixture.session.spaceId,
      space: access === "public" ? publicSpace : memberSpace,
    };
    const unsigned =
      access === "public"
        ? { ...base, documents: { rows: [] } }
        : access === "member"
          ? { ...base, members: { rows: memberRows(members) }, documents: { rows: [] } }
          : { ...base, members: { rows: memberRows(members) }, documents: { rows: [] }, invites: { rows: inviteRows(invites) }, capabilities };
    const receiptSha256 = createHash("sha256").update(JSON.stringify(unsigned)).digest("hex");
    const page = { ...unsigned, receiptSha256 } as Record<string, unknown>;
    return { canonical: JSON.stringify(page), page };
  };

  for (const vector of fixture.vectors) {
    const { canonical, page } = seal(vector.access, vector.expected.memberRows, vector.expected.inviteRows);
    if ("members" in page !== vector.expected.hasMembers || "invites" in page !== vector.expected.hasInvites || "capabilities" in page !== vector.expected.hasCapabilities) {
      throw new Error(`space administration shape differs for ${vector.name}`);
    }
    const rows = (page.members as { rows: unknown[] } | undefined)?.rows.length ?? 0;
    const invited = (page.invites as { rows: unknown[] } | undefined)?.rows.length ?? 0;
    if (rows !== vector.expected.memberRows || invited !== vector.expected.inviteRows) throw new Error(`space administration window differs for ${vector.name}`);
    if (rows > fixture.limits.windowRows || invited > fixture.limits.windowRows) throw new Error(`space administration window exceeded ${fixture.limits.windowRows} rows`);
    if (Buffer.byteLength(canonical, "utf8") > fixture.limits.pageBytes) throw new Error(`space administration page exceeded ${fixture.limits.pageBytes} bytes for ${vector.name}`);
    const { receiptSha256, ...unsigned } = page as Record<string, unknown> & { receiptSha256: string };
    if (createHash("sha256").update(JSON.stringify(unsigned)).digest("hex") !== receiptSha256) throw new Error(`space administration receipt differs for ${vector.name}`);
    for (const secret of ["selector", "secretDigest", "inviteToken", "passwordHash", "ssoSubject", "ssoProvider", "sessionId"]) {
      if (canonical.includes(secret)) throw new Error(`space administration page leaked ${secret} in ${vector.name}`);
    }
    if (vector.access !== "author" && (canonical.includes('"invites"') || canonical.includes('"capabilities"'))) throw new Error(`non-author page carried an author-only window in ${vector.name}`);
  }

  const author = seal("author", 2, 2);
  const memberOrder = (page: string): boolean => {
    const parsed = JSON.parse(page) as { members?: { rows: { userId: string }[] } };
    const rows = parsed.members?.rows ?? [];
    return rows.every((row, index) => index === 0 || rows[index - 1]!.userId < row.userId);
  };
  const inviteOrder = (page: string): boolean => {
    const parsed = JSON.parse(page) as { invites?: { rows: { createdAtMs: number; inviteId: string }[] } };
    const rows = parsed.invites?.rows ?? [];
    return rows.every((row, index) => index === 0 || rows[index - 1]!.createdAtMs > row.createdAtMs || (rows[index - 1]!.createdAtMs === row.createdAtMs && rows[index - 1]!.inviteId > row.inviteId));
  };
  if (!memberOrder(author.canonical) || !inviteOrder(author.canonical)) throw new Error("sealed author page is not keyset-ordered");

  const canonicalRejects = (candidate: string): boolean => {
    if (Buffer.byteLength(candidate, "utf8") > fixture.limits.pageBytes) return true;
    let parsed: Record<string, unknown>;
    try {
      parsed = JSON.parse(candidate) as Record<string, unknown>;
    } catch {
      return true;
    }
    if (JSON.stringify(parsed) !== candidate) return true;
    if (parsed.spaceId !== (parsed.space as { id?: unknown } | undefined)?.id) return true;
    if (!memberOrder(candidate) || !inviteOrder(candidate)) return true;
    const known = new Set(["access", "schema", "sessionBindingSha256", "authorizationGeneration", "spaceId", "space", "members", "documents", "invites", "capabilities", "receiptSha256"]);
    if (Object.keys(parsed).some((key) => !known.has(key))) return true;
    const { receiptSha256, ...unsigned } = parsed as Record<string, unknown> & { receiptSha256: string };
    return createHash("sha256").update(JSON.stringify(unsigned)).digest("hex") !== receiptSha256;
  };
  const hostile = (name: string): string => {
    switch (name) {
      case "receipt-substituted":
        return author.canonical.replace(/"receiptSha256":"[0-9a-f]{64}"/u, `"receiptSha256":"${"b".repeat(64)}"`);
      case "trailing-whitespace":
        return `${author.canonical} `;
      case "unknown-field":
        return author.canonical.replace('{"access":"author"', '{"actor":"user:secret","access":"author"');
      case "space-mismatch":
        return author.canonical.replace('"spaceId":"space-admin-01"', '"spaceId":"space-admin-02"');
      case "member-order-reversed":
        return JSON.stringify({ ...JSON.parse(author.canonical), members: { rows: [...memberRows(2)].reverse() } });
      case "invite-order-reversed":
        return JSON.stringify({ ...JSON.parse(author.canonical), invites: { rows: [...inviteRows(2)].reverse() } });
      case "window-row-max-plus-one":
        return JSON.stringify({ ...JSON.parse(author.canonical), members: { rows: memberRows(fixture.limits.windowRows + 1) } });
      case "page-byte-max-plus-one":
        return JSON.stringify({ ...JSON.parse(author.canonical), space: { ...(JSON.parse(author.canonical) as { space: Record<string, unknown> }).space, name: "x".repeat(fixture.limits.pageBytes) } });
      default:
        return JSON.stringify({ ...JSON.parse(author.canonical), invites: { rows: [{ ...inviteRows(1)[0], secretDigest: "ff".repeat(32) }] } });
    }
  };
  for (const name of fixture.hostiles) {
    const candidate = hostile(name);
    if (name === "window-row-max-plus-one") {
      const rows = (JSON.parse(candidate) as { members: { rows: unknown[] } }).members.rows.length;
      if (rows <= fixture.limits.windowRows) throw new Error("window max+1 hostile did not exceed the ceiling");
      continue;
    }
    if (name === "invite-secret-field" && !candidate.includes("secretDigest")) throw new Error("secret-shaped hostile lost its field");
    if (!canonicalRejects(candidate)) throw new Error(`space administration parser admitted hostile ${name}`);
  }

  const admitCursor = (query: string): string | null => {
    if (query.length === 0) return "";
    if (query.includes("&") || query.includes("%") || query.includes("+")) return null;
    const separator = query.indexOf("=");
    if (separator < 0) return null;
    const name = query.slice(0, separator);
    const value = query.slice(separator + 1);
    if (name !== "cursor" || value.length === 0 || value.length > fixture.limits.cursorBytes || !/^[A-Za-z0-9._-]+$/u.test(value)) return null;
    return value;
  };
  for (const cursorCase of fixture.cursorCases) {
    const admitted = admitCursor(cursorCase.query) !== null;
    const status = admitted ? 200 : 400;
    const reads = admitted ? 1 : 0;
    if (status !== cursorCase.status || reads !== cursorCase.reads) throw new Error(`space administration cursor admission differs for "${cursorCase.query}"`);
  }

  if (administrationMember({ ...fixture.members[0]!, unknown: true }) || hubSchemaExport(repoRoot, "schema://hub.directory/SpaceAdministrationSpaceV1")({ ...fixture.space, unknown: true }))
    throw new Error("space administration contracts admitted an unknown field");

  const validatePage = hubSchemaExport(repoRoot, "schema://os.directory/DirectorySpaceAdministrationPageV1");
  for (const vector of fixture.vectors) {
    const { page } = seal(vector.access, vector.expected.memberRows, vector.expected.inviteRows);
    if (!validatePage(page)) throw new Error(`component schema rejected the sealed ${vector.name} page`);
  }
  for (const name of ["unknown-field", "window-row-max-plus-one", "invite-secret-field"]) {
    if (validatePage(JSON.parse(hostile(name)))) throw new Error(`component schema admitted hostile ${name}`);
  }
  const memberShaped = JSON.parse(seal("member", 1, 0).canonical) as Record<string, unknown>;
  if (validatePage({ ...memberShaped, invites: { rows: [] } })) throw new Error("component schema admitted an invite window on a member page");
  if (validatePage({ ...memberShaped, capabilities: { renameSpace: true, setVisibility: true, deleteSpace: true, upsertMember: true, removeMember: true, createInvite: true, revokeInvite: true } }))
    throw new Error("component schema admitted capability flags on a member page");

  const contract = readModuleTreeSource(join(repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🧬️schema/🦀️.rs"));
  const typescript = readModuleTreeSource(join(repoRoot, "🧰️framework/🔨️modules/📇️directory/🧬️schema/🟦️.ts"));
  const hub = readHubBinSource(repoRoot);
  const sqlite = readDirectoryBackendSource(repoRoot, "🪶️sqlite");
  const postgres = readDirectoryBackendSource(repoRoot, "🐘️postgres");
  const neo4j = readNeo4jDirectorySource(repoRoot);
  const worker = readModuleTreeSource(join(repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/👷️worker/🟦️.ts"));
  const space = readFileSync(join(repoRoot, "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/🔎️explore/🪟️windows/🏠️main/🦀️.rs"), "utf8");
  const sourceClosed = (rust: string, ts: string, route: string, sq: string, pg: string, neo: string, browser: string, home: string): boolean => {
    const read = route.indexOf("list_space_administration_members_page(space_id, member_after.as_deref(), SPACE_ADMINISTRATION_PAGE_FETCH_MAX)");
    const revalidate = route.indexOf("revalidate_space_administration_caller(state, caller.as_ref(), space_id, binding, &space, access)", read);
    return (
      rust.includes("pub const DIRECTORY_SPACE_ADMINISTRATION_PAGE_MAX_ROWS: usize = 64") &&
      rust.includes("pub const DIRECTORY_SPACE_ADMINISTRATION_PAGE_MAX_BYTES: usize = 48 * 1024") &&
      rust.includes("pub enum DirectorySpaceAdministrationPageV1") &&
      !rust.includes("pub enum DirectorySpaceDetailV1") &&
      ts.includes("export async function parseDirectorySpaceAdministrationPageV1") &&
      ts.includes("space-administration-page.noncanonical") &&
      route.includes("fn space_administration_request_admission") &&
      route.includes("semio/hub/directory-space-administration/session-binding/v1\\0") &&
      route.includes('name != "cursor" || value.is_empty() || value.len() > DIRECTORY_SPACE_ADMINISTRATION_CURSOR_MAX_BYTES') &&
      read >= 0 &&
      revalidate > read &&
      route.includes("StatusCode::FORBIDDEN") &&
      !route.includes("DirectorySpaceDetailV1") &&
      sq.includes("SELECT u.id, u.email, u.display_name, m.role") &&
      !sq.includes("SELECT id, selector, secret_digest, space_id, role, created_at, expires_at, revoked_at, revoked_reason, accepted_at, accepted_event_id FROM hub_space_invite WHERE space_id = ?1 AND") &&
      pg.includes("async fn list_space_administration_members_page") &&
      pg.includes("async fn list_space_administration_invites_page") &&
      neo.includes("async fn list_space_administration_members_page") &&
      neo.includes("async fn list_space_administration_invites_page") &&
      browser.includes("function terminateDirectoryAdministration") &&
      browser.includes("const DIRECTORY_ADMINISTRATION_CAPACITY = 1") &&
      directoryAdministrationTerminateErases(browser) &&
      browser.includes("revokeDirectoryAdministrationForScope(scope.spaceId);") &&
      home.includes("row.role == Some(crate::DirectorySpaceRole::Author)")
    );
  };
  if (!sourceClosed(contract, typescript, hub, sqlite, postgres, neo4j, worker, space)) throw new Error("space administration source boundary is incomplete");
  const sourceHostiles: readonly [string, string, string, string, string, string, string, string][] = [
    [contract.replace("pub const DIRECTORY_SPACE_ADMINISTRATION_PAGE_MAX_ROWS: usize = 64", "pub const DIRECTORY_SPACE_ADMINISTRATION_PAGE_MAX_ROWS: usize = 4096"), typescript, hub, sqlite, postgres, neo4j, worker, space],
    [contract, typescript.replace("export async function parseDirectorySpaceAdministrationPageV1", "async function parseDirectorySpaceAdministrationPageV1"), hub, sqlite, postgres, neo4j, worker, space],
    [contract, typescript, hub.replace("revalidate_space_administration_caller(state, caller.as_ref(), space_id, binding, &space, access)", "Ok(access)"), sqlite, postgres, neo4j, worker, space],
    [contract, typescript, hub.replace('name != "cursor" || value.is_empty() || value.len() > DIRECTORY_SPACE_ADMINISTRATION_CURSOR_MAX_BYTES', "false"), sqlite, postgres, neo4j, worker, space],
    [contract, typescript, hub, sqlite, postgres.replace("async fn list_space_administration_invites_page", "async fn unused_invites_page"), neo4j, worker, space],
    [contract, typescript, hub, sqlite, postgres, neo4j.replace("async fn list_space_administration_members_page", "async fn unused_members_page"), worker, space],
    [contract, typescript, hub, sqlite, postgres, neo4j, worker.replace("  operation.inviteToken = null;\n", ""), space],
    [contract, typescript, hub, sqlite, postgres, neo4j, worker.replace("revokeDirectoryAdministrationForScope(scope.spaceId);", ""), space],
    [contract, typescript, hub, sqlite, postgres, neo4j, worker, space.replace("row.role == Some(crate::DirectorySpaceRole::Author)", "true")],
  ];
  sourceHostiles.forEach((candidate, index) => {
    if (sourceClosed(...candidate)) throw new Error(`space administration source oracle admitted removed fence ${index}`);
  });
  const checks = fixture.vectors.length * 2 + fixture.cursorCases.length + fixture.hostiles.length + sourceHostiles.length + 7;
  console.log(
    `space-administration-oracle: AJV=2 vectors=${fixture.vectors.length} cursors=${fixture.cursorCases.length} hostiles=${fixture.hostiles.length} source-hostiles=${sourceHostiles.length} component-schema=${fixture.vectors.length + 5} sha256=1 binding=1`,
  );
  return checks;
}

export class SpaceAdministrationCheckScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    const phase = segments[0] ?? "source";
    if (segments.length > 1 || !["source", "native"].includes(phase)) throw new Error("space-administration-check accepts source or native");
    const checks = await proveDirectorySpaceAdministrationPageV1(this.repoRoot);
    if (phase === "native") {
      const receipts = await runAdmittedExactCargoLawsV1({control:exactCargoCallerControlV1(this.invocation,"exact-cargo:caller",0),invocation: this.invocation,policy: { ...configuredExactCargoLawPolicyV1(), buildMilliseconds: buildBudgetMs(), listMilliseconds: 60_000, lawMilliseconds: 120_000 },cwd: this.root,...exactCargoStageEnvironments(),groups: [
          {
            package: "semio-hub-directory-route-laws",
            target: { kind: "lib" },
            cargoArgs: [],
            laws: ["space_administration_page_v1_route_returns_the_author_windows_with_a_canonical_receipt", "space_administration_page_v1_route_denies_a_spectator_the_author_windows", "space_administration_page_v1_route_denies_a_removed_member_and_leaks_no_rows", "space_administration_page_v1_route_rejects_a_noncanonical_query_and_a_foreign_cursor"],
          },
        ],progress(event) {
          console.log(`space-administration-${phase} ${event.stage}: ${event.package} ${event.law ?? ""} artifacts=${event.artifactDir}`);
        }});
      for (const receipt of receipts) console.log(`space-administration-${phase}-receipt: ${JSON.stringify(receipt)}`);
    }
    console.log(`space-administration-check: checks=${checks} phase=${phase}`);
  }
}

async function provePresenceLeaseFixture(repoRoot: string): Promise<number> {
  const root = join(repoRoot, "🌎️hub/🧪️tests/🧫️fixtures/👥️presence-lease-v1");
  const fixture = JSON.parse(readFileSync(join(root, "🔣️.json"), "utf8")) as PresenceLeaseFixture;
  const presenceOperation = hubSchemaExport(repoRoot, "schema://hub.directory/PresenceLeaseOperationV1");
  const presenceSummary = hubSchemaExport(repoRoot, "schema://hub.directory/PresenceScopeSummaryV1");
  if (fixture.schema !== "semio.hub.presence-lease/v1") throw new Error("presence lease fixture schema drift");
  if (Object.keys(fixture).sort().join(",") !== "limits,schema,vectors") throw new Error("presence lease fixture envelope drift");
  if (fixture.limits.ttlMs !== 15000 || fixture.limits.maximumItems !== 64 || fixture.limits.maximumEntryBytes !== 4096 || fixture.limits.maximumBytes !== 262144) throw new Error("presence lease bound drift");
  if (fixture.vectors.length < 8 || fixture.vectors.length > 16) throw new Error("presence lease vector inventory drift");
  if (new Set(fixture.vectors.map(({ name }) => name)).size !== fixture.vectors.length) throw new Error("presence lease fixture names are not unique");
  for (const vector of fixture.vectors) {
    for (const operation of vector.operations) if (!presenceOperation(operation)) throw new Error(`presence lease operation is not the owned lease-operation contract: ${vector.name}`);
    for (const summary of vector.expected.final) if (!presenceSummary(summary)) throw new Error(`presence lease roster is not the owned scope-summary contract: ${vector.name}`);
    if (vector.expected.durableWrites !== 0 || vector.expected.directoryRecipients.some((recipient) => recipient !== "member")) throw new Error(`presence lease projection escaped its ephemeral member fanout: ${vector.name}`);
    if (vector.expected.outcomes.some((outcome) => !["published", "no-change", "rejected", "restarted"].includes(outcome))) throw new Error(`presence lease outcome taxonomy drift: ${vector.name}`);
  }
  type Slot = { liveId: string; deadline: number; peerTag: string | null; peerBytes: number };
  for (const vector of fixture.vectors) {
    const slots = new Map<string, Slot>();
    const outcomes: string[] = [];
    let fanoutCount = 0;
    const roster = (scope: string) =>
      [...slots.entries()]
        .filter(([key, slot]) => key.startsWith(`${scope}\0`) && slot.peerTag !== null)
        .map(([key, slot]) => ({ actor: key.slice(scope.length + 1), bytes: slot.peerBytes }))
        .sort((left, right) => left.actor.localeCompare(right.actor));
    const publish = () => {
      fanoutCount += 1;
      outcomes.push("published");
    };
    for (const operation of vector.operations) {
      if (operation.kind === "restart") {
        slots.clear();
        outcomes.push("restarted");
        continue;
      }
      if (operation.kind === "fill") {
        for (let index = 0; index < operation.count; index += 1) {
          const suffix = String(index).padStart(3, "0");
          slots.set(`${operation.scope}\0actor-${suffix}`, { liveId: `fill-${suffix}`, deadline: operation.nowMs + fixture.limits.ttlMs, peerTag: `fill-${suffix}`, peerBytes: operation.peerBytes });
        }
        outcomes.push("no-change");
        continue;
      }
      const key = `${operation.scope}\0${operation.actor}`;
      const current = slots.get(key);
      if (operation.kind === "install") {
        slots.set(key, { liveId: operation.liveId, deadline: operation.nowMs + fixture.limits.ttlMs, peerTag: null, peerBytes: 0 });
        if (current?.peerTag !== null && current !== undefined) publish();
        else outcomes.push("no-change");
        continue;
      }
      if (!current || current.liveId !== operation.liveId) {
        outcomes.push("no-change");
        continue;
      }
      if (operation.kind === "refresh") {
        const visible = roster(operation.scope);
        const total = visible.reduce((sum, entry) => sum + entry.bytes, 0);
        const wasVisible = current.peerTag !== null;
        if (operation.peerBytes > fixture.limits.maximumEntryBytes || (!wasVisible && visible.length >= fixture.limits.maximumItems) || total - (wasVisible ? current.peerBytes : 0) + operation.peerBytes > fixture.limits.maximumBytes) {
          outcomes.push("rejected");
          continue;
        }
        const changed = current.peerTag !== operation.peerTag || current.peerBytes !== operation.peerBytes;
        current.deadline = operation.nowMs + fixture.limits.ttlMs;
        current.peerTag = operation.peerTag;
        current.peerBytes = operation.peerBytes;
        if (changed) publish();
        else outcomes.push("no-change");
      } else if (operation.kind === "tick") {
        if (current.peerTag !== null && operation.nowMs >= current.deadline) {
          current.peerTag = null;
          current.peerBytes = 0;
          publish();
        } else outcomes.push("no-change");
      } else {
        slots.delete(key);
        if (current.peerTag !== null) publish();
        else outcomes.push("no-change");
      }
    }
    const final = vector.expected.final.map((expected) => {
      const rows = roster(expected.scope);
      return { scope: expected.scope, count: rows.length, bytes: rows.reduce((sum, row) => sum + row.bytes, 0), actors: expected.actors.length === 0 && rows.length > 0 ? [] : rows.map((row) => row.actor) };
    });
    const actual = { outcomes, fanoutCount, final, durableWrites: 0 };
    const expected = { outcomes: vector.expected.outcomes, fanoutCount: vector.expected.fanoutCount, final: vector.expected.final, durableWrites: vector.expected.durableWrites };
    if (JSON.stringify(actual) !== JSON.stringify(expected)) throw new Error(`presence lease model differs for ${vector.name}: ${JSON.stringify(actual)}`);
  }
  const schemaHostiles: unknown[] = [
    { ...fixture.vectors[0]!.operations[0], clientDeadlineMs: 1 },
    { ...fixture.vectors[0]!.operations[0], kind: "forge" },
    { ...fixture.vectors[0]!.operations[0], nowMs: 1000001 },
  ];
  if (schemaHostiles.some((candidate) => presenceOperation(candidate))) throw new Error("presence lease operation contract admitted client authority or an unbounded deadline");
  const summaryHostiles: unknown[] = [
    { ...fixture.vectors[0]!.expected.final[0]!, actors: Array.from({ length: 65 }, (_, index) => `actor-${index}`) },
    { ...fixture.vectors[0]!.expected.final[0]!, bytes: 262145 },
    { ...fixture.vectors[0]!.expected.final[0]!, unknown: true },
  ];
  if (summaryHostiles.some((candidate) => presenceSummary(candidate))) throw new Error("presence lease scope-summary contract admitted an unbounded projection");
  const hub = readHubBinSource(repoRoot);
  const sourceClosed = (source: string): boolean => {
    const publish = source.indexOf("self.fanout_for(key).send(ServerFrame::Presence");
    const directory = source.indexOf("self.directory_service.publish(DirectoryStreamMessage::Presence", publish);
    return (
      source.includes("const PRESENCE_LEASE_TTL_MS: u64 = 15_000") &&
      source.includes("socket_live_id: String") &&
      source.includes("expires_at: tokio::time::Instant") &&
      source.includes("presence_publication_gate: Arc<tokio::sync::Mutex<()>>") &&
      source.includes("slot.socket_live_id == socket_live_id") &&
      source.includes("remove_if(&map_key, |slot| slot.socket_live_id == socket_live_id)") &&
      source.includes("rows.sort_by(|left, right| left.0.cmp(&right.0))") &&
      source.includes("peer.len() > PRESENCE_ROSTER_MAXIMUM_ENTRY_BYTES") &&
      source.includes("visible >= PRESENCE_ROSTER_MAXIMUM_ITEMS") &&
      source.includes("next > PRESENCE_ROSTER_MAXIMUM_BYTES") &&
      source.includes("now >= slot.expires_at") &&
      source.includes("state.install_presence_slot(") &&
      source.includes("self.refresh_presence(") &&
      source.includes("state.expire_presence_for_live(") &&
      source.includes("state.close_presence_for_live(") &&
      publish >= 0 &&
      directory > publish &&
      !source.includes("PresenceSession") &&
      !source.includes("connected_at_ms + PRESENCE_LEASE_TTL_MS")
    );
  };
  if (!sourceClosed(hub)) throw new Error("presence lease production ownership/publication fence is incomplete");
  const sourceHostiles = [
    hub.replace("const PRESENCE_LEASE_TTL_MS: u64 = 15_000", "const PRESENCE_LEASE_TTL_MS: u64 = 5_000"),
    hub.replaceAll("slot.socket_live_id == socket_live_id", "true"),
    hub.replace("remove_if(&map_key, |slot| slot.socket_live_id == socket_live_id)", "remove(&map_key)"),
    hub.replace("rows.sort_by(|left, right| left.0.cmp(&right.0));", ""),
    hub.replace("visible >= PRESENCE_ROSTER_MAXIMUM_ITEMS", "false"),
  ];
  sourceHostiles.forEach((source, index) => {
    if (sourceClosed(source)) throw new Error(`presence lease source oracle admitted removed fence ${index}`);
  });
  const shell = readModuleTreeSource(join(repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🟦️.tsx"));
  const helpers = readModuleTreeSource(join(repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🛠️ShellHelpers/🔩️plumbing/👥️presence-identity/🟦️.ts"));
  if (!helpers.includes("PRESENCE_HEARTBEAT_INTERVAL_MS = 5000") || !shell.includes("window.setInterval(beat, PRESENCE_HEARTBEAT_INTERVAL_MS)") || !shell.includes("window.clearInterval(timer)"))
    throw new Error("browser presence schedule is not a bounded five-second lifecycle");
  const checks = fixture.vectors.length + schemaHostiles.length + summaryHostiles.length + sourceHostiles.length + 1;
  console.log(`presence-lease-oracle: hub.directory-exports=2 vectors=${fixture.vectors.length} contract-hostiles=${schemaHostiles.length + summaryHostiles.length} source-hostiles=${sourceHostiles.length} browser-schedule=1`);
  return checks;
}

/** 🪪️ Pins canonical admitted presence bytes independently with AJV and third-party LEB128. */
async function provePresenceNormalizationFixture(repoRoot: string): Promise<number> {
  const root = join(repoRoot, "🌎️hub/🧪️tests/🧫️fixtures/🪪️presence-normalization-v1");
  const fixture = JSON.parse(readFileSync(join(root, "🔣️.json"), "utf8"));
  const presenceAdmission = hubSchemaExport(repoRoot, "schema://hub.directory/PresenceAdmissionV1");
  if (fixture.schema !== "semio.hub.presence-normalization/v1" || fixture.maximumEntryBytes !== 4096) throw new Error("presence normalization fixture schema/bound drift");
  if (Object.keys(fixture).sort().join(",") !== "maximumEntryBytes,schema,vectors") throw new Error("presence normalization fixture envelope drift");
  if (fixture.vectors.length < 12 || fixture.vectors.length > 32 || new Set(fixture.vectors.map((vector: { name: string }) => vector.name)).size !== fixture.vectors.length) throw new Error("presence normalization vector inventory drift");
  for (const vector of fixture.vectors) {
    if (!presenceAdmission(vector.admission)) throw new Error(`presence normalization admission is not the owned admission contract: ${vector.name}`);
    if (!/^(?:[0-9a-f]{2})*$/u.test(vector.rawPeerHex) || vector.rawPeerHex.length > 16384) throw new Error(`presence normalization raw peer bytes drift: ${vector.name}`);
    if (typeof vector.expected.accepted !== "boolean" || vector.expected.durableWrites !== 0) throw new Error(`presence normalization expectation drift: ${vector.name}`);
    if (vector.expected.normalizedPeerHex !== null && (!/^(?:[0-9a-f]{2})*$/u.test(vector.expected.normalizedPeerHex) || vector.expected.normalizedPeerHex.length > 16384)) throw new Error(`presence normalization normalized bytes drift: ${vector.name}`);
  }
  if (presenceAdmission({ ...fixture.vectors[0].admission, extra: true }) || presenceAdmission({ ...fixture.vectors[0].admission, role: "owner" }) || presenceAdmission({ ...fixture.vectors[0].admission, color: 256 }))
    throw new Error("presence normalization admission contract admitted a forged claim");
  const leb = unsignedLebOracleV1();
  const integer = (value: number): number[] => {
    if (!Number.isSafeInteger(value) || value < 0) throw new Error("neutral oracle integer outside the exact safe range");
    const bytes=Buffer.alloc(8); bytes.writeBigUInt64LE(BigInt(value));
    return Array.from(leb.encodeUnsignedBuffer(bytes));
  };
  const text = (value: string): number[] => {
    const bytes = Buffer.from(value);
    return [...integer(bytes.length), ...bytes];
  };
  const float = (value: number): number[] => {
    const bytes = Buffer.alloc(8);
    bytes.writeDoubleLE(value);
    return Array.from(bytes);
  };
  const independentEncode = (peer: ArtifactPresencePeer): Buffer => {
    const fields = [peer.label, peer.presencePack, peer.userId, peer.role, peer.dragGhostJson, peer.interaction, peer.color, peer.surface, peer.views.length ? peer.views : undefined, peer.ui, peer.toolRun, peer.principalKind, peer.activeTool, peer.historyEdit];
    const flags = fields.reduce<number>((mask, value, index) => (value === undefined ? mask : mask | (1 << index)), 0);
    const out = [...text(peer.actor), ...integer(flags), ...integer(peer.connectedAtMs)];
    for (const [index, value] of fields.entries()) {
      if (value === undefined) continue;
      if ([0, 2, 3, 4, 7, 12].includes(index)) out.push(...text(value as string));
      else if (index === 1) {
        const bytes = value as readonly number[];
        out.push(...integer(bytes.length), ...bytes);
      } else if (index === 5) {
        const interaction = peer.interaction!;
        out.push(...text(interaction.app_id), ...integer(interaction.domains.length));
        for (const domain of interaction.domains) {
          out.push(...text(domain.domain), ...text(domain.granularity));
          for (const ids of [domain.selected, domain.hovered]) {
            out.push(...integer(ids.length));
            for (const id of ids) out.push(...text(id));
          }
        }
      } else if (index === 6) out.push(value as number);
      else if (index === 8) {
        out.push(...integer(peer.views.length));
        for (const view of peer.views) {
          out.push(...text(view.windowId), ...text(view.space));
          const kind = view.kind;
          const values = kind.kind === "canvas" ? [0, kind.x, kind.y, kind.zoom] : kind.kind === "orbit" ? [1, ...kind.position, ...kind.target, ...kind.up, kind.fov] : [2, kind.lng, kind.lat, kind.zoom, kind.bearing, kind.pitch];
          out.push(values[0]!);
          for (const number of values.slice(1)) out.push(...float(number));
          out.push(...float(view.size[0]), ...float(view.size[1]), view.pointer ? 1 : 0);
          if (view.pointer) for (const number of view.pointer) out.push(...float(number));
          out.push(view.rayOrigin ? 1 : 0);
          if (view.rayOrigin) for (const number of view.rayOrigin) out.push(...float(number));
        }
      } else if (index === 9) {
        for (const path of [peer.ui!.hoveredPath, peer.ui!.focusedPath, peer.ui!.pressedPath]) {
          out.push(path === undefined ? 0 : 1);
          if (path !== undefined) out.push(...text(path));
        }
      } else if (index === 10) {
        const toolRun = peer.toolRun!;
        const states = ["starting", "running", "paused", "complete", "finalizing", "finalized", "aborting", "aborted", "faulted"];
        out.push(...text(toolRun.toolId), states.indexOf(toolRun.state), ...integer(toolRun.stage), ...integer(toolRun.completed), toolRun.total === undefined ? 0 : 1);
        if (toolRun.total !== undefined) out.push(...integer(toolRun.total));
      } else if (index === 11) out.push(PRESENCE_PRINCIPAL_KINDS.indexOf(value as ArtifactPresencePrincipalKind));
      else if (index === 13) {
        const historyEdit = peer.historyEdit!;
        out.push(...text(historyEdit.mutationId), ["editing", "replaying", "reviewing", "choosing", "finalizing"].indexOf(historyEdit.stage), ...integer(historyEdit.drafts));
      }
    }
    return Buffer.from(out);
  };
  for (const vector of fixture.vectors) {
    const raw = Buffer.from(vector.rawPeerHex, "hex");
    let normalized: Buffer | undefined;
    try {
      const input = decodePresencePeer(raw, [0]);
      if (!independentEncode(input).equals(raw)) throw new Error("raw canonical oracle mismatch");
      const admitted = vector.admission;
      const output: ArtifactPresencePeer = {
        actor: admitted.actor,
        connectedAtMs: admitted.connectedAtMs,
        label: admitted.label ?? undefined,
        userId: admitted.userId ?? undefined,
        role: admitted.role ?? undefined,
        color: admitted.color,
        surface: admitted.surface ?? undefined,
        presencePack: input.presencePack,
        dragGhostJson: input.dragGhostJson,
        interaction: input.interaction,
        views: input.views,
        ui: input.ui,
        toolRun: input.toolRun,
        principalKind: admitted.principalKind ?? "human",
        activeTool: input.activeTool,
        historyEdit: input.historyEdit,
      };
      normalized = independentEncode(output);
      if (!normalized.equals(Buffer.from(encodePresencePeer(output)))) throw new Error("output canonical oracle mismatch");
      decodePresencePeer(normalized, [0]);
    } catch {
      normalized = undefined;
    }
    if ((normalized !== undefined) !== vector.expected.accepted || (normalized?.toString("hex") ?? null) !== vector.expected.normalizedPeerHex) throw new Error(`presence normalization vector failed: ${vector.name}`);
  }
  console.log(`presence-normalization-independent-oracle: AJV=1 LEB128=1 exact-vectors=${fixture.vectors.length}`);
  const hub = readHubBinSource(repoRoot);
  const start = hub.indexOf("async fn refresh_document_presence(");
  const end = hub.indexOf("async fn refresh_presence(", start);
  const ingress = hub.slice(start, end);
  if (
    start < 0 ||
    !ingress.includes("protocol::decode_presence_peer(&peer).await") ||
    !ingress.includes("protocol::PresencePeer {") ||
    !ingress.includes("protocol::encode_presence_peer(&normalized).await") ||
    !ingress.includes("self.refresh_presence(")
  )
    throw new Error("Hub lacks canonical admitted presence reconstruction");
  for (const field of ["tool_run: input.tool_run", "history_edit: input.history_edit", "connected_at_ms: slot.connected_at_ms", "label: slot.label.clone()", "user_id: slot.user_id.clone()", "role: slot.role.clone()", "color: Some(slot.color)", "surface: slot.document_surface.clone()", "principal_kind: Some(slot.principal_kind)"])
    if (!ingress.includes(field)) throw new Error(`Hub presence authority missing: ${field}`);
  if (!hub.includes("state.refresh_document_presence(") || !hub.includes("socket_grant.document_plan.as_ref().map(|plan| plan.surface.surface_id.clone())")) throw new Error("presence ingress must use the admitted plan surface");
  return fixture.vectors.length;
}

export class PresenceNormalizationCheckScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    const phase = segments[0] ?? "source";
    if (segments.length > 1 || !["source", "native"].includes(phase)) throw new Error("presence-normalization-check accepts source or native");
    const checks = await provePresenceNormalizationFixture(this.repoRoot);
    if (phase === "native") {
      const receipts = await runAdmittedExactCargoLawsV1({control:exactCargoCallerControlV1(this.invocation,"exact-cargo:caller",0),invocation: this.invocation,policy: { ...configuredExactCargoLawPolicyV1(), buildMilliseconds: buildBudgetMs(), listMilliseconds: 60_000, lawMilliseconds: 60_000 },cwd: this.root,...exactCargoStageEnvironments(),groups: [
          {
            package: "semio-hub-presence-route-laws",
            target: { kind: "lib" },
            laws: ["presence_normalization_matches_neutral_authority_and_no_effect_rejections", "presence_normalization_socket_overwrites_identity_and_rejects_without_refresh", "presence_lease_reconnect_rejects_old_live_refresh_and_close", "presence_lease_expires_server_clocked_visibility_without_socket_close", "presence_lease_enforces_shared_roster_bounds_and_actor_order", "presence_lease_restart_is_empty_and_directory_presence_is_member_only"],
          },
        ],progress(event) {
          console.log(`presence-normalization-native ${event.stage}: ${event.package} ${event.law ?? ""} artifacts=${event.artifactDir}`);
        }});
      for (const receipt of receipts) console.log(`presence-normalization-native-receipt: ${JSON.stringify(receipt)}`);
    }
    console.log(`presence-normalization-check: checks=${checks} phase=${phase}`);
  }
}

/** 🛂️ Qualifies durable membership removal across live presence and selected target admission. */
export class AdminPresenceTargetRecoveryCheckScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    const phase = segments[0] ?? "source";
    if (segments.length > 1 || !["source", "native"].includes(phase)) throw new Error("admin-presence-target-recovery-check accepts source or native");
    const root = join(this.repoRoot, "🌎️hub/📦️packages/🦀️rust");
    const fixtureRoot = join(root, "🧪️tests/🧫️fixtures/🛂️admin-presence-target-recovery-v1");
    const fixture = JSON.parse(readFileSync(join(fixtureRoot, "🔣️.json"), "utf8"));
    if (fixture.schema !== "semio.hub.admin-presence-target-recovery/v1" || fixture.surfaceId !== "surface.test.editor") throw new Error("admin presence target recovery fixture schema drift");
    if (Object.keys(fixture).sort().join(",") !== "expected,members,remove,schema,scope,surfaceId") throw new Error("admin presence target recovery fixture envelope drift");
    if (!hubSchemaExport(this.repoRoot, "schema://hub.directory/DirectorySocketScopeV1")(fixture.scope)) throw new Error("admin presence target recovery scope is not the owned socket scope contract");
    if (fixture.members.length !== 2 || fixture.members.some((member: { id: string; role: string }) => member.role !== "author") || fixture.remove !== "removed") throw new Error("admin presence target recovery membership drift");
    const recoveryExpected = fixture.expected;
    if (Object.keys(recoveryExpected).sort().join(",") !== "members,observerStatus,removedCloseCode,removedCode,removedStatus,sqliteReopen,visibleAfterRemoval") throw new Error("admin presence target recovery expectation envelope drift");
    if (recoveryExpected.removedCloseCode !== 4401 || recoveryExpected.removedStatus !== 401 || recoveryExpected.removedCode !== "denied" || recoveryExpected.observerStatus !== 200 || recoveryExpected.visibleAfterRemoval !== 0 || recoveryExpected.sqliteReopen !== true)
      throw new Error("admin presence target recovery expectation drift");
    const members = fixture.members.map((member: { id: string }) => member.id).filter((id: string) => id !== fixture.remove);
    if (JSON.stringify(members) !== JSON.stringify(fixture.expected.members) || new Set(fixture.members.map((member: { id: string }) => member.id)).size !== 2) throw new Error("membership recovery oracle differs");
    console.log("admin-presence-target-recovery-independent-oracle: AJV=1 scoped-removal=1");
    const law = "admin_removal_revokes_visible_plan_presence_and_target_after_sqlite_reopen";
    const hub = readModuleTreeSource(join(root, "../../🏗️bootstrap/🦀️.rs"));
    if (!hub.includes(`fn ${law}(`) || !hub.includes("with_graceful_shutdown") || !hub.includes("test_state_with_directory")) throw new Error("missing composed SQLite shutdown/reopen acceptance journey");
    if (hub.match(/const STUDIO: &str = "([^"]+)";/)?.[1] !== fixture.scope.spaceId) throw new Error("recovery fixture differs from seeded test space");
    if (phase === "native") {
      const receipts = await runAdmittedExactCargoLawsV1({control:exactCargoCallerControlV1(this.invocation,"exact-cargo:caller",0),invocation: this.invocation,policy: { ...configuredExactCargoLawPolicyV1(), buildMilliseconds: buildBudgetMs(), listMilliseconds: 60_000, lawMilliseconds: 120_000 },cwd: this.repoRoot,...exactCargoStageEnvironments(),groups: hubLawGroups([["semio-hub-directory-route-laws", law]]),progress(event) {
          console.log(`admin-presence-target-recovery-native ${event.stage}: ${event.law ?? ""} artifacts=${event.artifactDir}`);
        }});
      for (const receipt of receipts) console.log(`admin-presence-target-recovery-native-receipt: ${JSON.stringify(receipt)}`);
    }
    console.log(`admin-presence-target-recovery-check: phase=${phase}`);
  }
}

export class PresenceLeaseCheckScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    const phase = segments[0] ?? "source";
    if (segments.length > 1 || !["source", "native", "process"].includes(phase)) throw new Error("presence-lease-check accepts source, native, or process");
    const checks = await provePresenceLeaseFixture(this.repoRoot);
    if (phase === "native" || phase === "process") {
      const hubLaws: [string, string][] = [["semio-hub-presence-route-laws", "presence_lease_reconnect_rejects_old_live_refresh_and_close"], ["semio-hub-presence-route-laws", "presence_lease_expires_server_clocked_visibility_without_socket_close"], ["semio-hub-presence-route-laws", "presence_lease_enforces_shared_roster_bounds_and_actor_order"], ["semio-hub-presence-route-laws", "presence_lease_restart_is_empty_and_directory_presence_is_member_only"]];
      const hubGroups = hubLawGroups(hubLaws, ["--all-features"]);
      const groups =
        phase === "native" ? [{ package: "semio-framework-os-kernel", target: { kind: "lib" as const, name: "semio_framework_os_kernel" }, laws: ["presence_roster_fixed_maximum_plus_one_returns_the_exact_rejected_owner"] }, ...hubGroups] : hubGroups;
      const receipts = await runAdmittedExactCargoLawsV1({control:exactCargoCallerControlV1(this.invocation,"exact-cargo:caller",0),invocation: this.invocation,policy: { ...configuredExactCargoLawPolicyV1(), buildMilliseconds: buildBudgetMs(), listMilliseconds: 60_000, lawMilliseconds: 60_000 },cwd: phase === "native" ? this.repoRoot : this.root,...exactCargoStageEnvironments(),groups,progress(event) {
          console.log(`presence-lease-${phase} ${event.stage}: ${event.package} ${event.law ?? ""} artifacts=${event.artifactDir}`);
        }});
      for (const receipt of receipts) console.log(`presence-lease-${phase}-receipt: ${JSON.stringify(receipt)}`);
    }
    console.log(`presence-lease-check: checks=${checks} phase=${phase}`);
  }
}
