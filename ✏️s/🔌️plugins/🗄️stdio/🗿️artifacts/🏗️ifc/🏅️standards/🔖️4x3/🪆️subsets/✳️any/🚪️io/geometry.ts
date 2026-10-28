import type {IfcSnapshot, IfcEntity, IfcValue} from "../🧬️schema/📸️snapshot/🟦️.ts";
import {IfcStepError, ifcNumber, ifcCheckpoint, type IfcIoControl} from "./step.ts";
import {captureIfcIoControl,captureIfcIoControlScope,drainIfcIoSteps,runIfcIoSteps,type IfcIoSteps} from "./control.ts";
import {ifcIsSubtype,ifc4x3Catalog} from "../🧬️schema/📚️catalog/🟦️.ts";
import {captureIfcExchangeGraph} from "./exchange-source.ts";
import {ifcRectangleExtrusionSteps} from "./geometry/rectangle.ts";
import {ifcPolylineExtrusionSteps} from "./geometry/polyline.ts";
import {ifcCircleExtrusionSteps} from "./geometry/circle.ts";
import {ifcRoundedRectangleExtrusionSteps} from "./geometry/rounded.ts";
import {ifcBooleanItem,IfcBooleanWork} from "./geometry/boolean.ts";
import {ifcFacetedBrepSteps} from "./geometry/faceted-brep.ts";
import {ifcPolygonalFaceSetSteps} from "./geometry/polygonal.ts";
import {ifcCsgSolidSteps} from "./geometry/csg.ts";
import {ifcFaceBasedSurfaceSteps} from "./geometry/surface.ts";
import {IfcTriangulatedFaceWork} from "./geometry/triangulated.ts";
import {IfcRepresentationCursor} from "./geometry/mapping.ts";
import {ifcMatrixOrientation} from "./orientation.ts";
import {captureIfcGeometryControl,type IfcGeometryControl} from "./geometry/control.ts";
import type {IfcIndexedQueryEntity} from "../../../../../📚️catalog/🔎️query/🟦️.ts";
import {ifcGeometryVectorSteps} from './geometry/vector.ts';
export type {IfcGeometryControl,IfcCurveSampling} from "./geometry/control.ts";

