import{withRepositoryToolingOperationV1}from"../../../💾️storage/🔒️enforcement/🛠️tools/🟦️.ts";
import{checkResourceLeaseControl,type ResourceLeaseControl}from"../../../../../../../../🔨️modules/🏃️process/🔒️leases/🟦️.ts";
import type{ToolingHeadroomRefusalV1}from"../🧹️retention/📏️headroom/🟦️.ts";
import {withManagedToolingProvisionV1,type ManagedToolingPublisherV1} from "../🧹️retention/📥️physical/🟦️.ts";
import retentionSchema from "../🧹️retention/📥️physical/🧬️schema/🔣️.json";
import type {RepositoryToolingStoreV1} from "../../../💾️storage/🔒️enforcement/🛠️tools/🟦️.ts";
import {requireNativeStorageReservationV1} from "../../../../../../../../🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/🟦️.ts";
import {requireNativeStoragePathV1} from "../../../../../../../../🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/🌐️platform/📋️contract/🟦️.ts";
import { createHash } from "node:crypto";
import { chmodSync, lstatSync, mkdirSync, mkdtempSync, readFileSync, renameSync, rmSync, writeFileSync } from "node:fs";
import { dirname, join } from "node:path";
import { inflateRawSync } from "node:zlib";
import {captureOwnedCommandV1} from "../../../../../../../../🔨️modules/🏃️process/🎛️owned-execution/🟦️.ts";
import {readNativeCallerAuthorityV1} from "../../../📦️artifacts/📋️native-orchestration/📥️invocation/🟦️.ts";
import manifestSchema from "./🧬️schema/🔣️.json" with {type:"json"};
const manifest=manifestSchema.definitions.Release.const;
import { withResourceLeases } from "../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🔒️leases/🟦️.ts";

type BunDistribution = { platform: string; arch: string; musl: boolean; archive: string; bytes: number; sha256: string };


/** 🧭️ Selects the authored portable Bun release without changing the installed runtime. */
export function bunDistribution(version: string, platform: string, arch: string, musl: boolean): BunDistribution {
  const row = version === manifest.version && manifest.distributions.find(row => row.platform === platform && row.arch === arch && row.musl === musl);
  if (!row) throw new Error(`Unsupported pinned Bun: ${version} ${platform}/${arch}${musl ? "/musl" : ""}`);
  return row;
}

/** 📦️ Extracts only the executable from a checksum-verified ordinary ZIP release. */
export function bunArchiveExecutable(archive: Buffer, member: string): Buffer {
  for (let at = Math.max(0, archive.length - 22); at >= Math.max(0, archive.length - 65557); at--) {
    if (archive.readUInt32LE(at) !== 0x06054b50) continue;
    let cursor = archive.readUInt32LE(at + 16);
    for (let index = 0; index < archive.readUInt16LE(at + 10); index++) {
      if (archive.readUInt32LE(cursor) !== 0x02014b50) throw new Error("Invalid Bun ZIP directory");
      const nameLength = archive.readUInt16LE(cursor + 28), name = archive.subarray(cursor + 46, cursor + 46 + nameLength).toString();
      if (name === member) {
        const flags = archive.readUInt16LE(cursor + 8), method = archive.readUInt16LE(cursor + 10), size = archive.readUInt32LE(cursor + 20), expanded = archive.readUInt32LE(cursor + 24), local = archive.readUInt32LE(cursor + 42);
        if (flags & 1 || ![0, 8].includes(method) || expanded > 256 * 1024 * 1024 || archive.readUInt32LE(local) !== 0x04034b50) throw new Error("Invalid Bun ZIP executable");
        const start = local + 30 + archive.readUInt16LE(local + 26) + archive.readUInt16LE(local + 28), compressed = archive.subarray(start, start + size);
        const payload = method === 0 ? Buffer.from(compressed) : inflateRawSync(compressed, { maxOutputLength: expanded });
        if (compressed.length !== size || payload.length !== expanded) throw new Error("Invalid Bun ZIP size");
        return payload;
      }
      cursor += 46 + nameLength + archive.readUInt16LE(cursor + 30) + archive.readUInt16LE(cursor + 32);
    }
    break;
  }
  throw new Error("Bun archive has no selected executable");
}

