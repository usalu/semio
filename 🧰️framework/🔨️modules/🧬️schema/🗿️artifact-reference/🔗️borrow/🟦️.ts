/** 🪪️ Exact target identity independent of value codecs and registries. */
export interface BorrowedArtifactRef {readonly artifactId:string;readonly artifactKind:string;readonly standard:string;readonly subset:string}
/** 🔡️ Matches exactly three lowercase ASCII kebab segments. */
export function isCanonicalArtifactKind(kind:string):boolean{return kind.split(".").length===3&&kind.split(".").every(segment=>segment.length>0&&!segment.startsWith("-")&&!segment.endsWith("-")&&!segment.includes("--")&&[...segment].every(value=>value>="a"&&value<="z"||value>="0"&&value<="9"||value==="-"));}
