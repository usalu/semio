import {Database} from "bun:sqlite";
import {expect,test} from "bun:test";
import Ajv from "ajv";
import Ajv2020 from "ajv/dist/2020";
import {fileURLToPath} from "node:url";
import recipeSchema from "../../../✨️derive/🧬️schema/🔣️.json";

test("fourteen original membership recipes preserve SQL event ordering and exact inverse ranks", async () => {
  const root=fileURLToPath(new URL("../../../",import.meta.url));
  const validateRecipe=new Ajv2020({strict:true}).compile(recipeSchema);
  let count=0;
  const recipes=new Set<string>();
  for await(const path of new Bun.Glob("*/*/🧪️native/🧫️fixtures/*.json").scan({cwd:root,absolute:true})){
    const law=await Bun.file(path).json();
    if(law.contract!=="semio.energy.original-membership/v1")continue;
    const schema=await Bun.file(new URL("../../🧬️schema/🔣️.json",`file://${path}`)).json();
    const r=schema["x-semio-energy-operation"];
    recipes.add(schema.title);
    expect(validateRecipe(r)).toBe(true);
    const valid=new Ajv({strict:false}).compile(schema);
    const camel=(s:string)=>s.replace(/_([a-z])/g,(_:string,c:string)=>c.toUpperCase());
    const db=new Database(":memory:");
    db.run("CREATE TABLE members(rank INTEGER PRIMARY KEY,id INTEGER)");
    db.run("CREATE TABLE targets(id INTEGER PRIMARY KEY)");
    const foreign=({Zones:"zones",Spaces:"spaces",Materials:"materials",PvSystems:"pv_systems",Batteries:"battery_storage"} as Record<string,string>)[r.memberReference];
    if(foreign)for(const row of law.model[foreign])db.run("INSERT INTO targets VALUES(?)",row.id);
    for(const example of law.cases){
      const p=example.payload;
      expect(valid({...p,mutation:schema.properties.mutation.const})).toBe(example.outcome!=="invalid");
      db.run("DELETE FROM members");
      law.original[law.field].forEach((id:number,rank:number)=>db.run("INSERT INTO members VALUES(?,?)",rank,id));
      const rows=db.query<{rank:number,id:number},[]>("SELECT rank,id FROM members ORDER BY rank").all();
      const item=r.member?p[camel(r.member)]:undefined;
      const existing=rows.find(row=>row.id===item);
      const insert=r.membership==="insert";
      const rank=r.indexMode==="optional-sorted"?(p.index??rows.find(row=>row.id>item)?.rank??rows.length):p.index;
      const missing=insert?rank>rows.length:r.membership==="remove-index"?rank>=rows.length:!existing;
      const duplicate=insert&&existing&&r.duplicate!=="allow";
      let outcome:string;
      if(p.id!==law.original.id)outcome="missing";
      else if(insert&&r.positiveMember&&item===0)outcome="invalid";
      else if(insert&&foreign&&!db.query("SELECT id FROM targets WHERE id=?").get(item))outcome="invalid-reference";
      else if(missing&&(!duplicate||r.rankBeforeDuplicate))outcome="missing";
      else if(duplicate)outcome=r.duplicate==="fatal"?"duplicate":"no-op";
      else if(missing)outcome="missing";
      else outcome="changed";
      expect(outcome).toBe(example.outcome);
      if(outcome==="changed"){
        const selected=insert?{index:rank,item}:r.membership==="remove-index"?{index:rank,item:rows[rank].id}:{index:existing!.rank,item};
        expect(selected).toEqual({index:example.inverseIndex,item:example.inverseItem});
        const after=rows.map(row=>row.id);
        if(insert)after.splice(selected.index,0,item);else after.splice(selected.index,1);
        expect(after).toEqual(example.after);
        if(insert)after.splice(selected.index,1);else after.splice(selected.index,0,selected.item);
        expect(after).toEqual(rows.map(row=>row.id));
      }
    }
    db.close();
    count++;
  }
  expect(count).toBe(16);
  expect(recipes.size).toBe(14);
});
