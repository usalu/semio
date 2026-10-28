import {test} from "bun:test";
import {verifyNistProjection} from "./verify.ts";
test("official NIST AP203 circles match independent placement and SI-unit geometry",async()=>{verifyNistProjection(await Bun.file(new URL("../../../../../../../../../../📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/🏅️conformance/📐️step/nist-ftc11-ap203.stp",import.meta.url)).text());});
