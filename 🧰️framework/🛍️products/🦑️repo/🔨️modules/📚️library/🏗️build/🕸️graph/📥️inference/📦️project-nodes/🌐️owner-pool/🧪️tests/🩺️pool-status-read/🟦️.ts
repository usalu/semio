/** 🩺️ Law: a read can say what this machine's own pool does, in the pool's own words. The status file the pool writes is read back as data (state, children, rate, reason in English and German) and an absent, unreadable or foreign file is no pool, never an error.
 * @see ../../📊️status/🟦️.ts */
import{afterAll,expect,test}from"bun:test";
import{mkdirSync,mkdtempSync,rmSync,writeFileSync}from"node:fs";
import{tmpdir}from"node:os";
import{join}from"node:path";
import{createOwnerPoolStatusV1,readOwnerPoolStatusV1}from"../../📊️status/🟦️.ts";

const roots:string[]=[];
const store=()=>{const root=mkdtempSync(join(tmpdir(),"pool-status-"));roots.push(root);return root;};
afterAll(()=>{for(const root of roots)rmSync(root,{recursive:true,force:true});});

test("what the pool wrote is read back with its reason in both languages",()=>{
 const root=store();
 const status=createOwnerPoolStatusV1({store:root,origin:"g1-m01",role:"puller"});
 status.update({state:"pulling",children:3,target:0,running:2,ownersPerMinute:1.2,reason:"pressure",reasonText:{en:"waiting: the machine swaps",de:"wartet: die Maschine lagert aus"}} as never);
 status.flush();
 const read=readOwnerPoolStatusV1({store:root,origin:"g1-m01"});
 expect(read).toMatchObject({state:"pulling",children:3,running:2,ownersPerMinute:1.2,reason:"pressure",text:{en:"waiting: the machine swaps",de:"wartet: die Maschine lagert aus"}});
});

test("a missing store, a missing file, a broken file or a status of another shape is no pool",()=>{
 const root=store();
 expect(readOwnerPoolStatusV1({store:undefined,origin:"g1-m01"})).toBeUndefined();
 expect(readOwnerPoolStatusV1({store:root,origin:"g1-m01"})).toBeUndefined();
 mkdirSync(join(root,"v1","origins","g1-m01"),{recursive:true});
 writeFileSync(join(root,"v1","origins","g1-m01","pool.json"),"{broken");
 expect(readOwnerPoolStatusV1({store:root,origin:"g1-m01"})).toBeUndefined();
 writeFileSync(join(root,"v1","origins","g1-m01","pool.json"),JSON.stringify({version:2}));
 expect(readOwnerPoolStatusV1({store:root,origin:"g1-m01"})).toBeUndefined();
});

test("a status without a reason is a pool that runs at full width",()=>{
 const root=store();
 const status=createOwnerPoolStatusV1({store:root,origin:"g1-m02",role:"puller"});
 status.update({state:"pulling",children:10,running:8} as never);status.flush();
 expect(readOwnerPoolStatusV1({store:root,origin:"g1-m02"})).toMatchObject({state:"pulling",children:10,running:8,reason:null,text:null});
});
