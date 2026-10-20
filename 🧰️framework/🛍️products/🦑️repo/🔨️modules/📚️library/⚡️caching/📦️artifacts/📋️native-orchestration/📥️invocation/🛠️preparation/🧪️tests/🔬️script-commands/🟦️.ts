import { withCompilerWorkspaceWriteV1 } from "../../../../../../💾️storage/🔒️enforcement/🏗️compiler/📏️capacity/🟦️.ts";
import { randomBytes } from "node:crypto";
import { cargoPreparationStorageFromEnvironmentV1 } from "../../../../../../../🗂️workspaces/🦀️cargo/🛠️preparation/📦️storage/🟦️.ts";
import { cargoPreparationControlV1, withRepositoryCargoPreparationV1 } from "../../🟦️.ts";
import { withPreparedCargoDependencyPairV1 } from "../../../../../../../🗂️workspaces/🦀️cargo/🛠️preparation/🟦️.ts";
import { runOwnedCommand, type OwnedCommandOptions } from "../../../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🎛️execution/🟦️.ts";
import { acquireQueuedResourceLease } from "../../../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🔒️leases/🟦️.ts";
import { createIssuedResourceControlV1 } from "../../../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { nativeCallerPhaseV1 } from "../../../🟦️.ts";
import { Script, findWorkspaceRoot } from "../../../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { getWorkspaceRoot } from "../../../../../../../🗂️workspaces/🟦️.ts";
import { cargoWorkspacePreparationInvocationV1, cargoWorkspaceForManifest, prepareCargoOwnersControlledV1, publishCargoWorkspaceMembership, logCargoPreparationDiagnosticV1 } from "../../../../../../../🗂️workspaces/🦀️cargo/🟦️.ts";
import { dirname as scriptModuleDirname, resolve as scriptModuleResolve } from "node:path";

/** 📍️ Location of the former `📜️script.ts` of this module, kept for code that resolves siblings of its entry. */
const scriptModulePath = scriptModuleResolve(import.meta.dir, "../../📜️script.ts");
const scriptModuleDirectory = scriptModuleDirname(scriptModulePath);



/** 🛠️ Prepares the selected Cargo owner under the original native caller and queued custody. */
export class PreparationScript extends Script {
 protected readonly environment:Readonly<Record<string,string|undefined>>=process.env;
 protected phase(){return nativeCallerPhaseV1(this.invocation,this.root,createIssuedResourceControlV1(this.invocation,"native:preparation-entry",0));}
 async run(args:string[]):Promise<void>{
  if(args.length<2||args[0]!=="--manifest"||(args.length-2)%2||args.slice(2).some((value,index)=>index%2===0?value!=="--package":!value))throw new Error("prepare --manifest <selected-scope> [--package <name>...]");
  const names=args.filter((_,index)=>index>=3&&index%2===1),phase=this.phase();await phase.advance("cargo:preparation");
  const waiting=performance.now();let lease:Awaited<ReturnType<typeof acquireQueuedResourceLease>>|undefined;
  try{lease=await acquireQueuedResourceLease({directory:cargoPreparationStorageFromEnvironmentV1(this.environment).leaseDirectory,resource:"cargo-preparation:"+this.root,mode:"exclusive",owner:randomBytes(16).toString("base64url"),signal:this.invocation.control.signal,control:phase.leaseControl,onWait:progress=>logCargoPreparationDiagnosticV1("lease-wait",args[1]!,progress.elapsedMs,1)});phase.budget();logCargoPreparationDiagnosticV1("lease",args[1]!,performance.now()-waiting,1);const scope=cargoWorkspaceForManifest(this.root,args[1]!);await prepareCargoOwnersControlledV1(this.root,scope,cargoPreparationControlV1(phase),names,cargoPreparationStorageFromEnvironmentV1(this.environment).directory);phase.budget();await phase.advance("cargo:publication");const publishing=performance.now(),changed=publishCargoWorkspaceMembership(this.root,cargoWorkspaceForManifest(this.root,scope.manifest),"write");phase.budget();logCargoPreparationDiagnosticV1("publication",scope.manifest,performance.now()-publishing,Number(changed));}
  catch(error){this.invocation.control.signal.throwIfAborted();throw error;}
  finally{lease?.release();}
 }
}

