import{crashScenariosV1}from"../🎬️scenarios/🟦️.ts";
const[id,directory]=process.argv.slice(2),scenario=crashScenariosV1.find(row=>row.id===id);
if(!scenario||!directory||process.argv.length!==4)throw Error("Known crash scenario and directory required");
await scenario.run(directory);
