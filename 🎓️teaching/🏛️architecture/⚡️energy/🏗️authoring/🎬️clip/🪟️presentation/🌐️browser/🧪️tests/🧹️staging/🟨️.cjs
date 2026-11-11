const path = require("path");
const { createRequire } = require("module");
const rootRequire = createRequire(path.join(process.cwd(), "package.json"));
const { Scanner } = rootRequire("@tailwindcss/oxide");
const base = process.argv[2];
const scanner = new Scanner({ sources: [{ base, pattern: process.argv[3], negated: false }] });
scanner.scan();
process.stdout.write(JSON.stringify(scanner.files.map(file => path.relative(base, file).split(path.sep).join("/")).sort()));
