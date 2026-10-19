import {withNxDaemonJournalReaderV1,type NxDaemonJournalDeploymentV1} from "../../../../🏁️startup/🚦️service/🟦️.ts";
import {requireNxGraphNativeDirectoryV1} from "../../../../../💾️storage/🔒️enforcement/🔌️nx/📦️graph/🟦️.ts";
import witnessSchema from "../../../../../🚀️bootstrap/🛠️tools/📥️use/🧬️schema/📥️witness.json" with {type:"json"};
import { isMainThread, parentPort, workerData, threadId } from "node:worker_threads";
import { createRequire } from "node:module";
import { createServer, type Socket } from "node:net";
import { mkdir, unlink, lstat, writeFile, rename, readFile } from "node:fs/promises";
import { dirname, isAbsolute, join } from "node:path";
import { fileURLToPath } from "node:url";
import schema from "./🧬️schema/🔣️.json" with { type: "json" };
import { createRepositoryFilesystemJournalV1, repositoryFilesystemJournalPolicyV1 } from "../🟦️.ts";
import { loadedFilesystemJournalToolingIdentityV1, readFilesystemJournalRpcRequestV1, filesystemJournalRpcRemainingMillisecondsV1 } from "../🌐️rpc/🟦️.ts";
import { isGlobalGraphIdentityPathV1 } from "../../../🎯️selection/🟦️.ts";
import { validateJsonSchemaSubset } from "@semio-tech/schema-validator-ts";
import type { FilesystemJournalChangeV1, FilesystemJournalControlV1 } from "@semio-tech/framework/filesystem/observation/journal";

/** 👷️ The immutable journal thread receives only its actual daemon-owned deployment. */
export type FilesystemJournalWorkerOptionsV1 = NxDaemonJournalDeploymentV1;

/** 🧬️ Checks the actual worker deployment before opening native observation or a socket. */
export function readFilesystemJournalWorkerOptionsV1(value: unknown): FilesystemJournalWorkerOptionsV1 {
  if (validateJsonSchemaSubset(schema, value,schema,[witnessSchema]).length) throw Error("Closed filesystem journal worker deployment required");
  const options = value as FilesystemJournalWorkerOptionsV1;
  if (process.platform!=="win32"&&Buffer.byteLength(options.socketPath)>95)throw Error("Filesystem journal socket byte limit exceeded");
  if (!isAbsolute(options.root) || !isAbsolute(options.directory)) throw Error("Actual filesystem journal worker directories required");
  return options;
}

