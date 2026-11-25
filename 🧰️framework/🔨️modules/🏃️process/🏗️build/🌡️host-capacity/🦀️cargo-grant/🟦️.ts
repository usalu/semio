import {existsSync,readFileSync}from"node:fs";
import {dirname,join}from"node:path";
import schema from"./🧬️schema/🔣️.json"with{type:"json"};

const policy=Object.freeze({...schema.definitions.Policy.const});

/** 📜️ The authored constants of the cargo grant. */
export const CARGO_GRANT_POLICY_V1=policy;

const clean=(text:string):string=>text.replace(/\s+/gu,"_").slice(0,policy.partMaximumLength);
const head=(args:readonly string[]):readonly string[]=>{const boundary=args.indexOf("--");return boundary<0?args:args.slice(0,boundary);};

/** 🪜️ Names the classes one Cargo invocation learns its memory under, from general to specific: `cargo`, `cargo:<subcommand>`, `cargo:<subcommand>:<package>` (`workspace` for `--workspace`/`--all`, the first package for several); a `+toolchain` selector never names a subcommand.
 * @see ./🧪️tests/🟦️.ts */
export function cargoGrantLineageV1(args:readonly string[]):readonly string[]{
 const own=head(args),subcommand=own.find(argument=>!argument.startsWith("-")&&!argument.startsWith("+"))??"",lineage=[policy.operation];
 let part:string|undefined;
 for(let index=0;index<own.length&&part===undefined;index++){
  const argument=own[index]!;
  if((argument==="-p"||argument==="--package")&&own[index+1]!==undefined)part=own[index+1];
  else if(argument.startsWith("--package="))part=argument.slice(10);
  else if(policy.workspaceFlags.includes(argument))part=policy.workspaceClass;
 }
 for(const name of[subcommand,part??""].filter(row=>row!==""))lineage.push(`${lineage[lineage.length-1]}:${clean(name)}`);
 return Object.freeze(lineage);
}

/** 🧮️ The jobs an invocation asks for: its own `-j`/`--jobs`, else `CARGO_BUILD_JOBS`, else the cores; never below one. */
export function cargoGrantJobsV1(args:readonly string[],environment:Readonly<Record<string,string|undefined>>,cores:number):number{
 const own=head(args);let value:string|undefined;
 for(let index=0;index<own.length;index++){
  const argument=own[index]!;
  if(policy.jobsFlags.includes(argument)){value=own[index+1];index++;}
  else if(argument.startsWith("--jobs="))value=argument.slice(7);
  else if(/^-j\d+$/u.test(argument))value=argument.slice(2);
 }
 const parsed=Number(value??environment[policy.jobsVariable]);
 return Number.isSafeInteger(parsed)&&parsed>0?parsed:Math.max(1,cores);
}

/** 🔒️ Caps the jobs a command line names itself at the granted jobs, so an explicit flag can never exceed the grant; a line without such a flag is limited through the environment alone. */
export function capCargoGrantJobsV1(args:readonly string[],jobs:number):string[]{
 const result=[...args],end=head(args).length,cap=(text:string|undefined):string=>String(Math.min(jobs,Number.isSafeInteger(Number(text))&&Number(text)>0?Number(text):jobs));
 for(let index=0;index<end;index++){
  const argument=result[index]!;
  if(policy.jobsFlags.includes(argument)&&index+1<end){result[index+1]=cap(result[index+1]);index++;}
  else if(argument.startsWith("--jobs="))result[index]=`--jobs=${cap(argument.slice(7))}`;
  else if(/^-j\d+$/u.test(argument))result[index]=`-j${cap(argument.slice(2))}`;
 }
 return result;
}

/** 🌐️ The Rust target triple of a host: the platform and the architecture of the process. */
export function cargoHostTripleV1(platform:NodeJS.Platform=process.platform,arch:string=process.arch):string{
 const cpu=arch==="arm64"?"aarch64":"x86_64";
 return platform==="darwin"?`${cpu}-apple-darwin`:platform==="win32"?`${cpu}-pc-windows-msvc`:`${cpu}-unknown-linux-gnu`;
}

/** 📌️ The toolchain a `rust-toolchain.toml` pins, named for rustup (`<channel>-<triple>`; a channel that already names a triple, or a version channel without one, is used as written plus the triple only when it is a dated or named channel); `null` when the file pins none. */
export function cargoPinnedToolchainV1(file:string,triple:string):string|null{
 const match=/^\s*channel\s*=\s*["']([^"']+)["']/mu.exec(file);
 if(match===null)return null;
 const channel=match[1]!;
 return /-(?:apple|pc|unknown)-/u.test(channel)?channel:/^(?:nightly|beta|stable)(?:-\d{4}-\d{2}-\d{2})?$/u.test(channel)?`${channel}-${triple}`:channel;
}

/** 🔎️ Reads the pinned toolchain from the nearest `rust-toolchain.toml` at or above a directory. */
export function readCargoPinnedToolchainV1(directory:string,triple:string=cargoHostTripleV1()):string|null{
 for(let current=directory;;current=dirname(current)){
  const file=join(current,"rust-toolchain.toml");
  if(existsSync(file))return cargoPinnedToolchainV1(readFileSync(file,"utf8"),triple);
  if(dirname(current)===current)return null;
 }
}

/** 🌱️ The environment a Cargo grant hands over: the granted jobs, and the pinned toolchain unless the caller already chose one, so a toolchain binary called directly resolves its sibling tools like the rustup proxy would. */
export function cargoGrantEnvironmentV1(jobs:number,environment:Readonly<Record<string,string|undefined>>,directory:string,triple:string=cargoHostTripleV1()):Readonly<Record<string,string>>{
 const pinned=environment.RUSTUP_TOOLCHAIN===undefined?readCargoPinnedToolchainV1(directory,triple):null;
 return Object.freeze({[policy.jobsVariable]:String(jobs),...pinned===null?{}:{RUSTUP_TOOLCHAIN:pinned}});
}
