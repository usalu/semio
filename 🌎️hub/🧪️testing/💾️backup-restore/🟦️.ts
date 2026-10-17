/** 🧪️ Backup restore: test support exporting `BackupRestoreRound`, `runBackupRestoreDrill`, `ShutdownDrillRound`, `SHUTDOWN_EXIT_BOUND_MS` and more. */
import {readHubDrillRequestV1} from "./🎛️request/🟦️.ts";
/** 💾️ The hub backup/restore drill, zero-touch: a fresh data root carrying a copy of a published trusted catalog, one
 * credential user, the real `os-hub` binary, and per round:
 *
 *   seed (a private space, one document of the chosen kind, N chained edits over the document socket, the reopened
 *   frontier, the active checkpoint pair digest, the descriptor digest, the directory listing) → SIGTERM (timed) →
 *   backup (`tar`, the platform's own archiver — bsdtar on macOS and Windows, GNU tar on Linux) → restore into a
 *   DIFFERENT root (relocation) → every restored file byte-identical to its original (sha256 per file) → the hub boots
 *   on the restored root (timed) → the same frontier, checkpoint pair, descriptor and listing, and the next chained edit
 *   is accepted.
 *
 * `README.md` § "Backup and restore" is the procedure this drill executes; the backup unit is the whole data root with the
 * hub stopped. The same module holds the graceful-shutdown drill (`runShutdownDrill`): the stop that procedure relies on,
 * taken while the hub is busy — its catalog verifying, a socket open with an edit in flight, a creation interpreting. Promoted from the session-12 ticket harness `wp-h10/h10-backup-drill.ts` + `h10-hub.sh` (ticket 26/09/23,
 * measured there SIGTERM 277 ms → tar 164 MB / 25 s → ready 13.5 s, byte-identical, 5/5).
 */
import { createHash, randomBytes } from "node:crypto";
import {lstat,opendir} from "node:fs/promises";
import {join,relative,dirname} from "node:path";
import {checkResourceLeaseControl,type ResourceLeaseControl} from "@semio-tech/process/leases";
import {captureOwnedCommandResultV1} from "@semio-tech/process/owned-execution";
import {observePhysicalFileV1} from "@semio-tech/framework/filesystem/observation";
import schema from "./🧬️schema/🔣️.json";
import {hubProbeCall,hubProbeCreateArtifact,hubProbeCreateSpace,hubProbeCreationCatalog,hubProbeOpenDocument,hubProbeSignIn} from "@semio-tech/framework-os/modules/directory/testkit/client-probe";
import {sealSpaceArtifactCreateV1} from "@semio-tech/framework-os/modules/directory/schema/space-artifact-creation-v1";
import {requireLocalHubHostHandleV1,withLocalHubHostCandidateV1,withLocalHubHostTaskV1,finishLocalHub,startLocalHub,stopLocalHubV1,TRUSTED_CATALOG_READINESS_STALL_BOUND_MS,waitForReadiness,LOCAL_HUB_DEVELOPMENT_CATALOG_PACKAGES,type LocalHubHostV1,type LocalHubHostContextV1} from "../../🚀️local-bootstrap/🏃️execution/🟦️.ts";
import { validateAndPublishTrustedStdioGisCandidate } from "../../🏗️bootstrap/🧾️provenance/🚚️bootstrap-publication/🟦️.ts";
import { withMaterializedTrustedCatalogBundleV1 } from "../../🏗️bootstrap/🧾️provenance/🚚️bootstrap-publication/🧱️materialized-bundle/🟦️.ts";
import {trustedBootstrapSelectPackages} from "../../🏗️bootstrap/🧾️provenance/🧭️bootstrap-selection/🟦️.ts";
const policy=Object.freeze({...schema.definitions.Policy.const});
/** 📊️ One round's measurements. */
export type BackupRestoreRound = {
  round: number;
  pass: boolean;
  kindId?: string;
  documentId?: string;
  seedMs?: number;
  sigtermToExitMs?: number;
  archiveBytes?: number;
  archiveMs?: number;
  restoreMs?: number;
  files?: number;
  byteIdentical?: boolean;
  restoredReadyMs?: number;
  checks?: Record<string, boolean>;
  undecodableFrames?: number;
  error?: string;
  roots?: string;
};

