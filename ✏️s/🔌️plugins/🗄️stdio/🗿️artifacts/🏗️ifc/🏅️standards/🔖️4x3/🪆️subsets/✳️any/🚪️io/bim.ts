import {ifcSourceEntriesSteps,ifcPrepareLevelsSteps,type IfcPreparedLevels} from "./source-rows.ts";
import {IfcExchangeRelations} from "./relations.ts";
import {appendBimPhasesSteps,readBimPhasesSteps} from "./bim/phases/🟦️.ts";
import {appendBimOccurrencePropertiesSteps,readBimOccurrencePropertiesSteps} from "./bim/properties/🟦️.ts";
import type {ModelSnapshot} from "../../../../../../../../🏙️bim/🗿️artifacts/🏢️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🟦️.ts";
import type {IfcSnapshot,IfcValue} from "../🧬️schema/📸️snapshot/🟦️.ts";
import {IfcGeometryGraph,ifcAggregate,ifcText,ifcIdentity,type IfcExchangeOutcome,type IfcExchangeDiagnostic} from "./geometry.ts";
import {createIfc4x3ExchangeGraph} from "./gltf.ts";
import {ifcReal,ifcNumber,ifcCheckpoint,IfcStepError,type IfcIoControl} from "./step.ts";
import {captureIfcIoControlScope,runIfcIoSteps} from "./control.ts";
import {captureIfcExternalSource} from "./external-source.ts";
import {IfcEditError} from "../🧬️schema/🔺️diff/🟦️.ts";

const unset:IfcValue={kind:"unset"},a=(value:IfcValue[]):IfcValue=>({kind:"aggregate",value}),s=(value:string):IfcValue=>({kind:"string",value}),e=(value:string):IfcValue=>({kind:"enum",value}),integer=(value:number):IfcValue=>{if(!Number.isSafeInteger(value))throw new IfcStepError("bim.level","Integral storey level required");return {kind:"integer",value:BigInt(value)};};
const refuse=(message:string):IfcExchangeOutcome<never>=>({fidelity:"unsupported",diagnostics:[{code:"bim.unsupported",severity:"error",message,entityIds:[]}]});
const compound=(degrees:number):IfcValue=>{if(!Number.isFinite(degrees))throw new IfcStepError("bim.angle","Finite geographic angle required");let micro=Math.round(Math.abs(degrees)*3600e6);const sign=degrees<0?-1:1,whole=Math.floor(micro/3600e6);micro%=3600e6;const minutes=Math.floor(micro/60e6);micro%=60e6;return a([whole,minutes,Math.floor(micro/1e6),micro%1e6].map(n=>integer(sign*n)));};
const angle=async(value:IfcValue|undefined,control:IfcIoControl)=>{if(!value||value.kind==="unset")throw new IfcStepError("bim.angle","Explicit latitude and longitude required");const parts:number[]=[];for(const item of ifcAggregate(value)){if(parts.length%(control.batchSize!)===0)await ifcCheckpoint(control,"bim-angle",parts.length,0);parts.push(ifcNumber(item));}return parts[0]!+parts[1]!/60+parts[2]!/3600+(parts[3]??0)/3600e6;};

/** 📇️ BIM identity-bearing source records comprise the project and each supplied family entry. */
function* bimSourceRecords(model:ModelSnapshot):Generator<unknown> {
  const record=(value:unknown):Record<string,unknown>=>{if(value===null||typeof value!=="object"||Array.isArray(value))throw new IfcEditError("invalidValue","BIM source records require objects");return value as Record<string,unknown>;};
  const root=record(model);yield record(root.project);
  for(const field in root) {
    if(field==="schema"||field==="project"||root[field]===undefined)continue;
    const family=record(root[field]);for(const id in family)yield record(family[id]);
  }
}

