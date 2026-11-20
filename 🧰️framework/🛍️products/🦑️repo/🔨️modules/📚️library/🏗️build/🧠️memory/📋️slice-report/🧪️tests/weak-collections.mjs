import {heapStats} from "bun:jsc";
import {captureObservedSourceInputsV1} from "../../../👁️observation/🟨️.mjs";
import vector from "./weak-collections.json" with {type:"json"};

const collect=async()=>{
 for(let pass=0;pass<8;pass++){await Bun.sleep(10);Bun.gc(true);}
};
await collect();
const before=heapStats().objectTypeCounts.WeakRef??0;
for(let scope=0;scope<vector.shortLivedScopes;scope++)await captureObservedSourceInputsV1(()=>null);
await collect();
console.log(JSON.stringify({retained:Math.max(0,(heapStats().objectTypeCounts.WeakRef??0)-before)}));
