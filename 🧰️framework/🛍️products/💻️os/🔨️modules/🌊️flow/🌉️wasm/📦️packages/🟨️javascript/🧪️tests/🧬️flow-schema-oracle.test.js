import { readFile } from "node:fs/promises";
import Ajv from "ajv/dist/2020.js";
import { FlowOperation, FlowOperationFields } from "../🟨️flow-host.js";

//#region 🔮️OwnedOraclePort

class FlowSchemaOracle {
  summarize(_schema) { throw new Error("Flow schema oracle is abstract"); }
}

class AjvFlowSchemaOracle extends FlowSchemaOracle {
  summarize(schema) {
    const validate = new Ajv({ strict: false }).compile(schema);
    if (!validate(schema)) throw new Error(`Flow schema oracle rejected: ${JSON.stringify(validate.errors)}`);
    return summary(schema);
  }
}

function summary(schema) {
  return {
    events: Object.keys(schema.events).length,
    framing: schema.framing.message,
    operations: Object.keys(schema.operations).length,
    schema: "valid",
    surfaceProtocol: schema.surfaceProtocol,
  };
}

//#endregion 🔮️OwnedOraclePort

const schemaUrl = new URL("../../../🧬️schema/🔣️.json", import.meta.url);
const oracleUrl = new URL("../../../🧪️fixtures/🔮️oracle.json", import.meta.url);
const schema = JSON.parse(await readFile(schemaUrl, "utf8"));
const expected = JSON.parse(await readFile(oracleUrl, "utf8"));
const owned = summary(schema);
const thirdParty = new AjvFlowSchemaOracle().summarize(schema);
if (JSON.stringify(owned) !== JSON.stringify(expected)) throw new Error("Flow owned schema summary drift");
if (JSON.stringify(thirdParty) !== JSON.stringify(expected)) throw new Error("Flow third-party oracle parity drift");
for (let code = 2_500; code <= 2_610; code += 1) if (!Object.values(schema.operations).includes(code)) throw new Error(`Flow operation ${code} missing`);
if (JSON.stringify(FlowOperation) !== JSON.stringify(schema.operations)) throw new Error("Flow JavaScript operation ledger drift");
const fieldTypes = { "utf8":"s", "optional-utf8":"o", "f64":"d", "u64":"q", "u32":"u", "u8":"c", "bool":"b", "bytes":"x" };
for (const name of Object.keys(schema.operations)) {
  const descriptor = schema.arguments[name].map((field) => `${field.name}:${fieldTypes[field.type]}`).join(",");
  if (descriptor !== (FlowOperationFields[name] ?? "")) throw new Error(`Flow argument descriptor ${name} drift`);
}
console.log(JSON.stringify(expected));
