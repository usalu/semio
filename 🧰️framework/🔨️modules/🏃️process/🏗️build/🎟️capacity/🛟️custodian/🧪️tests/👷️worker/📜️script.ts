import {writeFileSync,existsSync} from "node:fs";

writeFileSync(process.argv[2]!,JSON.stringify({parent:process.ppid,worker:process.pid}));
while(!existsSync(process.argv[3]!))await Bun.sleep(10);
process.stdout.write("paid-child");
