const fs = require("node:fs");
const meter = { lstat: 0, readdir: 0 };
const lstatSync = fs.lstatSync, readdirSync = fs.readdirSync;
fs.lstatSync = (...args: unknown[]) => { meter.lstat++; return lstatSync(...args); };
fs.readdirSync = (...args: unknown[]) => { meter.readdir++; return readdirSync(...args); };
/** 📏️ Counts every `lstatSync` and `readdirSync` of the child so a test can bound physical metadata work independent of timing. */
(globalThis as { semioFsMeter?: typeof meter }).semioFsMeter = meter;
