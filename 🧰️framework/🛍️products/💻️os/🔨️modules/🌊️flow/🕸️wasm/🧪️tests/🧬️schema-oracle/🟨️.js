import { readFile } from "node:fs/promises";
import { readFileSync } from "node:fs";
import Ajv from "ajv";
import { FlowOperation, FlowOperationFields } from "../../📦️packages/🟨️javascript/🖥️flow-host.js";

//#region 🔮️OwnedOraclePort

class FlowSchemaOracle {
  summarize(_abi) { throw new Error("Flow schema oracle is abstract"); }
}

class AjvFlowSchemaOracle extends FlowSchemaOracle {
  summarize(abi) {
    const document = JSON.parse(readFileSync(new URL("../../🧬️schema/🔣️.json", import.meta.url), "utf8"));
    const ajv = new Ajv({ strict: true, allErrors: true });
    ajv.addSchema(document);
    const validate = ajv.getSchema(`${document.$id}#/$defs/FlowEditorBrowserAbiV1`);
    if (!validate(abi)) throw new Error(`Flow schema oracle rejected: ${JSON.stringify(validate.errors)}`);
    return summary(abi);
  }
}

function summary(abi) {
  return {
    events: Object.keys(abi.events).length,
    framing: abi.framing.message,
    operations: Object.keys(abi.operations).length,
    schema: "valid",
    surfaceProtocol: abi.surfaceProtocol,
  };
}

//#endregion 🔮️OwnedOraclePort

const expected = JSON.parse(await readFile(new URL("../../🧪️fixtures/🔣️.json", import.meta.url), "utf8"));
const abi = JSON.parse(await readFile(new URL("../../🧪️fixtures/📡️abi.json", import.meta.url), "utf8"));
const owned = summary(abi);
const thirdParty = new AjvFlowSchemaOracle().summarize(abi);
if (JSON.stringify(owned) !== JSON.stringify(expected)) throw new Error("Flow owned schema summary drift");
if (JSON.stringify(thirdParty) !== JSON.stringify(expected)) throw new Error("Flow third-party oracle parity drift");
for (let code = 2_500; code <= 2_610; code += 1) {
  const supported = ![2_603, 2_604, 2_608].includes(code);
  if (Object.values(abi.operations).includes(code) !== supported) throw new Error(`Flow operation ${code} admission drift`);
}
if (JSON.stringify(FlowOperation) !== JSON.stringify(abi.operations)) throw new Error("Flow JavaScript operation ledger drift");
const fieldTypes = { "utf8":"s", "optional-utf8":"o", "f64":"d", "u64":"q", "u32":"u", "u8":"c", "bool":"b", "bytes":"x" };
for (const name of Object.keys(abi.operations)) {
  const descriptor = abi.arguments[name].map((field) => `${field.name}:${fieldTypes[field.type]}`).join(",");
  if (descriptor !== (FlowOperationFields[name] ?? "")) throw new Error(`Flow argument descriptor ${name} drift`);
}
console.log(JSON.stringify(expected));
