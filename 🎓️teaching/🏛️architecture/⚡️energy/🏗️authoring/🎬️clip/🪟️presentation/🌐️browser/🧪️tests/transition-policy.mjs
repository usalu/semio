/** 🎛️ Freeze incidental transitions without disabling authored keyframe animation. */
import assert from "node:assert/strict";
import {readFile} from "node:fs/promises";
import {presentationTransitionStyle} from "../🧰️tools.mjs";
const sample=JSON.parse(await readFile(new URL("./transition-policy.cases.json",import.meta.url),"utf8"));
assert.equal(presentationTransitionStyle,sample.css,sample.id);
assert.doesNotMatch(presentationTransitionStyle,/animation/);
console.log("PASS "+sample.id);
