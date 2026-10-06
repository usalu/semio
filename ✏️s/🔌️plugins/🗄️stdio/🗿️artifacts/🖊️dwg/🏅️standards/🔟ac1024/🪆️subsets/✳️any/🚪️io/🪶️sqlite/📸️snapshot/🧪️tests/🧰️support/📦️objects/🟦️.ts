/** 🧫️ Handcrafted DWG typed boundary fixture shared by component and complete snapshot laws. */
import type { DwgLogicalObjectBody,DwgAssociativeDependency,DwgAssociativeAction,DwgEvaluationExpression } from "../../../../../../🧬️schema/🟦️.ts";

const max=18446744073709551615n;
const dependency:DwgAssociativeDependency={status:"upToDate",isReadDependency:true,isWriteDependency:false,isAttachedToObject:true,isDelegatingToOwningAction:false,order:-2147483648,dependentOnObjectHandle:max,name:"",readDependencyHandle:0n,dependencyNodeHandle:undefined,dependencyBodyHandle:max,dependencyBodyId:2147483647};
const action:DwgAssociativeAction={status:"upToDate",owningNetworkHandle:max,actionBodyHandle:0n,actionIndex:-2147483648,maximumDependencyIndex:2147483647,dependencies:[{owned:true,dependencyHandle:max},{owned:false,dependencyHandle:0n}]};
const expressions:DwgEvaluationExpression[]=[{kind:"empty"},{kind:"double",value:{bits:0x7ff0000000000001n}},{kind:"pointGroup10",value:[{bits:0x8000000000000000n},{bits:0xfff0000000000000n}]},{kind:"pointGroup11",value:[]},{kind:"string",value:"\0preserved"},{kind:"integer32",value:-2147483648},{kind:"objectReference",value:max},{kind:"integer16",value:-32768}].map((value,index)=>({parentId:-2147483648,majorVersion:4294967295,minorVersion:0,nodeId:index,value:value as DwgEvaluationExpression["value"]}));
const bodies:DwgLogicalObjectBody[]=[
  {kind:"placeholder",value:{}},{kind:"dictionaryVariable",value:{value:"\0variable"}},{kind:"annotationScale",value:{name:"IEEE",paperUnits:{bits:0x7ff8123456789abcn},drawingUnits:{bits:0x8000000000000000n},isUnitScale:true}},
  {kind:"sortEntitiesTable",value:{blockHeaderHandle:max,entries:[{entityHandle:0n,sortHandle:max},{entityHandle:max,sortHandle:0n}]}},{kind:"blockParameterDependencyBody",value:{name:"parameter"}},{kind:"associativeDimensionDependencyBody",value:{name:"dimension"}},{kind:"blockRepresentationData",value:{representedBlockHeaderHandle:max}},{kind:"dynamicBlockPurgePreventer",value:{protectedBlockHeaderHandle:0n}},
  {kind:"evaluationGraph",value:{nodes:[{id:4294967295,expressionHandle:max},{id:0,expressionHandle:0n},{id:0,expressionHandle:max}],edges:[{fromNodeId:0,toNodeId:4294967295,referenceCount:4294967295,invertible:true,suppressed:false},{fromNodeId:0,toNodeId:0,referenceCount:0,invertible:false,suppressed:true}]}},
  {kind:"associativeDependency",value:dependency},{kind:"associativeValueDependency",value:{dependency,cachedValue:{kind:"integer32",value:-2147483648},valueName:"cached"}},{kind:"associativeGeometryDependency",value:{dependency,enabled:false,persistentSubentityClassName:"class",dependentOnCompoundObject:true}},
  {kind:"associativeVariable",value:{action,name:"n",expression:"1+2",evaluatorId:"e",description:"d",evaluatedValue:{kind:"integer32",value:2147483647},mergeable:false,mergeableVariableName:"",mustMerge:true,referencedValueDependencyHandles:[max,0n,max]}},
  {kind:"assocNetwork",value:{action,networkActionIndex:-2147483648,actions:[{kind:"network",handle:max},{kind:"action",handle:0n}]}},
  ...expressions.map(evaluationExpression=>({kind:"blockGripLocationComponent" as const,value:{evaluationExpression,gripType:4294967295,gripExpression:"expr"}})),{kind:"dynamicBlockProxyNode",value:{evaluationExpression:expressions[1]!}}
];

export { bodies as dwgObjectBodies };
