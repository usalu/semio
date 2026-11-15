import {startClusterV1} from "./🟦️.ts";

/** 🚀️ Runs a cache and a scheduler as a process of their own for tests of a fleet of processes: prints one JSON line with the endpoints, serves until SIGTERM. `SCHEDULER` carries scheduler options as JSON. */
const cluster=await startClusterV1({workers:0,scheduler:JSON.parse(process.env.SCHEDULER??"{}")});
console.log(JSON.stringify({grpc:cluster.server.grpcUrl,worker:cluster.server.workerUrl,cache:cluster.cache.url}));
process.on("SIGTERM",()=>{void cluster.stop().finally(()=>process.exit(0));});
setInterval(()=>{},1<<30);
