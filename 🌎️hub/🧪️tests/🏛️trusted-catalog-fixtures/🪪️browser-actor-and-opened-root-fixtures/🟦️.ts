/** 🪪️ The document browser actor identity, trusted browser actor catalog, native catalog receipt pair and trusted catalog opened root fixture proofs. */
import { createHash } from "node:crypto";
import { existsSync, readFileSync, readdirSync } from "node:fs";
import { join } from "node:path";
import { canonicalJson } from "@semio-tech/repo-lib/authoring";
import { hubSchemaExport } from "../../../📦️packages/🟦️typescript/🟦️.ts";
import { assertObservationExpectation, type ObservationExpectation } from "../../../🧪️testing/🛂️expectation/🟦️.ts";
import { trustedBootstrapBrowserActorEncoding, trustedBootstrapBrowserActorV1 } from "../../../🏗️bootstrap/🧾️provenance/🧮️bootstrap-claims/🟦️.ts";
import { proveBrowserDocumentOpenFixture } from "../../📄️browser-document-open/🟦️.ts";
import { proveExecutionTargetLeaseCorpus } from "../../🔗️execution-target-lease-checks/🟦️.ts";
import { proveDocumentOpenPlanFixture } from "../../🗺️open-plan-checks/🟦️.ts";
import { webSha256Hex } from "../../../🚀️local-relay/🛰️servers/🟦️.ts";
import { moduleRustSource, readHubBinSource } from "../../../🧪️testing/📖️source-readers/🟦️.ts";

