import assert from "node:assert/strict";
import { join } from "node:path";
import { playgroundReactReleaseNxOutput, playgroundReactReleaseOutputPath } from "../../🟦️.ts";

const workspace = "/repo";
const packageRoot = "/repo/🧰️framework/.../typescript";

assert.equal(playgroundReactReleaseNxOutput(packageRoot, { variant: "cad" }), "{projectRoot}/dist/build-cad-react-release");
assert.equal(playgroundReactReleaseNxOutput(packageRoot, { variant: "fem3d", distDir: "✏️s/🔌️plugins/🏗️fem/dist" }), "{workspaceRoot}/✏️s/🔌️plugins/🏗️fem/dist");
assert.equal(
  playgroundReactReleaseOutputPath(workspace, packageRoot, { variant: "fem3d", distDir: "✏️s/🔌️plugins/🏗️fem/dist" }),
  join(workspace, "✏️s/🔌️plugins/🏗️fem/dist"),
);
console.log("playground react release output paths PASS");
