import "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🏗️build/📤️output/🚪️entry/🟦️.ts";
import {isMainModuleV1} from "../../../../../../📦️packages/🟦️typescript/⚙️runtime/🏁️main/🟦️.ts";
import {findBuildDoorRootV1,runBuildDoorV1} from "./🟦️.ts";
import {profileMarkV1,profilePhaseV1} from "../⏱️profile/🟨️.mjs";

if(isMainModuleV1(import.meta.url)){
 profileMarkV1("boot.main");
 const argv=process.argv.slice(2),root=findBuildDoorRootV1(import.meta.dirname);
 if(await runBuildDoorV1(root,argv,process.env)==="full"){const{runBuildCliWithRestartV1}=await import("../📜️script.ts");await profilePhaseV1("cli.run",()=>runBuildCliWithRestartV1(root,argv).catch(async error=>{const{reportRepoErrorV1}=await import("../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🏗️build/🗣️language/📣️progress/🟦️.ts");if(!reportRepoErrorV1(error))throw error;}));}
}