/** 🔐️ Acquires and atomically publishes a pinned workspace-owned installer with cancellation. */
export function prepareBun(workspace:string,version:string,store:RepositoryToolingStoreV1,control:ResourceLeaseControl,publisher?:ManagedToolingPublisherV1):Promise<string|ToolingHeadroomRefusalV1>{return withRepositoryToolingOperationV1(workspace,store,control,actual=>prepareBunOwnedV1(workspace,version,store,actual,publisher));}
async function prepareBunOwnedV1(workspace:string,version:string,store:RepositoryToolingStoreV1,control:ResourceLeaseControl,publisher?:ManagedToolingPublisherV1):Promise<string|ToolingHeadroomRefusalV1>{
  requireNativeStorageReservationV1(store.storage,store.control);await store.storage.recheck();const signal=control.signal;signal.throwIfAborted();
  if (process.versions.bun === version) return process.execPath;
  const report = process.platform === "linux" ? process.report?.getReport?.() as { header?: { glibcVersionRuntime?: string } } : undefined;
  const row = bunDistribution(version, process.platform, process.arch, process.platform === "linux" && !report?.header?.glibcVersionRuntime);
  const base = requireNativeStoragePathV1(store.root,join(store.root,"bun")), directory = join(base, row.sha256), executable = join(directory, process.platform === "win32" ? "bun.exe" : "bun");
  for (let current = base; ; current = dirname(current)) {
    try { const stat = lstatSync(current); if (!stat.isDirectory() || stat.isSymbolicLink()) throw new Error(`Invalid Bun directory: ${current}`); }
    catch (error) { if ((error as NodeJS.ErrnoException).code !== "ENOENT") throw error; }
    if (dirname(current) === current) break;
  }
  const lease={directory:join(workspace,".cache/b/l/m/data"),signal,control},resource=`bun-${row.sha256}`;
    const validate = async (): Promise<string> => {
      const stat = lstatSync(executable), receipt = JSON.parse(readFileSync(join(directory, ".toolchain.json"), "utf8"));
      if (!stat.isFile() || stat.isSymbolicLink() || lstatSync(directory).isSymbolicLink() || receipt.archive !== row.sha256 || receipt.executable !== createHash("sha256").update(readFileSync(executable)).digest("hex")) throw new Error("Bun installation checksum mismatch");
      if ((await captureOwnedCommandV1(executable,["--version"],directory,{control,env:store.environment,output:{maximumBytes:65536,maximumLines:1024}})).trim() !== version) throw new Error("Bun executable version mismatch");
      return executable;
    };
    const ready=await withResourceLeases({...lease,resources:[{resource,mode:"shared"}]},async()=>{try{lstatSync(directory);return await validate();}catch(error){if((error as NodeJS.ErrnoException).code!=="ENOENT")throw error;}});if(ready)return ready;
    if(readNativeCallerAuthorityV1(store.invocation.capabilities).network.offline)throw Error("Original offline pinned Bun parser absent");
    const publish:ManagedToolingPublisherV1=publisher??((job,quote)=>withManagedToolingProvisionV1(workspace,store,control,{version:1,minimumAvailableBytes:retentionSchema.definitions.Protocol.const.minimumInstallerAvailableBytes},(original,control)=>original(job,quote)));
    return publish(()=>withResourceLeases({...lease,resources:[{resource,mode:"exclusive"}]},async()=>{
    try { lstatSync(directory); return await validate(); } catch (error) { if ((error as NodeJS.ErrnoException).code !== "ENOENT") throw error; }
    await store.recheck();mkdirSync(base,{recursive:true});
    const staging=mkdtempSync(join(base,".prepare-")),deadline=new AbortController(),watch=setInterval(()=>{try{checkResourceLeaseControl(control);}catch(error){deadline.abort(error);}},25),progress = setInterval(() => console.log(`Acquiring pinned Bun ${version}…`), 10000);
    try {
      console.log(`Acquiring pinned Bun ${version} for ${row.platform}/${row.arch}…`);
      const response = await fetch(`${manifest.release}/${row.archive}`, { signal:AbortSignal.any([signal,deadline.signal]) });
      if (!response.ok || !response.body) throw new Error(`Bun download failed: ${response.status}`);
      const chunks: Buffer[] = [], hash = createHash("sha256"), reader = response.body.getReader(); let size = 0;
      try { for (let read = await reader.read(); !read.done; read = await reader.read()) { checkResourceLeaseControl(control); size += read.value.length; if (size > row.bytes) throw new Error("Bun archive exceeds its pinned size"); const chunk = Buffer.from(read.value); chunks.push(chunk); hash.update(chunk); } }
      catch (error) { await reader.cancel().catch(() => undefined); throw error; }
      if (size !== row.bytes || hash.digest("hex") !== row.sha256) throw new Error("Bun archive checksum mismatch");
      const payload = bunArchiveExecutable(Buffer.concat(chunks), `${row.archive.slice(0, -4)}/bun${process.platform === "win32" ? ".exe" : ""}`), staged = join(staging, process.platform === "win32" ? "bun.exe" : "bun");
      writeFileSync(staged, payload, { flag: "wx" }); chmodSync(staged, 0o755);
      if ((await captureOwnedCommandV1(staged,["--version"],staging,{control,env:store.environment,output:{maximumBytes:65536,maximumLines:1024}})).trim() !== version) throw new Error("Bun release version mismatch");
      writeFileSync(join(staging, ".toolchain.json"), JSON.stringify({ archive: row.sha256, executable: createHash("sha256").update(payload).digest("hex") }));
      checkResourceLeaseControl(control); renameSync(staging, directory); return executable;
    } finally { clearInterval(watch);clearInterval(progress); rmSync(staging, { recursive: true, force: true }); }
  }));
}
