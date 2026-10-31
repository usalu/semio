/** 🧬 Independent AJV interface for portable roof cladding schema fixtures. */
const fs = require("node:fs");
const path = require("node:path");
const assert = require("node:assert/strict");
const Ajv = require("ajv/dist/2020").default;
const root = path.resolve(__dirname, "../..");
const read = file => JSON.parse(fs.readFileSync(path.join(root,file),"utf8"));
const model = read("🔣️.json");
const cases = read("🧪️cases/🏭️roof/🔣️.json");
const validate = new Ajv({strict:true,allErrors:true}).compile(read("🧬️schema/🔣️.schema.json"));
assert.equal(validate(model),true,JSON.stringify(validate.errors));
for (const test of cases.invalid) {
  const candidate = structuredClone(model);
  const parent = test.path.slice(0,-1).reduce((value,key)=>value[key],candidate);
  if(test.remove) delete parent[test.path.at(-1)]; else parent[test.path.at(-1)] = test.value;
  assert.equal(validate(candidate),false,test.id);
}
assert.equal(model.capital.cladding.panel_pitch.value,cases.panel_pitch);
assert.equal(model.capital.cladding.seam_projection.value,cases.seam_projection);
console.log("Roof cladding schema accepted; "+cases.invalid.length+" invalid portable models rejected.");
