import{spawn}from"node:child_process";
const[bun,file,mode,die]=process.argv.slice(2),child=spawn(bun!,[file!,mode!],{detached:true,stdio:"ignore"});
child.unref();
process.stdout.write(`CHILD ${child.pid}\n`);
setTimeout(()=>process.kill(process.pid,"SIGKILL"),Number(die));
setInterval(()=>{},1000);
