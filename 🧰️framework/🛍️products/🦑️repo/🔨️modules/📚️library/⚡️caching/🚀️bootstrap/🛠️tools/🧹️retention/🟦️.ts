export interface NxToolingRetentionImageV1 {readonly identity:string;readonly createdAt:number}
/** 🧹️ Plans bounded immutable images while retaining the selected image and deterministic newest cohort. */
export function planNxToolingRetentionV1(images:readonly NxToolingRetentionImageV1[],current:string,keepNewest:number):readonly string[]{
 if(!/^[a-f0-9]{64}$/u.test(current)||!Number.isSafeInteger(keepNewest)||keepNewest<1||keepNewest>32||new Set(images.map(image=>image.identity)).size!==images.length||images.some(image=>!/^[a-f0-9]{64}$/u.test(image.identity)||!Number.isFinite(image.createdAt)||image.createdAt<0)||!images.some(image=>image.identity===current))throw Error("Closed current immutable tooling cohort required");
 const ordered=[...images].sort((a,b)=>b.createdAt-a.createdAt||(a.identity<b.identity?-1:a.identity>b.identity?1:0)),retained=new Set([current,...ordered.slice(0,keepNewest).map(image=>image.identity)]);
 return ordered.filter(image=>!retained.has(image.identity)).map(image=>image.identity).sort();
}
