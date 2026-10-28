import {IfcExchangeRelations} from "./relations.ts";
import {ifcFemSelfWeight} from "./fem/self-weight/🟦️.ts";
import type {Fem3dSnapshot,FemDof,FemLoad} from "../../../../../../../../🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🌐️any/🧬️schema/📸️snapshot/🟦️.ts";
import type {IfcSnapshot,IfcEntity,IfcValue} from "../🧬️schema/📸️snapshot/🟦️.ts";
import {IfcGeometryGraph,ifcAggregate,ifcText,ifcTransform,type IfcExchangeOutcome,type IfcExchangeDiagnostic} from "./geometry.ts";
import {createIfc4x3ExchangeGraph} from "./gltf.ts";
import {ifcReal,ifcNumber,ifcCheckpoint,IfcStepError,type IfcIoControl} from "./step.ts";
import {captureIfcIoControlScope} from "./control.ts";
import {captureIfcExternalSource} from "./external-source.ts";
import {IfcEditError} from "../🧬️schema/🔺️diff/🟦️.ts";

const unset:IfcValue={kind:"unset"},a=(value:IfcValue[]):IfcValue=>({kind:"aggregate",value}),s=(value:string):IfcValue=>({kind:"string",value}),e=(value:string):IfcValue=>({kind:"enum",value}),ref=(value:bigint):IfcValue=>({kind:"reference",value});
const typed=(name:string,value:IfcValue):IfcValue=>({kind:"typedValue",value:{name,items:[value]}});
const real=(bits:{readonly bits:bigint}):IfcValue=>{ifcNumber({kind:"real",value:bits});return {kind:"real",value:bits};};
const word=(value:number)=>{const result=ifcReal(value);if(result.kind!=="real")throw Error("real");return result.value;};
const dofs:FemDof[]=["Tx","Ty","Tz","Rx","Ry","Rz"];
const failure=(code:string,message:string,entityIds:bigint[]=[]):IfcExchangeOutcome<never>=>({fidelity:"unsupported",diagnostics:[{code,severity:"error",message,entityIds}]});
const number=(value:IfcValue|undefined):number=>ifcNumber(value?.kind==="typedValue"?value.value.items[0]:value);
const dot=(a:number[],b:number[])=>a.reduce((sum,n,k)=>sum+n*b[k]!,0);
const cross=(a:number[],b:number[])=>[a[1]!*b[2]!-a[2]!*b[1]!,a[2]!*b[0]!-a[0]!*b[2]!,a[0]!*b[1]!-a[1]!*b[0]!];
const normalized=(v:number[])=>{const length=Math.hypot(...v);if(!length||!Number.isFinite(length))throw new IfcStepError("fem.axis","Nondegenerate finite frame basis required");return v.map(n=>n/length);};
/** 🧭 Mirror the native Frame3 reference basis before applying the authored roll. */
function frameBasis(delta:number[]):{x:number[];y:number[];z:number[]} {
  const x=normalized(delta),reference=Math.abs(x[2]!)>.99?[1,0,0]:[0,0,1],y=normalized(cross(reference,x));return {x,y,z:cross(x,y)};
}

/** 📇️ Count FEM identity-bearing domain rows, including case-scoped loads, after source capture. */
function* femSourceRecords(model:Fem3dSnapshot):Generator<unknown> {
  const record=(value:unknown):Record<string,unknown>=>{if(value===null||typeof value!=="object"||Array.isArray(value))throw new IfcEditError("invalidValue","FEM source records require objects");return value as Record<string,unknown>;};
  const root=record(model);record(root.analysis);
  for(const field of ["nodes","elements","materials","sections","solids","supports","loadCases","combinations"]){
    const items=root[field];if(!Array.isArray(items))throw new IfcEditError("invalidValue","FEM source collections require arrays");
    for(const item of items){
      const row=record(item);yield item;
      if(field==="loadCases"){
        if(!Array.isArray(row.loads))throw new IfcEditError("invalidValue","FEM load cases require load arrays");
        for(const load of row.loads){record(load);yield load;}
      }
    }
  }
}

