import {startRemoteCacheServerV1} from "../../🟦️.ts";

/** 🚀️ A cache server as a process of its own on a fixed port and an explicit directory, for the restart tests: prints "listening" and serves until SIGTERM. */
const server=await startRemoteCacheServerV1({directory:process.env.CACHE_DIRECTORY!,host:"127.0.0.1",port:Number(process.env.CACHE_PORT),maximumBytes:2**28,maximumEntryBytes:2**26,maximumAgeDays:30,defaultInstance:"main",tokens:{write:"w-secret",read:"w-secret"}});
console.log("listening "+server.url);
process.on("SIGTERM",()=>{void server.stop().finally(()=>process.exit(0));});
setInterval(()=>{},1<<30);
