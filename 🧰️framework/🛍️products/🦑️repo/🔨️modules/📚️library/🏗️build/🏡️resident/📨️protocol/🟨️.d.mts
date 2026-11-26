export class ProtocolErrorV1 extends Error{readonly code:string;constructor(code:string,message:string)}
export function encodeFrameV1(frame:unknown):string;
/** 📨️ One decoded wire frame: its type, the request id it answers or carries when it has one, and the fields of its type. */
export type HostFrameV1=Readonly<{t:string;id?:number}&Record<string,unknown>>;
export function createFrameDecoderV1(options:{maximumFrameBytes:number;onFrame:(frame:HostFrameV1)=>void;onError:(error:ProtocolErrorV1)=>void}):{push(chunk:string|Uint8Array):void;end():void};
export function compileSchemaV1(document:{definitions:Record<string,unknown>},name?:string):(value:unknown)=>string[];