/** 🏗️ Export actual analytical topology, material/section values, restraints and global loads. */
export async function fem3dToIfc4x3(model:Fem3dSnapshot,control:IfcIoControl={}):Promise<IfcExchangeOutcome<IfcSnapshot>> {
  const scope=captureIfcIoControlScope(control);control=scope.control;
  const captured=await scope.run(()=>captureIfcExternalSource(model,control,femSourceRecords));if(!captured.artifact)return{fidelity:captured.fidelity,diagnostics:captured.diagnostics};model=captured.artifact;
  await ifcCheckpoint(control,"fem-export",0,model.nodes.length+model.elements.length);
  const unsupported=()=>failure("fem.unsupported","Continuum solids, area loads and load combinations require additional analytical mappings");if(model.solids.length||model.combinations.length)return unsupported();let preflight=0;await ifcCheckpoint(control,"fem-source-index",0,0);for(const row of model.loadCases){if(typeof row.selfWeight!=="boolean")return failure("fem.self-weight","FEM selfWeight requires an explicit boolean");if(preflight++%(control.batchSize!)===0)await ifcCheckpoint(control,"fem-source-index",preflight,0);for(const load of row.loads){if(preflight++%(control.batchSize!)===0)await ifcCheckpoint(control,"fem-source-index",preflight,0);if(load.kind==="area")return unsupported();}}
  if(!model.elements.length)return failure("fem.empty","At least one analytical member is required for FEM export");
  const connected=new Set<string>(),topologyTotal=model.elements.length+model.nodes.length,batch=control.batchSize!;
  for(let i=0;i<model.elements.length;i++){
    if(i%batch===0)await ifcCheckpoint(control,"fem-topology",i,topologyTotal);
    const member=model.elements[i]!;connected.add(member.start);connected.add(member.end);
  }
  for(let i=0;i<model.nodes.length;i++){
    if(i%batch===0)await ifcCheckpoint(control,"fem-topology",model.elements.length+i,topologyTotal);
    const node=model.nodes[i]!;if(!connected.has(node.id))return failure("fem.isolated-node","Analytical node "+node.id+" requires an incident member");
  }
  await ifcCheckpoint(control,"fem-topology",topologyTotal,topologyTotal);
  const graph=createIfc4x3ExchangeGraph("FEM3d analysis"),{add}=graph,project=graph.snapshot.entities.find(row=>row.name==="IFCPROJECT")!;let guid=2;
  const si=(type:string,name:string,prefix:IfcValue=unset)=>add("IFCSIUNIT",[{kind:"derived"},e(type),prefix,e(name)]);
  const metre:IfcValue={kind:"reference",value:graph.snapshot.entities.find(row=>row.name==="IFCSIUNIT")!.id},area=si("AREAUNIT","SQUARE_METRE"),pressure=si("PRESSUREUNIT","PASCAL"),mass=si("MASSUNIT","GRAM",e("KILO")),force=si("FORCEUNIT","NEWTON");
  const derived=(type:string,elements:[IfcValue,number][])=>add("IFCDERIVEDUNIT",[a(elements.map(([unit,power])=>add("IFCDERIVEDUNITELEMENT",[unit,{kind:"integer",value:BigInt(power)}]))),e(type),unset,unset]);
  const density=derived("MASSDENSITYUNIT",[[mass,1],[metre,-3]]),inertia=derived("MOMENTOFINERTIAUNIT",[[metre,4]]),linear=derived("LINEARFORCEUNIT",[[force,1],[metre,-1]]),torque=derived("TORQUEUNIT",[[force,1],[metre,1]]);
  graph.snapshot.entities.find(row=>row.name==="IFCUNITASSIGNMENT")!.args[0]=a([metre,area,pressure,mass,force,density,inertia,linear,torque]);
  const propertyUnits:Record<string,IfcValue>={YoungModulus:pressure,ShearModulus:pressure,MassDensity:density,CrossSectionArea:area,MomentOfInertiaY:inertia,MomentOfInertiaZ:inertia,TorsionalConstant:inertia};
  const root=(name:string):IfcValue[]=>[s((guid++).toString().padStart(22,"0")),unset,s(name),unset];
  const relation=(name:string,tail:IfcValue[])=>add(name,[...root(""),...tail]);
  const properties=(values:[string,string,IfcValue][])=>values.map(([name,type,value])=>add("IFCPROPERTYSINGLEVALUE",[s(name),unset,typed(type,value),propertyUnits[name]??unset]));
  const nodes=new Map<string,{entity:IfcValue;vertex:IfcValue;xyz:number[]}>(),members=new Map<string,IfcValue>(),materials=new Map<string,IfcValue>(),sections=new Map<string,Fem3dSnapshot["sections"][number]>(),sectionDefinitions=new Map<string,IfcValue>();
  try {
    const unique=async(rows:readonly {id:string}[]):Promise<boolean>=>{const ids=new Set<string>();for(let i=0;i<rows.length;i++){if(i%(control.batchSize!)===0)await ifcCheckpoint(control,"fem-source-index",i,rows.length);const id=rows[i]!.id;if(ids.has(id))return false;ids.add(id);}return true;};
    if(!await unique(model.nodes)||!await unique(model.elements)||!await unique(model.materials)||!await unique(model.sections))throw new IfcStepError("fem.identity","Duplicate FEM identity");
    if(!await unique(model.supports)||!await unique(model.loadCases))throw new IfcStepError("fem.identity","Duplicate support, load-case or case-scoped load identity");
    for(const row of model.loadCases)if(!await unique(row.loads))throw new IfcStepError("fem.identity","Duplicate support, load-case or case-scoped load identity");
    for(let i=0;i<model.sections.length;i++){if(i%(control.batchSize!)===0)await ifcCheckpoint(control,"fem-source-index",i,model.sections.length);const row=model.sections[i]!;sections.set(row.id,row);}
    const supports=new Map<string,Fem3dSnapshot["supports"]>();for(let i=0;i<model.supports.length;i++){if(i%(control.batchSize!)===0)await ifcCheckpoint(control,"fem-source-index",i,model.supports.length);const row=model.supports[i]!;let rows=supports.get(row.nodeId);if(!rows){rows=[];supports.set(row.nodeId,rows);}rows.push(row);}
    let materialOrdinal=0;
    for(const material of model.materials) {
      if(materialOrdinal++%(control.batchSize!)===0)await ifcCheckpoint(control,"fem-materials",materialOrdinal,model.materials.length);
      const entity=add("IFCMATERIAL",[s(material.name),unset,unset]);materials.set(material.id,entity);
      const props=properties([["YoungModulus","IFCMODULUSOFELASTICITYMEASURE",real(material.e)],["ShearModulus","IFCMODULUSOFELASTICITYMEASURE",real(material.g)],["PoissonRatio","IFCRATIOMEASURE",real(material.nu)],["MassDensity","IFCMASSDENSITYMEASURE",real(material.rho)]]);
      add("IFCMATERIALPROPERTIES",[s("semio_FemMaterial"),unset,a(props),entity]);
    }
    for(let i=0;i<model.nodes.length;i++) {
      if(i%(control.batchSize!)===0)await ifcCheckpoint(control,"fem-export",i,model.nodes.length+model.elements.length);
      const node=model.nodes[i]!,xyz=[node.x,node.y,node.z].map(v=>ifcNumber(real(v))),point=add("IFCCARTESIANPOINT",[a(xyz.map(ifcReal))]),vertex=add("IFCVERTEXPOINT",[point]);
      const topology=add("IFCTOPOLOGYREPRESENTATION",[graph.context,s("Reference"),s("Vertex"),a([vertex])]),shape=add("IFCPRODUCTDEFINITIONSHAPE",[unset,unset,a([topology])]);
      const support=supports.get(node.id)??[];
      if(support.length>1)throw new IfcStepError("fem.support","Multiple supports on one node require consolidation");
      const fixed=support[0]?.fixed??[];if(fixed.length>6||new Set(fixed).size!==fixed.length||fixed.some(d=>!dofs.includes(d)))throw new IfcStepError("fem.support","Invalid fixed DOFs");
      const condition=support[0]?add("IFCBOUNDARYNODECONDITION",[s(support[0].id),...dofs.map(d=>typed("IFCBOOLEAN",e(fixed.includes(d)?"T":"F")))]):unset;
      const entity=add("IFCSTRUCTURALPOINTCONNECTION",[...root(node.id),unset,unset,shape,condition,unset]);nodes.set(node.id,{entity,vertex,xyz});
    }
    for(let i=0;i<model.supports.length;i++){if(i%(control.batchSize!)===0)await ifcCheckpoint(control,"fem-supports",i,model.supports.length);if(!nodes.has(model.supports[i]!.nodeId))throw new IfcStepError("fem.support","Support references a missing node");}
    for(let i=0;i<model.elements.length;i++) {
      if(i%(control.batchSize!)===0)await ifcCheckpoint(control,"fem-export",model.nodes.length+i,model.nodes.length+model.elements.length);
      const member=model.elements[i]!,start=nodes.get(member.start),end=nodes.get(member.end),material=materials.get(member.materialId),section=sections.get(member.sectionId);
      if(!start||!end||!material||!section)throw new IfcStepError("fem.reference","Member references missing node, material or section");
      const delta=start.xyz.map((n,k)=>end.xyz[k]!-n),length=Math.hypot(...delta);
      if(!length)throw new IfcStepError("fem.length","Member has zero length");
      const basis=frameBasis(delta),roll=member.kind==="frame"?ifcNumber(real(member.roll)):0,localZ=basis.z.map((n,k)=>n*Math.cos(roll)-basis.y[k]!*Math.sin(roll));
      const axis=add("IFCDIRECTION",[a(localZ.map(ifcReal))]);
      const edge=add("IFCEDGE",[start.vertex,end.vertex]),topology=add("IFCTOPOLOGYREPRESENTATION",[graph.context,s("Reference"),s("Edge"),a([edge])]),shape=add("IFCPRODUCTDEFINITIONSHAPE",[unset,unset,a([topology])]);
      const entity=add("IFCSTRUCTURALCURVEMEMBER",[...root(member.id),unset,unset,shape,e(member.kind==="bar"?"PIN_JOINED_MEMBER":"RIGID_JOINED_MEMBER"),axis]);members.set(member.id,entity);
      relation("IFCRELASSOCIATESMATERIAL",[a([entity]),material]);
      let pset=sectionDefinitions.get(section.id);
      if(!pset) {const props=properties([["CrossSectionArea","IFCAREAMEASURE",real(section.area)],["MomentOfInertiaY","IFCMOMENTOFINERTIAMEASURE",real(section.iy)],["MomentOfInertiaZ","IFCMOMENTOFINERTIAMEASURE",real(section.iz)],["TorsionalConstant","IFCMOMENTOFINERTIAMEASURE",real(section.j)]]);pset=add("IFCPROPERTYSET",[...root("semio_FemSection"),a([...props,add("IFCPROPERTYSINGLEVALUE",[s("SectionName"),unset,typed("IFCLABEL",s(section.name)),unset])])]);sectionDefinitions.set(section.id,pset);}
      relation("IFCRELDEFINESBYPROPERTIES",[a([entity]),pset]);
      for(const node of [start,end])relation("IFCRELCONNECTSSTRUCTURALMEMBER",[entity,node.entity,unset,unset,unset,unset]);
    }
    const cases:IfcValue[]=[];
    for(const loadCase of model.loadCases) {
      await ifcCheckpoint(control,"fem-loads",cases.length,model.loadCases.length);
      await ifcCheckpoint(control,"fem-self-weight",0,3);
      const entity=add("IFCSTRUCTURALLOADCASE",[...root(loadCase.name),unset,e("LOAD_CASE"),e("NOTDEFINED"),e("NOTDEFINED"),ifcReal(1),unset,loadCase.selfWeight?a([ifcReal(0),ifcReal(0),ifcReal(-1)]):unset]),actions:IfcValue[]=[];cases.push(entity);
      for(const load of loadCase.loads) {
        if(actions.length%(control.batchSize!)===0)await ifcCheckpoint(control,"fem-load-actions",actions.length,loadCase.loads.length);
        let target:IfcValue|undefined,action:IfcValue;
        if(load.kind==="nodal") {
          target=nodes.get(load.nodeId)?.entity;const ordinal=dofs.indexOf(load.dof);if(ordinal<0)throw new IfcStepError("fem.dof","Invalid load DOF");
          const values=dofs.map((_,i)=>i===ordinal?real(load.value):unset),force=add("IFCSTRUCTURALLOADSINGLEFORCE",[s(load.id),...values]);
          action=add("IFCSTRUCTURALPOINTACTION",[...root(load.id),unset,unset,unset,force,e("GLOBAL_COORDS"),unset]);
        } else if(load.kind==="memberUdl") {
          target=members.get(load.elementId);const force=add("IFCSTRUCTURALLOADLINEARFORCE",[s(load.id),real(load.wx),real(load.wy),real(load.wz),ifcReal(0),ifcReal(0),ifcReal(0)]);
          action=add("IFCSTRUCTURALCURVEACTION",[...root(load.id),unset,unset,unset,force,e("GLOBAL_COORDS"),unset,e("TRUE_LENGTH"),e("CONST")]);
        } else throw new IfcStepError("fem.load","Unsupported load kind");
        if(!target)throw new IfcStepError("fem.load","Missing load target");relation("IFCRELCONNECTSSTRUCTURALACTIVITY",[target,action]);actions.push(action);
      }
      if(actions.length)relation("IFCRELASSIGNSTOGROUP",[a(actions),unset,entity]);
    }
    const analysis=add("IFCSTRUCTURALANALYSISMODEL",[...root("FEM3d"),unset,e("LOADING_3D"),unset,cases.length?a(cases):unset,unset,unset]);
    const objects:IfcValue[]=[];for(const row of nodes.values()){if(objects.length%(control.batchSize!)===0)await ifcCheckpoint(control,"fem-analysis-membership",objects.length,nodes.size+members.size);objects.push(row.entity);}for(const row of members.values()){if(objects.length%(control.batchSize!)===0)await ifcCheckpoint(control,"fem-analysis-membership",objects.length,nodes.size+members.size);objects.push(row);}if(objects.length)relation("IFCRELASSIGNSTOGROUP",[a(objects),unset,analysis]);
    relation("IFCRELDECLARES",[ref(project.id),a([analysis])]);
  } catch(error) {scope.rethrowCallerFailure();if(error instanceof IfcStepError&&error.code==="io.cancelled")throw error;return failure(error instanceof IfcStepError?error.code:"fem.export",String(error));}
  await ifcCheckpoint(control,"fem-export",model.nodes.length+model.elements.length,model.nodes.length+model.elements.length);
  return {artifact:graph.snapshot,fidelity:"partial",diagnostics:[{code:"fem.analysis-settings",severity:"warning",message:"Solver counts, deformation display settings and artifact identities are not IFC analysis semantics; engineering section values use an explicit property set without inventing profile geometry",entityIds:[]}]};
}

