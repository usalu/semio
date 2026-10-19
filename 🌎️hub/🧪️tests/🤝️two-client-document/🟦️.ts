import type {LocalProfile} from "../../🚀️local-bootstrap/🛂authentication/🟦️.ts";
import {receiveScriptProcessInvocation,scriptInvocationEnvironment} from "@semio-tech/process/routing";
import {scriptCompilerAdmissionControlV1} from "@semio-tech/process/compiler-capacity";
import {captureOwnedCommandResultV1} from "@semio-tech/process/owned-execution";
import { withBuiltLocalHubHostV1, withLocalHubHostTaskV1, startLocalHub, waitForReadiness, TRUSTED_CATALOG_READINESS_STALL_BOUND_MS } from "../../🚀️local-bootstrap/🏃️execution/🟦️.ts";
import { withMaterializedTrustedCatalogBundleV1 } from "../../🏗️bootstrap/🧾️provenance/🚚️bootstrap-publication/🧱️materialized-bundle/🟦️.ts";
import {trustedBootstrapSelectPackages} from "../../🏗️bootstrap/🧾️provenance/🧭️bootstrap-selection/🟦️.ts";
// #region Header
/**
 * 🤝️ Two-client document collaboration e2e (language-agnostic fixture).
 * Gated by HUB_E2E=1. PR1 socket chain + HC1 catalog clone + note creation. The hubs run with
 * `SEMIO_TRACE_LEVEL=info` on the default stderr sink; after the first hub's graceful exit every trace line must validate
 * against the trace record schema (Ajv) and name the document-socket, directory-command, presence,
 * catalog and shutdown events the run caused.
 */
// #endregion Header
import Ajv from "ajv";
import { existsSync, mkdirSync, readdirSync, readFileSync, writeFileSync } from "node:fs";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";
import { describe, expect, it } from "vitest";
import { hubRoot, pick, repoRoot } from "./🧭️workspace-paths/🟦️.ts";
import { prepareScenarioHarness, prepareSocketHelpers } from "./🧰️scenario-harness/🟦️.ts";
import { createSpaceAndArtifact } from "./🌱️space-and-artifact-setup/🟦️.ts";
import { runFirstCommandAndLateJoin } from "./🔌️sockets-presence-and-commands/🟦️.ts";
import { runAgentDelegationAndRevocation, runPresenceExpiryAndGrowth } from "./🤖️agent-delegation-and-resume/🟦️.ts";
import { runCrashRecovery, runGracefulShutdown } from "./🛑️shutdown-and-crash-recovery/🟦️.ts";

const HUB_E2E = process.env.HUB_E2E === "1";
const TEST_TIMEOUT_MS = 2_400_000;

describe("two-client document collaboration fixture", () => {
  it("loads the language-agnostic scenario fixture", () => {
    const fixtures = join(hubRoot, "🧪️tests", "🧫️fixtures");
    const fixtureDir = pick(fixtures, (n) => n.includes("two-client-document"));
    const fixture = JSON.parse(readFileSync(join(fixtureDir, "🔣️.json"), "utf8"));
    expect(fixture.schema).toBe("semio.hub.two-client-document-scenario/v1");
    expect(fixture.authors).toEqual(["author-a", "author-b"]);
    expect(fixture.steps.length).toBeGreaterThanOrEqual(8);
    expect(fixture.expectations.bReceivesCommandsEnvelope).toBe(true);
    expect(fixture.expectations.presenceJoinReplayShowsA).toBe(true);
    expect(fixture.expectations.presenceLeaseExpiryStripsA).toBe(true);
    expect(fixture.presence.leaseTtlMs).toBe(15000);
    expect(fixture.command.diffSchema).toBe("db.pathmap.v1");
    expect(fixture.shutdown.closeCode).toBe(1012);
    expect(fixture.shutdown.restartWithinMs).toBe(1000);
    expect(fixture.growth.editsBeforeRestart).toBeGreaterThan(fixture.growth.distinctPaths * 4);
    expect(fixture.growth.payloadBytes).toBeGreaterThanOrEqual(16384);
    expect(fixture.expectations.gracefulShutdownClosesSocketsAndReleasesWriters).toBe(true);
    expect(fixture.expectations.crashReleasesWritersPerBackendContract).toBe(true);
    expect(fixture.expectations.revocationEndsAgentSession).toBe(true);
    expect(fixture.agent.revocationWithinMs).toBeLessThanOrEqual(5000);
  });
});

