/** 📦️ Canonical persisted OPC package fields shared by Office owners. */
export interface OpcPart {path:string;contentType:string;bytes:number[]}
export interface OpcRelationship {id:string;relType:string;target:string;targetMode:"internal"|"external"}
export interface OpcPackage {parts:OpcPart[];contentTypes:{defaults:[string,string][];overrides:[string,string][]};relationships:Record<string,OpcRelationship[]>;comment:string}
