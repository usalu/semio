import {test,expect} from "bun:test";
import {Database} from "bun:sqlite";
import law from "./🔣️.json";
test("original emitted metadata remains one owner while its caller loan follows custody",()=>{
 const db=new Database(":memory:");db.exec("CREATE TABLE source(owner TEXT, value TEXT); CREATE TABLE caller(phase TEXT, loans INTEGER)");
 const original="v".repeat(law.wideTextBytes);db.query("INSERT INTO source VALUES ('producer',?)").run(original);
 for(const owner of ["capture","mounted"]){db.transaction(()=>{db.query("UPDATE source SET owner=?").run(owner);db.query("INSERT INTO caller VALUES (?,1)").run(owner)})();const row=db.query("SELECT count(*) AS owners, length(value) AS bytes, value FROM source").get() as {owners:number,bytes:number,value:string};expect(row.owners).toBe(1);expect(row.bytes).toBe(law.wideTextBytes);expect(row.value).toBe(original);}
 expect(db.query("SELECT min(loans) AS loans FROM caller").get()).toEqual({loans:1});db.exec("DELETE FROM source; DELETE FROM caller");expect(db.query("SELECT count(*) AS owners FROM source").get()).toEqual({owners:0});db.close();
});

test("the native admission receipt preserves canonical u64 identity without a mutation mirror",()=>{const db=new Database(":memory:");db.exec("CREATE TABLE receipt(operation_id TEXT,generation TEXT,invocation_id TEXT,mutations INTEGER,inverse_mutations INTEGER)");const r=law.receipt;db.query("INSERT INTO receipt VALUES (?,?,?,?,?)").run(r.operationId,r.generation,r.invocationId,r.mutations,r.inverseMutations);const row=db.query("SELECT operation_id,generation,invocation_id,mutations,inverse_mutations FROM receipt").get() as Record<string,string|number>;const maximum=((1n<<64n)-1n).toString();expect(row).toEqual({operation_id:maximum,generation:maximum,invocation_id:`operation:${maximum}:${maximum}`,mutations:0,inverse_mutations:0});db.close();});
