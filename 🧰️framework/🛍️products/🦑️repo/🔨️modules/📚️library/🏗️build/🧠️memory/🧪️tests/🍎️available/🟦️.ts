/** 🍎️ Contract: the headroom of the graph service is its limit less its use and its margin; where no cgroup limit applies it starts from the memory the host can still give out as the one host reader answers it (`createHostAvailableBytesV1`, tested in the host capacity member), and a host that cannot tell leaves the whole total to the margin instead of failing the governor. */
import{expect,test}from"bun:test";
import{serviceMemoryHeadroomBytesV1}from"../../🟨️.mjs";

test("the headroom of a host without a cgroup is its total less what is not reclaimable, less the margin",()=>{
 const MiB=1048576,headroom=serviceMemoryHeadroomBytesV1({cgroup:()=>undefined,free:()=>3072*MiB,total:()=>16384*MiB,environment:{}});
 expect(headroom).toBeGreaterThan(0);
 const small=serviceMemoryHeadroomBytesV1({cgroup:()=>undefined,free:()=>64*MiB,total:()=>16384*MiB,environment:{}});
 expect(small).toBeLessThan(0);
 expect(headroom-small).toBe(3008*MiB);
 const unknown=serviceMemoryHeadroomBytesV1({cgroup:()=>undefined,free:()=>undefined,total:()=>16384*MiB,environment:{}});
 expect(unknown).toBeGreaterThan(headroom);
});
