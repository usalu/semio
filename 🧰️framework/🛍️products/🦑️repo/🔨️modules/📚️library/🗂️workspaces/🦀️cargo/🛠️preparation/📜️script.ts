#!/usr/bin/env bun
import {randomUUID} from "node:crypto";
import {join} from "node:path";
import {acquireQueuedResourceLease} from "../../../../../../../🔨️modules/🏃️process/🔒️leases/🟦️.ts";
import {Script,ScriptRouter} from "../../../../../../../🔨️modules/🏃️process/🧭️routing/🟦️.ts";
import {getWorkspaceRoot} from "../../🟦️.ts";
import {cargoWorkspaceForManifest,prepareCargoOwners,publishCargoWorkspaceMembership} from "../🟦️.ts";

/** 🛠️ Prepares the selected authored Cargo owner under its exclusive cancellable lease. */
export class PreparationScript extends Script {
  async run(args: string[]): Promise<void> {
    if(args.length!==2 || args[0]!=="--manifest")throw new Error("prepare --manifest <selected-scope>");
    const controller=new AbortController(), stop=():void=>controller.abort();process.once("SIGINT",stop);process.once("SIGTERM",stop);
    let lease: Awaited<ReturnType<typeof acquireQueuedResourceLease>> | undefined;
    try { lease=await acquireQueuedResourceLease({directory:join(this.root,".🧬semio/🦑️repo/⚡️cache/agents/resource-leases"),resource:`cargo-preparation:${this.root}`,mode:"exclusive",owner:randomUUID(),signal:controller.signal});
    const scope=cargoWorkspaceForManifest(this.root,args[1]!);prepareCargoOwners(this.root,scope);publishCargoWorkspaceMembership(this.root,cargoWorkspaceForManifest(this.root,scope.manifest),"write"); }
    finally { lease?.release();process.off("SIGINT",stop);process.off("SIGTERM",stop); }
  }
}

if(import.meta.main)await new ScriptRouter(getWorkspaceRoot()).register("prepare",PreparationScript).run(process.argv.slice(2));
