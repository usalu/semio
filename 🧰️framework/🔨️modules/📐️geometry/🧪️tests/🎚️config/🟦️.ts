import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";

const root = resolve(dirname(fileURLToPath(import.meta.url)), "../..");

export default {
  root,
  test: {
    root,
    name: "semio-framework-geometry-oracles",
    environment: "node",
    include: ["🧪️tests/🏙️aec-oracles/🟦️.ts"],
    passWithNoTests: false,
  },
};
