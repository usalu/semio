/** 🚨️ The kind of answer that refused work: `capacity` is a 507 (the store or its volume is full), `busy` a 503 (the server holds its ceiling of concurrent requests). */
export type RemoteCacheRefusalKindV1="capacity"|"busy";

/** 🧾️ Counts the refusals of a server and says so in the log: the first of a window at once, then one line per window with the count of that window, the total since the start and the detail of the latest, English and German in one line, so a refusal rate can be read from the server log of any hour. */
export type RemoteCacheRefusalLogV1=Readonly<{record(kind:RemoteCacheRefusalKindV1,detail?:string):void;total(kind:RemoteCacheRefusalKindV1):number;flush():void}>;

const TEXTS={
 capacity:{en:"refused {count} write(s) with 507 in the last {seconds} s ({total} since start): {detail}",de:"{count} Schreibvorgang/-vorgänge in den letzten {seconds} s mit 507 abgelehnt ({total} seit dem Start): {detail}"},
 busy:{en:"refused {count} request(s) with 503 busy in the last {seconds} s ({total} since start)",de:"{count} Anfrage(n) in den letzten {seconds} s mit 503 ausgelastet abgelehnt ({total} seit dem Start)"},
} as const;

/** 🏗️ Builds the log: `report` receives each line, `intervalMilliseconds` is the length of a window (the first refusal after a quiet window is reported at once, the rest of the window is summed up in the next line), `now` the clock. */
export function createRemoteCacheRefusalLogV1(options:Readonly<{report:(line:string)=>void;intervalMilliseconds:number;now?:()=>number}>):RemoteCacheRefusalLogV1{
 const now=options.now??Date.now,totals:Record<RemoteCacheRefusalKindV1,number>={capacity:0,busy:0},pending:Record<RemoteCacheRefusalKindV1,number>={capacity:0,busy:0},details:Partial<Record<RemoteCacheRefusalKindV1,string>>={},reportedAt:Record<RemoteCacheRefusalKindV1,number|undefined>={capacity:undefined,busy:undefined};
 const say=(kind:RemoteCacheRefusalKindV1,at:number):void=>{
  const count=pending[kind];if(count===0)return;
  const since=reportedAt[kind],seconds=Math.max(1,Math.round((since===undefined?0:at-since)/1000)),fill=(language:"en"|"de"):string=>TEXTS[kind][language].replace("{count}",String(count)).replace("{seconds}",String(since===undefined?0:seconds)).replace("{total}",String(totals[kind])).replace("{detail}",details[kind]??"");
  pending[kind]=0;reportedAt[kind]=at;
  try{options.report(`${fill("en")} / ${fill("de")}`);}catch{}
 };
 return Object.freeze({
  record(kind,detail){
   const at=now();totals[kind]++;pending[kind]++;if(detail!==undefined)details[kind]=detail;
   const since=reportedAt[kind];
   if(since===undefined||at<since||at-since>=options.intervalMilliseconds)say(kind,at);
  },
  total:kind=>totals[kind],
  flush(){const at=now();say("capacity",at);say("busy",at);},
 });
}
