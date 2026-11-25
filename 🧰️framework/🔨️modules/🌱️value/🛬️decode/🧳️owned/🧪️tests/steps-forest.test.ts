import {test,expect} from "bun:test";
import law from "./🧫️fixtures/📐️.json";
test("every forest row case costs far fewer steps and bytes than before the leaf pipeline",()=>{const b:any=law.forestRows;for(const name of ["coedge","loopEdge","vertex","coedgeAbsentDefaults","loopOfFourEdges","faceOfThreeInnerLoops"]){expect(b[name].steps).toBeLessThan(b.before[name].steps);expect(b[name].copyBytes).toBeLessThan(b.before[name].copyBytes)}expect(b.coedge.steps).toBeLessThan(b.before.coedge.steps/5);expect(b.coedgeAbsentDefaults.copyBytes).toBeLessThan(b.before.coedgeAbsentDefaults.copyBytes/20)});