/** 📥 Read explicitly analytical IFC members; geometry proxies never become analysis members. */
export async function ifc4x3ToFem3d(snapshot:IfcSnapshot,control:IfcIoControl={}):Promise<IfcExchangeOutcome<Fem3dSnapshot>> {
  const scope=captureIfcIoControlScope(control);control=scope.control;
  const admitted=await scope.run(()=>IfcGeometryGraph.capture(snapshot,control));if(!admitted.artifact)return{fidelity:admitted.fidelity,diagnostics:admitted.diagnostics};const graph=admitted.artifact;snapshot=graph.snapshot;
  await ifcCheckpoint(control,"fem-import",0,snapshot.entities.length);
  const relations=await scope.run(()=>IfcExchangeRelations.prepare(graph,control)),diagnostics:IfcExchangeDiagnostic[]=[],model:Fem3dSnapshot={nodes:[],elements:[],materials:[],sections:[],solids:[],supports:[],loadCases:[],combinations:[],analysis:{modalCount:0n,bucklingCount:0n,deformationScale:word(1)}};
  const nodeRows=new Map<string,Fem3dSnapshot["nodes"][number]>(),nodes=new Map<bigint,string>(),vertices=new Map<bigint,string>(),members=new Map<bigint,string>(),materialIds=new Map<bigint,string>(),sectionIds=new Map<bigint,string>();
  const dimensional:Record<string,string[]>={YoungModulus:["PRESSUREUNIT","MODULUSOFELASTICITYUNIT"],ShearModulus:["PRESSUREUNIT","SHEARMODULUSUNIT"],MassDensity:["MASSDENSITYUNIT"],CrossSectionArea:["AREAUNIT"],MomentOfInertiaY:["MOMENTOFINERTIAUNIT"],MomentOfInertiaZ:["MOMENTOFINERTIAUNIT"],TorsionalConstant:["MOMENTOFINERTIAUNIT"]};
  const measures:Record<string,string[]>={YoungModulus:["IFCMODULUSOFELASTICITYMEASURE","IFCPRESSUREMEASURE"],ShearModulus:["IFCMODULUSOFELASTICITYMEASURE","IFCPRESSUREMEASURE"],MassDensity:["IFCMASSDENSITYMEASURE"],CrossSectionArea:["IFCAREAMEASURE"],MomentOfInertiaY:["IFCMOMENTOFINERTIAMEASURE"],MomentOfInertiaZ:["IFCMOMENTOFINERTIAMEASURE"],TorsionalConstant:["IFCMOMENTOFINERTIAMEASURE"]};
  const props=async(values:IfcValue[]):Promise<Map<string,IfcValue|undefined>>=>{
    const definitions=new Map<string,IfcEntity>();let propertyOrdinal=0;
    for(const value of values) {
      if(propertyOrdinal++%(control.batchSize!)===0)await ifcCheckpoint(control,"fem-properties",propertyOrdinal,values.length);
      const row=graph.ref(value);if(row.name!=="IFCPROPERTYSINGLEVALUE")throw new IfcStepError("fem.property-definition","Single-value engineering properties are required");
      const name=ifcText(await relations.value(row,"Name","IFCPROPERTY")),previous=definitions.get(name);if(previous&&previous.id!==row.id)throw new IfcStepError("fem.property-definition",`Conflicting ${name} engineering properties require an explicit choice`);definitions.set(name,row);
      const expected=dimensional[name],nominal=await relations.value(row,"NominalValue","IFCPROPERTYSINGLEVALUE"),unitValue=await relations.value(row,"Unit","IFCPROPERTYSINGLEVALUE");
      if(expected) {const unit=graph.ref(unitValue),type=unit.args[1];if(nominal?.kind!=="typedValue"||nominal.value.items.length!==1||!measures[name]!.includes(nominal.value.name)||type?.kind!=="enum"||!expected.includes(type.value)||(await graph.unitMeasureAsync(unit,control)).scale!==1)throw new IfcStepError("fem.property-unit",`An explicitly dimensioned SI measure and unit are required for ${name}`);}
      if(name==="PoissonRatio"&&(nominal?.kind!=="typedValue"||nominal.value.items.length!==1||!["IFCRATIOMEASURE","IFCPOSITIVERATIOMEASURE","IFCNORMALISEDRATIOMEASURE"].includes(nominal.value.name)||unitValue?.kind!=="unset"))throw new IfcStepError("fem.property-unit","Poisson ratio requires a dimensionless ratio measure without a physical unit");
      if(name==="SectionName"&&(nominal?.kind!=="typedValue"||nominal.value.items.length!==1||!["IFCLABEL","IFCTEXT"].includes(nominal.value.name)||nominal.value.items[0]?.kind!=="string"||unitValue?.kind!=="unset"))throw new IfcStepError("fem.property-unit","Section name requires text without a physical unit");
    }
    const result=new Map<string,IfcValue|undefined>();for(const [name,row]of definitions){if(result.size%(control.batchSize!)===0)await ifcCheckpoint(control,"fem-property-values",result.size,definitions.size);result.set(name,await relations.value(row,"NominalValue","IFCPROPERTYSINGLEVALUE"));}return result;
  };
  const required=(values:Map<string,IfcValue|undefined>,name:string)=>word(number(values.get(name)));
  const topology=(entity:IfcEntity):IfcEntity=>{const shape=graph.ref(entity.args[6]),reps=ifcAggregate(shape.args[2]);if(reps.length!==1)throw new IfcStepError("fem.topology","Exactly one reference topology representation required");const representation=graph.ref(reps[0]);if(representation.name!=="IFCTOPOLOGYREPRESENTATION")throw new IfcStepError("fem.topology","Analytical topology representation required");const items=ifcAggregate(representation.args[3]);if(items.length!==1)throw new IfcStepError("fem.topology","One topology item required");return graph.ref(items[0]);};
  try {
    const analyses=await relations.rows("analyses");
    if(analyses.length!==1)throw new IfcStepError("fem.analysis","Exactly one structural analysis model required");
    if(analyses[0]!.args[9]?.kind==="reference")throw new IfcStepError("fem.shared-placement","Shared structural placement requires explicit frame conversion");
    if(await graph.lengthScaleAsync(control)!==1)throw new IfcStepError("fem.units","Analytical projection currently requires explicit SI metre project units and SI typed properties");
    const analysis=analyses[0]!,assignments=async(group:bigint):Promise<IfcEntity[]>=>{
      const found=await relations.related("group",group);
      for(let i=0;i<found.length;i++){if(i%(control.batchSize!)===0)await ifcCheckpoint(control,"fem-assignments",i,found.length);if(found[i]!.name!=="IFCRELASSIGNSTOGROUP")throw new IfcStepError("fem.group-factor","Factored or specialized group assignments require explicit analytical scaling");}return found;
    };
    const groupItems=async(group:bigint):Promise<bigint[]>=>{const result:bigint[]=[];for(const row of await assignments(group))for(const value of ifcAggregate(await relations.value(row,"RelatedObjects","IFCRELASSIGNS"))){if(result.length%(control.batchSize!)===0)await ifcCheckpoint(control,"fem-group-items",result.length,0);result.push(graph.ref(value).id);}return result;};
    const items=await relations.rows("items"),membership=new Set<bigint>(),itemIds=new Set<bigint>();
    for(const id of await groupItems(analysis.id)){if(membership.size%(control.batchSize!)===0)await ifcCheckpoint(control,"fem-membership",membership.size,0);membership.add(id);}
    let membershipValid=membership.size>0;for(let i=0;i<items.length;i++){if(i%(control.batchSize!)===0)await ifcCheckpoint(control,"fem-membership",i,items.length);const row=items[i]!;itemIds.add(row.id);if(!membership.has(row.id))membershipValid=false;}
    let checked=0;for(const id of membership){if(checked++%(control.batchSize!)===0)await ifcCheckpoint(control,"fem-membership-check",checked,membership.size);if(!itemIds.has(id))membershipValid=false;}
    if(!membershipValid)throw new IfcStepError("fem.membership","Every structural item must belong explicitly and exclusively to the selected analysis");
    for(let i=0;i<items.length;i++){if(i%(control.batchSize!)===0)await ifcCheckpoint(control,"fem-item-kinds",i,items.length);if(!["IFCSTRUCTURALPOINTCONNECTION","IFCSTRUCTURALCURVEMEMBER"].includes(items[i]!.name))throw new IfcStepError("fem.unsupported","Specialized structural items require additional analytical mappings");}
    const selectedCases:IfcEntity[]=[],caseIds=new Set<bigint>();
    for(const value of analysis.args[7]?.kind==="unset"?[]:ifcAggregate(analysis.args[7])){if(selectedCases.length%(control.batchSize!)===0)await ifcCheckpoint(control,"fem-load-cases",selectedCases.length,0);selectedCases.push(graph.ref(value));}
    let casesValid=true;for(let i=0;i<selectedCases.length;i++){if(i%(control.batchSize!)===0)await ifcCheckpoint(control,"fem-load-cases",i,selectedCases.length);const row=selectedCases[i]!;if(caseIds.has(row.id)||row.name!=="IFCSTRUCTURALLOADCASE")casesValid=false;caseIds.add(row.id);}
    if(!casesValid)throw new IfcStepError("fem.load-group","Only distinct direct structural load cases are supported");
    for(const row of await relations.rows("loadGroups")){await ifcCheckpoint(control,"fem-load-groups",0,0);if(!caseIds.has(row.id))throw new IfcStepError("fem.load-group","Every structural load group must be a direct selected load case");}
    const actionIds=new Set<bigint>();let duplicateActions=false;
    for(const row of selectedCases){const ids=await groupItems(row.id),seen=new Set<bigint>();for(let i=0;i<ids.length;i++){if(i%(control.batchSize!)===0)await ifcCheckpoint(control,"fem-action-membership",i,ids.length);const id=ids[i]!;if(seen.has(id))duplicateActions=true;seen.add(id);actionIds.add(id);}}
    if(duplicateActions)throw new IfcStepError("fem.action-membership","A structural action may occur only once within a load case");
    const actions=await relations.rows("actions"),actualActionIds=new Set<bigint>();let actionsValid=true;
    for(let i=0;i<actions.length;i++){if(i%(control.batchSize!)===0)await ifcCheckpoint(control,"fem-action-membership",i,actions.length);const row=actions[i]!;actualActionIds.add(row.id);if(!actionIds.has(row.id))actionsValid=false;}
    checked=0;for(const id of actionIds){if(checked++%(control.batchSize!)===0)await ifcCheckpoint(control,"fem-action-membership-check",checked,actionIds.size);if(!actualActionIds.has(id))actionsValid=false;}
    if(!actionsValid)throw new IfcStepError("fem.action-membership","Every structural action must be explicitly assigned to a selected load case");
    const units=await graph.projectUnitsAsync(control);
    const requireUnit=async(type:string):Promise<void>=>{let selected:IfcEntity|undefined;for(let index=0;index<units.length;index++){if(index%(control.batchSize!)===0)await ifcCheckpoint(control,"fem-load-units",index,units.length);const unit=units[index]!;if(unit.args[1]?.kind==="enum"&&unit.args[1].value===type){if(selected)throw new IfcStepError("fem.load-unit",`Exactly one SI ${type} is required for explicit analytical loads`);selected=unit;}}await ifcCheckpoint(control,"fem-load-units",units.length,units.length);if(!selected||(await graph.unitMeasureAsync(selected,control)).scale!==1)throw new IfcStepError("fem.load-unit",`Exactly one SI ${type} is required for explicit analytical loads`);};
    let contextOrdinal=0;for(const context of await relations.rows("contexts")) {if(contextOrdinal++%(control.batchSize!)===0)await ifcCheckpoint(control,"fem-contexts",contextOrdinal,0);const matrix=graph.axis(context.args[4]);if(matrix.some((n,i)=>Math.abs(n-([0,5,10,15].includes(i)?1:0))>1e-10))throw new IfcStepError("fem.context-placement","Nonidentity representation context requires explicit analytical frame conversion");}
    for(let i=0;i<snapshot.entities.length;i++) {
      if(i%(control.batchSize!)===0)await ifcCheckpoint(control,"fem-import",i,snapshot.entities.length);
      const entity=snapshot.entities[i]!;
      if(entity.name!=="IFCSTRUCTURALPOINTCONNECTION")continue;
      if(entity.args[8]?.kind==="reference")throw new IfcStepError("fem.support-axes","Local support coordinate systems require explicit conversion");
      const vertex=topology(entity);if(vertex.name!=="IFCVERTEXPOINT")throw new IfcStepError("fem.topology","Structural point must reference a vertex");
      const local=await graph.placementAsync(entity.args[5],control),xyz=ifcTransform(local,graph.vector(vertex.args[0])),id=`ifc${entity.id}`;
      if(vertices.has(vertex.id))throw new IfcStepError("fem.topology","Shared vertex has ambiguous structural point identity");
      if(local.some((n,i)=>Math.abs(n-([0,5,10,15].includes(i)?1:0))>1e-10))throw new IfcStepError("fem.connection-placement","Nonidentity point-connection placement requires explicit analytical frame conversion");
      const node={id,x:word(xyz[0]),y:word(xyz[1]),z:word(xyz[2])};model.nodes.push(node);nodeRows.set(id,node);nodes.set(entity.id,id);vertices.set(vertex.id,id);
      if(entity.args[7]?.kind==="reference") {
        const condition=graph.ref(entity.args[7]);if(condition.name!=="IFCBOUNDARYNODECONDITION")throw new IfcStepError("fem.support","Unsupported support condition");
        const fixed:FemDof[]=[];
        for(let k=0;k<6;k++) {const value=condition.args[k+1],boolean=value?.kind==="typedValue"&&value.value.name==="IFCBOOLEAN"?value.value.items[0]:value;if(boolean?.kind==="enum"&&boolean.value==="T")fixed.push(dofs[k]!);else if(boolean?.kind!=="unset"&&(boolean?.kind!=="enum"||boolean.value!=="F"))throw new IfcStepError("fem.spring","Spring restraints require a richer FEM model");}
        model.supports.push({id:`ifc${entity.id}`,nodeId:id,fixed});
      }
    }
    let materialOrdinal=0;
    for(const entity of await relations.rows("materials")) {
      if(materialOrdinal++%(control.batchSize!)===0)await ifcCheckpoint(control,"fem-materials",materialOrdinal,snapshot.entities.length);
      const definitions=await relations.related("materialProperties",entity.id),propertyValues:IfcValue[]=[];for(const row of definitions)for(const value of ifcAggregate(await relations.value(row,"Properties","IFCEXTENDEDPROPERTIES"))){if(propertyValues.length%(control.batchSize!)===0)await ifcCheckpoint(control,"fem-material-property-roster",propertyValues.length,0);propertyValues.push(value);}
      const values=await props(propertyValues);if(!values.size)continue;
      const id=`ifc${entity.id}`;model.materials.push({id,name:ifcText(entity.args[0]),e:required(values,"YoungModulus"),g:required(values,"ShearModulus"),nu:required(values,"PoissonRatio"),rho:required(values,"MassDensity")});materialIds.set(entity.id,id);
    }
    let memberOrdinal=0;
    for(const entity of await relations.rows("members")) {
      if(memberOrdinal++%(control.batchSize!)===0)await ifcCheckpoint(control,"fem-members",memberOrdinal,snapshot.entities.length);
      const edge=topology(entity);if(edge.name!=="IFCEDGE")throw new IfcStepError("fem.topology","Straight analytical edge required");
      const start=vertices.get(graph.ref(edge.args[0]).id),end=vertices.get(graph.ref(edge.args[1]).id);if(!start||!end)throw new IfcStepError("fem.topology","Member endpoint must have a structural point connection");
      const connections=await relations.related("connections",entity.id);
      const badConditions=()=>{throw new IfcStepError("fem.member-conditions","Exactly two endpoint connections without releases, springs, supported lengths or local condition axes are required");};if(connections.length!==2)badConditions();
      const endpoints:(string|undefined)[]=[];for(const row of connections){if(row.name!=="IFCRELCONNECTSSTRUCTURALMEMBER")badConditions();if(row.complex?.length){for(const name of ["AppliedCondition","AdditionalConditions","SupportedLength","ConditionCoordinateSystem"]){const value=await relations.value(row,name,"IFCRELCONNECTSSTRUCTURALMEMBER");if(value!==undefined&&value.kind!=="unset")badConditions();}}else for(let i=6;i<row.args.length;i++){if(i%(control.batchSize!)===0)await ifcCheckpoint(control,"fem-member-conditions",i,row.args.length);if(row.args[i]!.kind!=="unset")badConditions();}endpoints.push(nodes.get(graph.ref(await relations.value(row,"RelatedStructuralConnection","IFCRELCONNECTSSTRUCTURALMEMBER")).id));}
      if(new Set(endpoints).size!==2||!endpoints.includes(start)||!endpoints.includes(end))throw new IfcStepError("fem.member-endpoints","Member connection relations must identify both geometric endpoints");
      const associations=await relations.related("materialRelations",entity.id),assignedMaterials=new Map<bigint,IfcEntity>();for(const row of associations){await ifcCheckpoint(control,"fem-material-associations",assignedMaterials.size,associations.length);const material=graph.ref(await relations.value(row,"RelatingMaterial","IFCRELASSOCIATESMATERIAL"));assignedMaterials.set(material.id,material);}
      if(assignedMaterials.size!==1)throw new IfcStepError("fem.material","Exactly one unambiguous engineering material assignment is required");
      const material=[...assignedMaterials.values()][0]!,materialId=materialIds.get(material.id);
      if(!materialId)throw new IfcStepError("fem.material","An explicitly assigned SI material with complete engineering properties is required");
      const definitions=await relations.related("propertyRelations",entity.id,function*(row){const pset=graph.ref(yield* relations.valueSteps(row,"RelatingPropertyDefinition","IFCRELDEFINESBYPROPERTIES"));return ifcText(yield* relations.valueSteps(pset,"Name","IFCROOT"))==="semio_FemSection";}),sectionSets=new Map<bigint,IfcEntity>();for(const row of definitions){await ifcCheckpoint(control,"fem-section-definitions",sectionSets.size,definitions.length);const pset=graph.ref(await relations.value(row,"RelatingPropertyDefinition","IFCRELDEFINESBYPROPERTIES"));sectionSets.set(pset.id,pset);}
      if(sectionSets.size!==1)throw new IfcStepError("fem.section","Exactly one complete section definition is required; profile shapes are not inferred");
      const pset=[...sectionSets.values()][0]!,sectionId=sectionIds.get(pset.id)??`ifc${pset.id}`;
      if(!sectionIds.has(pset.id)) {const values=await props(ifcAggregate(await relations.value(pset,"HasProperties","IFCPROPERTYSET")));model.sections.push({id:sectionId,name:ifcText(values.get("SectionName")?.kind==="typedValue"?(values.get("SectionName") as Extract<IfcValue,{kind:"typedValue"}>).value.items[0]:undefined),area:required(values,"CrossSectionArea"),iy:required(values,"MomentOfInertiaY"),iz:required(values,"MomentOfInertiaZ"),j:required(values,"TorsionalConstant")});sectionIds.set(pset.id,sectionId);}
      const id=`ifc${entity.id}`,kind=entity.args[7];if(kind?.kind!=="enum"||!["PIN_JOINED_MEMBER","RIGID_JOINED_MEMBER"].includes(kind.value))throw new IfcStepError("fem.member","Only pin or rigid members are supported");
      if(entity.args[5]?.kind==="reference")throw new IfcStepError("fem.member-placement","Member-local placement requires explicit frame conversion");
      const n1=nodeRows.get(start)!,n2=nodeRows.get(end)!,delta=[n2.x,n2.y,n2.z].map((v,k)=>ifcNumber(real(v))-ifcNumber(real([n1.x,n1.y,n1.z][k]!))),length=Math.hypot(...delta);
      if(!length)throw new IfcStepError("fem.length","Zero-length member");
      const basis=frameBasis(delta),axis=normalized(graph.vector(entity.args[8]));if(Math.abs(dot(axis,basis.x))>1e-8)throw new IfcStepError("fem.axis","Member axis must be perpendicular to its edge");
      const roll=Math.atan2(-dot(axis,basis.y),dot(axis,basis.z));
      if(kind.value==="PIN_JOINED_MEMBER")model.elements.push({kind:"bar",id,start,end,materialId,sectionId});else model.elements.push({kind:"frame",id,start,end,materialId,sectionId,roll:word(roll)});
      members.set(entity.id,id);
    }
    for(const loadCase of selectedCases) {
      if(loadCase.args[8]?.kind!=="unset"&&number(loadCase.args[8])!==1)throw new IfcStepError("fem.load-factor","Nonunit load-case coefficient requires explicit scaling");
      const selfWeight=await ifcFemSelfWeight(await relations.value(loadCase,"SelfWeightCoefficients","IFCSTRUCTURALLOADCASE"),control);
      const loads:FemLoad[]=[];
      for(const assignment of await assignments(loadCase.id))for(const value of ifcAggregate(await relations.value(assignment,"RelatedObjects","IFCRELASSIGNS"))) {
        await ifcCheckpoint(control,"fem-load-actions",loads.length,0);
        const action=graph.ref(value);if(action.args[8]?.kind!=="enum"||action.args[8].value!=="GLOBAL_COORDS")throw new IfcStepError("fem.load-axes","Only global-coordinate loads supported");
        if(action.args[5]?.kind!=="unset"||action.args[6]?.kind!=="unset"||action.args[9]?.kind!=="unset")throw new IfcStepError("fem.action-placement","Action placement, representation and destabilizing-load semantics require explicit evaluation");
        const links=await relations.related("activityRelations",action.id);if(links.length!==1||links[0]!.name!=="IFCRELCONNECTSSTRUCTURALACTIVITY")throw new IfcStepError("fem.load-target","Exactly one supported structural action target required");const link=links[0]!;
        const target=graph.ref(await relations.value(link,"RelatingElement","IFCRELCONNECTSSTRUCTURALACTIVITY")),load=graph.ref(action.args[7]);
        if(action.name==="IFCSTRUCTURALPOINTACTION"&&load.name==="IFCSTRUCTURALLOADSINGLEFORCE") {
          const nodeId=nodes.get(target.id);if(!nodeId)throw new IfcStepError("fem.load-target","Point action requires a structural connection");
          let present=0;for(let k=0;k<6;k++){if(k%(control.batchSize!)===0)await ifcCheckpoint(control,"fem-load-components",k,6);const value=load.args[k+1];if(value?.kind==="unset")continue;const n=number(value);await requireUnit(k<3?"FORCEUNIT":"TORQUEUNIT");loads.push({kind:"nodal",id:`ifc${action.id}_${k}`,nodeId,dof:dofs[k]!,value:word(n)});present++;}
          if(!present)throw new IfcStepError("fem.load-components","Point action requires at least one explicit force or moment component for analytical projection");
        } else if(action.name==="IFCSTRUCTURALCURVEACTION"&&load.name==="IFCSTRUCTURALLOADLINEARFORCE") {
          if(action.args[10]?.kind!=="enum"||action.args[10].value!=="TRUE_LENGTH"||action.args[11]?.kind!=="enum"||action.args[11].value!=="CONST")throw new IfcStepError("fem.load-length","Only constant true-length member actions are supported");
          const elementId=members.get(target.id);if(!elementId)throw new IfcStepError("fem.load-target","Curve action requires a member");
          for(let i=4;i<load.args.length;i++){if(i%(control.batchSize!)===0)await ifcCheckpoint(control,"fem-member-moments",i,load.args.length);const value=load.args[i]!;if(value.kind!=="unset"&&number(value)!==0)throw new IfcStepError("fem.member-moment","Distributed member moments are not represented in FEM3d");}
          await requireUnit("LINEARFORCEUNIT");
          loads.push({kind:"memberUdl",id:`ifc${action.id}`,elementId,wx:word(number(load.args[1])),wy:word(number(load.args[2])),wz:word(number(load.args[3]))});
        } else throw new IfcStepError("fem.load","Nonuniform, reaction or unsupported structural action requires richer mapping");
      }
      model.loadCases.push({id:`ifc${loadCase.id}`,name:ifcText(loadCase.args[2]),loads,selfWeight});
    }
    if(!model.nodes.length)throw new IfcStepError("fem.empty","No analytical structural nodes");
  } catch(error) {scope.rethrowCallerFailure();if(error instanceof IfcStepError&&error.code==="io.cancelled")throw error;return failure(error instanceof IfcStepError?error.code:"fem.import",String(error));}
  diagnostics.push({code:"fem.projection",severity:"warning",message:"FEM projection retains supported analytical data; original IFC graph, identities, results and solver/display settings remain in the source artifact",entityIds:[]});
  await ifcCheckpoint(control,"fem-import",snapshot.entities.length,snapshot.entities.length);
  return {artifact:model,fidelity:"partial",diagnostics};
}
