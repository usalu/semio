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
/** 📍️ True while an invocation is committed to the implementation it loaded, so no self-check refuses a Source edited since. */
export function implementationPinnedV1():boolean;
/** 📌️ Runs `body` committed to the implementation this process loaded; pins nest and a failing body releases its pin. */
export function withImplementationPinV1<T>(body:()=>Promise<T>|T):Promise<T>;
/** 📸️ Reads authority data once per pinned invocation and returns the first answer to every later read of the key. */
export function pinnedAuthorityReadV1<T>(key:string,read:()=>T):T;
/** 🧽️ Forgets the authority snapshot. */
export function clearPinnedAuthorityV1():void;
