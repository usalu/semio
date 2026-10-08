#!/usr/bin/env bun
import {randomUUID} from "node:crypto";
import {join} from "node:path";
import {acquireQueuedResourceLease} from "../../../../../../../🔨️modules/🏃️process/🔒️leases/🟦️.ts";
import {Script,ScriptRouter} from "../../../../../../../🔨️modules/🏃️process/🧭️routing/🟦️.ts";
import {getWorkspaceRoot} from "../../🟦️.ts";
import {cargoWorkspaceForManifest,prepareCargoOwners,publishCargoWorkspaceMembership,logCargoPreparationDiagnosticV1} from "../🟦️.ts";

/** 🛠️ Prepares the selected authored Cargo owner under its exclusive cancellable lease. */
export class PreparationScript extends Script {
  async run(args: string[]): Promise<void> {
    if(args.length<2 || args[0]!=="--manifest" || (args.length-2)%2 || args.slice(2).some((value,index)=>index%2===0?value!=="--package":!value))throw new Error("prepare --manifest <selected-scope> [--package <name>...]");
    const names=args.filter((_,index)=>index>=3 && index%2===1);
    const controller=new AbortController(), stop=():void=>controller.abort();process.once("SIGINT",stop);process.once("SIGTERM",stop);
    let lease: Awaited<ReturnType<typeof acquireQueuedResourceLease>> | undefined;
    try { const waiting=performance.now();lease=await acquireQueuedResourceLease({directory:join(this.root,".🧬semio/🦑️repo/⚡️cache/agents/resource-leases"),resource:`cargo-preparation:${this.root}`,mode:"exclusive",owner:randomUUID(),signal:controller.signal,onWait:process.env.SEMIO_CARGO_PREPARATION_TIMING==="1"?()=>logCargoPreparationDiagnosticV1("lease-wait",args[1]!,performance.now()-waiting,1):undefined});
    logCargoPreparationDiagnosticV1("lease",args[1]!,performance.now()-waiting,1);
    const scope=cargoWorkspaceForManifest(this.root,args[1]!);prepareCargoOwners(this.root,scope,names);const publishing=performance.now();const changed=publishCargoWorkspaceMembership(this.root,cargoWorkspaceForManifest(this.root,scope.manifest),"write");logCargoPreparationDiagnosticV1("publication",scope.manifest,performance.now()-publishing,Number(changed)); }
    finally { lease?.release();process.off("SIGINT",stop);process.off("SIGTERM",stop); }
  }
}

if(import.meta.main)await new ScriptRouter(getWorkspaceRoot()).register("prepare",PreparationScript).run(process.argv.slice(2));
