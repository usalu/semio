import { createScript } from "../../../../../../📜️script.ts";

export default createScript(import.meta, {
  check: async ({ cargo }) => {
    await cargo(["check", "--manifest-path", "Cargo.toml"]);
  },
  test: async ({ cargo }) => {
    await cargo(["test", "--manifest-path", "Cargo.toml", "--lib"]);
  },
});