/** 🏛️ Exchange authored BIM spatial structure; unsupported element families remain explicit. */
export async function bimSpatialToIfc4x3(model:ModelSnapshot,control:IfcIoControl={}):Promise<IfcExchangeOutcome<IfcSnapshot>> {
  const scope=captureIfcIoControlScope(control);control=scope.control;
  const captured=await scope.run(()=>captureIfcExternalSource(model,control,bimSourceRecords));if(!captured.artifact)return{fidelity:captured.fidelity,diagnostics:captured.diagnostics};model=captured.artifact;
  await ifcCheckpoint(control,"bim-spatial",0,1);
  const graph=createIfc4x3ExchangeGraph(model.project.name),{add}=graph,diagnostics:IfcExchangeDiagnostic[]=[];let guid=2;
  const root=(name:string,description=""):IfcValue[]=>[s((guid++).toString().padStart(22,"0")),unset,s(name),s(description)];
  const relation=(parent:IfcValue,children:IfcValue[])=>{if(children.length)add("IFCRELAGGREGATES",[...root(""),parent,a(children)]);};
  const placement=(parent:IfcValue,xyz:number[],rotation=0):IfcValue=>{const point=add("IFCCARTESIANPOINT",[a(xyz.map(ifcReal))]),direction=rotation?add("IFCDIRECTION",[a([Math.cos(rotation),Math.sin(rotation),0].map(ifcReal))]):unset,normal=rotation?add("IFCDIRECTION",[a([0,0,1].map(ifcReal))]):unset,axis=add("IFCAXIS2PLACEMENT3D",[point,normal,direction]);return add("IFCLOCALPLACEMENT",[parent,axis]);};
  const project=graph.snapshot.entities.find(row=>row.name==="IFCPROJECT")!;project.args[3]=s(model.project.description);project.args[6]=unset;
  graph.snapshot.header.fileName[2]=a([s(model.project.author)]);graph.snapshot.header.fileName[3]=a([s(model.project.organization)]);
  const projectRef:IfcValue={kind:"reference",value:project.id},sites=new Map<string,{entity:IfcValue;placement:IfcValue}>(),buildings=new Map<string,{entity:IfcValue;placement:IfcValue}>(),holders=new Map<string,IfcValue|null>();
  const holder=(id:string,entity:IfcValue)=>holders.set(id,holders.has(id)?null:entity);
  try {
    const siteRows=await runIfcIoSteps(ifcSourceEntriesSteps(model.sites),control,"bim-source-records"),buildingRows=await runIfcIoSteps(ifcSourceEntriesSteps(model.buildings),control,"bim-source-records"),storeyRows=await runIfcIoSteps(ifcSourceEntriesSteps(model.storeys),control,"bim-source-records");
    for(let i=0;i<siteRows.length;i++){if(i%(control.batchSize!)===0)await ifcCheckpoint(control,"bim-north",i,siteRows.length);if(siteRows[i]![1].true_north!==siteRows[0]![1].true_north)throw new IfcStepError("bim.north","Multiple site north directions need distinct representation contexts");}
    if(siteRows.length){const north=add("IFCDIRECTION",[a([-Math.sin(siteRows[0]![1].true_north),Math.cos(siteRows[0]![1].true_north)].map(ifcReal))]);graph.snapshot.entities.find(row=>row.name==="IFCGEOMETRICREPRESENTATIONCONTEXT")!.args[5]=north;}
    for(const [id,site] of siteRows) {
      await ifcCheckpoint(control,"bim-sites",sites.size,siteRows.length);
      const place=placement(unset,[0,0,0]),entity=add("IFCSITE",[...root(site.name),s(id),place,unset,unset,e("ELEMENT"),compound(site.latitude),compound(site.longitude),ifcReal(site.elevation),unset,unset]);sites.set(id,{entity,placement:place});
      holder(id,entity);
      if(site.boundary.length)diagnostics.push({code:"bim.site-boundary",severity:"warning",message:"Site boundary curves are omitted by spatial projection",entityIds:entity.kind==="reference"?[entity.value]:[]});
    }
    const siteChildren:IfcValue[]=[];for(const row of sites.values()){if(siteChildren.length%(control.batchSize!)===0)await ifcCheckpoint(control,"bim-site-relations",siteChildren.length,sites.size);siteChildren.push(row.entity);}relation(projectRef,siteChildren);
    for(const [id,building] of buildingRows) {
      await ifcCheckpoint(control,"bim-buildings",buildings.size,buildingRows.length);
      const parent=sites.get(building.site);if(!parent)throw new IfcStepError("bim.site","Building references missing site");
      const place=placement(parent.placement,[building.origin.x,building.origin.y,building.elevation],building.rotation),entity=add("IFCBUILDING",[...root(building.name),s(id),place,unset,unset,e("ELEMENT"),ifcReal(building.elevation),unset,unset]);buildings.set(id,{entity,placement:place});relation(parent.entity,[entity]);
      holder(id,entity);
    }
    const groups=new Map<string,(typeof storeyRows)[number][1][]>();for(let i=0;i<storeyRows.length;i++){if(i%(control.batchSize!)===0)await ifcCheckpoint(control,"bim-storey-groups",i,storeyRows.length);const row=storeyRows[i]![1];let peers=groups.get(row.building);if(!peers){peers=[];groups.set(row.building,peers);}peers.push(row);}
    const levelPlans=new Map<string,IfcPreparedLevels>();for(const [building,rows]of groups)levelPlans.set(building,await runIfcIoSteps(ifcPrepareLevelsSteps(rows),control,"bim-storey-heights"));
    let storeyOrdinal=0;
    for(const [id,storey] of storeyRows) {
      await ifcCheckpoint(control,"bim-storeys",storeyOrdinal++,storeyRows.length);
      const parent=buildings.get(storey.building);if(!parent)throw new IfcStepError("bim.building","Storey references missing building");
      const levels=levelPlans.get(storey.building)!;
      if(levels.count(storey.level)!==1||storey.height<=0)throw new IfcStepError("bim.storey","Unique storey levels and positive heights required");
      const elevation=await runIfcIoSteps(levels.elevationSteps(storey.level),control,"bim-storey-heights");
      const place=placement(parent.placement,[0,0,elevation]),entity=add("IFCBUILDINGSTOREY",[...root(storey.name),s(id),place,unset,unset,e("ELEMENT"),ifcReal(elevation)]);relation(parent.entity,[entity]);
      holder(id,entity);
      const values:[string,string,IfcValue][]=[["Level","IFCINTEGER",integer(storey.level)],["Height","IFCLENGTHMEASURE",ifcReal(storey.height)]];
      if(storey.cut_height!==undefined)values.push(["CutHeight","IFCLENGTHMEASURE",ifcReal(storey.cut_height)]);
      const props=values.map(([name,type,value])=>add("IFCPROPERTYSINGLEVALUE",[s(name),unset,{kind:"typedValue",value:{name:type,items:[value]}},unset]));
      const pset=add("IFCPROPERTYSET",[...root("semio_BimStorey"),a(props)]);add("IFCRELDEFINESBYPROPERTIES",[...root(""),a([entity]),pset]);
    }
    await runIfcIoSteps(appendBimOccurrencePropertiesSteps({add,root},model.properties,holders),control,"bim-occurrence-properties");
    for(const [key,value]of await runIfcIoSteps(ifcSourceEntriesSteps(model),control,"bim-source-families"))if(!["schema","project","sites","buildings","storeys","properties"].includes(key)&&value&&typeof value==="object"){let nonempty=false;for(const item in value)if(Object.hasOwn(value,item)){nonempty=true;break;}if(nonempty)diagnostics.push({code:"bim.unsupported-family",severity:"warning",message:`${key} is outside spatial structure projection`,entityIds:[]});}
    await runIfcIoSteps(appendBimPhasesSteps({add,root,project:projectRef},model.project.phase_names),control,"bim-project-phases");
  } catch(error) {scope.rethrowCallerFailure();if(error instanceof IfcStepError&&error.code==="io.cancelled")throw error;return refuse(String(error));}
  diagnostics.push({code:"bim.spatial-projection",severity:"warning",message:"Only authored spatial structure is projected; original artifact identities and unsupported families remain in the source artifact",entityIds:[]});
  await ifcCheckpoint(control,"bim-spatial",1,1);return {artifact:graph.snapshot,fidelity:"partial",diagnostics};
}

