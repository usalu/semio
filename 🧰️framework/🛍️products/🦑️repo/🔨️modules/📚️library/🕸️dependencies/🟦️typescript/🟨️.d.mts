/** 🧬️ Declares the validated first-party source contract without compiler-owned types. */
export interface SourceInputContractV1 {entries:readonly string[];generated:readonly string[];externalDependencies:readonly string[]}
/** 📋️ Describes the actual collected Source paths and declared external package names. */
export interface SourceInputClosureV1 {files:string[];externalDependencies:string[]}
export function sourceInputContract(value:unknown):SourceInputContractV1;
export function readSourceInputContract(path:string):SourceInputContractV1;
export function sourceRuntimeImportsV1(path:string):string[];
export function sourceRuntimeImportsFromBytesV1(path:string,bytes:Uint8Array):string[];
export function assertGeneratedSources(value:unknown,workspace:string):void;
export function relativeSourceInputs(value:unknown,workspace:string):SourceInputClosureV1;
export function relativeSourceInputsAsyncV1(value:unknown,workspace:string,advance?:(index:number)=>void|Promise<void>):Promise<SourceInputClosureV1>;
