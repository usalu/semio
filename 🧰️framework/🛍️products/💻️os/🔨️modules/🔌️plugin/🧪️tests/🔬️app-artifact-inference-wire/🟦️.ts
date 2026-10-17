/** 🎟️ Original inference context is independently admitted before the existing cold inspection call. */
import{test,expect}from"bun:test";
import{Database}from"bun:sqlite";
import Ajv from"ajv";
import{readFileSync}from"node:fs";
import input from"./🧫️fixtures/🎟️caller.json";
import schema from"./🧬️schema/🔣️.json";
const source=readFileSync(new URL("./🦀️.rs",import.meta.url),"utf8");
test("original caller context admits finite policy independently of each incoming child wallet",()=>{const canonical=new URL("./🧬️schema/🔣️.json",import.meta.url).href,grantUrl=new URL(schema.properties.grant.$ref,canonical);const grant=JSON.parse(readFileSync(grantUrl,"utf8"));const ajv=new Ajv({strict:true});ajv.addSchema(grant,grantUrl.href);const admit=ajv.compile({...schema,$id:canonical});expect(admit(input)).toBe(true);expect(admit({...input,grant:{...input.grant,extraAuthority:true}})).toBe(false);expect(admit({...input,fuel:0})).toBe(false);const db=new Database(":memory:",{safeIntegers:true});try{const observed=db.query("SELECT ? AS operation,? AS generation,? - ? AS remaining").get(input.operation,input.generation,input.deadlineUs,input.clockUs)as{operation:bigint,generation:bigint,remaining:bigint};expect(observed).toEqual({operation:71n,generation:3n,remaining:999999n});expect(input.grant.maximumCopyBytes).not.toBe(input.grant.maximumCapacityBytes);}finally{db.close();}});
test("every actual cold inference fixture forwards its original context to the installed gateway",()=>{const callers=source.split("\n").filter(line=>/wire_artifact_infer_(?:from_)?cold\(/.test(line));expect(callers.length).toBe(5);for(const caller of callers)expect(caller).toContain(",71,3,&mut cx)");expect(source).toContain("fn original_context");expect(source).toContain("StepBudget::new(policy.fuel,policy.deadline_us,policy.grant)");expect(source).not.toContain("StepBudget::new(usize::MAX");expect(source.match(/\becho\(request,cx\)/g)?.length??0).toBe(3);});
