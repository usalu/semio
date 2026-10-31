export function watcherSourceDigestV1(sourcePath:string):string;
export function readWatcherPinV1(pinPath:string,sourcePath:string):{version:string;sha256:string};
export function watcherManifestV1():string;
export function ensureWatcherV1(options:{sourcePath:string;pinPath:string;outputRoot:string;locate:()=>Promise<string|undefined>|string|undefined;compile:(request:{directory:string;signal?:AbortSignal})=>Promise<string>;workRoot?:string;writable?:()=>boolean|Promise<boolean>;signal?:AbortSignal;progress?:(phase:"compile"|"ready")=>void;platform?:string;probe?:(path:string)=>boolean}):Promise<{path:string;built:boolean;version:string;sha256:string}>;
