/** 🧭️ Maps admitted inference namespaces into the existing finite fact-writer catalog. */
export function buildGraphWriterResourceV1(workspace:string,kind:"selection"|"dataset"|"canonical",identity:string):string;
/** 🔒️ Names the complete selected publication writer shared by readers, publishers and retirement. */
export function buildGraphPublicationWriterResourceV1(graphRoot:string,scope:string):string;
/** 🪪️ Names the exclusive claim on one selected scope for the whole of its load, in a fixed catalog of buckets. */
export function buildGraphScopeWriterResourceV1(graphRoot:string,scope:string):string;
/** 🔎️ Reads the lock a graph writer resource names: its kind and number, or undefined for any other resource. */
export function describeGraphWriterResourceV1(resource:string):Readonly<{kind:"facts"|"publication"|"scope";number:number}>|undefined;
