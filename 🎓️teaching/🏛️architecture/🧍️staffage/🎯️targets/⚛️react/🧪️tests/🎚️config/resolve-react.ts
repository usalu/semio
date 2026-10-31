/** ⚛️ Pins React to the package copy before a test file resolves a second one from a parent folder. */
import { Module } from "node:module";
import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";

const modules = resolve(dirname(fileURLToPath(import.meta.url)), "../../📦️packages/🟦️typescript/node_modules");
const pinned = new Set(["react", "react-dom", "react-dom/client", "react/jsx-runtime", "react/jsx-dev-runtime", "@testing-library/react"]);
const original = Module._resolveFilename;
Module._resolveFilename = function resolvePinned(request: string, parent: NodeModule, isMain: boolean, options: object) {
  if (pinned.has(request)) return original.call(this, request, parent, isMain, { ...options, paths: [modules] });
  return original.call(this, request, parent, isMain, options);
};
