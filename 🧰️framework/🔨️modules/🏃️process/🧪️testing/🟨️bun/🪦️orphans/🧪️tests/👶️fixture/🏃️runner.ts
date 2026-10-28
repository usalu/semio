import{spawn}from"node:child_process";
const[bun,file,...flags]=process.argv.slice(2),child=spawn(bun!,["test",...flags,file!],{detached:true,stdio:"ignore"});
child.unref();
process.stdout.write(`CHILD ${child.pid}\n`);
setTimeout(()=>process.kill(process.pid,"SIGKILL"),Number(process.env.RUNNER_DIE_MS??1500));
setInterval(()=>{},1000);
