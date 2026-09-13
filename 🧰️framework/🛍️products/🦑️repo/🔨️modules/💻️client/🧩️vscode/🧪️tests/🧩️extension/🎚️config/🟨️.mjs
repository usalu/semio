import { defineConfig } from "@vscode/test-cli";
import path from "path";
import { fileURLToPath } from "url";

const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);
const packageRoot = path.resolve(__dirname, "../../../📦️packages/🟦️typescript");

const launchArgs = [];
if (process.platform === "linux") {
  launchArgs.push("--no-sandbox", "--disable-gpu", "--disable-dev-shm-usage");
}

export default defineConfig({
  files: path.resolve(packageRoot, "out/test/**/*.test.js"),
  launchArgs,
  workspaceFolder: path.resolve(packageRoot, "../.."),
});
