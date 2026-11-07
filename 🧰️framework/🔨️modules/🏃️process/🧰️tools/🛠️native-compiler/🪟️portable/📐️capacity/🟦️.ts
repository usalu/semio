import schema from"./🧬️schema/🔣️.json" with{type:"json"};

const policy=schema.definitions.Policy.const;
/** 📐️ Estimates pinned compiler staging on the observed volume without granting publication authority. */
export function portableMsvcCapacityV1(allocationBytes:number):Readonly<{version:1;allocationBytes:number;minimumAvailableBytes:number}>|null{
 if(!Number.isSafeInteger(allocationBytes)||allocationBytes<1||!Number.isInteger(Math.log2(allocationBytes)))return null;
 const files=policy.outputFiles+policy.payloadFiles+policy.metadataFiles+1,bytes=policy.outputBytes+policy.payloadBytes+policy.catalogBytes+policy.metadataBytes,estimate=bytes+files*(allocationBytes-1)+policy.entryAllowance*allocationBytes;
 if(!Number.isSafeInteger(estimate)||estimate>policy.maximumBytes)return null;
 return Object.freeze({version:1,allocationBytes,minimumAvailableBytes:estimate});
}
