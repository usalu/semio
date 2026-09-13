import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";

const testRoot = resolve(dirname(fileURLToPath(import.meta.url)), "../../📦️packages/🟦️typescript");

const root = resolve(dirname(fileURLToPath(import.meta.url)), "../../📦️packages/🟦️typescript");

export default {
  root: testRoot,
  test: {
    root: testRoot,
    name: "@semio-tech/puzzle-5d-react",
    environment: "node",
    include: ["../../🟦️.tsx"],
    coverage: { include: ["../../🟦️.tsx"] },
  },
};
