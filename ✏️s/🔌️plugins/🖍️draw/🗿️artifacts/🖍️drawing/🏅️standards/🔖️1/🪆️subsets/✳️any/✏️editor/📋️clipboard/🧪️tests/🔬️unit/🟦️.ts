/** 🧪️ Neutral clipboard placements are independently composed with Three.js. */
import {expect,test} from "bun:test";
import {Matrix3} from "three";
import {binary64} from "../../../../../../../../../../../../../🧰️framework/🔨️modules/🌱️value/🔢️ieee754/🟦️.ts";
import Ajv from "ajv";
import cases from "../../🧫️fixtures/🔣️.json";
import {clipboardPasteMatrix,remapDrawingClipboard,DrawingClipboardValidationJob,type DrawingClipboard,type ClipboardMatrix} from "../../🟦️.ts";
import {Graph,alg} from "graphlib";
import {CLIPBOARD_TEXT_MAX_BYTES,CLIPBOARD_METADATA_MAX_WIRE_BYTES,CLIPBOARD_FRAGMENT_MAX_WIRE_BYTES,CLIPBOARD_PASTE_MAX_WIRE_BYTES,clipboardJsonStringBytes} from "../../../../../../../../../../../../../🧰️framework/🔨️modules/🎠️kernel/📋️clipboard/🟦️.ts";
import envelopeSchema from "../../../../../../../../../../../../../🧰️framework/🔨️modules/🎠️kernel/📋️clipboard/🧬️schema/🔣️.json";
import {parseDrawingArtifact,parseDrawingLayerNode} from "../../../../🧬️schema/🟦️.ts";
import {importImageAssetDiff,foldImageAssetDelta} from "../../../../../🧱️structure/🧬️schema/🧬️mutations/📥️import-image-asset/🦠️mutation/🟦️.ts";
import {removeImageAssetDiff,removeImageAssetInverse} from "../../../../../🧱️structure/🧬️schema/🧬️mutations/🗑️remove-image-asset/🦠️mutation/🟦️.ts";
import jsonPatch from "fast-json-patch";
import completionCustody from "../../../📬️completion/🧫️fixtures/🔣️.json";
import completionCustodySchema from "../../../📬️completion/🧬️schema/🔣️.json";
import completionFaultCases from "../../../../../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📬️completion/⚠️fault/🧫️fixtures/🔣️.json";
import completionFaultSchema from "../../../../../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📬️completion/⚠️fault/🧬️schema/🔣️.json";
import {createCompletionFault} from "../../../../../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📬️completion/⚠️fault/🟦️.ts";
import assetBefore from "../../../../../🧱️structure/🧫️fixtures/🧬️mutations/📥️import-image-asset/➕️adds/📸️snapshot/⬅️before/🔣️.json";
import assetAfter from "../../../../../🧱️structure/🧫️fixtures/🧬️mutations/📥️import-image-asset/➕️adds/📸️snapshot/➡️after/🔣️.json";
import assetDiff from "../../../../../🧱️structure/🧫️fixtures/🧬️mutations/📥️import-image-asset/➕️adds/🔺️diff/🔣️.json";
import schema from "../../🧬️schema/🔣️.json";
import documentSchema from "../../../../🧬️schema/🔣️.json";
const matrix=(value:ClipboardMatrix)=>new Matrix3().set(value[0],value[2],value[4],value[1],value[3],value[5],0,0,1);
test("completion fault projection retains original diagnostic while matching independent JSON wire",()=>{
 expect(new Ajv({strict:false}).compile(completionFaultSchema)(completionFaultCases)).toBe(true);
 for(const row of completionFaultCases.cases){const original={...row,origin:"app" as const,severity:"error" as const,scope:{module:"draw"},causes:[{message:"original retained cause"}]};const actual=createCompletionFault(original);expect(actual.original).toBe(original);expect(actual.original.scope).toBe(original.scope);const wire=JSON.parse(actual.report) as {message:string;code:string};let expected="";for(const scalar of original.message){const candidate=expected+scalar;if(Buffer.byteLength(JSON.stringify(candidate))-2>completionFaultCases.messageEscapeBytes)break;expected=candidate;}expect(wire.message).toBe(expected);const code=Buffer.byteLength(JSON.stringify(row.code))-2<=completionFaultCases.codeEscapeBytes?row.code:"interactive-job.fault-capacity";expect(wire.code).toBe(code);expect(actual.report).toBe(JSON.stringify({origin:"app",code,severity:"error",message:expected,scope:{},retryable:row.retryable}));expect(Buffer.byteLength(actual.report)).toBeLessThanOrEqual(completionFaultCases.frameBytes);}
});

