import fixture from "../../🧫️fixtures/🔣️.json";
import type {LasSnapshot} from "../../../../../🧬️schema/📸️snapshot/🟦️.ts";
const f=(index:number)=>({bits:BigInt("0x"+fixture.float64Bits[index%fixture.float64Bits.length]!)});
export const lasSnapshotFixture:LasSnapshot={schema:fixture.schema,header:{versionMajor:255,versionMinor:255,systemIdentifier:"owned 🌠",generatingSoftware:"SQL",creationDayOfYear:65535,creationYear:65535,headerSize:65535,offsetToPointData:4294967295,numberOfVlrs:4294967295,pointDataFormatId:255,pointDataRecordLength:65535,numberOfPointRecords:4294967295,pointsByReturn:[4294967295,0,17,17,1],xScale:f(6),yScale:f(7),zScale:f(8),xOffset:f(1),yOffset:f(4),zOffset:f(5),maxX:f(3),minX:f(2),maxY:f(0),minY:f(6),maxZ:f(7),minZ:f(8)},vlrs:fixture.vlrs.map(v=>({...v,data:[...v.data]})),points:[
  {x:f(6),y:f(1),z:f(4),intensity:65535,returnNumber:255,numberOfReturns:255,scanDirectionFlag:true,edgeOfFlightLine:false,classification:255,scanAngleRank:-128,userData:255,pointSourceId:65535},
  {x:f(7),y:f(2),z:f(5),intensity:0,returnNumber:0,numberOfReturns:0,scanDirectionFlag:false,edgeOfFlightLine:true,classification:0,scanAngleRank:127,userData:0,pointSourceId:0,gpsTime:f(6)},
  {x:f(8),y:f(3),z:f(0),intensity:17,returnNumber:1,numberOfReturns:2,scanDirectionFlag:false,edgeOfFlightLine:false,classification:17,scanAngleRank:0,userData:17,pointSourceId:17,rgb:[0,65535,17]},
  {x:f(1),y:f(4),z:f(5),intensity:17,returnNumber:7,numberOfReturns:7,scanDirectionFlag:true,edgeOfFlightLine:true,classification:255,scanAngleRank:-1,userData:255,pointSourceId:65535,gpsTime:f(1),rgb:[65535,0,65535]}
]};
