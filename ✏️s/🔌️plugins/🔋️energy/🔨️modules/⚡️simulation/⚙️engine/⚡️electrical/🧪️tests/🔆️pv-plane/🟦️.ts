import assert from "node:assert/strict";
import Ajv from "ajv";
import { dot, multiply } from "mathjs";
import fixture from "../🧫️fixtures/🔆️pv-plane/🔣️.json" with { type: "json" };
import schema from "../../🧬️schema/🔆️pv-plane/🔣️.json" with { type: "json" };

const coefficients = [[-0.0083117,0.5877285,-0.0620636,-0.0596012,0.0721249,-0.0220216],[0.1299457,0.6825954,-0.1513752,-0.0189325,0.0659650,-0.0288748],[0.3296958,0.4868735,-0.2210958,0.0554140,-0.0639588,-0.0260542],[0.5682053,0.1874525,-0.2951290,0.1088631,-0.1519229,-0.0139754],[0.8730280,-0.3920403,-0.3616149,0.2255647,-0.4620442,0.0012448],[1.1326077,-1.2367284,-0.4118494,0.2877813,-0.8230357,0.0558651],[1.0601591,-1.5999137,-0.3589221,0.2642124,-1.1272340,0.1310694],[0.6777470,-0.3272588,-0.2504286,0.1561313,-1.3765031,0.2506212]];

export const solarPvFixtureCases = fixture.cases;

export function calculateSolarPvReference(row: typeof fixture.cases[number]): number[] {
  const [altitude, azimuth, tilt, direction] = [...row.sun, ...row.collector].map(value => value * Math.PI / 180);
  const sun = [Math.cos(altitude!) * Math.sin(azimuth!), Math.cos(altitude!) * Math.cos(azimuth!), Math.sin(altitude!)];
  const normal = [Math.sin(tilt!) * Math.sin(direction!), Math.sin(tilt!) * Math.cos(direction!), Math.cos(tilt!)];
  const [dni, dhi] = row.irradiance as [number,number];
  const [air, wind] = row.weather as [number,number];
  const [capacity, area, module, inverter] = row.pv as [number,number,number,number];
  let components = [0,0,0];
  if (sun[2]! > 0) {
    let f1 = 0, f2 = 0;
    if (dhi > 0) {
      const zenith = Math.acos(sun[2]!);
      const degrees = zenith * 180 / Math.PI;
      const airMass = (1 - row.elevation / 10000) / (sun[2]! + (degrees > 75 ? 0.15 * Math.pow(93.9 - degrees, -1.253) : 0));
      const clearness = ((dni + dhi) / dhi + 1.041 * zenith ** 3) / (1 + 1.041 * zenith ** 3);
      const bin = [1.065,1.23,1.5,1.95,2.8,4.5,6.2].findIndex(limit => clearness < limit);
      const c = coefficients[bin < 0 ? 7 : bin]!;
      const brightening = multiply([c.slice(0,3),c.slice(3)], [1,dhi * airMass / 1353,zenith]) as unknown as number[];
      f1 = Math.max(0,brightening[0]!); f2 = brightening[1]!;
    }
    const incidence = Math.max(0, dot(normal,sun));
    components = [dni * incidence, Math.max(0,dhi * dot([(1+normal[2]!) / 2,incidence / Math.max(sun[2]!,0.0871557),Math.sin(tilt!)],[1-f1,f1,f2])), (dni * sun[2]! + dhi) * row.reflectance * (1-normal[2]!) / 2];
  }
  const poa = dot(components,[1,1,1]);
  const gradient = {OpenRack:3,CloseRoof:1,InsulatedBack:0}[row.thermal.mounting as 'OpenRack'|'CloseRoof'|'InsulatedBack'];
  const cell = air + poa / (row.thermal.constant + row.thermal.wind * Math.max(0,wind)) + poa / 1000 * gradient;
  const dc = Math.min(capacity,Math.max(0, area * poa * module * (1+row.thermal.coefficient * (cell-25))));
  return [...components,cell,dc * inverter];
}

/** ☀️ The five PV mutation admission schemas, supplied by the artifact that owns them so the plugin core never imports its artifacts. */
export type SolarPvMutationSchemas = Readonly<{ coefficient: object; constant: object; wind: object; mounting: object; create: object }>;

export function verifySolarPvOracle(schemas: SolarPvMutationSchemas): void {
  const validate = new Ajv({strict:true}).compile(schema);
  assert(validate(fixture),JSON.stringify(validate.errors));
  for (const row of fixture.cases) calculateSolarPvReference(row).forEach((value,index) => assert(Math.abs(value-row.expected[index]!) <= fixture.tolerance,`${row.id} component ${index}: ${value}`));
  assert(fixture.cases[2]!.expected[1]! > 0);
  assert(fixture.cases[8]!.expected[3]! < fixture.cases[7]!.expected[3]!);
  assert(fixture.cases[8]!.expected[4]! > fixture.cases[7]!.expected[4]!);
  const semantic = new Ajv({strict:true}).addKeyword("x-semio-ui");
  const coefficient = semantic.compile(schemas.coefficient), constant = semantic.compile(schemas.constant), wind = semantic.compile(schemas.wind), mounting = semantic.compile(schemas.mounting), create = semantic.compile(schemas.create);
  for (const row of fixture.cases) {
    assert(coefficient({mutation:"changePvSystemTemperatureCoefficient",id:900,newTemperatureCoefficient:row.thermal.coefficient}));
    assert(constant({mutation:"changePvSystemFaimanHeatLossConstant",id:900,newFaimanHeatLossConstantWM2k:row.thermal.constant}));
    assert(wind({mutation:"changePvSystemFaimanHeatLossWind",id:900,newFaimanHeatLossWindWSM3k:row.thermal.wind}));
    assert(mounting({mutation:"changePvSystemMounting",id:900,newMounting:row.thermal.mounting}));
    assert(create({mutation:"createPvSystem",index:0,id:900,dcCapacityW:row.pv[0],areaM2:row.pv[1],moduleEfficiency:row.pv[2],inverterEfficiency:row.pv[3],tiltDeg:row.collector[0],azimuthDeg:row.collector[1],temperatureCoefficient:row.thermal.coefficient,faimanHeatLossConstantWM2k:row.thermal.constant,faimanHeatLossWindWSM3k:row.thermal.wind,mounting:row.thermal.mounting}));
  }
  assert(!coefficient({mutation:"changePvSystemTemperatureCoefficient",id:900,newTemperatureCoefficient:NaN}));
  assert(!constant({mutation:"changePvSystemFaimanHeatLossConstant",id:900,newFaimanHeatLossConstantWM2k:0}));
  assert(!constant({mutation:"changePvSystemFaimanHeatLossConstant",id:900,newFaimanHeatLossConstantWM2k:Infinity}));
  assert(!wind({mutation:"changePvSystemFaimanHeatLossWind",id:900,newFaimanHeatLossWindWSM3k:-1}));
  assert(!mounting({mutation:"changePvSystemMounting",id:900,newMounting:"Roof"}));
  console.log(`[DEBUG] ${fixture.cases.length * 5 + 5} authored PV semantic payloads match independent Ajv admission and physical refusal`);
  console.log(`[DEBUG] ${fixture.cases.length} PV plane cases match independent mathjs matrix/vector irradiance, Faiman temperature and clipped authored-efficiency power`);
}