/** ⚠️ A conversion explicitly identifies omitted semantics and unsupported geometry. */
export interface IfcExchangeDiagnostic {code: string; severity: "warning" | "error"; message: string; entityIds: bigint[]}
/** 📦 Only completed conversions publish an artifact; fidelity never implies semantic equivalence. */
export interface IfcExchangeOutcome<T> {artifact?: T; fidelity: "lossless" | "geometry" | "partial" | "unsupported"; diagnostics: IfcExchangeDiagnostic[]}
/** 🔺 World coordinates in SI metres, IFC Z-up; indices are zero based. */
export interface IfcTriangleMesh {entityId: bigint; productId?: bigint; name: string; positions: [number, number, number][]; indices: number[];normals?:[number,number,number][];normalIndices?:number[]}
export type IfcDimensions = [number,number,number,number,number,number,number];
export interface IfcUnitMeasure {scale:number;dimensions:IfcDimensions}
const dimensions=(length=0,mass=0,time=0):IfcDimensions=>[length,mass,time,0,0,0,0];
const unitDimensions:Record<string,IfcDimensions>={LENGTHUNIT:dimensions(1),AREAUNIT:dimensions(2),VOLUMEUNIT:dimensions(3),MASSUNIT:dimensions(0,1),TIMEUNIT:dimensions(0,0,1),PLANEANGLEUNIT:dimensions(),FORCEUNIT:dimensions(1,1,-2),PRESSUREUNIT:dimensions(-1,1,-2),MASSDENSITYUNIT:dimensions(-3,1),MOMENTOFINERTIAUNIT:dimensions(4),MODULUSOFELASTICITYUNIT:dimensions(-1,1,-2),SHEARMODULUSUNIT:dimensions(-1,1,-2),LINEARFORCEUNIT:dimensions(0,1,-2),TORQUEUNIT:dimensions(2,1,-2)};
const measureDimensions:Record<string,IfcDimensions>={IFCLENGTHMEASURE:dimensions(1),IFCAREAMEASURE:dimensions(2),IFCVOLUMEMEASURE:dimensions(3),IFCMASSMEASURE:dimensions(0,1),IFCTIMEMEASURE:dimensions(0,0,1),IFCPLANEANGLEMEASURE:dimensions(),IFCFORCEMEASURE:dimensions(1,1,-2),IFCPRESSUREMEASURE:dimensions(-1,1,-2),IFCMODULUSOFELASTICITYMEASURE:dimensions(-1,1,-2),IFCMASSDENSITYMEASURE:dimensions(-3,1),IFCMOMENTOFINERTIAMEASURE:dimensions(4),IFCLINEARFORCEMEASURE:dimensions(0,1,-2),IFCTORQUEMEASURE:dimensions(2,1,-2)};
const geometryItemNames=new Set(['IFCTRIANGULATEDFACESET','IFCPOLYGONALFACESET','IFCFACETEDBREP','IFCFACETEDBREPWITHVOIDS','IFCCSGSOLID','IFCBOOLEANRESULT','IFCBOOLEANCLIPPINGRESULT','IFCFACEBASEDSURFACEMODEL','IFCEXTRUDEDAREASOLID']);
export const ifcEqualDimensions=(a:IfcDimensions,b:IfcDimensions):boolean=>a.every((n,i)=>n===b[i]);
type Vec = [number, number, number];
export type IfcMatrix = [number,number,number,number,number,number,number,number,number,number,number,number,number,number,number,number];
export const ifcIdentity = (): IfcMatrix => [1,0,0,0,0,1,0,0,0,0,1,0,0,0,0,1];
export function ifcMultiply(a: IfcMatrix, b: IfcMatrix): IfcMatrix {
  return Array.from({length:16},(_,i)=>[0,1,2,3].reduce((sum,k)=>sum+a[k*4+i%4]!*b[Math.floor(i/4)*4+k]!,0)) as IfcMatrix;
}
export function ifcTransform(m: IfcMatrix, p: Vec): Vec {return [0,1,2].map(i=>m[i]!*p[0]+m[4+i]!*p[1]+m[8+i]!*p[2]+m[12+i]!) as Vec;}
const cross = (a: Vec,b: Vec): Vec => [a[1]*b[2]-a[2]*b[1],a[2]*b[0]-a[0]*b[2],a[0]*b[1]-a[1]*b[0]];
/** 🧭️ Scale before normalizing finite axes so extreme original magnitudes remain usable. */
export const ifcNormalizeDirection = (v: Vec): Vec => {const scale=Math.max(...v.map(Math.abs));if(!scale||!Number.isFinite(scale))throw new IfcStepError("exchange.axis","Non-zero finite axis required");const scaled=v.map(n=>n/scale) as Vec,length=Math.hypot(...scaled);return scaled.map(n=>n/length) as Vec;};
export const ifcAggregate = (v: IfcValue|undefined): IfcValue[] => {if(v?.kind!=="aggregate")throw new IfcStepError("exchange.aggregate","Aggregate argument required");return v.value;};
export const ifcText = (v: IfcValue|undefined): string => v?.kind==="string"?v.value:"";

