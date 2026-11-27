import {existsSync,writeFileSync} from "node:fs";
import {join} from "node:path";
import {acquireProductLaneV1} from "../../🟦️.ts";

const [root,key,marks]=process.argv.slice(2) as [string,string,string],lane=await acquireProductLaneV1(root,{key,crates:["same"],quotaBytes:Number.MAX_SAFE_INTEGER,owner:{session:"0123456789ab.h",agent:"holder"}},{signal:new AbortController().signal});
writeFileSync(join(marks,"holder-ready"),"1");
while(!existsSync(join(marks,"holder-release")))await new Promise(done=>setTimeout(done,20));
lane.release();
