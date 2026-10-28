/** 🔀️ Tests for directory exchange. */
import{test,expect}from"bun:test";
import{nativeDirectoryProtocolV1}from"../../📋️contract/🟦️.ts";
import libc from"../../../../../../../⚙️runtime/🧲️native/📚️libc/🧬️schema/🔣️.json"with{type:"json"};
import{bindNativeDirectoryExchangeV1,planNativeDirectoryExchangeEntriesV1}from"../🟦️.ts";
const protocol=nativeDirectoryProtocolV1();
const system=(symbols:readonly string[],log:unknown[][],status=0,errno=0)=>({FFIType:{i32:"i32",i64:"i64",u32:"u32",ptr:"ptr"},read:{i32:()=>errno},dlopen:(library:string,wanted:Record<string,unknown>)=>{
 for(const name of Object.keys(wanted))if(!symbols.includes(name))throw Error("Symbol "+name+" not found in "+library);
 return{close:()=>{},symbols:Object.fromEntries(Object.keys(wanted).map(name=>[name,(...args:unknown[])=>{log.push([name,...args]);return name===protocol.exchange.linux.error||name===protocol.exchange.darwin.error?1:status;}]))};
}})as never;
test("the plan names the wrapper first and the raw system call of the architecture second",()=>{
 expect(planNativeDirectoryExchangeEntriesV1("linux","x64")).toEqual([{symbol:"renameat2",syscall:null},{symbol:"syscall",syscall:316}]);
 expect(planNativeDirectoryExchangeEntriesV1("linux","arm64")).toEqual([{symbol:"renameat2",syscall:null},{symbol:"syscall",syscall:276}]);
 expect(planNativeDirectoryExchangeEntriesV1("linux","ia32")).toEqual([{symbol:"renameat2",syscall:null}]);
 expect(planNativeDirectoryExchangeEntriesV1("darwin","arm64")).toEqual([{symbol:"renameatx_np",syscall:null}]);
 expect(()=>planNativeDirectoryExchangeEntriesV1("win32","x64")).toThrow("Linux or macOS");
 expect(Object.keys(protocol.exchange.linux.syscalls).sort()).toEqual(Object.keys(libc.definitions.Protocol.const.linux).sort());
});
test("a library with the wrapper uses it, a library without it falls back to the system call, a library with neither is refused by name",()=>{
 for(const[layout,symbols,expected]of[["glibc 2.28+",["renameat2","syscall","__errno_location"],"renameat2"],["musl with the wrapper",["renameat2","syscall","__errno_location"],"renameat2"],["glibc 2.17 (RHEL 7)",["syscall","__errno_location"],"syscall"]]as const){
  const log:unknown[][]=[],call=bindNativeDirectoryExchangeV1(system(symbols,log),"libc.so.6",planNativeDirectoryExchangeEntriesV1("linux","x64"),"linux","x64");
  expect(call.exchange("/a","/b"),layout).toEqual({status:0,errno:0});
  expect(log[0]![0],layout).toBe(expected);
  const flags=expected==="syscall"?[316,-100,expect.any(Buffer),-100,expect.any(Buffer),2]:[-100,expect.any(Buffer),-100,expect.any(Buffer),2];expect(log[0]!.slice(1),layout).toEqual(flags);
 }
 expect(()=>bindNativeDirectoryExchangeV1(system(["__errno_location"],[]),"libc.musl-x86_64.so.1",planNativeDirectoryExchangeEntriesV1("linux","x64"),"linux","x64")).toThrow("renameat2 and syscall");
 expect(()=>bindNativeDirectoryExchangeV1(system(["syscall","__errno_location"],[]),"libc.so.6",planNativeDirectoryExchangeEntriesV1("linux","ia32"),"linux","ia32")).toThrow("no system call is declared for ia32");
 const failing=system(["renameat2","__errno_location"],[],-1,95),call=bindNativeDirectoryExchangeV1(failing,"libc.so.6",planNativeDirectoryExchangeEntriesV1("linux","x64"),"linux","x64");
 expect(call.exchange("/a","/b")).toEqual({status:-1,errno:95});
});
