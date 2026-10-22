/** 🏊️ Opens and closes one custody session on behalf of many requests. */
export type CustodyOpenerV1<S>=()=>Promise<Readonly<{session:S;close():Promise<void>}>>;
/** ⏲️ Injectable monotonic clock and one-shot timer. */
export type CustodyTimerV1=Readonly<{now():number;set(milliseconds:number,run:()=>void):unknown;clear(handle:unknown):void}>;
/** 🏊️ A lazily opened session shared by concurrent requests, released when idle or old. */
export type CustodyPoolV1<S>=Readonly<{with<T>(run:(session:S)=>Promise<T>):Promise<T>;close():Promise<void>;opens():number;closes():number;live():boolean}>;

/** 🏊️ Keeps one custody session open across requests so a warm request pays no store open, while an idle or old session is closed so exclusive waiters are never starved and a failed request reopens custody from scratch. */
export function createCustodyPoolV1<S>(options:Readonly<{open:CustodyOpenerV1<S>;idleMilliseconds:number;maximumLifetimeMilliseconds:number;timer:CustodyTimerV1}>):CustodyPoolV1<S>{
 if(!(options.idleMilliseconds>0)||!(options.maximumLifetimeMilliseconds>=options.idleMilliseconds))throw Error("Bounded custody pool lifetimes required");
 let current:Readonly<{opened:number;ready:Promise<Awaited<ReturnType<CustodyOpenerV1<S>>>>}>|undefined,inflight=0,idle:unknown,opened=0,closed=0;const dropped=new WeakSet<object>();
 const drop=async(entry:NonNullable<typeof current>)=>{if(current===entry)current=undefined;if(dropped.has(entry))return;dropped.add(entry);try{const held=await entry.ready;closed++;await held.close();}catch{}};
 const arm=()=>{if(idle!==undefined)options.timer.clear(idle);idle=undefined;const entry=current;if(entry&&inflight===0)idle=options.timer.set(options.idleMilliseconds,()=>{idle=undefined;if(current===entry&&inflight===0)void drop(entry);});};
 const acquire=async()=>{
  if(current&&inflight===0&&options.timer.now()-current.opened>=options.maximumLifetimeMilliseconds)await drop(current);
  if(!current){const entry={opened:options.timer.now(),ready:options.open()};current=entry;opened++;entry.ready.catch(()=>{if(current===entry)current=undefined;});}
  return current;
 };
 return Object.freeze({
  async with<T>(run:(session:S)=>Promise<T>){
   if(idle!==undefined){options.timer.clear(idle);idle=undefined;}
   const entry=await acquire();inflight++;
   try{return await run((await entry.ready).session);}
   catch(error){if(current===entry)current=undefined;void drop(entry);throw error;}
   finally{inflight--;arm();}
  },
  async close(){if(idle!==undefined)options.timer.clear(idle);idle=undefined;const entry=current;if(entry)await drop(entry);},
  opens:()=>opened,closes:()=>closed,live:()=>current!==undefined,
 });
}
