#!/usr/bin/env bun
import {randomBytes} from "node:crypto";
import {cargoPreparationStorageFromEnvironmentV1} from "../../../../../🗂️workspaces/🦀️cargo/🛠️preparation/📦️storage/🟦️.ts";
import {cargoPreparationControlV1} from "./🟦️.ts";
import {withPreparedCargoDependencyPairV1} from "../../../../../🗂️workspaces/🦀️cargo/🛠️preparation/🟦️.ts";
import {runOwnedCommand,type OwnedCommandOptions} from "../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🎛️execution/🟦️.ts";
import {acquireQueuedResourceLease} from "../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🔒️leases/🟦️.ts";
import {receiveScriptProcessInvocation} from "../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import {nativeCallerPhaseV1} from "../🟦️.ts";
import {Script,ScriptRouter} from "../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import {getWorkspaceRoot} from "../../../../../🗂️workspaces/🟦️.ts";
import {cargoWorkspaceForManifest,prepareCargoOwnersControlledV1,publishCargoWorkspaceMembership,logCargoPreparationDiagnosticV1} from "../../../../../🗂️workspaces/🦀️cargo/🟦️.ts";

/** 🛠️ Prepares the selected Cargo owner under the original native caller and queued custody. */
export class PreparationScript extends Script {
 async run(args:string[]):Promise<void>{
  if(args.length<2||args[0]!=="--manifest"||(args.length-2)%2||args.slice(2).some((value,index)=>index%2===0?value!=="--package":!value))throw Error("prepare --manifest <selected-scope> [--package <name>...]");
  const names=args.filter((_,index)=>index>=3&&index%2===1),phase=nativeCallerPhaseV1(this.invocation,this.root);await phase.advance("cargo:preparation");
  const waiting=performance.now();let lease:Awaited<ReturnType<typeof acquireQueuedResourceLease>>|undefined;
  try{lease=await acquireQueuedResourceLease({directory:cargoPreparationStorageFromEnvironmentV1(process.env).directory,resource:"cargo-preparation:"+this.root,mode:"exclusive",owner:randomBytes(16).toString("base64url"),signal:this.invocation.control.signal,control:phase.leaseControl});phase.budget();logCargoPreparationDiagnosticV1("lease",args[1]!,performance.now()-waiting,1);const scope=cargoWorkspaceForManifest(this.root,args[1]!);await prepareCargoOwnersControlledV1(this.root,scope,cargoPreparationControlV1(phase),names);phase.budget();await phase.advance("cargo:publication");const publishing=performance.now(),changed=publishCargoWorkspaceMembership(this.root,cargoWorkspaceForManifest(this.root,scope.manifest),"write");phase.budget();logCargoPreparationDiagnosticV1("publication",scope.manifest,performance.now()-publishing,Number(changed));}
  catch(error){this.invocation.control.signal.throwIfAborted();throw error;}
  finally{lease?.release();}
 }
}

/** 🔗️ Synchronizes one unchanged owner pair and bounded identity through the original operation. */
export class SynchronizeScript extends Script {
 protected async execute(command:string,args:string[],cwd:string,label:string,budget:number,options:OwnedCommandOptions):Promise<void>{await runOwnedCommand(command,args,cwd,label,budget,options);}
 protected async capture(command:string,args:string[],cwd:string,label:string,budget:number,options:OwnedCommandOptions):Promise<string>{let output="";await runOwnedCommand(command,args,cwd,label,budget,{...options,stdout:"stderr",onLine:(line,stream)=>{if(stream==="stdout")output+=line+"\n";}});return output;}
 async run(args:string[]):Promise<void>{
  if(args.length<2||args[0]!=="--manifest"||(args.length-2)%2||args.slice(2).some((value,index)=>index%2===0?value!=="--package":!value))throw Error("synchronize --manifest <selected-owner> [--package <name>...]");
  const packages=args.filter((_,index)=>index>=3&&index%2===1);if(new Set(packages).size!==packages.length)throw Error("Duplicate selected Cargo package");
  const phase=nativeCallerPhaseV1(this.invocation,this.root);await phase.advance("cargo:synchronization");const owner=cargoWorkspaceForManifest(this.root,args[1]!);
  try{await withPreparedCargoDependencyPairV1(cargoPreparationStorageFromEnvironmentV1(process.env),this.root,owner.manifest,phase.leaseControl,async(argv,cwd,signal)=>{if(signal!==this.invocation.control.signal)throw Error("Original Cargo dependency signal required");await phase.advance("cargo:"+argv[0]);await this.execute("cargo",argv,cwd,"cargo:selected-dependencies",phase.budget(),phase.options(process.env));phase.budget();},async(cwd,signal)=>{if(signal!==this.invocation.control.signal)throw Error("Original Cargo identity signal required");await phase.advance("cargo:identity");const identity=await this.capture("cargo",["--version"],cwd,"cargo:selected-identity",phase.budget(),phase.options(process.env));phase.budget();return identity;},cargoPreparationControlV1(phase),[],packages);}
  catch(error){this.invocation.control.signal.throwIfAborted();throw error;}
 }
}


if(import.meta.main) await receiveScriptProcessInvocation(process.env,original=>new ScriptRouter(getWorkspaceRoot()).register("prepare",PreparationScript).register("synchronize",SynchronizeScript).run(process.argv.slice(2),original));