/** 🧑‍💻️ The launcher profile the drill hub is bootstrapped for (the drill itself signs in with a credential). */
const DRILL_PROFILE = Object.freeze({ profileId: "developer", subject: "local-backup-drill-01", displayName: "Backup Drill", allowedClientClasses: Object.freeze(["native", "mcp", "react-relay"] as const) });
const EMAIL = "backup-drill@semio.dev";

async function fileDigests(root:string,control:ResourceLeaseControl):Promise<Map<string,string>>{
 const digests=new Map<string,string>(),directories=new Map<string,Awaited<ReturnType<typeof lstat>>>();let entries=0,total=0;
 const parents=async(path:string)=>{for(let parent=dirname(path);;parent=dirname(parent)){const before=directories.get(parent),after=await lstat(parent);if(!before||!after.isDirectory()||after.isSymbolicLink()||before.dev!==after.dev||before.ino!==after.ino)throw Error("Drill directory ancestry changed");if(parent===root)break;}};
 const walk=async(path:string,depth:number):Promise<void>=>{checkResourceLeaseControl(control);if(++entries>policy.maximumEntries||depth>policy.maximumDepth||path.length>policy.maximumPathUnits)throw Error("Drill snapshot inventory capacity exceeded");const facts=await lstat(path);if(facts.isSymbolicLink())throw Error("Drill snapshot symbolic link refused");if(facts.isDirectory()){directories.set(path,facts);const directory=await opendir(path);for await(const row of directory)await walk(join(path,row.name),depth+1);const after=await lstat(path);if(after.dev!==facts.dev||after.ino!==facts.ino)throw Error("Drill snapshot directory changed");return;}if(!facts.isFile()||facts.nlink!==1||facts.size>policy.maximumSnapshotBytes-total)throw Error("Finite regular drill snapshot required");total+=facts.size;await parents(path);const claim=await observePhysicalFileV1(path,{maxBytes:facts.size,maxWork:Math.ceil(facts.size/65536)+16,chunkBytes:65536,cancelled:()=>control.signal.aborted,remainingMs:()=>checkResourceLeaseControl(control)??Number.MAX_SAFE_INTEGER,onProgress:()=>{checkResourceLeaseControl(control);}});if(claim.byteLength!==facts.size)throw Error("Drill snapshot extent changed");await parents(path);digests.set(relative(root,path),claim.sha256);};await walk(root,0);return digests;
}
async function command(context:LocalHubHostContextV1,program:"tar"|"credential",args:readonly string[],input?:string):Promise<string>{const result=await captureOwnedCommandResultV1(program==="credential"?context.binaryPath:"tar",args,context.diagnosticsRoot,{control:context.control,env:{...context.environment,OS_HUB_DATA:context.dataRoot},...(input===undefined?{}:{input:{version:1 as const,text:input}}),output:{maximumBytes:65536,maximumLines:512}});if(result.status!==0||result.signal!==null)throw Error("Owned drill child failed: "+result.stderr);return result.stdout;}
async function withDrillHostV1<T>(parent:LocalHubHostV1,phase:ResourceLeaseControl,body:(host:LocalHubHostV1,context:LocalHubHostContextV1)=>Promise<T>):Promise<T>{return withLocalHubHostCandidateV1(parent,phase,(host,context)=>withMaterializedTrustedCatalogBundleV1(host,context.control,trustedBootstrapSelectPackages(LOCAL_HUB_DEVELOPMENT_CATALOG_PACKAGES),async(receipt,actual)=>{await validateAndPublishTrustedStdioGisCandidate(host,actual,receipt);return withLocalHubHostTaskV1(host,actual,context=>body(host,context));}));}

