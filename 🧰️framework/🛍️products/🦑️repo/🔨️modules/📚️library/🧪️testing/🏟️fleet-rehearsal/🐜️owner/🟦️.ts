/** 🐜️ One owner of the rehearsal as its own process: it holds the declared memory (every page touched), burns processor time for half of its duration and rests for the other half, then ends; it carries the marker of its run in its arguments so that the run can find and end it. */
const argument=(name:string):string|undefined=>process.argv.find(value=>value.startsWith(name+"="))?.slice(name.length+1);
const milliseconds=Number(argument("--duration")??1000),megabytes=Number(argument("--mb")??64);
const held=Buffer.alloc(megabytes*1048576,1);let sum=0;for(let at=0;at<held.length;at+=4096)sum+=held[at]!;
const burnUntil=Date.now()+milliseconds/2;while(Date.now()<burnUntil)sum+=Math.sqrt(Date.now());
await new Promise<void>(done=>setTimeout(done,milliseconds/2));
process.exit(sum<0?1:0);