/** 🕸️ Observes and serves independently of Nx's graph compilation event loop. */
export async function runFilesystemJournalWorkerV1(value: unknown): Promise<void> {
  const options = readFilesystemJournalWorkerOptionsV1(value);
  if (!parentPort || isMainThread) throw Error("Original immutable daemon journal worker required");
  const port=parentPort;return withNxDaemonJournalReaderV1(options,threadId,async original=>{
  const require = createRequire(import.meta.url), identity = loadedFilesystemJournalToolingIdentityV1(dirname(require.resolve("nx/package.json")));
  if(identity!==options.toolingIdentity)throw Error("Original immutable daemon journal toolkit required");
  const lifetime = new AbortController(), subscribers = new Set<(error: unknown, events: readonly FilesystemJournalChangeV1[]) => void>(), pending: FilesystemJournalChangeV1[] = [], sockets = new Set<Socket>();
  let watcher: any, journal: Awaited<ReturnType<typeof createRepositoryFilesystemJournalV1>> | undefined, server: ReturnType<typeof createServer> | undefined, closing: Promise<void> | undefined;
  const metadata = join(options.directory, "service.json");let settled!:()=>void,refused!:(error:unknown)=>void;const terminal=new Promise<void>((accept,reject)=>{settled=accept;refused=reject;});
  void terminal.catch(()=>{});
  const close = (): Promise<void> => closing ??= Promise.resolve().then(async () => {
    lifetime.abort(Error("Original daemon journal worker retired")); for (const socket of sockets) socket.destroy();
    if (server?.listening) await new Promise<void>(accept => server!.close(() => accept()));
    await journal?.close(); await watcher?.stop();
    if (process.platform !== "win32") await unlink(options.socketPath).catch(() => {});
    try { const current = JSON.parse(await readFile(metadata, "utf8")); if (current.generation === options.generation) await unlink(metadata); } catch {}
    parentPort?.close();settled();
  }).catch(error=>{refused(error);throw error;});
  const cancel = () => { void close().catch(error=>parentPort?.postMessage({type:"refused",reason:String(error)})); };
  port.once("close", cancel); port.on("message", message => { if (message?.type === "stop") cancel(); });
  try {
  const policy = await repositoryFilesystemJournalPolicyV1(options.root), excluded = (path: string) => policy.roots.some(root => path === root || path.startsWith(root + "/")) || path.split("/").some(part => policy.names.includes(part));
  const control: FilesystemJournalControlV1 = { signal: lifetime.signal, remainingMilliseconds: () => null, publish: async progress => { parentPort?.postMessage({ type: "progress", progress }); }, yieldContinuation: () => new Promise(resolve => setImmediate(resolve)) };
  original.control.signal.throwIfAborted();lifetime.signal.throwIfAborted();
  const { Watcher, WorkspaceContext } = require("nx/src/native"), transport = require("nx/src/utils/consume-messages-from-socket"), framing = require("nx/src/daemon/socket-utils");
  await mkdir(options.directory, { recursive: true });
  for (let path = options.directory; ; path = dirname(path)) { const status = await lstat(path); if (!status.isDirectory() || status.isSymbolicLink()) throw Error("Original journal worker directory ancestry required"); if (dirname(path) === path) break; }
  let native: any, refusal: unknown;
  watcher = new Watcher(options.root, [...policy.roots.map(root => "!/" + root + "/**"), ...policy.names.map(name => "!**/" + name + "/**")], false);
  const receive = (error: unknown, events: readonly FilesystemJournalChangeV1[]) => {
    if (error) refusal = error;
    const invalid = events.some(event => !event || typeof event.path !== "string" || !["create", "update", "delete"].includes(event.type));
    if (invalid) refusal = Error("Native journal worker change refused");
    const selected = invalid ? [] : events.filter(event => !excluded(event.path));
    if (subscribers.size) for (const subscriber of subscribers) subscriber(refusal ?? error, selected);
    else { pending.push(...selected); if (pending.length > 200_000) refusal = Error("Native journal worker startup overflow"); }
  };
  watcher.watch((error: unknown, events: FilesystemJournalChangeV1[] | null) => receive(error, events ?? []));
  native = new WorkspaceContext(options.root, requireNxGraphNativeDirectoryV1(options.root,process.env));
  journal = await createRepositoryFilesystemJournalV1(options.root, {
    watch(callback) { subscribers.add(callback); callback(refusal, pending.splice(0)); return () => { subscribers.delete(callback); }; },
    async flush() { await control.yieldContinuation(); if (refusal) throw refusal; const events = watcher.forceFlushPending(); if (events.some((event: FilesystemJournalChangeV1) => !event || typeof event.path !== "string" || !["create", "update", "delete"].includes(event.type))) throw Error("Native journal worker flush refused"); return events.filter((event: FilesystemJournalChangeV1) => !excluded(event.path)); },
    async census() { requireNxGraphNativeDirectoryV1(options.root,process.env);return native.allFileData().map((row: { file: string }) => row.file); },
    async admittedPaths(paths) { requireNxGraphNativeDirectoryV1(options.root,process.env);native.incrementalUpdate([...paths], []);return native.glob([...paths]); },
    isGlobalIdentity: isGlobalGraphIdentityPathV1
  }, control);
  lifetime.signal.throwIfAborted();
  server = createServer(socket => {
    sockets.add(socket); socket.once("close", () => sockets.delete(socket));
    let requested = false;
    socket.on("data", transport.consumeMessagesFromSocket(async (bytes: Uint8Array) => {
      if (requested) { socket.destroy(); return; } requested = true;
      const stop = new AbortController(), abort = () => stop.abort(Error("Original journal request socket closed"));
      socket.once("close", abort);
      try {
        const payload = transport.parseMessage(bytes);
        if (!payload || Object.keys(payload).some(key => !["type", "version", "scope", "deadlineUnixMilliseconds", "workspaceRoot"].includes(key)) || payload.workspaceRoot !== options.root) throw Error("Original journal workspace envelope refused");
        const request = readFilesystemJournalRpcRequestV1({ type: payload.type, version: payload.version, scope: payload.scope, deadlineUnixMilliseconds:payload.deadlineUnixMilliseconds });
        const query: FilesystemJournalControlV1 = { signal: AbortSignal.any([stop.signal, lifetime.signal]), remainingMilliseconds: () => filesystemJournalRpcRemainingMillisecondsV1(request,Date.now()), publish: control.publish, yieldContinuation: control.yieldContinuation };
        const witness = await journal!.capture(request.scope, query);
        framing.sendMessage(socket, { version: 1, toolingIdentity: identity, witness }, "json");
      } catch (error) { parentPort?.postMessage({ type: "query-refused", reason: String(error) }); if (!socket.destroyed) framing.sendMessage(socket, null, "json"); }
      finally { socket.removeListener("close", abort); }
    }, () => socket.destroy()));
  });
  await new Promise<void>((accept, reject) => { server!.once("error", reject); server!.listen(options.socketPath, accept); });
  const service = { version: 1, root: options.root, processId: process.pid, generation: options.generation, socketPath: options.socketPath, toolingIdentity: identity, graphIdentity:options.graphIdentity,implementationIdentity:options.implementationIdentity };
  const temporary = metadata + "." + options.generation.slice(0, 12) + ".tmp"; await writeFile(temporary, JSON.stringify(service) + "\n", { flag: "wx" }); await rename(temporary, metadata);
  lifetime.signal.throwIfAborted(); parentPort!.postMessage({ type: "ready", ...service });
  await terminal;
  } catch (error) { parentPort?.postMessage({ type: "refused", reason: String(error) }); await close(); throw error; }
  });
}

if (!isMainThread && workerData?.kind === "filesystem-journal") void runFilesystemJournalWorkerV1(workerData).catch(error => { parentPort?.postMessage({ type: "refused", reason: String(error) }); parentPort?.close(); });