/** 🔗️ Synchronizes one unchanged owner pair and bounded identity through the original operation. */
export class SynchronizeScript extends Script {
 protected readonly environment:Readonly<Record<string,string|undefined>>=process.env;
 protected phase(){return nativeCallerPhaseV1(this.invocation,this.root,createIssuedResourceControlV1(this.invocation,"native:preparation-entry",0));}
 protected async execute(command:string,args:string[],cwd:string,label:string,budget:number,options:OwnedCommandOptions):Promise<void>{await runOwnedCommand(command,args,cwd,label,budget,options);}
 protected async capture(command:string,args:string[],cwd:string,label:string,budget:number,options:OwnedCommandOptions):Promise<string>{let output="";await runOwnedCommand(command,args,cwd,label,budget,{...options,stdout:"stderr",onLine:(line,stream)=>{if(stream==="stdout")output+=line+"\n";}});return output;}
 async run(args:string[]):Promise<void>{
  if(args[0]==="--cargo-arguments"){const request=cargoWorkspacePreparationInvocationV1(cargoPreparationStorageFromEnvironmentV1(this.environment),this.root,args.slice(1),process.cwd(),this.environment);if(!request)throw new Error("Selected exact Cargo preparation required");args=[...request.args.slice(2)];}
  if(args.length<2||args[0]!=="--manifest"||(args.length-2)%2||args.slice(2).some((value,index)=>index%2===0?value!=="--package":!value))throw new Error("synchronize --manifest <selected-owner> [--package <name>...]");
  const packages=args.filter((_,index)=>index>=3&&index%2===1);if(new Set(packages).size!==packages.length)throw new Error("Duplicate selected Cargo package");
  const phase=this.phase();await phase.advance("cargo:synchronization");const owner=cargoWorkspaceForManifest(this.root,args[1]!);
  try{await withPreparedCargoDependencyPairV1(cargoPreparationStorageFromEnvironmentV1(this.environment),this.root,owner.manifest,phase.leaseControl,async(argv,cwd,signal)=>{if(signal!==phase.leaseControl.signal)throw new Error("Original Cargo dependency signal required");await phase.advance("cargo:"+argv[0]);await this.execute("cargo",argv,cwd,"cargo:selected-dependencies",phase.budget(),phase.options(this.environment));phase.budget();},async(cwd,signal)=>{if(signal!==phase.leaseControl.signal)throw new Error("Original Cargo identity signal required");await phase.advance("cargo:identity");const identity=await this.capture("cargo",["--version"],cwd,"cargo:selected-identity",phase.budget(),phase.options(this.environment));phase.budget();return identity;},cargoPreparationControlV1(phase),[],packages);}
  catch(error){this.invocation.control.signal.throwIfAborted();throw error;}
 }
}


/** 🛡️ Runs `prepare` inside the bounded repository preparation scope that the entry opens before any command. */
export class BoundedPreparationScript extends Script {
  async run(args: string[]): Promise<void> {
    const root = getWorkspaceRoot();
    await withRepositoryCargoPreparationV1(findWorkspaceRoot(scriptModuleDirectory), root, this.invocation, createIssuedResourceControlV1(this.invocation, "native:preparation-entry", 0), process.env, async (environment, phase) => {
      class BoundedPreparation extends PreparationScript { protected override readonly environment = environment; protected override phase() { return phase; } }
      await new BoundedPreparation(root, findWorkspaceRoot(root), this.invocation).run(args);
    });
  }
}

/** 🔒️ Runs `synchronize` inside the bounded repository preparation scope, writing through the compiler workspace custody. */
export class BoundedSynchronizeScript extends Script {
  async run(args: string[]): Promise<void> {
    const root = getWorkspaceRoot();
    await withRepositoryCargoPreparationV1(findWorkspaceRoot(scriptModuleDirectory), root, this.invocation, createIssuedResourceControlV1(this.invocation, "native:preparation-entry", 0), process.env, async (environment, phase, store) => {
      class BoundedSynchronize extends SynchronizeScript {
        protected override readonly environment = environment;
        protected override phase() { return phase; }
        protected override execute(command: string, args: string[], cwd: string, label: string, budget: number, options: OwnedCommandOptions): Promise<void> { return withCompilerWorkspaceWriteV1(store.storage, cargoPreparationStorageFromEnvironmentV1(environment).leaseDirectory, phase.leaseControl, () => super.execute(command, args, cwd, label, budget, options)); }
      }
      await new BoundedSynchronize(root, findWorkspaceRoot(root), this.invocation).run(args);
    });
  }
}
