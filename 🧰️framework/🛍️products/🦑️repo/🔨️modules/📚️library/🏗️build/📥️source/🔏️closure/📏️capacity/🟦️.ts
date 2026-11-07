import schema from "../🧬️schema/🔣️.json" with { type: "json" };

/** 📏️ Applies the shared schema bounds to source counts and, when present, the serialized image. */
export function buildSourceCapacityV1(files:number,sourceBytes:number,snapshotBytes?:number):boolean{
 const policy=schema.definitions.Policy.const;
 return Number.isSafeInteger(files)&&files>=0&&files<=policy.maximumFiles&&Number.isSafeInteger(sourceBytes)&&sourceBytes>=0&&sourceBytes<=policy.maximumSourceBytes&&(snapshotBytes===undefined||Number.isSafeInteger(snapshotBytes)&&snapshotBytes>=0&&snapshotBytes<=policy.maximumSnapshotBytes);
}
