#!/usr/bin/env bun
import { randomUUID } from "node:crypto";
import { acquireQueuedResourceLease } from "../../../../../../🔨️modules/🏃️process/🔒️leases/🟦️.ts";
import { join, relative, isAbsolute } from "node:path";
import { existsSync, lstatSync, readFileSync } from "node:fs";
import { runtimeInputAdmissionV1 } from "../../🕸️dependencies/🧩️runtime/🟨️.mjs";
import { Script, ScriptRouter } from "../../../../../../🔨️modules/🏃️process/🧭️routing/🟦️.ts";
import { runRepositoryCommand } from "../../🏃️process/🎛️owned-execution/🟦️.ts";
import { repoTestArtifactEnvironment } from "../../🏃️process/🌿️environment/🧪️test-output/🟦️.ts";
import { getWorkspaceRoot } from "../🟦️.ts";
import { discoverCargoWorkspaces, publishCargoWorkspaceMembership, cargoWorkspaceForManifest, cargoRepositoryPackages, prepareCargoOwners } from "./🟦️.ts";

/** 📣️ Checks or publishes current native owner membership through its authored regular-manifest recipe. */
class MembersScript extends Script {
  async run(args: string[]): Promise<void> {
    if (args.length !== 1 || !["--check", "--write"].includes(args[0]!)) throw new Error("members --check|--write");
    for (const owner of discoverCargoWorkspaces(this.root)) {
      const written = publishCargoWorkspaceMembership(this.root, owner, args[0] === "--write" ? "write" : "check");
      console.log(`[cargo-members] ${owner.manifest} ${written ? "published" : "current"}`);
    }
  }
}

/** 🧪️ Executes schema, independent oracle and native owner-removal laws under the existing source-test budget. */
class PreparationScript extends Script {
  async run(args: string[]): Promise<void> {
    if(args.length!==2 || args[0]!=="--manifest")throw new Error("prepare --manifest <selected-scope>");
    const controller=new AbortController(), stop=():void=>controller.abort();process.once("SIGINT",stop);process.once("SIGTERM",stop);
    let lease: Awaited<ReturnType<typeof acquireQueuedResourceLease>> | undefined;
    try { lease=await acquireQueuedResourceLease({directory:join(this.root,".🧬semio/🦑️repo/⚡️cache/agents/resource-leases"),resource:`cargo-preparation:${this.root}`,mode:"exclusive",owner:randomUUID(),signal:controller.signal});
    const scope=cargoWorkspaceForManifest(this.root,args[1]!);prepareCargoOwners(this.root,scope);publishCargoWorkspaceMembership(this.root,cargoWorkspaceForManifest(this.root,scope.manifest),"write"); }
    finally { lease?.release();process.off("SIGINT",stop);process.off("SIGTERM",stop); }
  }
}

class RuntimeInputScript extends Script {
  async run(args: string[]): Promise<void> {
    if(args.length!==1)throw new Error("runtime-input-check requires one source request");
    const request=JSON.parse(Buffer.from(args[0]!,"base64").toString("utf8"));
    if(!request || typeof request!=="object" || Object.keys(request).some(k=>!["component","appScoped","sources"].includes(k)) || typeof request.component!=="string" || typeof request.appScoped!=="boolean" || !Array.isArray(request.sources) || request.sources.some((path:unknown)=>typeof path!=="string" || isAbsolute(path) || path.split("/").includes("..") || path.includes("\\")))throw new Error("Invalid selected runtime input request");
    const components=cargoRepositoryPackages(this.root).flatMap(pkg=>{const document=Bun.TOML.parse(readFileSync(join(this.root,pkg.manifest),"utf8")) as any,metadata=document.package?.metadata;if(!["plugin","extension"].includes(metadata?.semio?.["component-kind"]))return [];const row=metadata.semio;return [{...row,pluginId:metadata.component.package.slice(6),dependsOn:[...(row.extends?[row.extends]:[]),...(row["depends-on"]??[])]}];});
    const admission=runtimeInputAdmissionV1(components,[{id:request.component,appScoped:request.appScoped}],request.sources,(path:string)=>{let current=this.root;for(const part of path.split("/").filter(Boolean)){current=join(current,part);if(!existsSync(current) || lstatSync(current).isSymbolicLink())return false;}return lstatSync(current).isFile();});
    if(admission.status!=="admitted")throw new Error(`Selected runtime input refused: ${JSON.stringify(admission.missing)}`);
    console.log(`[runtime-input] ${request.component} ${admission.selected.length} admitted components`);
  }
}
class RuntimeContractScript extends Script {
 async run(args:string[]):Promise<void>{if(args.length)throw new Error("runtime-contract-check accepts no overrides");await runRepositoryCommand("bun",["test",join(import.meta.dir,"../../🕸️dependencies/🧩️runtime/📥️admission/🧪️tests/🟦️.ts")],this.root,"runtime-input-contract",15_000);}
}

class NativeInputScript extends Script {
  async run(args: string[]): Promise<void> {
    if (args.length !== 2 || args[0] !== "--manifest") throw new Error("native-input-check --manifest <current-owner>");
    await runRepositoryCommand("cargo", ["metadata", "--no-deps", "--offline", "--locked", "--format-version", "1", "--manifest-path", join(this.root, args[1]!)], this.root, "native-input-admission", 30_000);
  }
}

class ContractScript extends Script {
  async run(args: string[]): Promise<void> {
    if (args.length) throw new Error("contract-check accepts no overrides");
    await runRepositoryCommand("bun", ["test", join(import.meta.dir, "🧪️tests/🟦️.ts")], this.root, "cargo-workspace-contract", 15_000);
  }
}
/** 🕰️ Verifies queued preparation with an independent owner holding the native lease. */
class QueuedContractScript extends Script {
  async run(args: string[]): Promise<void> {
    if (args.length) throw new Error("queued-contract-check accepts no overrides");
    await runRepositoryCommand("bun", ["test", join(import.meta.dir, "🧪️tests/🕰️queued-preparation/🟦️.ts")], this.root, "cargo-queued-preparation-contract", 45_000, { env: repoTestArtifactEnvironment(this.root, "cargo-queued-preparation-contract") });
  }
}
class BunContractScript extends Script {
  async run(args: string[]): Promise<void> { if(args.length)throw new Error("bun-contract-check accepts no overrides");await runRepositoryCommand("bun",["test",join(import.meta.dir,"../🟦️bun/🧪️tests/🟦️.ts")],this.root,"bun-workspace-contract",15_000); }
}
class CapabilityContractScript extends Script {
  async run(args: string[]): Promise<void> {
    if (args.length) throw new Error("capability-contract-check accepts no overrides");
    const { runCargoCapabilityContributionChecks, runCargoCapabilityPhysicalChecks } = await import("./🧩️capabilities/🧪️tests/🟦️.ts");
    console.log(`cargo capability contract: ${runCargoCapabilityContributionChecks() + runCargoCapabilityPhysicalChecks()} laws`);
  }
}
if (import.meta.main) await new ScriptRouter(getWorkspaceRoot()).register("capability-contract-check", CapabilityContractScript).register("runtime-contract-check",RuntimeContractScript).register("runtime-input-check",RuntimeInputScript).register("bun-contract-check",BunContractScript).register("members", MembersScript).register("prepare",PreparationScript).register("contract-check", ContractScript).register("queued-contract-check", QueuedContractScript).register("native-input-check", NativeInputScript).run(process.argv.slice(2));
