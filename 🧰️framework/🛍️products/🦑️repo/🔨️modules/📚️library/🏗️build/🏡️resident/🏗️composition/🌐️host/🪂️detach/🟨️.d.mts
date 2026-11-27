/** 🪂️ Starts a process outside every owned execution of the caller; answers its pid. */
export function spawnDetachedV1(options:{argv:readonly string[];cwd:string;env:Readonly<Record<string,string|undefined>>;log:string;run?:unknown}):number|undefined;
