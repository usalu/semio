import {statSync} from "node:fs";
import {isAbsolute,normalize,resolve,sep} from "node:path";

/** 🧭️ One `--remap-path-prefix=FROM=TO` of a compiler command line. */
export type RemapV1=Readonly<{from:string;to:string}>;

/** 🧭️ Every path remapping of a command line, in order (`--remap-path-prefix=F=T` and `--remap-path-prefix F=T`; the compiler splits at the last `=`). */
export function remapsOfV1(args:readonly string[]):readonly RemapV1[]{
 const rows:RemapV1[]=[];
 for(let index=0;index<args.length;index++){
  const argument=args[index]!,spec=argument==="--remap-path-prefix"?args[++index]??"":argument.startsWith("--remap-path-prefix=")?argument.slice("--remap-path-prefix=".length):undefined;
  if(spec===undefined)continue;
  const cut=spec.lastIndexOf("=");if(cut>0)rows.push({from:spec.slice(0,cut),to:spec.slice(cut+1)});
 }
 return rows;
}

const trimmed=(path:string):string=>path.length>1?path.replace(/\/+$/u,""):path;

/** 🧭️ The name the compiler records for `path`: the LAST remapping whose `from` is a whole-component prefix wins and is applied once (`to` joined with the rest, `to` alone when nothing is left); an uncovered path keeps its spelling. This is `FilePathMapping::map_prefix` of rustc; the test compares it with the compiler itself.
 * @see https://doc.rust-lang.org/rustc/command-line-arguments.html#--remap-path-prefix-remap-source-names-in-output */
export function remappedNameV1(remaps:readonly RemapV1[],path:string):string{
 for(let index=remaps.length-1;index>=0;index--){
  const from=trimmed(remaps[index]!.from),to=remaps[index]!.to;
  const rest=path===from?"":path.startsWith(from==="/"?"/":from+"/")?path.slice(from==="/"?1:from.length+1).replace(/^\/+/u,""):undefined;
  if(rest===undefined)continue;
  return rest===""?to:to===""?rest:trimmed(to)+"/"+rest;
 }
 return path;
}

/** 🧭️ Whether some remapping covers `path` at all. */
export const remappedV1=(remaps:readonly RemapV1[],path:string):boolean=>remappedNameV1(remaps,path)!==path||remaps.some(row=>row.from===path&&row.to===path);

const BUILD_DIRECTORY_NAME="/cargo/build-dir";
const OUT_DIRECTORY_NAME="/cargo/out-dir";
const MACHINE_ROOTS:readonly (readonly [RegExp,string])[]=[[/[\\/]registry[\\/]src$/u,"⟨cargo-registry-src⟩"],[/[\\/]git[\\/]checkouts$/u,"⟨cargo-git-checkouts⟩"],[/[\\/]lib[\\/]rustlib[\\/]src[\\/]rust$/u,"⟨rust-src⟩"]];

/** 🧭️ The same argument with the machine-specific `FROM` of a remapping replaced by the role it plays (the cargo registry source cache, the git checkouts, the compiler's source copy, and Cargo's build directory, which Cargo remaps to `/cargo/build-dir` from wherever the lane or the target directory lives). What such a remapping does to the recorded names is fixed by its `TO`; where the cache lives on the machine does not matter, so two machines with different homes give the same key. Other arguments and remappings of unknown roots are returned unchanged. */
export function machineNeutralRemapV1(argument:string):string{
 if(!argument.startsWith("--remap-path-prefix="))return argument;
 const spec=argument.slice("--remap-path-prefix=".length),cut=spec.lastIndexOf("=");if(cut<=0)return argument;
 const from=spec.slice(0,cut).replace(/[\\/]+$/u,"");
 if(spec.slice(cut+1)===BUILD_DIRECTORY_NAME)return"--remap-path-prefix=⟨cargo-build-dir⟩="+BUILD_DIRECTORY_NAME;
 if(spec.slice(cut+1)===OUT_DIRECTORY_NAME)return"--remap-path-prefix=⟨cargo-out-dir⟩="+OUT_DIRECTORY_NAME;
 for(const [pattern,token] of MACHINE_ROOTS)if(pattern.test(from))return"--remap-path-prefix="+token+spec.slice(cut);
 return argument;
}

const argumentValues=(args:readonly string[],flag:string):string[]=>args.flatMap((argument,index)=>argument===flag?[args[index+1]??""]:argument.startsWith(flag+"=")?[argument.slice(flag.length+1)]:[]);
const codegenValue=(args:readonly string[],name:string):string|undefined=>args.flatMap((argument,index)=>argument==="-C"?[args[index+1]??""]:argument.startsWith("-C")&&argument.length>2?[argument.slice(2)]:[]).find(value=>value.startsWith(name+"="))?.slice(name.length+1);

/** 🍎️ Whether the unit links for macOS: Apple's linker options belong to the Mach-O link only, so a unit cross-compiled to another target (a wasm component linked by `wasm-component-ld` on a macOS host) gets none of them. Cargo gives no `--target` to the host's own units (proc-macros, build scripts). */
export const appleLinkV1=(args:readonly string[]):boolean=>{const target=argumentValues(args,"--target")[0];return target===undefined||/-apple-/u.test(target);};

