import {nativeStorageBackendsV1} from "../../🧭️selection/🟦️.ts";

/** 🏠️ Tells whether the directory partition is declared to run on a host platform; macOS, Linux and Windows all are. */
export function directoryPartitionHostV1(platform:string=process.platform):boolean{return nativeStorageBackendsV1.some(row=>row.id==="directory-partition"&&row.platforms.includes(platform));}