/** 📥 Import only explicitly authored spatial parameters; no invented site or storey heights. */
export async function ifc4x3ToBimSpatial(snapshot:IfcSnapshot,control:IfcIoControl={}):Promise<IfcExchangeOutcome<ModelSnapshot>> {
  const scope=captureIfcIoControlScope(control);control=scope.control;
  const admitted=await scope.run(()=>IfcGeometryGraph.capture(snapshot,control));if(!admitted.artifact)return{fidelity:admitted.fidelity,diagnostics:admitted.diagnostics};const graph=admitted.artifact;snapshot=graph.snapshot;
  await ifcCheckpoint(control,"bim-import",0,snapshot.entities.length);
  const relations=await scope.run(()=>IfcExchangeRelations.prepare(graph,control)),diagnostics:IfcExchangeDiagnostic[]=[],project=(await relations.rows("projects"))[0];
  if(!project)return refuse("IFC project required");
  const model:ModelSnapshot={schema:"s.bim.model@1",project:{name:ifcText(project.args[2]),description:ifcText(project.args[3]),author:ifcText(snapshot.header.fileName[2]?.kind==="aggregate"?snapshot.header.fileName[2].value[0]:undefined),organization:ifcText(snapshot.header.fileName[3]?.kind==="aggregate"?snapshot.header.fileName[3].value[0]:undefined),phase_names:[]},sites:{},buildings:{},storeys:{}};
  const parents=new Map<bigint,bigint>();
  const id=(entity:bigint)=>`ifc${entity}`;
  try {
    for(const row of await relations.rows("decomposition")) {
      if(row.name!=="IFCRELAGGREGATES")throw new IfcStepError("bim.hierarchy","Only spatial aggregate decomposition is supported");
      let childOrdinal=0;for(const child of ifcAggregate(await relations.value(row,"RelatedObjects","IFCRELAGGREGATES"))) {if(childOrdinal++%(control.batchSize!)===0)await ifcCheckpoint(control,"bim-decomposition",childOrdinal,0);const parent=graph.ref(await relations.value(row,"RelatingObject","IFCRELAGGREGATES")),entity=graph.ref(child);if(parents.has(entity.id))throw new IfcStepError("bim.hierarchy","Spatial child has multiple aggregate parents");parents.set(entity.id,parent.id);}
    }
    const contexts:ReturnType<IfcGeometryGraph["ref"]>[]=[];for(const value of ifcAggregate(project.args[7])){if(contexts.length%(control.batchSize!)===0)await ifcCheckpoint(control,"bim-contexts",contexts.length,0);contexts.push(graph.ref(value));}
    if(!contexts.length)throw new IfcStepError("bim.context","Explicit geometric representation context required");
    let contextOrdinal=0;for(const context of contexts) {if(contextOrdinal++%(control.batchSize!)===0)await ifcCheckpoint(control,"bim-contexts",contextOrdinal,contexts.length);if(context.name!=="IFCGEOMETRICREPRESENTATIONCONTEXT"||graph.axis(context.args[4]).some((n,i)=>Math.abs(n-ifcIdentity()[i]!)>1e-10))throw new IfcStepError("bim.context-placement","Only identity project geometry-context frames can be projected");}
    const scale=await graph.lengthScaleAsync(control),context=contexts[0]!,north=context.args[5]?.kind==="reference"?graph.vector(context.args[5]):undefined;
    if(!north)throw new IfcStepError("bim.north","An explicit project true-north direction is required");
    if(Math.hypot(north[0],north[1])===0||Math.abs(north[2])>1e-10)throw new IfcStepError("bim.north","A finite horizontal true-north direction is required");
    const azimuth=Math.atan2(-north[0],north[1]);
    for(let i=1;i<contexts.length;i++) {if(i%(control.batchSize!)===0)await ifcCheckpoint(control,"bim-north-contexts",i,contexts.length);const additional=contexts[i]!,direction=graph.vector(additional.args[5]);if(Math.abs(Math.atan2(-direction[0],direction[1])-azimuth)>1e-10)throw new IfcStepError("bim.north","Conflicting geometry-context true-north directions");}
    const spatial=await relations.rows("spatial");
    for(let i=0;i<spatial.length;i++){if(i%(control.batchSize!)===0)await ifcCheckpoint(control,"bim-spatial-kinds",i,spatial.length);if(!["IFCSITE","IFCBUILDING","IFCBUILDINGSTOREY"].includes(spatial[i]!.name))throw new IfcStepError("bim.spatial-kind","Additional spatial structure kinds require explicit mapping");}
    for(let i=0;i<snapshot.entities.length;i++) {
      if(i%(control.batchSize!)===0)await ifcCheckpoint(control,"bim-import",i,snapshot.entities.length);
      const entity=snapshot.entities[i]!;
      if(entity.name==="IFCSITE") {
        if(parents.get(entity.id)!==project.id)throw new IfcStepError("bim.hierarchy","Site must aggregate directly under the selected project");
        const placement=graph.ref(entity.args[5]);
        if(placement.name!=="IFCLOCALPLACEMENT"||placement.args[0]?.kind!=="unset"||graph.axis(placement.args[1]).some((n,k)=>Math.abs(n-ifcIdentity()[k]!)>1e-10))throw new IfcStepError("bim.site-placement","Site translation, rotation or relative placement is outside authored BIM site parameters");
        model.sites![id(entity.id)]={name:ifcText(entity.args[2]),latitude:await angle(entity.args[9],control),longitude:await angle(entity.args[10],control),elevation:ifcNumber(entity.args[11])*scale,true_north:azimuth,boundary:[]};
      }
      if(entity.name==="IFCBUILDING") {
        const parent=parents.get(entity.id);if(parent===undefined||graph.lookup(parent)?.name!=="IFCSITE")throw new IfcStepError("bim.site","Building must aggregate under an explicit site");
        const placement=graph.ref(entity.args[5]),parentPlacement=graph.lookup(parent)!.args[5];
        if(placement.name!=="IFCLOCALPLACEMENT"||placement.args[0]?.kind!=="reference"||parentPlacement?.kind!=="reference"||placement.args[0].value!==parentPlacement.value)throw new IfcStepError("bim.placement-parent","Building placement must be directly relative to its aggregate site placement");const axis=graph.axis(placement.args[1]);
        if(Math.abs(axis[2])+Math.abs(axis[6])+Math.abs(axis[8])+Math.abs(axis[9])+Math.abs(axis[10]-1)>1e-8)throw new IfcStepError("bim.placement","Building requires a horizontal local frame");
        model.buildings![id(entity.id)]={name:ifcText(entity.args[2]),site:id(parent),origin:{x:axis[12]*scale,y:axis[13]*scale},rotation:Math.atan2(axis[1],axis[0]),elevation:axis[14]*scale};
      }
      if(entity.name==="IFCBUILDINGSTOREY") {
        const parent=parents.get(entity.id);if(parent===undefined||graph.lookup(parent)?.name!=="IFCBUILDING")throw new IfcStepError("bim.building","Storey must aggregate under an explicit building");
        const placement=graph.ref(entity.args[5]),parentPlacement=graph.lookup(parent)!.args[5];
        if(placement.name!=="IFCLOCALPLACEMENT"||placement.args[0]?.kind!=="reference"||parentPlacement?.kind!=="reference"||placement.args[0].value!==parentPlacement.value)throw new IfcStepError("bim.placement-parent","Storey placement must be directly relative to its aggregate building placement");
        const axis=graph.axis(placement.args[1]),identity=ifcIdentity();
        if(axis.some((n,k)=>k!==14&&Math.abs(n-identity[k]!)>1e-10)||Math.abs(axis[14]-ifcNumber(entity.args[9]))>1e-8)throw new IfcStepError("bim.storey-placement","Storey placement must be an unrotated vertical translation agreeing with its Elevation");
        const definitions=await relations.related("propertyRelations",entity.id,function*(row){const pset=graph.ref(yield* relations.valueSteps(row,"RelatingPropertyDefinition","IFCRELDEFINESBYPROPERTIES"));return ifcText(yield* relations.valueSteps(pset,"Name","IFCROOT"))==="semio_BimStorey";}),psets=new Map<bigint,ReturnType<IfcGeometryGraph["ref"]>>();for(const row of definitions){await ifcCheckpoint(control,"bim-definitions",psets.size,definitions.length);const pset=graph.ref(await relations.value(row,"RelatingPropertyDefinition","IFCRELDEFINESBYPROPERTIES"));psets.set(pset.id,pset);}
        if(psets.size!==1)throw new IfcStepError("bim.authored-height","Exactly one authored storey definition is required; IFC elevation alone cannot reconstruct level and height");
        const pset=[...psets.values()][0]!,values=new Map<string,ReturnType<IfcGeometryGraph["ref"]>>();
        let propertyOrdinal=0;for(const value of ifcAggregate(await relations.value(pset,"HasProperties","IFCPROPERTYSET"))) {if(propertyOrdinal++%(control.batchSize!)===0)await ifcCheckpoint(control,"bim-properties",propertyOrdinal,0);const property=graph.ref(value),name=ifcText(await relations.value(property,"Name","IFCPROPERTY"));if(property.name!=="IFCPROPERTYSINGLEVALUE"||values.has(name)||!["Level","Height","CutHeight"].includes(name))throw new IfcStepError("bim.property-definition","Distinct declared single-value authored storey properties are required");values.set(name,property);}
        const n=async(key:string):Promise<number>=>{
          const property=values.get(key),value=property?await relations.value(property,"NominalValue","IFCPROPERTYSINGLEVALUE"):undefined,unit=property?await relations.value(property,"Unit","IFCPROPERTYSINGLEVALUE"):undefined;
          if(value?.kind!=="typedValue"||value.value.items.length!==1)throw new IfcStepError("bim.property-measure",`Explicit typed ${key} measure required`);
          if(key==="Level") {if(value.value.name!=="IFCINTEGER"||value.value.items[0]?.kind!=="integer"||unit?.kind!=="unset")throw new IfcStepError("bim.property-unit","Authored Level requires an ordinal integer without a physical unit");return ifcNumber(value.value.items[0]);}
          if(!["IFCLENGTHMEASURE","IFCPOSITIVELENGTHMEASURE","IFCNONNEGATIVELENGTHMEASURE"].includes(value.value.name))throw new IfcStepError("bim.property-measure",`${key} requires a typed length measure`);
          let factor=scale;
          if(unit?.kind!=="unset") {const resolved=await graph.unitMeasureAsync(graph.ref(unit),control);if(resolved.dimensions.some((exponent,index)=>exponent!==(index===0?1:0)))throw new IfcStepError("bim.property-unit",`${key} requires a length unit`);factor=resolved.scale;}
          const result=ifcNumber(value.value.items[0])*factor;if(!Number.isFinite(result))throw new IfcStepError("bim.property-unit",`${key} exceeds finite SI length range`);return result;
        };
        const level=await n("Level"),height=await n("Height");if(!Number.isSafeInteger(level)||height<=0)throw new IfcStepError("bim.storey","Invalid authored storey level or height");
        model.storeys![id(entity.id)]={name:ifcText(entity.args[2]),building:id(parent),level,height,...(values.has("CutHeight")?{cut_height:await n("CutHeight")}:{})};
      }
    }
    const storeys=await relations.rows("storeys"),groups=new Map<string,NonNullable<ModelSnapshot["storeys"]>[string][]>();
    for(const entity of storeys){await ifcCheckpoint(control,"bim-storey-groups",groups.size,storeys.length);const storey=model.storeys![id(entity.id)]!;let rows=groups.get(storey.building);if(!rows){rows=[];groups.set(storey.building,rows);}rows.push(storey);}
    const levelPlans=new Map<string,IfcPreparedLevels>();for(const [building,rows]of groups)levelPlans.set(building,await runIfcIoSteps(ifcPrepareLevelsSteps(rows),control,"bim-storey-heights"));
    for(const entity of storeys) {
      const storey=model.storeys![id(entity.id)]!,levels=levelPlans.get(storey.building)!;
      if(levels.count(storey.level)!==1)throw new IfcStepError("bim.storey-level","Storey levels must be unique within the building");
      const expected=await runIfcIoSteps(levels.elevationSteps(storey.level),control,"bim-storey-heights");
      if(Math.abs(expected-ifcNumber(entity.args[9])*scale)>1e-8)throw new IfcStepError("bim.storey-authored-placement","Authored storey level and cumulative heights contradict IFC placement");
    }
    model.project.phase_names=await runIfcIoSteps(readBimPhasesSteps(graph,relations,project),control,"bim-project-phases");
    let propertyHolderOrdinal=0;for(const entity of spatial){await ifcCheckpoint(control,"bim-occurrence-properties",propertyHolderOrdinal++,spatial.length);const properties=await runIfcIoSteps(readBimOccurrencePropertiesSteps(graph,relations,entity,diagnostics),control,"bim-occurrence-properties");if(properties){model.properties??=Object.create(null);model.properties![id(entity.id)]=properties;}}
    for(let i=0;i<snapshot.entities.length;i++){if(i%(control.batchSize!)===0)await ifcCheckpoint(control,"bim-omitted",i,snapshot.entities.length);const entity=snapshot.entities[i]!;if(entity.args[6]?.kind==="reference"&&graph.lookup(entity.args[6].value)?.name==="IFCPRODUCTDEFINITIONSHAPE")diagnostics.push({code:"bim.geometry-omitted",severity:"warning",message:"Product geometry does not determine authored BIM family parameters",entityIds:[entity.id]});}
  } catch(error) {scope.rethrowCallerFailure();if(error instanceof IfcStepError&&error.code==="io.cancelled")throw error;return refuse(String(error));}
  diagnostics.push({code:"bim.spatial-projection",severity:"warning",message:"Spatial projection retains supported authored spatial data and dimensionless occurrence values; original IFC identities and unsupported semantics remain in the source graph",entityIds:[]});
  await ifcCheckpoint(control,"bim-import",snapshot.entities.length,snapshot.entities.length);return {artifact:model,fidelity:"partial",diagnostics};
}
