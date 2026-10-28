import {liveImplementationStartV1} from "../🏛️sealed/🧭️mode/🟦️.ts";
import {realpathSync} from "node:fs";

/** 🏁️ Tells whether this file is the process entry point; the door cannot import the runtime seam without leaving its startup module budget, so it compares the real paths itself, exactly as the seam does for every other module. */
const mainEntryV1=():boolean=>{const entry=process.argv[1];if(entry===undefined||entry==="")return false;try{const canonical=(path:string):string=>process.platform==="win32"?realpathSync(path).toLowerCase():realpathSync(path);return canonical(entry)===canonical(import.meta.filename);}catch{return false;}};

if(mainEntryV1()){
 const argv=process.argv.slice(2),runLive=async(root:string)=>{const{runBuildDoorMainV1}=await import("./🏃️main/🟦️.ts");await runBuildDoorMainV1(root,argv,process.env);};
 if(liveImplementationStartV1(process.env)){const{findBuildDoorRootV1}=await import("./🟦️.ts");await runLive(findBuildDoorRootV1(import.meta.dirname));}
 else{
  const[{findSealedRootV1},{startImplementationV1}]=await Promise.all([import("../🏛️sealed/🟦️.ts"),import("../🏛️sealed/🚦️trampoline/🟦️.ts")]),root=findSealedRootV1(import.meta.dirname);
  await startImplementationV1(root,argv,process.env,()=>runLive(root));
 }
}