describe.skipIf(!HUB_E2E)("two-client document collaboration e2e", () => {
  it(
    "proves A commands and presence reach B over a real os-hub binary",
    async () => {
      return receiveScriptProcessInvocation(process.env,invocation=>{const backend=process.env.HUB_E2E_BACKEND;if(backend!=="sqlite")throw Error("External backend scenarios require genuine owned claim, cleanup and writer observation");const phase=scriptCompilerAdmissionControlV1(invocation,"hub:e2e",0);return withBuiltLocalHubHostV1(invocation,phase,(host,initial)=>withMaterializedTrustedCatalogBundleV1(host,initial.control,trustedBootstrapSelectPackages("stdio,gis,note"),(_receipt,actualPhase)=>withLocalHubHostTaskV1(host,actualPhase,async context=>{
      const dataRoot=context.dataRoot,bin=context.binaryPath;

      const packages = pick(hubRoot, (n) => n.includes("packages"));
      const hubRustRoot = pick(packages, (n) => n.includes("rust"));
      const localBootstrap = pick(hubRoot, (n) => n.includes("local-bootstrap"));
      const execution = pick(localBootstrap, (n) => n.includes("execution"));
      const fw = pick(repoRoot, (n) => n.includes("framework"));
      let osRoot = "";
      for (const pn of readdirSync(fw).filter((n) => n.includes("products"))) {
        for (const on of readdirSync(join(fw, pn)).filter((n) => n.endsWith("os"))) {
          const modsName = readdirSync(join(fw, pn, on)).find((n) => n.includes("modules"));
          if (!modsName) continue;
          const kids = readdirSync(join(fw, pn, on, modsName));
          if (kids.some((k) => k.includes("directory")) && kids.some((k) => k.includes("store"))) {
            osRoot = join(fw, pn, on);
            break;
          }
        }
        if (osRoot) break;
      }
      if (!osRoot) throw new Error("os product not found");
      const osMods = pick(osRoot, (n) => n.includes("modules"));
      const directory = pick(osMods, (n) => n.includes("directory"));
      const schemaDir = pick(directory, (n) => n.includes("schema"));
      const creation = pick(schemaDir, (n) => n.includes("space-artifact-creation"));
      const modules = pick(fw, (n) => n.includes("modules") && !n.includes("products"));
      const replication = pick(modules, (n) => n.includes("replication"));
      const trace = pick(modules, (n) => n.endsWith("trace"));
      const traceRecord = pick(trace, (n) => n.includes("record"));
      const validateTraceLine = new Ajv({ allErrors: true, strict: true }).compile(JSON.parse(readFileSync(join(pick(traceRecord, (n) => n.includes("schema")), "🔣️.json"), "utf8")));
      const traceVocabulary = JSON.parse(readFileSync(join(pick(pick(trace, (n) => n.includes("fixtures")), (n) => n.includes("span-vocabulary")), "🔣️.json"), "utf8")) as { events: string[] };

      const { sealDirectoryCommandRequestV1, directoryCommandRequestJson } = await import(join(schemaDir, "🟦️.ts"));
      const { sealSpaceArtifactCreateV1 } = await import(join(creation, "🟦️.ts"));
      const { encodeClientFrame, decodeServerFrame, encodePresencePeer, decodePresencePeer } = await import(join(replication, "🟦️.ts"));
      const { encodePackValue, parseDocumentSocketGrantReceiptV1 } = await import(join(osRoot, "🟦️.ts"));

      const fixtures = join(hubRoot, "🧪️tests", "🧫️fixtures");
      const fixtureDir = pick(fixtures, (n) => n.includes("two-client-document"));
      const fixture = JSON.parse(readFileSync(join(fixtureDir, "🔣️.json"), "utf8"));
      const dbMods = pick(osMods, (n) => n.endsWith("db"));
      const storageDir = pick(dbMods, (n) => n.includes("storage"));
      const writerDir = pick(storageDir, (n) => n.includes("writer"));
      const writerFence = JSON.parse(readFileSync(join(pick(pick(writerDir, (n) => n.includes("fixtures")), (n) => n.includes("remote-guard")), "🔣️.json"), "utf8"));

      const ADA = { email: "ada-tc@example.org", password: "correct horse battery staple", display: "Ada TC" };
      const BO = { email: "bo-tc@example.org", password: "another perfectly fine phrase", display: "Bo TC" };
      for (const account of [ADA, BO]) {
        const result=await captureOwnedCommandResultV1(bin,["credential","set","--email",account.email,"--display-name",account.display],context.root,{control:context.control,env:scriptInvocationEnvironment(invocation,{...context.environment,OS_HUB_DATA:dataRoot,OS_HUB_STORAGE_BACKEND:"sqlite",OS_HUB_DIRECTORY_BACKEND:"sqlite"}),input:{version:1,text:account.password},output:{maximumBytes:65536,maximumLines:128}});
        if(result.status!==0||result.signal!==null)throw Error("Credential set failed: "+result.stderr);
      }

      const port = Number(process.env.HUB_TWO_CLIENT_PORT ?? 7711);
      const profiles:readonly LocalProfile[] = [
        { profileId: "a", subject: "author-a", displayName: "Author A", allowedClientClasses: ["native", "mcp"] },
        { profileId: "b", subject: "author-b", displayName: "Author B", allowedClientClasses: ["native", "mcp"] },
      ];
      const run = await startLocalHub(host, profiles, { port, capture: true,credentialSignIn:true,traceLevel:"info" });
      const origin = `http://127.0.0.1:${port}`;
      const wsOrigin = `ws://127.0.0.1:${port}`;

      const waitReady = async (hubRun: typeof run, label: string) => {
        try {
          return await waitForReadiness(hubRun, true, TRUSTED_CATALOG_READINESS_STALL_BOUND_MS);
        } catch (error) {
          console.error(`${label}: ${hubRun.output().slice(-12000)}`);throw error;
        }
      };

      try {
        const { signIn, cmd, fetchTimed } = await prepareScenarioHarness({ waitReady, run, origin, context, directoryCommandRequestJson, sealDirectoryCommandRequestV1 });

        const { scenario, tokenA, tokenB } = await createSpaceAndArtifact({ signIn, ADA, BO, cmd, fixture, fetchTimed, origin, sealSpaceArtifactCreateV1, run });

        const { mint, openSocket, waitFrame } = prepareSocketHelpers({ scenario, run, fetchTimed, origin, parseDocumentSocketGrantReceiptV1, wsOrigin, decodeServerFrame, encodeClientFrame });

        const { a, beat, b, rosterWith, since, rosterWithout, roster, envelope, ack } = await runFirstCommandAndLateJoin({ mint, tokenA, tokenB, scenario, openSocket, waitFrame, decodePresencePeer, encodeClientFrame, encodePresencePeer, fixture, encodePackValue, signIn, BO });

        const { agentRevocationMs, agentExecutionTargetMs } = await runAgentDelegationAndRevocation({ tokenA, fetchTimed, origin, scenario, fixture, openSocket, mint, a, waitFrame, beat, b, rosterWith, encodeClientFrame, encodePackValue, since, rosterWithout });

        const { growthMsBeforeRestart, grow, lastGrown, frontierBefore } = await runPresenceExpiryAndGrowth({ since, b, beat, a, waitFrame, roster, fixture, encodeClientFrame, envelope, encodePackValue, scenario, openSocket, mint, tokenB, ack });

        const { note, exitedAt, sigtermAt, writerProbe, liveWriters, run2, exitOf, fenceRelease, reopen } = await runGracefulShutdown({ backend, fixture, growthMsBeforeRestart, agentRevocationMs, scenario, agentExecutionTargetMs, writerFence, run, openSocket, mint, host, profiles, port });
        await runCrashRecovery({ a, fixture, run, note, exitedAt, sigtermAt, writerProbe, liveWriters, validateTraceLine, traceVocabulary, scenario, waitReady, run2, signIn, ADA, fetchTimed, origin, openSocket, mint, grow, lastGrown, frontierBefore, exitOf, fenceRelease, host, profiles, port, writerFence, reopen });
      } catch (error) {
        console.error(run.output().slice(-12000));throw error;
      }
      })));});
    },
    TEST_TIMEOUT_MS,
  );
});
