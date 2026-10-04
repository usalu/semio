import type{Binary64}from"../../../../../../../../🔨️modules/🚪️io/🪶️sqlite-snapshot/🔢️ieee754/🟦️.ts";
/** 🎛️ Authored media classes in their native declaration order. */
export type WorkflowMediaClass="twoD"|"threeD"|"text"|"data"|"graph"|"kit"|"computation"|"presentation";
/** 🧩️ Authored media forms in their native declaration order. */
export type WorkflowMediaForm="any"|"vector"|"raster"|"brep"|"mesh"|"document"|"value"|"dag"|"trinity"|"type"|"design"|"kit"|"flow"|"sequence"|"procedure"|"deck";
/** 🔌️ Literal persisted endpoint and complete port specification. */
export interface WorkflowPort{id:string;spec:{id:string;label:string;direction:"in"|"out";mediaType:{class:WorkflowMediaClass;form:WorkflowMediaForm};kindId?:string;required:boolean;multiplicity:"one"|"many"}}
/** 🕸️ Complete persisted app node, including exact geometry words. */
export interface WorkflowNode{id:string;pluginId:string;appId:string;label:string;yields:string;artifactRef:string;configRef:string;x:Binary64;y:Binary64;width:Binary64;height:Binary64;inputs:WorkflowPort[];outputs:WorkflowPort[]}
/** 🤝️ Complete persisted edge contract and optional conversion pair. */
export interface WorkflowContract{kindId:string;mediaType:{class:WorkflowMediaClass;form:WorkflowMediaForm};wire:{kind:"binary";format_kind:string}|{kind:"document";schema:string}|{kind:"intrinsic";schema:string};conversion:[WorkflowMediaForm,WorkflowMediaForm]|null}
/** 🔗️ Literal unresolved graph endpoints are persisted independently of entity keys. */
export interface WorkflowEdge{id:string;sourceNodeId:string;sourcePortId:string;targetNodeId:string;targetPortId:string;contract:WorkflowContract}
/** 🎚️ Every authored parameter variant with exact optional numeric identities. */
export type WorkflowParameter={type:"numeric";id:string;name:string;value:Binary64;min:Binary64|null;max:Binary64|null;step:Binary64|null}|{type:"categorical";id:string;name:string;value:string;options:string[]}|{type:"toggle";id:string;name:string;value:boolean}|{type:"text";id:string;name:string;value:string};
/** 📸️ Complete root at the existing native Value field spellings. */
export interface WorkflowSnapshot{schema:string;graph:{schema:string;nodes:WorkflowNode[];edges:WorkflowEdge[]};parameters:WorkflowParameter[];parameter_bindings:{parameterId:string;nodeId:string;fieldPath:string}[];inputs:{id:string;kindId:string;selector:string;required:boolean;multiplicity:"one"|"many"}[];input_bindings:{inputId:string;nodeId:string;portId:string}[];output_bindings:{nodeId:string;portId:string;pathTemplate:string}[]}
export{WORKFLOW_SQLITE_SCHEMA,workflowSnapshotToSqliteDatabase,workflowSnapshotFromSqliteDatabase}from"./🪶️sqlite/🟦️.ts";
