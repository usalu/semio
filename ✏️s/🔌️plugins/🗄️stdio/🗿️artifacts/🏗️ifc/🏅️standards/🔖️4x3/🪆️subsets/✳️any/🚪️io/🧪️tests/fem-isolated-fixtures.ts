/** 🧪️ Neutral analytical topology vectors use genuine engineering definitions and load/support semantics. */
import type {Fem3dSnapshot} from "../../../../../../../../../🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🌐️any/🧬️schema/📸️snapshot/🟦️.ts";
import {ifcReal} from "../step.ts";
interface ConnectivityCase {nodes:readonly string[];members:readonly {start:string;end:string}[];supports:readonly string[];loads:readonly string[];}
const word=(number:number)=>{const value=ifcReal(number);if(value.kind!=="real")throw Error("Expected real");return value.value;};

/** 🏗️ Build exactly the declared nodes and member endpoints without inventing absent topology. */
export function femConnectivityFixture(row:ConnectivityCase):Fem3dSnapshot {
 return {
  nodes:row.nodes.map((id,index)=>({id,x:word(index*2),y:word(0),z:word(0)})),
  elements:row.members.map((member,index)=>({kind:"frame",id:"member"+index,start:member.start,end:member.end,materialId:"steel",sectionId:"section",roll:word(0)})),
  materials:[{id:"steel",name:"Steel",e:word(2e11),g:word(8e10),nu:word(.25),rho:word(7850)}],
  sections:[{id:"section",name:"Section",area:word(.01),iy:word(1e-5),iz:word(2e-5),j:word(3e-5)}],solids:[],
  supports:row.supports.map(nodeId=>({id:"support-"+nodeId,nodeId,fixed:["Tx","Ty","Tz","Rx","Ry","Rz"]})),
  loadCases:row.loads.length?[{id:"load",name:"Load",selfWeight:false,loads:row.loads.map(nodeId=>({kind:"nodal",id:"load-"+nodeId,nodeId,dof:"Tz",value:word(-1000)}))}]:[],combinations:[],
  analysis:{modalCount:0n,bucklingCount:0n,deformationScale:word(1)}
 };
}
