import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";

const root = resolve(dirname(fileURLToPath(import.meta.url)), "..");

export default {
  root,
  test: {
    name: "@semio-tech/puzzle-5d-react",
    environment: "node",
    include: ["🟦️.tsx"],
    coverage: { include: ["🟦️.tsx"] },
  },
};
