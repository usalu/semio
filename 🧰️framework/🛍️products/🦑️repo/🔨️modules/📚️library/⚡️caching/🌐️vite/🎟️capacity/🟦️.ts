import {availableParallelism} from "node:os";
import {fixedCompilerCapacityRequestV1,type CompilerCapacityRequestV1} from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🏗️build/🎟️capacity/🟦️.ts";

/** 🎟️ The compiler capacity a Vite command service (dev listener) holds for its whole life: the authored `vite-dev-service` fixed class, so two listeners beside each other never take the counted share of the host each (which booked every core for the length of a course e2e and parked every other compile).
 * @see ../../../../../../../🔨️modules/🏃️process/🏗️build/🎟️capacity/🧮️units/🧬️schema/🔣️.json */
export function viteCommandServiceCapacityRequestV1(cores:number=availableParallelism()):CompilerCapacityRequestV1{
 return fixedCompilerCapacityRequestV1("vite-dev-service",cores);
}
