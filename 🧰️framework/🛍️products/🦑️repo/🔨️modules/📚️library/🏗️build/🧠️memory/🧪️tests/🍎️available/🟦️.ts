/** 🍎️ Contract: the memory a host can still give out is the reclaimable memory of its kernel, not its free pages alone: free plus inactive pages on macOS, `MemAvailable` on Linux, the operating system's free memory where neither source reads; an answer is reused within its age and a failing source never fails the governor. */
import{expect,test}from"bun:test";
import{availableMemoryBytesV1,createAvailableMemoryV1,darwinAvailableBytesV1,linuxAvailableBytesV1,serviceMemoryHeadroomBytesV1}from"../../🟨️.mjs";

const VM_STAT=`Mach Virtual Memory Statistics: (page size of 16384 bytes)
Pages free:                                9174.
Pages active:                            196691.
Pages inactive:                          194763.
Pages speculative:                          392.
Pages throttled:                              0.
Pages wired down:                        129882.
Pages purgeable:                           6819.
`;
const MEMINFO="MemTotal:       11584000 kB\nMemFree:          212000 kB\nMemAvailable:    7340032 kB\nBuffers:           10000 kB\n";

test("macOS counts free and inactive pages of the reported page size",()=>{
 expect(darwinAvailableBytesV1(VM_STAT)).toBe((9174+194763)*16384);
 expect(darwinAvailableBytesV1("Pages free: 1.\nPages inactive: 2.\n")).toBeUndefined();
 expect(darwinAvailableBytesV1("Mach Virtual Memory Statistics: (page size of 4096 bytes)\nPages free: 5.\n")).toBeUndefined();
});

test("Linux reads MemAvailable in kilobytes",()=>{
 expect(linuxAvailableBytesV1(MEMINFO)).toBe(7340032*1024);
 expect(linuxAvailableBytesV1("MemFree: 1 kB\n")).toBeUndefined();
});

test("the reader picks its source by platform, reuses an answer within its age and falls back to the operating system",()=>{
 let runs=0,time=0;const common={now:()=>time,free:()=>123,maximumAgeMilliseconds:1000};
 const mac=createAvailableMemoryV1({...common,platform:"darwin",run:()=>{runs++;return VM_STAT;}});
 expect(mac()).toBe((9174+194763)*16384);expect(mac()).toBe((9174+194763)*16384);expect(runs).toBe(1);
 time=1500;mac();expect(runs).toBe(2);
 expect(createAvailableMemoryV1({...common,platform:"linux",read:path=>{expect(path).toBe("/proc/meminfo");return MEMINFO;}})()).toBe(7340032*1024);
 expect(createAvailableMemoryV1({...common,platform:"darwin",run:()=>{throw Error("vm_stat missing");}})()).toBe(123);
 expect(createAvailableMemoryV1({...common,platform:"linux",read:()=>"nothing useful"})()).toBe(123);
 expect(createAvailableMemoryV1({...common,platform:"win32"})()).toBe(123);
});

test("the live reader answers a positive number on this host",()=>{
 expect(availableMemoryBytesV1()).toBeGreaterThan(0);
});

test("the headroom of a host without a cgroup is its total less what is not reclaimable, less the margin",()=>{
 const MiB=1048576,headroom=serviceMemoryHeadroomBytesV1({cgroup:()=>undefined,free:()=>3072*MiB,total:()=>16384*MiB,environment:{}});
 expect(headroom).toBeGreaterThan(0);
 const small=serviceMemoryHeadroomBytesV1({cgroup:()=>undefined,free:()=>64*MiB,total:()=>16384*MiB,environment:{}});
 expect(small).toBeLessThan(0);
 expect(headroom-small).toBe(3008*MiB);
});