/** 📬️ JSON Patch independently preserves custody until the shared exact release boundary. */
test("completion neutral custody corpus agrees with JSON Patch removal oracle",()=>{
 const validate=new Ajv({strict:false}).compile(completionCustodySchema);expect(validate(completionCustody)).toBe(true);
 for(const capacity of completionCustody.vacantCapacities){
  const original={owner:{capacity,live:[]},released:0};
  for(const grant of [0,capacity-1,capacity]){
   const admitted=grant>=capacity;
   const oracle=jsonPatch.applyPatch(structuredClone(original),admitted?[{op:"remove",path:"/owner"},{op:"replace",path:"/released",value:capacity}]:[]).newDocument;
   expect(Object.hasOwn(oracle,"owner")).toBe(!admitted);expect(oracle.released).toBe(admitted?capacity:0);expect(original.owner.capacity).toBe(capacity);
  }
 }
 for(const count of completionCustody.aliasCounts){
  const original={aliases:Array.from({length:count},(_,index)=>index),payload:{bytes:completionCustody.bodyBytes}};
  let oracle:Record<string,unknown>=structuredClone(original);
  for(let index=count-1;index>=0;index--){oracle=jsonPatch.applyPatch(oracle,[{op:"remove",path:`/aliases/${index}`},...index===0?[{op:"remove" as const,path:"/payload"}]:[]]).newDocument;expect(Object.hasOwn(oracle,"payload")).toBe(index!==0);}
  expect(original.payload.bytes).toBe(completionCustody.bodyBytes);expect(original.aliases).toHaveLength(count);
 }
 expect(completionCustody.unsupportedRetains).toBe(true);
});
for(const row of cases.dependencyCases)test(`clipboard dependency admission: ${row.name}`,()=>{
 const base={name:"Fixture",visible:true,locked:false,opacity:binary64(1),blendMode:"normal",transform:{x:binary64(0),y:binary64(0),scaleX:binary64(1),scaleY:binary64(1),rotation:binary64(0),shear:binary64(0)},attributes:{fillRule:"nonzero"}};
 const byId=new Map(row.nodes.map(node=>[node.id,node]));
 const layer=(id:string):ReturnType<typeof parseDrawingLayerNode>=>{const node=byId.get(id)!;return parseDrawingLayerNode({...base,id,kind:node.kind,...node.kind==="boolean"?{operation:"union",children:node.edges}:node.kind==="group"?{children:node.edges.map(layer),isolation:false}:{shapeKind:"rect",rect:{x:binary64(0),y:binary64(0),width:binary64(10),height:binary64(20)}}});};
 const packet:DrawingClipboard={schema:"drawing.clipboard.v1",roots:row.roots.map(layer),selected:row.selected,assets:{}},before=structuredClone(packet),job=new DrawingClipboardValidationJob(packet);
 let accepted=true,turns=0;try{while(!job.advance(1).done){turns++;expect(()=>job.result()).toThrow();expect(turns).toBeLessThan(1000);}job.result();}catch{accepted=false;}
 const graph=new Graph({directed:true});for(const node of row.nodes)graph.setNode(node.id);for(const node of row.nodes)for(const id of node.edges)graph.setEdge(node.id,id);
 const complete=row.nodes.every(node=>node.edges.every(id=>byId.has(id)));expect(accepted).toBe(row.accepted);expect(accepted).toBe(complete&&alg.isAcyclic(graph));expect(packet).toEqual(before);
 const cancelled=new DrawingClipboardValidationJob(packet);cancelled.advance(1);cancelled.cancel();expect(()=>cancelled.advance(1)).toThrow(/cancel/i);expect(()=>cancelled.result()).toThrow();expect(packet).toEqual(before);
});
test("clipboard envelope admits a self-produced maximum body after exact JSON escaping",()=>{
 const validate=new Ajv({strict:true}).compile(envelopeSchema);expect(validate({maximumTextBytes:CLIPBOARD_TEXT_MAX_BYTES,maximumMetadataWireBytes:CLIPBOARD_METADATA_MAX_WIRE_BYTES,maximumFragmentWireBytes:CLIPBOARD_FRAGMENT_MAX_WIRE_BYTES,maximumPasteWireBytes:CLIPBOARD_PASTE_MAX_WIRE_BYTES})).toBe(true);
 for(const row of cases.envelopeCases){const scalarBytes=new TextEncoder().encode(row.scalar).length,text=row.scalar.repeat(row.bodyBytes/scalarBytes);expect(new TextEncoder().encode(text).length).toBe(CLIPBOARD_TEXT_MAX_BYTES);const escaped=JSON.stringify(text);expect(clipboardJsonStringBytes(text)).toBe(new TextEncoder().encode(escaped).length);expect(clipboardJsonStringBytes(text)).toBe(text.length*row.escapedBytesPerScalar+2);
  const fragment={schema:"drawing.clipboard.v1",mediaType:{class:"twoD",form:"design"},dslText:text,sourceApp:"drawing.play",label:"1024 layers / Ebenen"},wire=JSON.stringify(fragment);expect(new TextEncoder().encode(wire).length).toBeLessThanOrEqual(CLIPBOARD_FRAGMENT_MAX_WIRE_BYTES);const input={fragment,anchor:"original",position:[0,0,0],parentId:"destination"};expect(new TextEncoder().encode(JSON.stringify(input)).length).toBeLessThanOrEqual(CLIPBOARD_PASTE_MAX_WIRE_BYTES);expect(JSON.parse(JSON.stringify(input)).fragment.dslText).toBe(text);
 }
 for(const text of ["😀","\\","\b","\u2028","\ud800"])expect(clipboardJsonStringBytes(text)).toBe(new TextEncoder().encode(JSON.stringify(text)).length);
 console.log("[DEBUG] Clipboard maximum self-produced body fits shared fragment and paste wire envelopes");
});
for(const sample of cases.placements)test(`clipboard world placement: ${sample.name}`,()=>{
 const source:ClipboardMatrix=[2,0,0,3,30,40],destination:ClipboardMatrix=[1.7,0.3,0.8,-0.5,-7,8];
 const offset:[number,number]=sample.position?[sample.position[0]!,sample.position[1]!]:[16,16];
 const local=clipboardPasteMatrix(source,destination,offset)!;
 const oracle=matrix(destination).invert().multiply(new Matrix3().makeTranslation(...offset)).multiply(matrix(source));
 matrix(local).elements.forEach((value,index)=>expect(value).toBeCloseTo(oracle.elements[index]!,12));
 const world=matrix(destination).multiply(matrix(local));expect(world.elements[6]).toBeCloseTo(sample.expected[0]!,12);expect(world.elements[7]).toBeCloseTo(sample.expected[1]!,12);
});
for(const sample of cases.booleanPlacements)test(`clipboard Boolean placement: ${sample.name}`,()=>{
 const parent=sample.sourceParent as ClipboardMatrix,source=sample.sourceLocal as ClipboardMatrix,destination=sample.destination as ClipboardMatrix,offset=sample.offset as [number,number];
 const displacement=matrix(parent).multiply(matrix(source)).multiply(matrix(parent).invert());expect(displacement.elements[6]).toBeCloseTo(sample.worldDisplacement[4]!,12);
 const local=clipboardPasteMatrix(sample.worldDisplacement as ClipboardMatrix,destination,offset,true)!;
 const oracle=matrix(destination).invert().multiply(new Matrix3().makeTranslation(...offset)).multiply(displacement).multiply(matrix(destination));matrix(local).elements.forEach((value,index)=>expect(value).toBeCloseTo(oracle.elements[index]!,12));local.forEach((value,index)=>expect(value).toBeCloseTo(sample.expectedLocal[index]!,12));
});
test("clipboard destination singularity and nonfinite placement are rejected",()=>{
 expect(clipboardPasteMatrix([1,0,0,1,0,0],[1,2,2,4,0,0],[0,0])).toBeNull();
 expect(clipboardPasteMatrix([1,0,0,1,0,0],[1,0,0,1,0,0],[Infinity,0])).toBeNull();
});
test("clipboard schema rejects foreign versions and duplicate selection identities",()=>{
 const ajv=new Ajv({strict:false});ajv.addSchema(documentSchema);const validate=ajv.compile(schema);
 const shape={kind:"shape",id:"a",name:"A",visible:true,locked:false,opacity:1,blendMode:"normal",transform:{x:0,y:0,scaleX:1,scaleY:1,rotation:0,shear:0},attributes:{fillRule:"nonzero"},shapeKind:"rect",rect:{x:0,y:0,width:10,height:20}};
 expect(validate({schema:"drawing.clipboard.v1",roots:[shape],selected:["a"],assets:{}})).toBe(true);
 expect(validate({schema:"drawing.clipboard.v1",roots:[{}],selected:["a"],assets:{}})).toBe(false);
 expect(validate({schema:"unknown",roots:[{}],selected:["a"],assets:{}})).toBe(false);
 expect(validate({schema:"drawing.clipboard.v1",roots:[{}],selected:["a","a"],assets:{}})).toBe(false);
});
test("neutral image asset deltas and inverses agree with independent JSON Patch",()=>{
 const before=parseDrawingArtifact(assetBefore),after=parseDrawingArtifact(assetAfter),asset=after.assets.bitmap!;
 const delta=importImageAssetDiff(before,{assetId:"bitmap",asset});expect(delta).toEqual(assetDiff.assets);
 const actual=foldImageAssetDelta(before,delta);expect(actual).toEqual(after);
 expect(jsonPatch.applyPatch(structuredClone(before),[{op:"add",path:"/assets/bitmap",value:asset}]).newDocument).toEqual(actual);
 const removal=removeImageAssetDiff(actual,{assetId:"bitmap"});expect(foldImageAssetDelta(actual,removal)).toEqual(before);
 expect(jsonPatch.applyPatch(structuredClone(actual),[{op:"remove",path:"/assets/bitmap"}]).newDocument).toEqual(before);
 const inverse=removeImageAssetInverse(actual,{assetId:"bitmap"});expect(foldImageAssetDelta(before,importImageAssetDiff(before,inverse))).toEqual(actual);
 expect(()=>importImageAssetDiff(actual,{assetId:"bitmap",asset})).toThrow();expect(()=>importImageAssetDiff(before,{assetId:"invalid",asset:{...asset,width:2}})).toThrow();
});
test("clipboard metadata membership agrees with independent JSON Patch",()=>{
 const actual=new Set<string>();let oracle:Record<string,boolean>={};
 for(const row of cases.metadataCases){const before=actual.size,added=row.operation==="insert"&&!actual.has(row.key);if(row.operation==="insert"){actual.add(row.key);if(added)oracle=jsonPatch.applyPatch(oracle,[{op:"add",path:"/"+row.key,value:true}]).newDocument;}else{actual.delete(row.key);oracle=jsonPatch.applyPatch(oracle,[{op:"remove",path:"/"+row.key}]).newDocument;}expect(added).toBe(row.added);expect([...actual].sort()).toEqual(row.live);expect(Object.keys(oracle).sort()).toEqual(row.live);if(row.operation==="insert"&&!added)expect(actual.size).toBe(before);}
});
test("clipboard remaps cross-root Boolean and image identities without mutating the packet",()=>{
 const base={name:"Fixture",visible:true,locked:false,opacity:binary64(1),blendMode:"normal",transform:{x:binary64(0),y:binary64(0),scaleX:binary64(1),scaleY:binary64(1),rotation:binary64(0),shear:binary64(0)},attributes:{fillRule:"nonzero"}};
 const shape=(id:string)=>parseDrawingLayerNode({...base,kind:"shape",id,shapeKind:"rect",rect:{x:binary64(0),y:binary64(0),width:binary64(10),height:binary64(20)}});
 const packet:DrawingClipboard={schema:"drawing.clipboard.v1",roots:[shape("a"),shape("b"),parseDrawingLayerNode({...base,kind:"boolean",id:"result",operation:"difference",children:["a","b"]}),parseDrawingLayerNode({...base,kind:"image",id:"image",imageKey:"bitmap",width:binary64(1),height:binary64(1)})],selected:["result","image"],assets:{bitmap:parseDrawingArtifact(assetAfter).assets.bitmap!}};
 const before=structuredClone(packet),identities=new Map(packet.roots.map(layer=>[layer.id,layer.id+"-clone"])),assets=new Map([["bitmap","bitmap-clone"]]);const mapped=remapDrawingClipboard(packet,identities,assets)!;
 expect(packet).toEqual(before);expect(mapped.selected).toEqual(["result-clone","image-clone"]);const boolean=mapped.roots[2]!;expect(boolean.kind==="boolean"&&boolean.children).toEqual(["a-clone","b-clone"]);const image=mapped.roots[3]!;expect(image.kind==="image"&&image.imageKey).toBe("bitmap-clone");expect(mapped.assets["bitmap-clone"]).toEqual(packet.assets.bitmap);
 const oracle=jsonPatch.applyPatch(structuredClone(packet),[{op:"replace",path:"/roots/0/id",value:"a-clone"},{op:"replace",path:"/roots/1/id",value:"b-clone"},{op:"replace",path:"/roots/2/id",value:"result-clone"},{op:"replace",path:"/roots/2/children",value:["a-clone","b-clone"]},{op:"replace",path:"/roots/3/id",value:"image-clone"},{op:"replace",path:"/roots/3/imageKey",value:"bitmap-clone"},{op:"replace",path:"/selected",value:["result-clone","image-clone"]},{op:"move",from:"/assets/bitmap",path:"/assets/bitmap-clone"}]).newDocument;expect(mapped).toEqual(oracle);
});