async function snapshot(origin: string, token: string, spaceId: string, documentId: string) {
  const scope = `/spaces/${encodeURIComponent(spaceId)}/documents/${encodeURIComponent(documentId)}`;
  const descriptor = await hubProbeCall(origin, "GET", scope, token);
  const pair = await hubProbeCall(origin, "GET", `${scope}/active-checkpoint/pair`, token, undefined, "application/vnd.semio.canonical-checkpoint-pair.v1");
  const spaces = await hubProbeCall(origin, "GET", "/directory/spaces", token);
  return {
    descriptorStatus: descriptor.status,
    descriptorSha256: createHash("sha256").update(descriptor.bytes).digest("hex"),
    pairStatus: pair.status,
    pairSha256: createHash("sha256").update(pair.bytes).digest("hex"),
    spaceListed: JSON.stringify(spaces.json ?? []).includes(spaceId),
  };
}

const frontierOf = (welcome: any): string => JSON.stringify({ lastCommitSeq: welcome.server_frontier.last_commit_seq, headEditId: welcome.server_frontier.head_edit_id, headEditOrdinal: welcome.server_frontier.head_edit_ordinal, chainHash: Buffer.from(welcome.server_frontier.chain_hash).toString("hex") });

/** 🌎️ One booted drill hub: its origin, a SIGTERM stop that answers how long the process took to exit, how it exited
 * (`null` while it runs) and everything it printed. */
type DrillHub = Readonly<{ baseUrl: string; stop: () => Promise<number>; exit: () => { code: number | null; signal: NodeJS.Signals | null } | null; output: () => string }>;

/** 🚀️ Boots the hub on `dataDir` the way every local launcher does — the authenticated local-bootstrap pipe on fd 3
 * (`startLocalHub`), credential sign-in on — and answers once its own `/readyz` admits it, bounded by the hub's own
 * catalog no-progress bound. `stop` sends SIGTERM (the backup procedure's stop), waits for the exit, then releases the
 * launcher's run root. */
async function bootHub(host:LocalHubHostV1):Promise<{hub:DrillHub;readyMs:number}>{
 const started=Date.now(),run=await startLocalHub(host,[DRILL_PROFILE],{capture:true,credentialSignIn:true,adminSubjects:[`credential.password.v1:${SHUTDOWN_EMAIL}`]});
 await waitForReadiness(run,false,TRUSTED_CATALOG_READINESS_STALL_BOUND_MS);
 const stop=async()=>{const at=Date.now();await stopLocalHubV1(run,"graceful");await finishLocalHub(run);return Date.now()-at;},exit=()=>(run.child.exitCode===null&&run.child.signalCode===null?null:{code:run.child.exitCode,signal:run.child.signalCode});return{hub:{baseUrl:`http://127.0.0.1:${run.port}`,stop,exit,output:run.output},readyMs:Date.now()-started};
}

