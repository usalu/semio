/** 📽️ Orthographic straight and circular trim-edge projection over firstparty Part21 values. */
import type {Part21Document,Part21Value,Part21Instance} from "../../../../../../../../📇️registry/🧬️contract/📐️part21/🟦️.ts";
export interface ProjectedCurve{sourceId:bigint;points:[number,number,number][]}
export interface EdgeProjection{curves:ProjectedCurve[];unitToPoints:number;issues:string[]}
type V=[number,number,number];
const record=(instance:Part21Instance|undefined,kind:string)=>instance?.entities.find(entity=>entity.name.toUpperCase()===kind)?.args;
const ref=(value:Part21Value|undefined)=>value?.kind==="reference"?value.value:undefined;
const number=(value:Part21Value|undefined)=>value?.kind==="real"||value?.kind==="integer"?Number(value.value):undefined;
const enumValue=(value:Part21Value|undefined)=>value?.kind==="enum"?value.value:undefined;
const dot=(a:V,b:V)=>a[0]*b[0]+a[1]*b[1]+a[2]*b[2];
const cross=(a:V,b:V):V=>[a[1]*b[2]-a[2]*b[1],a[2]*b[0]-a[0]*b[2],a[0]*b[1]-a[1]*b[0]];
const norm=(a:V):V=>{const length=Math.sqrt(dot(a,a));if(!Number.isFinite(length)||length<=1e-12)throw Error("step.projection.placement");return a.map(value=>value/length)as V;};
/** 🧭️ Samples at most64segments peredge and converts SI length units into typographic points. */
export function projectEdges(doc:Part21Document):EdgeProjection{
    const instances=new Map(doc.instances.map(instance=>[instance.id,instance]));
    const get=(id:bigint|undefined)=>id===undefined?undefined:instances.get(id);
    const coords=(id:bigint|undefined,kind:string):V=>{const values=record(get(id),kind)?.[1];if(values?.kind!=="aggregate")throw Error("step.projection.vertex");const x=number(values.value[0]),y=number(values.value[1]),z=number(values.value[2])??0;if(x===undefined||y===undefined)throw Error("step.projection.vertex");return[x,y,z];};
    const vertex=(id:bigint|undefined)=>coords(ref(record(get(id),"VERTEX_POINT")?.[1]),"CARTESIAN_POINT");
    const basis=(circle:Part21Value[])=>{const place=get(ref(circle[1]));const two=record(place,"AXIS2_PLACEMENT_3D")===undefined;const placement=record(place,"AXIS2_PLACEMENT_3D")??record(place,"AXIS2_PLACEMENT_2D");if(!placement)throw Error("step.projection.placement");
                const center=coords(ref(placement[1]),"CARTESIAN_POINT"),z=norm(two||ref(placement[2])===undefined?[0,0,1]:coords(ref(placement[2]),"DIRECTION"));const axis=ref(placement[two?2:3])===undefined?[1,0,0]as V:coords(ref(placement[two?2:3]),"DIRECTION");const component=dot(axis,z);const x=norm([axis[0]-component*z[0],axis[1]-component*z[1],axis[2]-component*z[2]]),y=cross(z,x);
                const radius=number(circle[2]);if(radius===undefined||!Number.isFinite(radius)||radius<=0)throw Error("step.projection.radius");
return{center,x,y,radius};};
    let scale:number|undefined;
    for(const instance of doc.instances.filter(instance=>record(instance,"LENGTH_UNIT"))){const args=record(instance,"SI_UNIT");if(!args||enumValue(args[1])!=="METRE")throw Error("step.projection.length-unit");const prefix=enumValue(args[0]);const factors:Record<string,number>={MILLI:0.001,CENTI:0.01,DECI:0.1,MICRO:0.000001,KILO:1000};const factor=prefix===undefined?1:factors[prefix];if(factor===undefined)throw Error("step.projection.length-prefix");if(scale!==undefined&&Math.abs(scale-factor)>1e-12)throw Error("step.projection.mixed-units");scale=factor;}
    const output:EdgeProjection={curves:[],unitToPoints:(scale??0.0254/72)*72/0.0254,issues:[]};
    let pointCount=0;const add=(sourceId:bigint,points:V[])=>{if(points.some(point=>point.some(value=>!Number.isFinite(value))))throw Error("step.projection.non-finite");pointCount+=points.length;if(pointCount>131072)throw Error("step.projection.point-limit");output.curves.push({sourceId,points:points.map(point=>point.map(value=>value*output.unitToPoints)as V)});};
    for(const instance of doc.instances.filter(instance=>record(instance,"EDGE_CURVE"))){
        if(output.curves.length+output.issues.length>=16384)throw Error("step.projection.edge-limit");
        try{
            const args=record(instance,"EDGE_CURVE")!;const start=vertex(ref(args[1])),end=vertex(ref(args[2]));const curve=get(ref(args[3]));let points:V[];
            if(record(curve,"LINE"))points=[start,end];else{
                const circle=record(curve,"CIRCLE");if(!circle)throw Error("step.projection.unsupported-curve:"+(curve?.entities[0]?.name??"?"));
                const {center,x,y,radius}=basis(circle);
                const angle=(point:V)=>{const relative:V=[point[0]-center[0],point[1]-center[1],point[2]-center[2]];return Math.atan2(dot(relative,y),dot(relative,x));};const first=angle(start),last=angle(end),forward=enumValue(args[4])==="T",tau=2*Math.PI,mod=(value:number)=>((value%tau)+tau)%tau;const distance=Math.hypot(start[0]-end[0],start[1]-end[1],start[2]-end[2]);const sweep=distance<=radius*1e-9?(forward?tau:-tau):forward?mod(last-first):-mod(first-last);const segments=Math.max(1,Math.min(64,Math.ceil(Math.abs(sweep)/(Math.PI/32))));points=Array.from({length:segments+1},(_,index)=>{const a=first+sweep*index/segments;return center.map((value,dimension)=>value+radius*(Math.cos(a)*x[dimension]!+Math.sin(a)*y[dimension]!))as V;});
            }
            add(instance.id,points);
        }catch(error){if(error instanceof Error&&error.message==="step.projection.point-limit")throw error;output.issues.push("#"+instance.id+":"+(error instanceof Error?error.message:String(error)));}
    }
    const seen=new Set<bigint>();for(const set of doc.instances.filter(instance=>record(instance,"GEOMETRIC_CURVE_SET"))){const values=record(set,"GEOMETRIC_CURVE_SET")![1];if(values?.kind!=="aggregate")throw Error("step.projection.curve-set");for(const value of values.value){const id=ref(value);if(id===undefined)throw Error("step.projection.curve");if(seen.has(id))continue;seen.add(id);if(output.curves.length+output.issues.length>=16384)throw Error("step.projection.edge-limit");try{const instance=get(id),polyline=record(instance,"POLYLINE"),circle=record(instance,"CIRCLE");let points:V[];if(polyline){const refs=polyline[1];if(refs?.kind!=="aggregate"||refs.value.length<2||refs.value.length>1025)throw Error("step.projection.polyline-limit");points=refs.value.map(value=>coords(ref(value),"CARTESIAN_POINT"));}else if(circle){const{center,x,y,radius}=basis(circle);points=Array.from({length:65},(_,index)=>{const angle=2*Math.PI*index/64;return center.map((value,axis)=>value+radius*(Math.cos(angle)*x[axis]+Math.sin(angle)*y[axis]))as V;});}else throw Error("step.projection.unsupported-curve:"+(instance?.entities[0]?.name??"?"));add(id,points);}catch(error){if(error instanceof Error&&error.message==="step.projection.point-limit")throw error;output.issues.push("#"+id+":"+(error instanceof Error?error.message:String(error)));}}}
    if(doc.instances.some(instance=>record(instance,"MAPPED_ITEM")||record(instance,"ITEM_DEFINED_TRANSFORMATION")))output.issues.push("step.projection.assembly-occurrence-transforms-not-applied");
    if(scale===undefined)output.issues.push("step.projection.unspecified-unit:one-coordinate-unit-is-one-point");return output;
}