/** 🧬️ Independently validates the exact closed stdio+GIS profile and full-generation framing. */
export async function proveDocumentBrowserActorIdentityFixture(repoRoot: string): Promise<void> {
  const integerRoot = join(repoRoot, "🧰️framework/🔨️modules/🌱️value/🔁️codec/🧫️fixtures");
  const integerFixture = JSON.parse(readFileSync(join(integerRoot, "🔣️.json"), "utf8"));
  let integerAccepted = 0;
  for (const target of integerFixture.targets) {
    if (target.name !== `${target.signed ? "i" : "u"}${target.bits}`) throw new Error("integer target policy mismatch");
    const limit = 1n << BigInt(target.bits - Number(target.signed));
    const minimum = target.signed ? -limit : 0n,
      maximum = limit - 1n;
    for (const raw of integerFixture.raw) {
      JSON.parse(raw);
      const integer = /^-?(?:0|[1-9][0-9]*)$/u.test(raw) ? BigInt(raw) : undefined;
      if (integer !== undefined && integer >= minimum && integer <= maximum) integerAccepted++;
    }
  }
  console.log(`exact-integer-value-oracle: targets=${integerFixture.targets.length} raw=${integerFixture.raw.length} admitted=${integerAccepted} arithmetic=BigInt; native production parity is a separate exact group`);
  const root = join(repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🧬️schema/🌐️browser-actor");
  const fixture = JSON.parse(readFileSync(join(root, "🧪️tests/🧫️fixtures/🔣️.json"), "utf8"));
  const validators = {
    plan: hubSchemaExport(repoRoot, "schema://os.directory/DocumentBrowserActorPlan"),
    lease: hubSchemaExport(repoRoot, "schema://os.directory/DocumentBrowserActorLease"),
  };
  const contract = await import("@semio-tech/framework/directory/schema/browser-actor");
  const source = { componentSha256: fixture.componentSha256, descriptorByteSha256: fixture.descriptorByteSha256 };
  const parsers = { plan: contract.parseDocumentOpenBrowserActorV1, lease: contract.parseDocumentExecutionTargetBrowserActorV1 };
  for (const law of fixture.cases) {
    const candidate = law.kind === "none" ? { kind: "none" } : { ...structuredClone(fixture.closed), ...(law.view === "lease" ? { byteLength: fixture.byteLength } : {}) };
    Object.assign(candidate, law.set);
    if (law.remove) delete candidate[law.remove];
    const shape = Boolean(validators[law.view as "plan" | "lease"](candidate));
    const oracle =
      shape &&
      (candidate.kind === "none"
        ? ["react", "wgpu"].includes(law.renderer)
        : law.renderer === "wasm" &&
          candidate.sourceComponentSha256 === fixture.componentSha256 &&
          candidate.sourceDescriptorByteSha256 === fixture.descriptorByteSha256 &&
          JSON.stringify(candidate.importInterfaces) === JSON.stringify([...new Set(candidate.importInterfaces)].sort()));
    if (shape !== law.schemaAccepted || oracle !== law.accepted) throw new Error(`browser actor independent oracle mismatch: ${law.id}`);
    let parsed: ReturnType<typeof contract.parseDocumentOpenBrowserActorV1> | undefined;
    try {
      parsed = parsers[law.view as "plan" | "lease"](candidate, source, law.renderer);
    } catch {}
    if (Boolean(parsed) !== oracle) throw new Error(`browser actor production admission mismatch: ${law.id}`);
    if (parsed) {
      if (canonicalJson(parsed) !== canonicalJson(candidate) || !Object.isFrozen(parsed)) throw new Error(`browser actor projection mismatch: ${law.id}`);
      if (parsed.kind === "closed-browser-actor" && (!Object.isFrozen(parsed.importInterfaces) || parsed.importInterfaces === candidate.importInterfaces)) throw new Error(`browser actor borrowed mutable interfaces: ${law.id}`);
      if (!contract.sameDocumentBrowserActorV1(parsed, parsers[law.view as "plan" | "lease"](structuredClone(candidate), source, law.renderer))) throw new Error(`browser actor self equality mismatch: ${law.id}`);
    }
  }
  const plan = contract.parseDocumentOpenBrowserActorV1(fixture.closed, source, "wasm");
  const lease = contract.documentBrowserActorLeaseFromPlanV1(plan, source, "wasm", fixture.byteLength);
  if (canonicalJson(lease) !== canonicalJson({ ...fixture.closed, byteLength: fixture.byteLength })) throw new Error("browser actor lease projection mismatch");
  for (const field of Object.keys(lease)) {
    const changed = structuredClone(lease) as Record<string, any>;
    changed[field] = field === "importInterfaces" ? [] : field === "byteLength" ? fixture.byteLength + 1 : "different";
    if (contract.sameDocumentBrowserActorV1(lease, changed as any)) throw new Error(`browser actor equality omitted ${field}`);
  }
  if (contract.sameDocumentBrowserActorV1(plan, lease)) throw new Error("browser actor equality omitted lease length presence");
  const none = contract.parseDocumentOpenBrowserActorV1({ kind: "none" }, source, "react");
  if (!contract.sameDocumentBrowserActorV1(none, contract.documentBrowserActorLeaseFromPlanV1(none, source, "react", undefined))) throw new Error("browser actor none projection mismatch");
  let denied = 0;
  for (const [actor, length] of [
    [none, 1],
    [plan, undefined],
    [plan, 0],
    [plan, 67108865],
  ] as const) {
    try {
      contract.documentBrowserActorLeaseFromPlanV1(actor, source, actor.kind === "none" ? "react" : "wasm", length);
    } catch {
      denied++;
    }
  }
  let accessorReads = 0;
  for (const candidate of [
    undefined,
    null,
    [],
    Object.create(fixture.closed),
    {
      ...fixture.closed,
      get sha256() {
        accessorReads++;
        return fixture.closed.sha256;
      },
    },
  ]) {
    try {
      contract.parseDocumentOpenBrowserActorV1(candidate, source, "wasm");
    } catch {
      denied++;
    }
  }
  if (denied !== 9 || accessorReads !== 0) throw new Error("browser actor exact data ownership mismatch");
  for (const field of ["sourceComponentSha256", "sourceDescriptorByteSha256"]) {
    try {
      contract.documentBrowserActorLeaseFromPlanV1({ ...fixture.closed, [field]: "e".repeat(64) }, source, "wasm", fixture.byteLength);
    } catch {
      denied++;
    }
  }
  try {
    contract.documentBrowserActorLeaseFromPlanV1(plan, source, "react", fixture.byteLength);
  } catch {
    denied++;
  }
  try {
    contract.documentBrowserActorLeaseFromPlanV1(none, source, "wasm", undefined);
  } catch {
    denied++;
  }
  if (denied !== 13) throw new Error("browser actor projection bypassed package or renderer binding");
  console.log(`document-browser-actor-identity: AJV=2 cases=${fixture.cases.length} projection=2 equality=${Object.keys(lease).length + 1} ownership=9 projection-binding=4; metadata only, no catalog/activation claim`);
  await proveDocumentOpenPlanFixture(repoRoot);
  await proveBrowserDocumentOpenFixture(repoRoot);
  await proveExecutionTargetLeaseCorpus(repoRoot);
  await proveTrustedBrowserActorCatalogFixture(repoRoot);
}

/** 🧪️ Exercises the private catalog actor against the scope-owned closed actor contract. */
async function proveTrustedBrowserActorCatalogFixture(repoRoot: string): Promise<void> {
  const assert: (typeof import("node:assert"))["strict"] = (await import("node:assert/strict")).default;
  const root = join(repoRoot, "🌎️hub/🗿️artifact-authority/🔏️trusted-catalog");
  const fixture = JSON.parse(readFileSync(join(root, "🧪️tests/🧫️fixtures/🌐️browser-actor/🔣️.json"), "utf8"));
  const validate = hubSchemaExport(repoRoot, "schema://hub.artifact-authority.trusted-catalog/TrustedBundleBrowserActorV1");
  assert.deepEqual(Object.keys(fixture), ["schema", "bodyHex", "encodingSha256", "noneEncodingSha256", "closed", "rawLengths", "loadCases", "cases"]);
  assert.equal(fixture.schema, "semio.hub.trusted-browser-actor-fixture.v1");
  assert.match(fixture.bodyHex, /^(?:[0-9a-f]{2})+$/u);
  for (const key of ["encodingSha256", "noneEncodingSha256"] as const) assert.match(fixture[key], /^[0-9a-f]{64}$/u);
  assert(validate(fixture.closed), "closed catalog actor must satisfy its own scope-owned contract");
  assert.equal(fixture.cases.length, 26);
  assert.equal(fixture.loadCases.length, 8);
  assert.equal(fixture.rawLengths.length, 8);
  assert.equal(new Set(fixture.cases.map((row: any) => row.id)).size, fixture.cases.length);
  assert.equal(new Set(fixture.loadCases.map((row: any) => row.id)).size, fixture.loadCases.length);
  assert.equal(new Set(fixture.rawLengths.map((row: any) => row.token)).size, fixture.rawLengths.length);
  assert.deepEqual([...new Set(fixture.cases.map((row: any) => row.kind))].sort(), ["closed-browser-actor", "none"]);
  assert.deepEqual([...new Set(fixture.cases.map((row: any) => row.renderer))].sort(), ["react", "wasm", "wgpu"]);
  const source = { componentSha256: fixture.closed.sourceComponentSha256, descriptorByteSha256: fixture.closed.sourceDescriptorByteSha256 };
  for (const law of fixture.cases) {
    const actor = law.kind === "none" ? { kind: "none" } : structuredClone(fixture.closed);
    Object.assign(actor, law.set);
    if (law.remove) delete actor[law.remove];
    const shape = Boolean(validate(actor));
    assertObservationExpectation(law.id, law.expectation as ObservationExpectation, shape);
    const oracle =
      shape &&
      (actor.kind === "none"
        ? law.renderer !== "wasm"
        : law.renderer === "wasm" &&
          actor.sourceComponentSha256 === source.componentSha256 &&
          actor.sourceDescriptorByteSha256 === source.descriptorByteSha256 &&
          JSON.stringify(actor.importInterfaces) === JSON.stringify([...new Set(actor.importInterfaces)].sort()));
    assert.equal(oracle, law.accepted, `${law.id} oracle`);
    let encoded: Buffer | undefined;
    try {
      encoded = trustedBootstrapBrowserActorEncoding(actor, source, law.renderer);
    } catch {}
    assert.equal(Boolean(encoded), oracle, `${law.id} production`);
  }
  const bytes = Buffer.from(fixture.bodyHex, "hex");
  assert.equal(bytes.byteLength, fixture.closed.byteLength);
  assert.equal(await webSha256Hex(bytes), fixture.closed.sha256);
  for (const [actor, renderer, expected] of [
    [fixture.closed, "wasm", fixture.encodingSha256],
    [{ kind: "none" }, "react", fixture.noneEncodingSha256],
  ] as const) {
    const encoded = trustedBootstrapBrowserActorEncoding(actor, source, renderer);
    assert.equal(await webSha256Hex(encoded), expected);
    assert.equal(createHash("sha256").update(encoded).digest("hex"), expected);
  }
  for (const law of fixture.loadCases) {
    const body = law.bodyHex === null ? undefined : Buffer.from(law.bodyHex, "hex");
    const oracle = body !== undefined && body.byteLength === law.byteLength && body.byteLength > 0 && await webSha256Hex(body) === fixture.closed.sha256 && !law.cancelAfterDescriptor;
    assert.equal(oracle, law.accepted, `${law.id} body oracle`);
  }
  for (const law of fixture.rawLengths) {
    const actor = { ...fixture.closed, byteLength: JSON.parse(law.token) };
    assertObservationExpectation(`raw actor length ${law.token}`, law.expectation as ObservationExpectation, Boolean(validate(actor)));
    let accepted = false;
    try {
      trustedBootstrapBrowserActorV1(actor, source, "wasm");
      accepted = true;
    } catch {}
    assert.equal(accepted, law.accepted, `raw actor length ${law.token} production`);
  }
  console.log(`trusted-browser-actor-catalog: scope-export=TrustedBundleBrowserActorV1 cases=${fixture.cases.length} bodies=${fixture.loadCases.length} raw-lengths=${fixture.rawLengths.length} WebCrypto=1; metadata/framing oracle only, no native loader/activation claim`);
}

/** 🪢 Verifies independent kind coverage and the complete schema-qualified receipt roster. */
async function proveNativeCatalogReceiptPairs(repoRoot: string): Promise<void> {
  const assert: (typeof import("node:assert"))["strict"] = (await import("node:assert/strict")).default;
  const root = join(repoRoot, "🌎️hub/🧩️compositions/🗄️stdio/📇️catalog");
  const fixture = JSON.parse(readFileSync(join(root, "🧪️tests/🧫️fixtures/📇️native-catalog-surface/🧪️receipt-pairs.json"), "utf8"));
  const full = JSON.parse(readFileSync(join(root, "🧪️tests/🧫️fixtures/📇️native-catalog-surface/🧪️full-roster.json"), "utf8"));
  const Ajv2020 = (await import("ajv/dist/2020.js")).default;
  const ajv = new Ajv2020({ strict: true, allErrors: true });
  const triples = full.identities as string[][];
  const expectedTriples = new Set(triples.map((row) => JSON.stringify(row)));
  for (const row of full.cases) {
    const hostile = structuredClone(full);
    switch (row.mutation) {
      case "none": break;
      case "reverse": hostile.identities.reverse(); break;
      case "missing": hostile.identities.splice(17, 1); break;
      case "duplicate": hostile.identities[17] = [...hostile.identities[16]]; break;
      case "extra": hostile.identities.push(["s.stdio.pdf", "foreign.schema", "foreign.factory"]); break;
      case "schema": hostile.identities[17][1] = "foreign.schema"; break;
      case "factory": hostile.identities[17][2] = "foreign.factory"; break;
      default: throw new Error(`unknown full catalog mutation ${row.mutation}`);
    }
    const keys = new Set(hostile.identities.map((identity: string[]) => JSON.stringify(identity)));
    const accepted = keys.size === 30 && hostile.identities.length === 30 && [...keys].every((key) => expectedTriples.has(key as string));
    assert.equal(accepted, row.accepted, `${row.id} full owned identity roster`);

  }
  const FullDatabase = (await import("bun:sqlite")).Database;
  const fullCounts = new FullDatabase(":memory:");
  try {
    fullCounts.exec("CREATE TABLE codec_identity(kind TEXT NOT NULL, schema TEXT NOT NULL, factory TEXT NOT NULL)");
    const insert = fullCounts.prepare("INSERT INTO codec_identity VALUES (?, ?, ?)");
    for (const identity of triples) insert.run(identity[0]!, identity[1]!, identity[2]!);
    assert.deepEqual(fullCounts.query("SELECT count(*) AS rows, count(DISTINCT kind) AS kinds, count(DISTINCT factory) AS factories FROM codec_identity").get(), { rows: 30, kinds: 29, factories: 30 });
    assert.deepEqual(fullCounts.query("SELECT count(*) AS pairs FROM (SELECT DISTINCT kind, schema FROM codec_identity)").get(), { pairs: 30 });
  } finally {
    fullCounts.close();
  }
  const selection = readFileSync(join(root, "../🤖️generated/🧩️composition/🦀️.rs"), "utf8").split("\n").find((line) => line.startsWith("fn selected_contributions()"))!;
  const owners = new Set([...selection.matchAll(/semio_s_artifact_stdio_(\w+)::contribution/g)].map((match) => match[1]!));
  assert.equal(owners.size, full.artifactAssemblies);
  const artifacts = join(repoRoot, "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts");
  const actualTriples: string[][] = [];
  const ledger = { declared: { codecs: 0, mutations: 0, inferences: 0 }, registered: { codecs: 0, mutations: 0, inferences: 0 }, implemented: { codecs: 0, mutations: 0, inferences: 0 }, verified: { codecs: 0, mutations: 0, inferences: 0 } };
  let definitions = 0;
  for (const directory of readdirSync(artifacts)) {
    const path = join(artifacts, directory, "📜️artifact-definition.json");
    if (!existsSync(path)) continue;
    const definition = JSON.parse(readFileSync(path, "utf8"));
    if (!owners.has(definition.artifact)) continue;
    definitions++;
    for (const category of ["codecs", "mutations", "inferences"] as const) {
      for (const capability of definition[category]) {
        ledger.declared[category]++;
        if (capability.executable_registration) ledger.registered[category]++;
        if (capability.status === "implemented") ledger.implemented[category]++;
        if (capability.status === "verified") ledger.verified[category]++;
        if (category === "codecs" && capability.native_factory) {
          const binding = capability.native_factory;
          actualTriples.push([binding.artifact_kind, binding.artifact_schema, binding.factory_id]);
        }
      }
    }
  }
  assert.equal(definitions, full.artifactAssemblies);
  assert.deepEqual(ledger, full.ledger, "actual authored capability declarations retain exact independent ledger");
  assert.deepEqual([...actualTriples].sort(), [...triples].sort(), "actual selected definition factory identities equal the closed full roster");
  console.log(`native-catalog-full-roster: schemas=30 kinds=29 factories=30 declared=36 cases=${full.cases.length} independent-sqlite=30/29/30`);
  const nativeLaw = readFileSync(join(root, "../🔌️plugin/📇️catalog/🧪️tests/🔬️unit/🦀️.rs"), "utf8");
  assert(nativeLaw.includes('fixture["identities"]') && nativeLaw.includes('fixture["artifactKinds"]'), "original native count law must retain exact schema-qualified roster and unique kinds");
  const expected = new Set(fixture.pairs.map((pair: string[]) => JSON.stringify(pair)));
  const { Database } = await import("bun:sqlite");
  const counts = new Database(":memory:");
  try {
    counts.exec("CREATE TABLE codec_identity(kind TEXT NOT NULL, schema TEXT NOT NULL)");
    const insert = counts.prepare("INSERT INTO codec_identity VALUES (?, ?)");
    for (const pair of fixture.pairs) insert.run(pair[0], pair[1]);
    const independent = counts.query("SELECT (SELECT count(*) FROM (SELECT DISTINCT kind, schema FROM codec_identity)) AS codecIdentities, count(DISTINCT kind) AS artifactKinds FROM codec_identity").get();
    assert.deepEqual(independent, fixture.identityCounts, "independent SQLite identity projections");
    assert.equal(expected.size, fixture.identityCounts.codecIdentities, "owned schema-qualified identity count");
    assert.equal(new Set(fixture.pairs.map((pair: string[]) => pair[0])).size, fixture.identityCounts.artifactKinds, "owned artifact-kind count");
  } finally {
    counts.close();
  }
  for (const row of fixture.cases) {
    const pairs: string[][] = fixture.pairs.map((pair: string[]) => [...pair]);
    switch (row.mutation) {
      case "none": break;
      case "reverse": pairs.reverse(); break;
      case "missing-schema": pairs.splice(1, 1); break;
      case "missing-kind": pairs.pop(); break;
      case "duplicate": pairs[1] = [...pairs[0]!]; break;
      case "foreign-schema": pairs[1]![1] = "foreign.schema"; break;
      case "foreign-kind": pairs[1]![0] = "foreign.artifact"; break;
      case "extra": pairs.push(["s.stdio.pdf", "foreign.schema"]); break;
      default: throw new Error(`unknown receipt pair mutation ${row.mutation}`);
    }
    const keys = new Set(pairs.map((pair) => JSON.stringify(pair)));
    const kinds = [...new Set(pairs.map((pair) => pair[0]!))].sort();
    const accepted = keys.size === pairs.length && keys.size === expected.size && [...keys].every((key) => expected.has(key)) && JSON.stringify(kinds) === JSON.stringify([...fixture.kinds].sort());
    assert.equal(accepted, row.accepted, `${row.id} owned pair roster`);
  }
  console.log(`native-catalog-receipt-pairs: cases=${fixture.cases.length} schemas=${fixture.pairs.length} kinds=${fixture.kinds.length} independent-sqlite-identities=${fixture.identityCounts.codecIdentities}/${fixture.identityCounts.artifactKinds}`);
}

export async function proveTrustedCatalogOpenedRootFixture(repoRoot: string): Promise<void> {
  await proveNativeCatalogReceiptPairs(repoRoot);
  const assert: (typeof import("node:assert"))["strict"] = (await import("node:assert/strict")).default;
  const root = join(repoRoot, "🌎️hub/🗿️artifact-authority/🔏️trusted-catalog");
  const fixtureRoot = join(root, "🧪️tests/🧫️fixtures/🛡️opened-root");
  const fixture = JSON.parse(readFileSync(join(fixtureRoot, "🔣️.json"), "utf8"));
  const validatePointer = hubSchemaExport(repoRoot, "schema://hub.artifact-authority.trusted-catalog/TrustedCatalogCurrentPointerV1");
  const validateRelativePath = hubSchemaExport(repoRoot, "schema://hub.artifact-authority.trusted-catalog/TrustedCatalogRelativePathV1");
  assert.deepEqual(Object.keys(fixture), ["schema", "limits", "current", "relativePaths", "denials", "openedHandle"]);
  assert.equal(fixture.schema, "semio.hub.trusted-catalog-opened-root/v1");
  assert.deepEqual(fixture.limits, { pathBytes: 1024, pathSegments: 64, chunkBytes: 65536, pointerBytes: 65536 });
  assert(validatePointer(fixture.current), "opened-root current pointer must satisfy its scope-owned contract");
  assert.deepEqual(Object.keys(fixture.openedHandle), ["initialHex", "replacementHex", "maximum", "expectedHex", "providerCalls", "codecPublished"]);
  for (const key of ["initialHex", "replacementHex", "expectedHex"] as const) assert.match(fixture.openedHandle[key], /^(?:[0-9a-f]{2})+$/u);
  assert(Number.isInteger(fixture.openedHandle.maximum) && fixture.openedHandle.maximum >= 1 && fixture.openedHandle.maximum <= 67_108_864);
  assert.equal(fixture.openedHandle.providerCalls, 0);
  assert.equal(fixture.openedHandle.codecPublished, false);
  for (const row of fixture.relativePaths) assert.equal(validateRelativePath(row.value), row.accepted, `${row.id} contract`);
  const acceptedPath = (value: string): boolean => {
    const bytes = new TextEncoder().encode(value).byteLength;
    const segments = value.split("/");
    return (
      bytes > 0 &&
      bytes <= fixture.limits.pathBytes &&
      segments.length <= fixture.limits.pathSegments &&
      !value.includes("\\") &&
      !value.includes("\0") &&
      !value.startsWith("/") &&
      segments.every((segment: string) => segment.length > 0 && segment !== "." && segment !== "..")
    );
  };
  assert.equal(new Set(fixture.relativePaths.map((row: any) => row.id)).size, fixture.relativePaths.length);
  assert(fixture.relativePaths.length >= 12 && fixture.relativePaths.length <= 32);
  for (const row of fixture.relativePaths) assert.equal(acceptedPath(row.value), row.accepted, row.id);
  const pointer = Buffer.from(`${JSON.stringify(fixture.current)}\n`, "utf8");
  assert(pointer.byteLength > 0 && pointer.byteLength <= fixture.limits.pointerBytes);
  assert.deepEqual(JSON.parse(pointer.toString("utf8")), fixture.current);
  assert.equal(Buffer.from(fixture.openedHandle.initialHex, "hex").toString("hex"), fixture.openedHandle.expectedHex);
  assert.notEqual(fixture.openedHandle.initialHex, fixture.openedHandle.replacementHex);
  assert.deepEqual(
    fixture.denials.map((row: any) => row.kind),
    ["leaf-link", "intermediate-link", "actor-link", "data-root-link", "current-link", "generation-link", "reparse-point"],
  );
  assert(fixture.denials.every((row: any) => row.providerCalls === 0 && row.codecPublished === false));
  const source = moduleRustSource(root);
  const startup = readHubBinSource(repoRoot);
  const required = [
    "TrustedCatalogDataRoot",
    "TrustedCatalogGenerationRoot",
    "TrustedCatalogRelativePathV1",
    "TrustedCatalogOpenedFile",
    "open_server_owned",
    "open_current",
    "open_regular",
    "read_regular",
    "O_NOFOLLOW",
    "O_NONBLOCK",
    "fstat",
    "trusted_catalog_opened_root_rejects_linked_roots_leaves_intermediates_and_actors",
    "trusted_catalog_opened_handle_is_swap_stable_bounded_and_cancel_safe",
    "trusted_catalog_relative_paths_match_the_neutral_no_link_corpus",
  ];
  assert(
    required.every((needle) => source.includes(needle)),
    "trusted catalog lacks the opened-root/no-link owner or exact laws",
  );
  assert(startup.includes("configured_artifact_authority(&data_dir") && !startup.includes("OS_HUB_TRUSTED_CATALOG_BUNDLE") && !startup.includes("OS_HUB_TRUSTED_CATALOG_PROFILE"), "Hub startup still accepts ambient trusted bundle/profile paths");
  console.log(`trusted-catalog-opened-root: scope-exports=2 paths=${fixture.relativePaths.length} denials=${fixture.denials.length} same-handle=1 source=${required.length} startup=no-ambient-path; source contract only, native platform laws require --native`);
}
