import {test,expect} from "bun:test";
import {Database} from "bun:sqlite";
import Ajv from "ajv/dist/2020.js";
import {createHash} from "node:crypto";
import law from "./transfer.json" with {type:'json'};
import schema from "./transfer.schema.json" with {type:'json'};
test("original transport custody moves once or retains its source",()=>{
 expect(new Ajv({strict:true}).validate(schema,law)).toBe(true);
 const bytes=Buffer.from(Array.from({length:law.bytes},(_,i)=>law.pattern[i%law.pattern.length]));
 for(const commit of [false,true]){const db=new Database(":memory:");try{db.exec("CREATE TABLE custody(owner TEXT PRIMARY KEY,payload BLOB)");db.query("INSERT INTO custody VALUES('source',?)").run(bytes);db.exec("BEGIN");db.exec("UPDATE custody SET owner='recipient' WHERE owner='source'");db.exec(commit?"COMMIT":"ROLLBACK");const row=db.query("SELECT owner,payload FROM custody").get() as {owner:string,payload:Uint8Array};expect(row.owner).toBe(commit?"recipient":"source");expect(createHash("sha256").update(row.payload).digest("hex")).toBe(createHash("sha256").update(bytes).digest("hex"));expect(db.query("SELECT COUNT(*) AS count FROM custody").get()).toEqual({count:1});}finally{db.close()}}
});

test("ordinary actor wire keeps the canonical header and paged payload", () => {
  const payload = Buffer.from(Array.from({ length: law.bytes }, (_, index) => law.pattern[index % law.pattern.length]));
  const length = [payload.length % 128 + 128, Math.floor(payload.length / 128)];
  const prefix = Buffer.from([1, 1, 0, 1, 0, 8, ...length]);
  expect(prefix.toString("hex")).toBe(law.actorWirePrefix);
  const db = new Database(":memory:");
  try {
    const source = Buffer.concat([prefix, payload]);
    const row = db.query("SELECT hex(substr(?1, 1, 8)) AS prefix, substr(?1, 9) AS payload").get(source) as { prefix: string, payload: Uint8Array };
    expect(row.prefix.toLowerCase()).toBe(law.actorWirePrefix);
    expect(Buffer.from(row.payload)).toEqual(payload);
  } finally {
    db.close();
  }
});
