import { spawn } from "node:child_process";

const command = process.argv[2];
if (command === "dev") {
  const server = Bun.serve({ port: Number(process.env.DASHBOARD_TEST_PORT), hostname: "127.0.0.1", fetch: () => new Response("dashboard server") });
  console.log(`[DEBUG] dev listening ${server.port}`);
} else if (command === "build" || command === "test") {
  console.log(`[DEBUG] ${command} completed ${process.env.DASHBOARD_TEST_VALUE}`);
} else if (command === "view") {
  const view = spawn(process.env.SEMIO_TEST_CLI!, [], { detached: process.platform !== "win32", windowsHide: true, stdio: ["inherit", "pipe", "pipe"] });
  view.stdout.pipe(process.stdout);
  view.stderr.pipe(process.stderr);
  view.once("error", (error) => { console.error(error); process.exit(1); });
  view.once("exit", (code) => process.exit(code ?? 1));
} else {
  throw Error(`Unknown fixture command: ${command}`);
}
