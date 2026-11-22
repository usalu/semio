import {readFileSync,writeFileSync} from "node:fs";
const seen:string[]=[];
Bun.plugin({name:"loaded-modules",setup(build){build.onLoad({filter:/\.(ts|mjs)$/},args=>{seen.push(args.path);return {contents:readFileSync(args.path,"utf8"),loader:args.path.endsWith(".ts")?"ts":"js"};});}});
const capacity=await import("../../../🟦️.ts"),mark=seen.length;
const sealed=await capacity.sealedCurrentReadV1(["show","project","p","--graph-policy=current"],{});
writeFileSync(process.argv[2]!,JSON.stringify({before:seen.slice(0,mark),after:seen.slice(mark),sealed}));
