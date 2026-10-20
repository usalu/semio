/** 🧾️ Copies a bounded ordinary Source observation without a descriptor, owner or effect port. */
export interface BoundedSourceRevisionV1{readonly sha256:string;readonly byteLength:number;readonly text:string|null}
/** 🔏️ Allows only reducing byte extents; retained UTF8 text also requires the private text ceiling. */
export function captureBoundedSourceRevisionV1(path:string,maximumBytes:number,retainText?:boolean):BoundedSourceRevisionV1;

/** 📦️ Compiler-private immutable DATA binds original paths to captured bytes without granting custody. */
export interface CapturedPhysicalSourcesV1{readonly version:1;readonly root:string;readonly sources:readonly{readonly path:string;readonly sha256:string}[]}

/** 🧭️ The compiler substitutes this caller-owned field without changing ordinary artifact metadata. */
export interface PhysicalSourceModuleMetaV1{readonly semioPhysicalSourceURL?:string}
/** 🧭️ Selects only explicit physical Source provenance; the ordinary URL keeps execution identity. */
export function physicalSourceOriginV1(moduleURL:string,compiledSourceURL?:string):string;
/** 📄️ Returns reducing UTF8 contract DATA under actual physical descriptor and captured-byte checks. */
export function capturePhysicalSourceTextV1(url:URL,maximumBytes:number):string;