/** 🧷️ The linker flag that gives a proc-macro, cdylib or dylib library the same install name wherever it is linked: the output path (a lane directory, a worker's work directory) would otherwise be inside the linked bytes. The library is loaded by path (by the compiler, a plugin host or `dlopen`), so the name is never used to find it. `[]` off macOS and for every other crate type. */
export function installNameArgumentsV1(args:readonly string[],platform:NodeJS.Platform=process.platform):readonly string[]{
 if(platform!=="darwin"||!appleLinkV1(args)||args.some(argument=>argument.includes("install_name"))||!argumentValues(args,"--crate-type").some(type=>type==="proc-macro"||type==="cdylib"||type==="dylib"))return[];
 const name=argumentValues(args,"--crate-name")[0];
 return name===undefined?[]:["-Clink-arg=-Wl,-install_name,@rpath/lib"+name+(codegenValue(args,"extra-filename")??"")+".dylib"];
}

/** 🧷️ The flag that makes a Mach-O output independent of the directory it was linked in: the debug map of the linked file names every object file by its absolute path, so two links of the same unit in different directories differ in those bytes. Rooting the object names at the output directory (which holds them) leaves `name.o` in both a local and a worker link. Given for the units that link for macOS (proc-macro, bin, dylib, cdylib), never for another target and never when the command line already says so. */
export function objectRootArgumentsV1(args:readonly string[],outDir:string,platform:NodeJS.Platform=process.platform):readonly string[]{
 if(platform!=="darwin"||!appleLinkV1(args)||args.some(argument=>argument.includes("oso_prefix")))return[];
 return argumentValues(args,"--crate-type").some(type=>["bin","proc-macro","dylib","cdylib"].includes(type))?["-Clink-arg=-Wl,-oso_prefix,"+outDir.replace(/\/+$/u,"")+"/"]:[];
}

/** 🦀️ The remap that keeps the standard library's source out of a unit: with the `rust-src` component installed rustc names the code of `core` that it inlines by its real path below the sysroot (`<sysroot>/lib/rustlib/src/rust/library/...`) in the debug information of the unit, without it by the virtual `/rustc/<commit>/library/...`, so the same unit gave other bytes (and a path of the machine in them) on the machines that have the component, and the leak check cannot see it where the sysroot is the default rustup home. Cargo's `trim-paths` gives this remap only for some profiles; this adds it when the command line has none for that directory. It is placed by the caller before Cargo's remaps (the last matching one wins) and names a path that does not exist where the component is missing, where it changes nothing. `[]` when the sysroot or the commit is unknown. */
export function rustSourceRemapArgumentsV1(args:readonly string[],sysroot:string|undefined,commit:string|undefined):readonly string[]{
 if(sysroot===undefined||sysroot===""||commit===undefined)return[];
 const trimmed=sysroot.replace(/[\\/]+$/u,""),from=trimmed+(trimmed.includes("\\")?"\\":"/")+["lib","rustlib","src","rust"].join(trimmed.includes("\\")?"\\":"/");
 return remapsOfV1(args).some(row=>row.from.replace(/[\\/]+$/u,"")===from)?[]:[`--remap-path-prefix=${from}=/rustc/${commit}`];
}

/** 🧷️ Everything the wrapper adds to a local link so that its output holds no path of this machine (`objectRootArgumentsV1`, `installNameArgumentsV1`). */
export const linkRootArgumentsV1=(args:readonly string[],outDir:string,platform:NodeJS.Platform=process.platform):readonly string[]=>[...objectRootArgumentsV1(args,outDir,platform),...installNameArgumentsV1(args,platform)];

/** 🧷️ Spells the crate root of a compiler command line in its normal form (`a/b/../c.rs` becomes `a/c.rs`), when that names the same file: the compiler records the spelling in every output (names, spans, the crate hash), so one file spelled two ways by two routes (Cargo's `[lib] path = "../../lib.rs"` against an offloaded or module-remapped compile) gives two different outputs under one key. A source through a link, an absolute one or one that cannot be resolved stays as given. */
export function canonicalSourceArgumentsV1(args:readonly string[],source:string,cwd:string):readonly string[]{
 if(source===""||isAbsolute(source))return args;
 const normal=normalize(source),at=args.lastIndexOf(source);
 if(normal===source||at<0)return args;
 try{const spelled=statSync(cwd+sep+source,{bigint:true}),named=statSync(resolve(cwd,normal),{bigint:true});if(spelled.dev!==named.dev||spelled.ino!==named.ino)return args;}catch{return args;}
 return Object.freeze(args.map((argument,index)=>index===at?normal:argument));
}

/** 🧷️ Names the generated-source directory of a build script (`OUT_DIR`, `<build-dir>/<package>/<hash>/out`) the same way in every compile: Cargo derives `<hash>` from the unit identity of its own workspace, so two machines compile one slot with two spellings, the compiler records the spelling of every `include!`d file in the outputs (names, spans, the crate hash) and every dependent of the crate inherits the difference, although the key (which sees the dependency by its interface) is equal. The remapping goes last, where the compiler gives it precedence over the build-directory remapping of Cargo; a unit without `OUT_DIR`, a relative one, or one that is already remapped to this name stays as given.
 * @see https://doc.rust-lang.org/cargo/reference/build-scripts.html#outputs-of-the-build-script */
export function canonicalOutDirectoryArgumentsV1(args:readonly string[],outDir:string|undefined):readonly string[]{
 if(outDir===undefined||outDir===""||!isAbsolute(outDir))return args;
 if(remapsOfV1(args).some(row=>row.from===outDir&&row.to===OUT_DIRECTORY_NAME))return args;
 return Object.freeze([...args,`--remap-path-prefix=${outDir.replace(/[\\/]+$/u,"")}=${OUT_DIRECTORY_NAME}`]);
}
