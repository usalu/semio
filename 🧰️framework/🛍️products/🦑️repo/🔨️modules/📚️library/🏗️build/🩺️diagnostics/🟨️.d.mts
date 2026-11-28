/** 🗺️ The areas a diagnostic can belong to. */
export type DiagnosticAreaV1="cache"|"capacity"|"cargo-admission"|"graph"|"leases"|"ready"|"remote-execution"|"resident"|"timing";
/** 🧾️ One diagnostic record: a plain JSON object, conventionally with an `event` name. */
export type DiagnosticRecordV1=Readonly<Record<string,unknown>>;
export interface DiagnosticsProtocolV1{readonly variable:"SEMIO_DIAGNOSTICS";readonly everything:"all";readonly silent:readonly string[];readonly linePrefix:string;readonly lineSuffix:string;readonly maximumLineCharacters:number}
export interface DiagnosticsOptionsV1{environment?:Readonly<Record<string,string|undefined>>;now?:()=>number;write?:(text:string)=>unknown}
export interface DiagnosticsV1{readonly areas:readonly DiagnosticAreaV1[];enabled(area:DiagnosticAreaV1):boolean;diagnose(area:DiagnosticAreaV1,build:()=>DiagnosticRecordV1):boolean}
export const DIAGNOSTICS_PROTOCOL_V1:DiagnosticsProtocolV1;
export const DIAGNOSTICS_AREAS_V1:Readonly<Record<DiagnosticAreaV1,Readonly<{en:string;de:string}>>>;
export const DIAGNOSTICS_VARIABLE_V1:"SEMIO_DIAGNOSTICS";
export function readDiagnosticAreasV1(environment?:Readonly<Record<string,string|undefined>>):readonly DiagnosticAreaV1[];
export function renderDiagnosticLineV1(area:DiagnosticAreaV1,record:DiagnosticRecordV1,now?:number):string;
export function createDiagnosticsV1(options?:DiagnosticsOptionsV1):DiagnosticsV1;
export function diagnosticsV1():DiagnosticsV1;
export function resetDiagnosticsV1():void;
export function diagnoseV1(area:DiagnosticAreaV1,build:()=>DiagnosticRecordV1):boolean;