/** 🗺️ Schema-neutral graph queries provide reusable IFC placements and SI unit inference. */
export class IfcGeometryGraph {
  readonly #index = new Map<bigint,IfcIndexedQueryEntity<IfcValue>&{readonly entity:IfcEntity}>();
  private readonly projects:IfcEntity[]=[];
  private constructor(readonly snapshot: IfcSnapshot) {}
  /** 🗺️ Index only a centrally admitted private graph, with original cancellation between bounded row batches. */
  static async capture(source:IfcSnapshot,control:IfcIoControl={}):Promise<IfcExchangeOutcome<IfcGeometryGraph>> {
    control=captureIfcIoControl(control);
    const admitted=await captureIfcExchangeGraph(source,control);if(!admitted.artifact)return{fidelity:admitted.fidelity,diagnostics:admitted.diagnostics};
    const graph=new IfcGeometryGraph(admitted.artifact),batch=control.batchSize!,total=graph.snapshot.entities.length;
    for(let index=0;index<total;index++){
      if(index%batch===0)await ifcCheckpoint(control,'index-entities',index,total);
      const entity=graph.snapshot.entities[index]!;graph.#index.set(entity.id,Object.freeze({entity,ordinal:index}));if(entity.name==="IFCPROJECT"&&graph.projects.length<2)graph.projects.push(entity);
    }
    await ifcCheckpoint(control,'index-entities',total,total);
    return{artifact:graph,fidelity:'lossless',diagnostics:[]};
  }
  /** 📍️ Borrow the actual admitted row and immutable source ordinal for catalog-owned queries. */
  indexed(id:bigint){return this.#index.get(id);}
  /** 🔗️ Resolve only an actual captured entity identity through the canonical source index. */
  lookup(id:bigint):IfcEntity|undefined{return this.#index.get(id)?.entity;}
  ref(value: IfcValue|undefined): IfcEntity {
    const entity=value?.kind==="reference"?this.lookup(value.value):undefined;
    if(!entity)throw new IfcStepError("exchange.reference","Existing entity reference required");
    return entity;
  }
  vector(value: IfcValue|undefined): Vec {
    return drainIfcIoSteps(ifcGeometryVectorSteps(this,value));
  }
  axis(value: IfcValue|undefined): IfcMatrix {
    const axis=this.ref(value),dimension=axis.name==="IFCAXIS2PLACEMENT3D"?3:axis.name==="IFCAXIS2PLACEMENT2D"?2:0;
    if(!dimension||axis.args.length!==dimension||axis.complex?.length)throw new IfcStepError("exchange.placement","A simple Cartesian placement with its declared arguments is required");
    const coordinate=(value:IfcValue|undefined,kind:string):Vec=>{
      if(kind!=='IFCCARTESIANPOINT'&&kind!=='IFCDIRECTION')throw new IfcStepError('exchange.placement','A declared placement coordinate role is required');try{return drainIfcIoSteps(ifcGeometryVectorSteps(this,value,kind,dimension));}catch(error){if(error instanceof IfcStepError&&(error.code==="geometry.declaration"||error.code==="exchange.vector"||error.code==="geometry.domain"))throw new IfcStepError("exchange.placement",error.message);throw error;}
    };
    const origin=coordinate(axis.args[0],"IFCCARTESIANPOINT");
    let x:Vec,z:Vec;
    const direction=(value:IfcValue|undefined,fallback:Vec):Vec=>{
      if(value?.kind==="unset")return fallback;
      if(value?.kind!=="reference")throw new IfcStepError("exchange.placement","Optional placement directions must be explicit references or unset");
      return ifcNormalizeDirection(coordinate(value,"IFCDIRECTION"));
    };
    if(dimension===3) {
      if((axis.args[1]?.kind==="unset")!==(axis.args[2]?.kind==="unset"))throw new IfcStepError("exchange.placement","Axis and RefDirection must both be supplied or both be unset");
      z=direction(axis.args[1],[0,0,1]);x=direction(axis.args[2],[1,0,0]);
    } else {z=[0,0,1];x=direction(axis.args[1],[1,0,0]);}
    const y=ifcNormalizeDirection(cross(z,x));x=ifcNormalizeDirection(cross(y,z));
    return [...x,0,...y,0,...z,0,...origin,1] as IfcMatrix;
  }
  placement(value:IfcValue|undefined):IfcMatrix{return drainIfcIoSteps(this.placementSteps(value));}
  placementAsync(value:IfcValue|undefined,control:IfcIoControl={}):Promise<IfcMatrix>{return runIfcIoSteps(this.placementSteps(value),control,"placements");}
  private *placementSteps(value:IfcValue|undefined,active=new Set<bigint>()):IfcIoSteps<IfcMatrix>{
    yield;
    if(!value||value.kind==="unset")return ifcIdentity();
    const entity=this.ref(value);
    if(entity.name!=="IFCLOCALPLACEMENT")throw new IfcStepError("exchange.placement","Grid and linear placements need an explicit geometry evaluator");
    if(entity.args.length!==2||active.has(entity.id)||active.size>=256)throw new IfcStepError("exchange.placement-cycle","Cyclic, malformed or too deep local placement");
    active.add(entity.id);try{const parent=yield* this.placementSteps(entity.args[0],active);yield;return ifcMultiply(parent,this.axis(entity.args[1]));}finally{active.delete(entity.id);}
  }
  lengthScale():number{return drainIfcIoSteps(this.lengthScaleSteps());}
  lengthScaleAsync(control:IfcIoControl={}):Promise<number>{return runIfcIoSteps(this.lengthScaleSteps(),control,"units");}
  projectUnitsAsync(control:IfcIoControl={}):Promise<IfcEntity[]>{return runIfcIoSteps(this.projectUnitsSteps(),control,"units");}
  private *projectUnitsSteps():IfcIoSteps<IfcEntity[]>{
    yield;if(this.projects.length!==1)throw new IfcStepError("exchange.units","Exactly one project is required to resolve geometry units");
    const units=this.ref(this.projects[0]!.args[8]);
    if(units.name!=="IFCUNITASSIGNMENT"||units.args.length!==1)throw new IfcStepError("exchange.units","Declared project unit assignment required");
    const result:IfcEntity[]=[];
    for(const value of ifcAggregate(units.args[0])){
      yield;const unit=this.ref(value),named=ifcIsSubtype(unit.name,"IFCNAMEDUNIT");
      if(!named&&unit.name!=="IFCDERIVEDUNIT"&&unit.name!=="IFCMONETARYUNIT")throw new IfcStepError("exchange.units","Assignment members must be declared units");
      result.push(unit);
    }
    return result;
  }
  private *lengthScaleSteps():IfcIoSteps<number>{
    const units=yield* this.projectUnitsSteps();let selected:IfcEntity|undefined;
    for(const unit of units){yield;if(ifcIsSubtype(unit.name,"IFCNAMEDUNIT")&&unit.args[1]?.kind==="enum"&&unit.args[1].value==="LENGTHUNIT"){if(selected)throw new IfcStepError("exchange.units","One project length unit is required");selected=unit;}}
    if(!selected)throw new IfcStepError("exchange.units","One project length unit is required");
    return (yield* this.unitMeasureSteps(selected)).scale;
  }
  unitScale(unit:IfcEntity):number {return this.unitMeasure(unit).scale;}
  unitMeasure(unit:IfcEntity):IfcUnitMeasure{return drainIfcIoSteps(this.unitMeasureSteps(unit));}
  unitMeasureAsync(unit:IfcEntity,control:IfcIoControl={}):Promise<IfcUnitMeasure>{return runIfcIoSteps(this.unitMeasureSteps(unit),control,"units");}
  private *unitMeasureSteps(unit:IfcEntity,active=new Set<bigint>()):IfcIoSteps<IfcUnitMeasure>{
    yield;if(active.has(unit.id)||active.size>=64)throw new IfcStepError("exchange.units","Cyclic or too deep unit definition");
    active.add(unit.id);try{const type=unit.args[1];
    if(type?.kind!=="enum"||!unitDimensions[type.value])throw new IfcStepError("exchange.units","Unsupported physical unit type");
    const expected=unitDimensions[type.value]!,declaration=ifc4x3Catalog.types[unit.name==="IFCDERIVEDUNIT"?"IFCDERIVEDUNITENUM":"IFCUNITENUM"];let result:IfcUnitMeasure;
    if(!declaration?.members.includes(type.value)||unit.args.length!==4)throw new IfcStepError("exchange.units","Declared physical unit arguments required");
    if(unit.name==="IFCSIUNIT"&&unit.args[3]?.kind==="enum") {
      const names:Record<string,{type:string;base:number;power:number}>={METRE:{type:"LENGTHUNIT",base:1,power:1},SQUARE_METRE:{type:"AREAUNIT",base:1,power:2},CUBIC_METRE:{type:"VOLUMEUNIT",base:1,power:3},GRAM:{type:"MASSUNIT",base:.001,power:1},SECOND:{type:"TIMEUNIT",base:1,power:1},NEWTON:{type:"FORCEUNIT",base:1,power:1},PASCAL:{type:"PRESSUREUNIT",base:1,power:1},RADIAN:{type:"PLANEANGLEUNIT",base:1,power:1}};
      const named=names[unit.args[3].value],prefix=unit.args[2],prefixes:Record<string,number>={EXA:1e18,PETA:1e15,TERA:1e12,GIGA:1e9,MEGA:1e6,KILO:1e3,HECTO:1e2,DECA:10,DECI:.1,CENTI:.01,MILLI:.001,MICRO:1e-6,NANO:1e-9,PICO:1e-12,FEMTO:1e-15,ATTO:1e-18};
      if(!named||named.type!==type.value||(prefix?.kind!=="unset"&&(prefix?.kind!=="enum"||!(prefix.value in prefixes))))throw new IfcStepError("exchange.unit-dimensions","SI unit name, prefix and physical type disagree");
      result={scale:named.base*(prefix.kind==="enum"?prefixes[prefix.value]!:1)**named.power,dimensions:[...unitDimensions[named.type]!]};
    } else if(unit.name==="IFCDERIVEDUNIT") {
      result={scale:1,dimensions:dimensions()};const elements=ifcAggregate(unit.args[0]);if(!elements.length)throw new IfcStepError("exchange.units","Derived unit requires elements");
      for(const value of elements) {
        yield;const element=this.ref(value);if(element.name!=="IFCDERIVEDUNITELEMENT"||element.args.length!==2||element.args[1]?.kind!=="integer")throw new IfcStepError("exchange.units","Derived unit element and integral exponent required");
        const child=this.ref(element.args[0]);if(!ifcIsSubtype(child.name,"IFCNAMEDUNIT"))throw new IfcStepError("exchange.units","Derived elements require named units");
        const exponent=ifcNumber(element.args[1]);if(elements.length===1&&exponent===1)throw new IfcStepError("exchange.units","A sole exponent-one element must use its named unit");
        if(!Number.isSafeInteger(exponent)||Math.abs(exponent)>64)throw new IfcStepError("exchange.units","Unsupported derived unit exponent");
        const measure=yield* this.unitMeasureSteps(child,active);yield;
        result.scale*=measure.scale**exponent;result.dimensions=result.dimensions.map((n,i)=>n+measure.dimensions[i]!*exponent) as IfcDimensions;
      }
    } else if(unit.name==="IFCCONVERSIONBASEDUNIT") {
      const declaration=this.ref(unit.args[0]);if(declaration.name!=="IFCDIMENSIONALEXPONENTS"||declaration.args.length!==7||declaration.args.some(v=>v.kind!=="integer"))throw new IfcStepError("exchange.unit-dimensions","Seven declared dimension exponents required");
      const declared=declaration.args.map(ifcNumber) as IfcDimensions,factor=this.ref(unit.args[3]);if(factor.name!=="IFCMEASUREWITHUNIT"||factor.args.length!==2)throw new IfcStepError("exchange.units","Conversion factor requires measure with unit");
      yield;const nominal=factor.args[0],scalar=nominal?.kind==="typedValue"&&nominal.value.items.length===1?nominal.value.items[0]:nominal,measure=yield* this.unitMeasureSteps(this.ref(factor.args[1]),active);
      if(nominal?.kind!=="typedValue"||nominal.value.items.length!==1||!measureDimensions[nominal.value.name]||!ifcEqualDimensions(measureDimensions[nominal.value.name]!,declared))throw new IfcStepError("exchange.unit-dimensions","Conversion nominal measure must agree with declared physical dimensions");
      if(!ifcEqualDimensions(declared,measure.dimensions))throw new IfcStepError("exchange.unit-dimensions","Conversion dimensions disagree with factor unit");
      result={scale:ifcNumber(scalar)*measure.scale,dimensions:declared};
    } else throw new IfcStepError("exchange.units","Unsupported physical unit definition");
    if(!ifcEqualDimensions(result.dimensions,expected))throw new IfcStepError("exchange.unit-dimensions",`Physical dimensions disagree with ${type.value}`);
    if(!Number.isFinite(result.scale)||result.scale<=0)throw new IfcStepError("exchange.units","Positive finite unit scale required");
    yield;return result;}finally{active.delete(unit.id);}
  }
}

/** 🔍 Extract represented triangulations; unsupported representation items are never hidden. */
export async function extractIfc4x3Triangles(snapshot: IfcSnapshot,control: IfcGeometryControl={}): Promise<IfcExchangeOutcome<IfcTriangleMesh[]>> {
  const captured=captureIfcGeometryControl(control),scope=captureIfcIoControlScope(captured);control=scope.control;
  const admitted=await scope.run(()=>IfcGeometryGraph.capture(snapshot,control));if(!admitted.artifact)return{fidelity:admitted.fidelity,diagnostics:admitted.diagnostics};const graph=admitted.artifact;snapshot=graph.snapshot;
  await ifcCheckpoint(control,"geometry",0,snapshot.entities.length);
  const diagnostics:IfcExchangeDiagnostic[]=[],meshes:IfcTriangleMesh[]=[];
  let remainingValues=control.maxValues!;
  const publish=async(mesh:IfcTriangleMesh,frame:IfcMatrix,winding:'represented'|'outward')=>{const values=mesh.positions.length*3+mesh.indices.length+(mesh.normals?.length??0)*3+(mesh.normalIndices?.length??0);if(!Number.isSafeInteger(values)||values>remainingValues)throw new IfcStepError('geometry.limit','Geometry coordinates and indices exceed the original value grant');const sign=ifcMatrixOrientation(frame);if(!sign)throw new IfcStepError('geometry.placement','Finite nonsingular geometry frame required');if(sign<0&&winding==='represented'){function* reverse():IfcIoSteps<void>{for(let at=0;at<mesh.indices.length;at+=3){yield;[mesh.indices[at+1],mesh.indices[at+2]]=[mesh.indices[at+2]!,mesh.indices[at+1]!];if(mesh.normalIndices)[mesh.normalIndices[at+1],mesh.normalIndices[at+2]]=[mesh.normalIndices[at+2]!,mesh.normalIndices[at+1]!];}}await runIfcIoSteps(reverse(),control,'mesh-orientation');}remainingValues-=values;meshes.push(mesh);};
  let scale:number;
  try {scale=await graph.lengthScaleAsync(control);} catch(error) {scope.rethrowCallerFailure();if(error instanceof IfcStepError&&error.code==="io.cancelled")throw error;return {fidelity:"unsupported",diagnostics:[{code:"exchange.units",severity:"error",message:String(error),entityIds:[]}]};}
  const visitedItems=new Set<bigint>();
  const booleanWork=new IfcBooleanWork(control,captured.booleanEvaluation,captured.curveSampling);
  try{
  for(let i=0;i<snapshot.entities.length;i++) {
    if(i>0&&i%(control.batchSize!)===0)await ifcCheckpoint(control,"geometry",i,snapshot.entities.length);
    const product=snapshot.entities[i]!,definition=product.args[6];
    if(product.name==="IFCRELVOIDSELEMENT"){const ids=[product.id];let resolved=true;for(const ordinal of [4,5]){const value=product.args[ordinal];if(value?.kind==="reference"&&graph.lookup(value.value))ids.push(value.value);else resolved=false;}diagnostics.push({code:resolved?"geometry.opening-subtraction":"geometry.opening-reference",severity:resolved?"warning":"error",message:resolved?"Relational opening subtraction requires an explicit solid evaluator":"Opening relation requires existing host and opening references",entityIds:ids});continue;}
    if(!ifcIsSubtype(product.name,"IFCPRODUCT"))continue;
    if(definition?.kind!=="reference"||graph.lookup(definition.value)?.name!=="IFCPRODUCTDEFINITIONSHAPE")continue;
    try {
      const world=await graph.placementAsync(product.args[5],control);
      const shape=graph.ref(definition);
      const representations=ifcAggregate(shape.args[2]);
      for(let ri=0;ri<representations.length;ri++) {
        if(ri%(control.batchSize!)===0)await ifcCheckpoint(control,"representations",ri,representations.length);
        const representation=graph.ref(representations[ri]),context=graph.ref(representation.args[0]);
        let parent=context;
        if(context.name==="IFCGEOMETRICREPRESENTATIONSUBCONTEXT")parent=graph.ref(context.args[6]);
        const transform=parent.name==="IFCGEOMETRICREPRESENTATIONCONTEXT"?ifcMultiply(graph.axis(parent.args[4]),world):world;
        const items=ifcAggregate(representation.args[3]);
        for(let ii=0;ii<items.length;ii++) {
          if(ii%(control.batchSize!)===0)await ifcCheckpoint(control,"items",ii,items.length);
          const cursor=new IfcRepresentationCursor(graph,[items[ii]!],transform,control.maxDepth!,control.maxValues!);
          for(;;){const represented=await runIfcIoSteps(cursor.nextSteps(),control,'mappings');if(!represented)break;const {item,world:frame}=represented;visitedItems.add(item.id);
          if(item.name==='IFCFACETEDBREP'||item.name==='IFCFACETEDBREPWITHVOIDS'){const mesh=await runIfcIoSteps(ifcFacetedBrepSteps(graph,item,frame,scale,remainingValues),control,'facets');await publish({...mesh,entityId:item.id,productId:product.id,name:ifcText(product.args[2])},frame,'represented');continue;}
          if(item.name==='IFCPOLYGONALFACESET'){const mesh=await runIfcIoSteps(ifcPolygonalFaceSetSteps(graph,item,frame,scale,remainingValues),control,'polygonal');await publish({...mesh,entityId:item.id,productId:product.id,name:ifcText(product.args[2])},frame,'represented');continue;}
          if(ifcBooleanItem(graph,item)){const result=await booleanWork.evaluate(graph,item,frame,scale,remainingValues,visitedItems);if(result.kind==='empty')diagnostics.push({code:'geometry.empty-boolean',severity:'warning',message:'The regularized Boolean expression is explicitly empty',entityIds:[product.id,item.id]});else await publish({...result.mesh,entityId:item.id,productId:product.id,name:ifcText(product.args[2])},frame,'outward');continue;}
          if(item.name==='IFCCSGSOLID'){const mesh=await runIfcIoSteps(ifcCsgSolidSteps(graph,item,frame,scale,captured.curveSampling,remainingValues),control,'csg');await publish({...mesh,entityId:item.id,productId:product.id,name:ifcText(product.args[2])},frame,'represented');continue;}
          if(item.name==='IFCFACEBASEDSURFACEMODEL'){const mesh=await runIfcIoSteps(ifcFaceBasedSurfaceSteps(graph,item,frame,scale,remainingValues),control,'surfaces');await publish({...mesh,entityId:item.id,productId:product.id,name:ifcText(product.args[2])},frame,'represented');continue;}
          if(item.name==="IFCEXTRUDEDAREASOLID"){const profile=graph.ref(item.args[0]),steps=profile.name==="IFCRECTANGLEPROFILEDEF"?ifcRectangleExtrusionSteps(graph,item,frame,scale,remainingValues):profile.name==="IFCCIRCLEPROFILEDEF"?ifcCircleExtrusionSteps(graph,item,frame,scale,captured.curveSampling,remainingValues):profile.name==="IFCROUNDEDRECTANGLEPROFILEDEF"?ifcRoundedRectangleExtrusionSteps(graph,item,frame,scale,captured.curveSampling,remainingValues):ifcPolylineExtrusionSteps(graph,item,frame,scale),mesh=await runIfcIoSteps(steps,control,"extrusions");await publish({...mesh,entityId:item.id,productId:product.id,name:ifcText(product.args[2])},frame,'represented');continue;}
          if(item.name!=="IFCTRIANGULATEDFACESET") {diagnostics.push({code:"geometry.unsupported-item",severity:"warning",message:`${item.name} requires an explicit evaluator`,entityIds:[product.id,item.id]});continue;}
          const work=await runIfcIoSteps(IfcTriangulatedFaceWork.captureSteps(graph,item,frame,scale,remainingValues),control,'triangulation-source');
          await runIfcIoSteps(work.verticesSteps(),control,'vertices');
          await runIfcIoSteps(work.normalsSteps(),control,'triangulation-normals');
          await runIfcIoSteps(work.pnSteps(),control,'pn-indices');
          await runIfcIoSteps(work.trianglesSteps(),control,'triangles');
          await runIfcIoSteps(work.validationSteps(),control,'triangulation-validation');
          await publish({...work.mesh(),entityId:item.id,productId:product.id,name:ifcText(product.args[2])},frame,'represented');}
        }
        await ifcCheckpoint(control,"items",items.length,items.length);
      }
      await ifcCheckpoint(control,"representations",representations.length,representations.length);
    } catch(error) {
      scope.rethrowCallerFailure();
      if(error instanceof IfcStepError&&error.code==="io.cancelled")throw error;
      diagnostics.push({code:"geometry.unsupported-product",severity:"error",message:String(error),entityIds:[product.id]});
    }
  }
  const unbound:bigint[]=[];
  for(let index=0;index<snapshot.entities.length;index++){if(index%(control.batchSize!)===0)await ifcCheckpoint(control,"geometry-unbound",index,snapshot.entities.length);const entity=snapshot.entities[index]!;if(geometryItemNames.has(entity.name)&&!visitedItems.has(entity.id))unbound.push(entity.id);}
  await ifcCheckpoint(control,"geometry-unbound",snapshot.entities.length,snapshot.entities.length);
  if(unbound.length)diagnostics.push({code:"geometry.unbound-item",severity:"warning",message:"Unbound geometry items have no product placement",entityIds:unbound});
  await ifcCheckpoint(control,"geometry",snapshot.entities.length,snapshot.entities.length);
  return {artifact:meshes,fidelity:diagnostics.length?"partial":"geometry",diagnostics};
  }finally{booleanWork.close();}
}