/** 💾️ Runs every round; each round retains both genuine candidate roots through exact archive/restore and all child settlement. */
export async function runBackupRestoreDrill(parent:LocalHubHostV1,phase:ResourceLeaseControl,value:unknown,onProgress:(line:string)=>void):Promise<BackupRestoreRound[]>{
  requireLocalHubHostHandleV1(parent);const options=readHubDrillRequestV1(value);
  const rounds: BackupRestoreRound[] = [];
  const password = randomBytes(18).toString("hex");
  for (let round = 1; round <= options.rounds; round += 1) {
    const result: BackupRestoreRound = { round, pass: false };
    await withDrillHostV1(parent,phase,(originalHost,context)=>withLocalHubHostCandidateV1(parent,context.control,async(restoredHost,restoredContext)=>{
    const original=context.dataRoot,restored=restoredContext.dataRoot,archive=join(context.diagnosticsRoot,"backup.tar");
    let hub: DrillHub | undefined;let primary:{error:unknown}|undefined;
    try {
      await command(context,"credential",["credential","set","--email",EMAIL,"--display-name","Backup Drill"],password);
      onProgress(`round ${round}: booting on ${original}`);
      const booted = await bootHub(originalHost);
      hub = booted.hub;
      onProgress(`round ${round}: ready in ${booted.readyMs} ms; seeding ${options.edits} edits`);
      const seedStarted = Date.now();
      const token = await hubProbeSignIn(hub.baseUrl, EMAIL, password, "backupdrill");
      const spaceId = await hubProbeCreateSpace(hub.baseUrl, token, `Backup drill ${round} ${new Date().toISOString()}`);
      const catalog = await hubProbeCreationCatalog(hub.baseUrl, token, spaceId);
      const kind = catalog.kinds.find((entry) => entry.kindId === options.kind || entry.schema.startsWith(options.kind));
      if (!kind) throw new Error(`the catalog offers no kind ${options.kind}; it offers ${catalog.kinds.map((entry) => entry.kindId).join(", ")}`);
      result.kindId = kind.kindId;
      const { artifactId } = await hubProbeCreateArtifact(hub.baseUrl, token, spaceId, catalog.generationId, kind.kindId, `Backup drill ${kind.kindId}`);
      result.documentId = artifactId;
      const session = await hubProbeOpenDocument(hub.baseUrl, token, spaceId, artifactId, "backup-drill");
      let last = "";
      for (let index = 0; index < options.edits; index += 1) last = await session.edit(index, last);
      result.undecodableFrames = session.undecodableFrames();
      session.close();
      const reopened = await hubProbeOpenDocument(hub.baseUrl, token, spaceId, artifactId, "backup-drill");
      const frontier = frontierOf(reopened.welcome);
      reopened.close();
      const before = await snapshot(hub.baseUrl, token, spaceId, artifactId);
      result.seedMs = Date.now() - seedStarted;
      result.sigtermToExitMs = await hub.stop();
      hub = undefined;
      onProgress(`round ${round}: SIGTERM → exit ${result.sigtermToExitMs} ms; archiving`);
      const archiveStarted = Date.now();
      const originalDigests = await fileDigests(original,context.control);
      await command(context,"tar",["-cf",archive,"-C",original,"."]);
      result.archiveMs = Date.now() - archiveStarted;
      result.archiveBytes = (await lstat(archive)).size;
      const restoreStarted = Date.now();
      await command(restoredContext,"tar",["-xf",archive,"-C",restored]);
      result.restoreMs = Date.now() - restoreStarted;
      const restoredDigests = await fileDigests(restored,restoredContext.control);
      result.files = originalDigests.size;
      result.byteIdentical = originalDigests.size === restoredDigests.size && [...originalDigests].every(([path, digest]) => restoredDigests.get(path) === digest);
      onProgress(`round ${round}: archive ${result.archiveBytes} B in ${result.archiveMs} ms, restored ${result.files} files byte-identical=${result.byteIdentical}; booting the restored root`);
      const rebooted = await bootHub(restoredHost);
      hub = rebooted.hub;
      result.restoredReadyMs = rebooted.readyMs;
      const tokenAfter = await hubProbeSignIn(hub.baseUrl, EMAIL, password, "backupdrill");
      const session2 = await hubProbeOpenDocument(hub.baseUrl, tokenAfter, spaceId, artifactId, "backup-drill");
      const after = await snapshot(hub.baseUrl, tokenAfter, spaceId, artifactId);
      const checks: Record<string, boolean> = {
        byteIdentical: result.byteIdentical,
        frontier: frontierOf(session2.welcome) === frontier,
        descriptor: after.descriptorStatus === 200 && after.descriptorSha256 === before.descriptorSha256,
        checkpointPair: after.pairStatus === 200 && after.pairSha256 === before.pairSha256,
        spaceListed: after.spaceListed,
        nextEditAccepted: false,
      };
      await session2.edit(options.edits, last);
      checks.nextEditAccepted = true;
      session2.close();
      result.checks = checks;
      result.pass = Object.values(checks).every(Boolean);
    } catch (error) {
      result.error = String(error instanceof Error ? error.message : error).slice(0, 600);
      primary={error};throw error;
    } finally {
      if(hub)try{await hub.stop();}catch(error){if(primary&&!Object.is(primary.error,error))throw new AggregateError([primary.error,error],"Drill body and owned child close failed");throw error;}
    }
    }));
    rounds.push(result);
    onProgress(`round ${round}: ${result.pass ? "PASS" : "FAIL"} ${JSON.stringify(result.checks ?? {})}${result.error ? ` ${result.error}` : ""}`);
  }
  return rounds;
}

/** 🛑️ One graceful-shutdown drill: the drill hub's binary and catalog as the backup drill's, the kind whose document takes
 * the acknowledged edits, the heavy kinds one of which is created while the hub is stopped (the first the catalog offers). */
