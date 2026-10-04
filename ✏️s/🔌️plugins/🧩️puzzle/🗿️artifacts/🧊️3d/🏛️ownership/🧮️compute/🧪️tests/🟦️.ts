/** 🧩️ Executes this owner's complete canonical compute witness. */
import {readFileSync} from "node:fs";
import {resolve,join} from "node:path";
import {registerComputeConsumerTests} from "../../../../../../../../🧰️framework/🔨️modules/◻️2d/🧮️compute/🧪️testing/📍️consumer/🏃️host/🟦️.ts";
import {type ComputeConsumerDescriptor} from "../../../../../../../../🧰️framework/🔨️modules/◻️2d/🧮️compute/🧪️testing/📍️consumer/🟦️.ts";
import descriptor from "../🧫️fixtures/🔣️.json";
import schema from "../🧬️schema/🔣️.json";
const root=resolve(import.meta.dir,"../../../../../../../..");
registerComputeConsumerTests(descriptor as ComputeConsumerDescriptor,schema,path=>readFileSync(join(root,path),"utf8"));
