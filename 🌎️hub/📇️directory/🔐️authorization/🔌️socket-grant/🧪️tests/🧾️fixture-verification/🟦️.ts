import { readFileSync } from "node:fs";
import { join } from "node:path";
import { hubSchemaExport } from "../../../../../🤝️integration-harness/🟦️.ts";
import { localRelayUpstreamPath } from "../../../../../🚀️local-relay/🧭️routing/🟦️.ts";
import { directorySocketGrantDecision, type DirectorySocketGrantVector } from "../../🧭️decision/🟦️.ts";

/** 🧾️ Verifies the scoped socket fixture against owned schemas, decisions, and relay admission. */
export async function proveScopedDirectorySocketRevocationFixture(repoRoot: string): Promise<void> {
  const root = join(repoRoot, "🌎️hub", "📇️directory", "🧫️fixtures", "🔌️scoped-socket-revocation-v1");
  const fixture = JSON.parse(readFileSync(join(root, "🔣️.json"), "utf8"));
  const scopeContract = hubSchemaExport(repoRoot, "schema://hub.directory/DirectorySocketScopeV1");
  const messageContract = hubSchemaExport(repoRoot, "schema://hub.directory/DirectorySocketMessageRoutingV1");
  if (fixture.schema !== "semio.hub.directory-scoped-socket-revocation/v1") throw new Error("scoped directory socket fixture schema drift");
  if (Object.keys(fixture).sort().join(",") !== "clientCloses,limits,schema,scope,vectors") throw new Error("scoped directory socket fixture envelope drift");
  if (fixture.limits.identifierBytes !== 4096 || fixture.limits.grantRequestBytes !== 256 || fixture.limits.authorizationDeadlineMs !== 2000) throw new Error("scoped directory socket bound drift");
  if (fixture.vectors.length < 12 || fixture.vectors.length > 32 || new Set(fixture.vectors.map((vector: { name: string }) => vector.name)).size !== fixture.vectors.length) throw new Error("scoped directory socket vector inventory drift");
  if (fixture.clientCloses.length !== 3 || new Set(fixture.clientCloses.map((close: { code: number | null }) => close.code)).size !== 3) throw new Error("scoped directory socket client-close inventory drift");
  if (!scopeContract(fixture.scope)) throw new Error("scoped directory socket fixture scope is not the owned socket scope contract");
  for (const vector of fixture.vectors) {
    if (!scopeContract(vector.grantScope) || !scopeContract(vector.urlScope)) throw new Error(`scoped directory socket vector scope is not the owned contract: ${vector.name}`);
    if (!messageContract(vector.message)) throw new Error(`scoped directory socket vector message is not the owned routing contract: ${vector.name}`);
    if (!["active", "unauthorized", "unavailable"].includes(vector.binding) || !["send", "removal", "neither"].includes(vector.gateWinner)) throw new Error(`scoped directory socket vector taxonomy drift: ${vector.name}`);
    if (!["deliver", "skip-unrelated", "close-unauthorized", "close-unavailable", "deny-before-upgrade"].includes(vector.expected)) throw new Error(`scoped directory socket expectation taxonomy drift: ${vector.name}`);
    const actual = directorySocketGrantDecision(vector as DirectorySocketGrantVector);
    const expected = { outcome: vector.expected, closeCode: vector.closeCode, cursorAdvance: vector.cursorAdvance, textFrames: vector.textFrames };
    if (JSON.stringify(actual) !== JSON.stringify(expected)) throw new Error(`scoped directory decision differs for ${vector.name}: ${JSON.stringify(actual)}`);
  }
  const hostile = [{ ...fixture.scope, spaceId: "x".repeat(129) }, { ...fixture.scope, documentId: "" }, { ...fixture.scope, extra: true }];
  if (hostile.some((candidate) => scopeContract(candidate))) throw new Error("scoped directory socket scope contract admitted a hostile boundary mutation");
  if (messageContract({ ...fixture.vectors[0].message, extra: true }) || messageContract({ ...fixture.vectors[0].message, class: "forged" })) throw new Error("scoped directory socket routing contract admitted a hostile message");
  for (const close of fixture.clientCloses) {
    const terminal = close.code === 4401;
    if (terminal !== close.terminal || close.reconnect === terminal) throw new Error(`scoped directory client close mismatch for ${close.code}`);
  }
  const relayPath = "/directory/spaces/space%2Fa/documents/document%20b/socket-grants";
  if (localRelayUpstreamPath("POST", new URL(`http://relay.invalid/_semio/hub${relayPath}`)) !== relayPath) throw new Error("scoped directory relay denied the exact bounded grant path");
  if (localRelayUpstreamPath("POST", new URL(`http://relay.invalid/_semio/hub${relayPath}?extra=1`)) !== undefined) throw new Error("scoped directory relay admitted an arbitrary query");
  if (localRelayUpstreamPath("GET", new URL(`http://relay.invalid/_semio/hub${relayPath}`)) !== undefined) throw new Error("scoped directory relay admitted the wrong method");
  console.log(`scoped-directory-socket-oracle: hub.directory-exports=2 decisions=${fixture.vectors.length} hostiles=${hostile.length} client-closes=${fixture.clientCloses.length} relay=3`);
}
