import {countedCompilerCapacityRequestV1} from "../../🏗️build/🎟️capacity/🟦️.ts";
import { basename,dirname,join,resolve } from "node:path";
import { mkdirSync,mkdtempSync,rmSync } from "node:fs";
import { withCompilerCapacityScopeV1, compilerAdmissionBudgetV1, requestedCompilerAdmissionControlV1, type CompilerAdmissionControl } from "../../🏗️build/🎟️capacity/🟦️.ts";
import { readBuildMode,type BuildMode } from "../../🏗️build/🎚️mode/🟦️.ts";
import { runOwnedCommand } from "../../🎛️owned-execution/🟦️.ts";
import { stageArtifacts } from "../📤️publication/🟦️.ts";
import { validateJsonSchemaSubset } from "../../../🧬️schema/✅️validator/📦️packages/🟦️typescript/🟦️.ts";
import schema from "./🧬️schema/🔣️.json" with { type:"json" };

/** 🐹️ Declares one Go compiler's intent, source owner, artifact custody and deadline. */
export type GoArtifactBuildPolicyV1=Readonly<{version:1;mode:BuildMode;cwd:string;captureDirectory:string;leaseDirectory:string;budgetMs:number}>;

/** 🎚️ Owns Go optimization flags so caller arguments cannot silently override the selected mode. */
export function goBuildArgumentsV1(mode:BuildMode,args:readonly string[]):string[]{
 readBuildMode(mode);
 if(args.some(argument=>/^-+(?:gcflags|ldflags|trimpath|p)(?:=|$)/u.test(argument)))throw Error("Go optimization and worker flags belong to the build policy");
 return [...(mode==="release"?["-trimpath","-ldflags=-s -w"]:["-gcflags=all=-N -l"]),...args.map(argument=>argument==="..."?"./...":argument)];
}

/** 📦️ Captures one Go deliverable privately and publishes it after the compiler releases host capacity. */
export async function buildGoArtifactsV1(input:readonly string[],policy:GoArtifactBuildPolicyV1,options:Readonly<{control:CompilerAdmissionControl;environment?:Readonly<Record<string,string|undefined>>}>):Promise<void>{
 const errors=validateJsonSchemaSubset(schema,policy);if(errors.length)throw Error(`Invalid Go build policy: ${errors.join("; ")}`);
 compilerAdmissionBudgetV1(options.control);
 const args:string[]=[];let output:string|undefined;
 for(let index=0;index<input.length;index++){const argument=input[index]!;if(argument==="-o"){if(output||!input[index+1])throw Error("Go build requires one output");output=resolve(policy.cwd,input[++index]!);}else if(argument.startsWith("-o=")){if(output)throw Error("Go build requires one output");output=resolve(policy.cwd,argument.slice(3));}else args.push(argument);}
 const configured=goBuildArgumentsV1(policy.mode,args),control=requestedCompilerAdmissionControlV1(options.control,policy.budgetMs),signal=control.signal;
 mkdirSync(policy.captureDirectory,{recursive:true});const staging=mkdtempSync(join(policy.captureDirectory,"go-")),artifact=output?join(staging,basename(output)):undefined;
 try{
  await withCompilerCapacityScopeV1({request:countedCompilerCapacityRequestV1(),directory:policy.leaseDirectory,control,environment:options.environment},async(capacity,control)=>{const environment={...(options.environment??process.env),SEMIO_COMPILER_PARALLELISM:String(capacity.parallelism),GOMAXPROCS:String(capacity.parallelism),GOTMPDIR:staging};
  await runOwnedCommand("go",["build",`-p=${capacity.parallelism}`,...(artifact?["-o",artifact]:[]),...configured],policy.cwd,"go:build",compilerAdmissionBudgetV1(control),{env:environment,signal:control.signal});
  });
  if(output&&artifact)await stageArtifacts(dirname(output),`${resolve(policy.cwd)}:go:${policy.mode}`,new Map([[basename(output),artifact]]),{signal,leaseDirectory:policy.leaseDirectory});
 }finally{rmSync(staging,{recursive:true,force:true});}
}
