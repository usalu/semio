/** 🧪️ Immutable caller font sources survive child retirement. */
import {test,expect} from "bun:test";
import {loadBuiltinUnicodeSource} from "../../../🧵️shape/🌐️unicode/🟦️.ts";
import fixture from "./🧫️fixtures/🔣️.json";
import {checkBorrowedFaces} from "./🔮️oracle.ts";
export {checkBorrowedFaces};
test("outline retirement borrows immutable caller fonts",async()=>{const source=new Uint8Array(await Bun.file(new URL("../../../../../../🖼️assets/🔤️fonts/⌨️share-tech-mono/🏛️latin/📖️regular/🔤️outline.ttf",import.meta.url)).arrayBuffer());expect(checkBorrowedFaces(source,await loadBuiltinUnicodeSource())).toBe(12);});
