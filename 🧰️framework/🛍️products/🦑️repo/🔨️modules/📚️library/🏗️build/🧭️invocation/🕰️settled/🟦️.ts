/** 🧾️ What the clients of one checkout remember between processes: the implementation whose service they attach to (`settled`) and when they first saw the tree move away from it (`firstAt`, absent while the tree equals it). */
export type SettledRecordV1=Readonly<{settled:string;firstAt?:number}>;

/** 🕰️ The implementation a client asks for while the files of the host service are being edited: the one its service already runs (`record.settled`, only while a service of it answers) until the tree was quiet for `quietMilliseconds` (the newest edit is that old) or has differed for `maximumDeferralMilliseconds`, then the wanted one, unless `generations.maximum` services of other implementations already live and the wanted one is not among them: a successor starts beside the settled service only while fewer than the maximum live (the older one drains after its plans), and the settled one is kept for at most `generations.maximumMilliseconds` of deferral. A burst of edits therefore makes one new generation after it, not one per edit and per client; a service that no longer answers, no record and an unmoved tree ask for the wanted implementation at once. Pure: the caller reads the clock, the edit time and the liveness.
 * @see ../🛰️plan/🟦️.ts `settledHostImplementationV1`, ../../🏡️resident/🔄️reload/🟨️.mjs the same rule for a running service
 * @see ./🧪️tests/🔣️.json */
export function settleHostImplementationV1({wanted,record,now,editedAt,answers,quietMilliseconds,maximumDeferralMilliseconds,generations}:Readonly<{wanted:string;record:SettledRecordV1|undefined;now:number;editedAt:number;answers:boolean;quietMilliseconds:number;maximumDeferralMilliseconds:number;generations?:Readonly<{live:readonly string[];maximum:number;maximumMilliseconds:number}>}>):Readonly<{implementation:string;record:SettledRecordV1}>{
 if(record===undefined||record.settled===wanted||!answers)return{implementation:wanted,record:{settled:wanted}};
 const firstAt=record.firstAt??now,due=now-editedAt>=quietMilliseconds||now-firstAt>=maximumDeferralMilliseconds;
 if(!due)return{implementation:record.settled,record:{settled:record.settled,firstAt}};
 if(generations!==undefined&&!generations.live.includes(wanted)&&generations.live.length>=generations.maximum&&now-firstAt<generations.maximumMilliseconds)return{implementation:record.settled,record:{settled:record.settled,firstAt}};
 return{implementation:wanted,record:{settled:wanted}};
}
