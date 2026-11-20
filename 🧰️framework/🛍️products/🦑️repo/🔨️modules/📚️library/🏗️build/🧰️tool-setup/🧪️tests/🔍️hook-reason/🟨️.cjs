const cases = require("./🔣️.json");
const result = cases[process.argv[2]];
console.log(result.output);
process.exitCode = result.code;
