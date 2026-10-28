import{spawn}from"node:child_process";
const children=Array.from({length:Number(process.argv[2]??1)},()=>spawn("sleep",["60"],{detached:true,stdio:"ignore"}));
for(const child of children)child.unref();
process.stdout.write(`CHILDREN ${children.map(child=>child.pid).join(" ")}\n`);
setTimeout(()=>process.exit(0),Number(process.argv[3]??300));