/** 📊️ One shutdown round's measurements. */
export type ShutdownDrillRound = {
  round: number;
  pass: boolean;
  kindId?: string;
  inFlightKindId?: string;
  documentId?: string;
  bootReadyMs?: number;
  packagesVerifyingAtSigterm?: number;
  observabilityStatus?: number;
  sigtermToExitMs?: number;
  exitCode?: number | null;
  exitSignal?: string | null;
  socketClose?: { code: number; clean: boolean } | null;
  restartReadyMs?: number;
  headEditOrdinal?: number;
  inFlightCreationStatus?: number;
  inFlightCreationPhase?: string;
  checks?: Record<string, boolean>;
  error?: string;
  roots?: string;
};

/** ⏱️ How long a stopping hub may take to exit: the database's own shutdown deadline (`DATABASE_SHUTDOWN_DEADLINE`, 10 s),
 * past which the hub exits 1 anyway. README § "Backup and restore" asks operators for a 30 s stop timeout on top. */
export const SHUTDOWN_EXIT_BOUND_MS = 10_000;

const SHUTDOWN_EMAIL = "shutdown-drill@semio.dev";

/** 🛑️ Runs every round of the graceful-shutdown drill on a fresh root seeded with a copy of a published catalog: cold boot
 * (the catalog's background verification starts interpreting guests as soon as the hub serves) → one document with
 * `edits` acknowledged edits, its socket left open with one more edit in flight → a creation of a heavy kind in flight →
 * SIGTERM → the process exits 0 within [`SHUTDOWN_EXIT_BOUND_MS`] and records its `server.shutdown` with the database
 * closed, the open socket is ended with a close frame → the hub boots on the same root, every acknowledged edit is there,
 * the next edit is accepted and the interrupted creation answers a typed status. */
