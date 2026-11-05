import {test} from "bun:test";
import {checkDrawingAdmission} from "./🧪️tests.ts";
import {graphValid} from "./🔮️oracle.mjs";

test("Drawing source admission preserves explicit null paint and nonnull refusals",()=>{checkDrawingAdmission(graphValid);},45000);
