/** 🐏️ The declared environment that caps a process reading the project graph. */
export function graphChildEnvironmentV1(policy?:{readonly heapEnvironment:string;readonly childHeapMegabytes:number},purge?:{readonly environmentVariable:string;readonly delayMilliseconds:number}):Readonly<Record<string,string>>;