export async function runShutdownDrill(parent:LocalHubHostV1,phase:ResourceLeaseControl,value:unknown,onProgress:(line:string)=>void):Promise<ShutdownDrillRound[]>{
  requireLocalHubHostHandleV1(parent);const options=readHubDrillRequestV1(value);
  const rounds: ShutdownDrillRound[] = [];
  const password = randomBytes(18).toString("hex");
    for (let round = 1; round <= options.rounds; round += 1) {
      const result: ShutdownDrillRound = { round, pass: false };
      await withDrillHostV1(parent,phase,async(host,context)=>{
      const root=context.dataRoot;
      let hub: DrillHub | undefined;let primary:{error:unknown}|undefined;
      try {
        await command(context,"credential",["credential","set","--email",SHUTDOWN_EMAIL,"--display-name","Shutdown Drill"],password);
        onProgress(`round ${round}: cold boot on ${root}`);
        const booted = await bootHub(host);
        hub = booted.hub;
        result.bootReadyMs = booted.readyMs;
        const token = await hubProbeSignIn(hub.baseUrl, SHUTDOWN_EMAIL, password, "shutdowndrill");
        const spaceId = await hubProbeCreateSpace(hub.baseUrl, token, `Shutdown drill ${round} ${new Date().toISOString()}`);
        const catalog = await hubProbeCreationCatalog(hub.baseUrl, token, spaceId);
        const kind = catalog.kinds.find((entry) => entry.kindId === options.kind || entry.schema.startsWith(options.kind));
        if (!kind) throw new Error(`the catalog offers no kind ${options.kind}; it offers ${catalog.kinds.map((entry) => entry.kindId).join(", ")}`);
        const heavy = options.heavyKinds.map((kindId) => catalog.kinds.find((entry) => entry.kindId === kindId)).find(Boolean) ?? kind;
        result.kindId = kind.kindId;
        result.inFlightKindId = heavy.kindId;
        const { artifactId } = await hubProbeCreateArtifact(hub.baseUrl, token, spaceId, catalog.generationId, kind.kindId, `Shutdown drill ${kind.kindId}`);
        result.documentId = artifactId;
        const session = await hubProbeOpenDocument(hub.baseUrl, token, spaceId, artifactId, "shutdown-drill");
        let last = "";
        for (let index = 0; index < options.edits; index += 1) last = await session.edit(index, last);
        const inFlightEdit = session.submit(options.edits, last).catch((error: unknown) => String(error));
        const requestId = randomBytes(16).toString("hex");
        const creation = await hubProbeCall(hub.baseUrl, "POST", `/spaces/${encodeURIComponent(spaceId)}/artifact-creations`, token, JSON.stringify(sealSpaceArtifactCreateV1({ requestId, expectedCatalogGenerationId: catalog.generationId, kindId: heavy.kindId, name: `Shutdown drill in flight ${heavy.kindId}` })));
        if (creation.status !== 202) throw new Error(`in-flight creation ${creation.status} ${creation.text.slice(0, 300)}`);
        await new Promise((resolveDelay) => setTimeout(resolveDelay, 1_000));
        const observability = await hubProbeCall(hub.baseUrl, "GET", "/admin/api/observability", token);
        result.observabilityStatus = observability.status;
        if (observability.status === 200) result.packagesVerifyingAtSigterm = (observability.json?.catalog?.packages ?? []).filter((entry: { phase: string }) => entry.phase === "verifying").length;
        onProgress(`round ${round}: ${options.edits} edits acknowledged, one in flight, ${heavy.kindId} creation in flight, ${result.packagesVerifyingAtSigterm ?? `? (observability ${result.observabilityStatus})`} package(s) verifying; SIGTERM`);
        const stopping = hub;
        result.sigtermToExitMs = await stopping.stop();
        hub = undefined;
        const exited = stopping.exit();
        result.exitCode = exited?.code ?? null;
        result.exitSignal = exited?.signal ?? null;
        result.socketClose = await session.ended(5_000);
        await inFlightEdit;
        const shutdownLine = stopping.output().split("\n").find((line) => line.includes("server.shutdown") && line.includes("database=closed"));
        onProgress(`round ${round}: SIGTERM → exit ${result.sigtermToExitMs} ms code ${result.exitCode} signal ${result.exitSignal}; socket ${JSON.stringify(result.socketClose)}; restarting on the same root`);
        const rebooted = await bootHub(host);
        hub = rebooted.hub;
        result.restartReadyMs = rebooted.readyMs;
        const tokenAfter = await hubProbeSignIn(hub.baseUrl, SHUTDOWN_EMAIL, password, "shutdowndrill");
        const reopened = await hubProbeOpenDocument(hub.baseUrl, tokenAfter, spaceId, artifactId, "shutdown-drill");
        result.headEditOrdinal = Number(reopened.welcome.server_frontier.head_edit_ordinal);
        const checks: Record<string, boolean> = {
          exitedZero: result.exitCode === 0 && result.exitSignal === null,
          exitedInBound: result.sigtermToExitMs <= SHUTDOWN_EXIT_BOUND_MS,
          shutdownRecorded: shutdownLine !== undefined,
          socketClosedWithFrame: result.socketClose !== null && result.socketClose.code !== 1006,
          acknowledgedEditsDurable: result.headEditOrdinal >= options.edits,
          nextEditAccepted: false,
          interruptedCreationTyped: false,
        };
        await reopened.edit(options.edits + 1, last);
        checks.nextEditAccepted = true;
        reopened.close();
        const status = await hubProbeCall(hub.baseUrl, "GET", `/spaces/${encodeURIComponent(spaceId)}/artifact-creations/${requestId}`, tokenAfter);
        result.inFlightCreationStatus = status.status;
        result.inFlightCreationPhase = String(status.json?.phase ?? status.text.slice(0, 80));
        checks.interruptedCreationTyped = status.status < 500 || status.status === 503;
        result.checks = checks;
        result.pass = Object.values(checks).every(Boolean);
      } catch (error) {
        result.error = String(error instanceof Error ? error.message : error).slice(0, 600);
        primary={error};throw error;
      } finally {
        if(hub)try{await hub.stop();}catch(error){if(primary&&!Object.is(primary.error,error))throw new AggregateError([primary.error,error],"Drill body and owned child close failed");throw error;}
      }
      });
      rounds.push(result);
      onProgress(`round ${round}: ${result.pass ? "PASS" : "FAIL"} ${JSON.stringify(result.checks ?? {})}${result.error ? ` ${result.error}` : ""}`);
    }
  return rounds;
}
