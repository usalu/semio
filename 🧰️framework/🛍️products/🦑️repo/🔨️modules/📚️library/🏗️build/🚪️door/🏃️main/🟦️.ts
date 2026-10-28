import "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🏗️build/📤️output/🚪️entry/🟦️.ts";
import {runBuildDoorV1,startBuildDoorLivenessV1} from "../🟦️.ts";
import {profileMarkV1,profilePhaseV1} from "../../⏱️profile/🟨️.mjs";

/** 🚪️ Runs the front door of one command line: what needs no build implementation is answered here, every other command is handed to the build script inside the restart supervisor. This is the body the `repo` entry script runs, and the body the sealed snapshot runs when the entry script starts it from the store.
 * @see ../../🏛️sealed/🚦️trampoline/🟦️.ts */
export async function runBuildDoorMainV1(root:string,argv:readonly string[],environment:NodeJS.ProcessEnv=process.env):Promise<void>{
 profileMarkV1("boot.main");
 if(await runBuildDoorV1(root,argv,environment)==="full"){const stopLiveness=startBuildDoorLivenessV1(environment);try{const{runBuildWithRestartV1}=await import("../🔄️restart/🟦️.ts");await profilePhaseV1("cli.run",()=>runBuildWithRestartV1(async()=>{const{runBuildCliV1}=await import("../../📜️script.ts");await runBuildCliV1(root,argv);},argv).catch(async error=>{const{reportRepoErrorV1}=await import("../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🏗️build/🗣️language/📣️progress/🟦️.ts");if(!reportRepoErrorV1(error))throw error;}));}finally{stopLiveness();}}
}
