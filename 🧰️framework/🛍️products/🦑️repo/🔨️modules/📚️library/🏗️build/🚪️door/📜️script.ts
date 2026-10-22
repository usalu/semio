import "../../../../../../🔨️modules/🏃️process/🏗️build/📤️output/🚪️entry/🟦️.ts";
import {isMainModuleV1} from "../../../../../../🔨️modules/⚙️runtime/🏁️main/🟦️.ts";
import {findBuildDoorRootV1,runBuildDoorV1} from "./🟦️.ts";
import {profileMarkV1,profilePhaseV1} from "../⏱️profile/🟨️.mjs";

if(isMainModuleV1(import.meta.url)){
 profileMarkV1("boot.main");
 const argv=process.argv.slice(2),root=findBuildDoorRootV1(import.meta.dirname);
 if(await runBuildDoorV1(root,argv,process.env)==="full"){const{runBuildCliV1}=await import("../📜️script.ts");await profilePhaseV1("cli.run",()=>runBuildCliV1(root,argv));}
}
