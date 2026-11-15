import {appendFileSync,writeFileSync} from "node:fs";

const mode=process.argv[2],log=process.env.SIDECAR_TEST_LOG;
if(log)appendFileSync(log,`${mode} ${Date.now()}\n`);
if(process.env.SIDECAR_TEST_ENV)writeFileSync(process.env.SIDECAR_TEST_ENV,JSON.stringify({origin:process.env.SEMIO_FLEET_ORIGIN,group:process.env.SEMIO_FLEET_GROUP,store:process.env.SEMIO_FLEET_STORE_ROOT,root:process.env.SEMIO_FLEET_ROOT,name:process.env.SEMIO_FLEET_SIDECAR,record:process.env.SEMIO_FLEET_SIDECAR_RECORD,lease:process.env.SEMIO_FLEET_SIDECAR_LEASE,leaseMs:process.env.SEMIO_FLEET_SIDECAR_LEASE_MS}));
console.log(`child ${mode} started`);
if(mode==="crash")setTimeout(()=>process.exit(3),Number(process.argv[3]??30));
else if(mode==="stay")process.on("SIGTERM",()=>process.exit(0));
else if(mode==="stubborn")process.on("SIGTERM",()=>{});
if(mode!=="crash")setInterval(()=>{},1<<30);
