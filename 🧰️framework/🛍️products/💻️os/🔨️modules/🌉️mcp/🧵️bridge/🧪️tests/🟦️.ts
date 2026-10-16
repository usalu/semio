import {test,expect} from "bun:test";
import Ajv from "ajv";
import{decodeShellToGateway,decodeGatewayToShell,encodeShellToGateway,encodeGatewayToShell}from"@semio-tech/framework/mcp/bridge";
import{encodeChannelBase64,decodeChannelBase64}from"@semio-tech/framework/mcp/channel";
import frames from "./🧫️fixtures/📨️frames.json";
import invalid from "./🧫️fixtures/🚫️boundaries.json";
import schema from "./🧬️schema/🔣️.json";
test("neutral MCP bridge exact wire fixtures match independent Node buffer and Base64",()=>{
 for(const row of frames){const bytes=Buffer.from(row.hex,"hex"),actual=row.direction==="shell_to_gateway"?decodeShellToGateway(bytes):decodeGatewayToShell(bytes),encoded=row.direction==="shell_to_gateway"?encodeShellToGateway(decodeShellToGateway(bytes)):encodeGatewayToShell(decodeGatewayToShell(bytes));expect(Buffer.from(encoded).toString("hex")).toBe(row.hex);const neutral=(value:unknown):unknown=>typeof value==="bigint"?Number(value):value instanceof Uint8Array?Array.from(value):Array.isArray(value)?value.map(neutral):value&&typeof value==="object"?Object.fromEntries(Object.entries(value).map(([key,item])=>[key,neutral(item)])):value;const data=neutral(actual);expect(data).toEqual(row.frame);expect(encodeChannelBase64(bytes)).toBe(bytes.toString("base64"));expect(decodeChannelBase64(bytes.toString("base64"))).toEqual(new Uint8Array(bytes));for(let size=0;size<bytes.length;size++)expect(()=>row.direction==="shell_to_gateway"?decodeShellToGateway(bytes.subarray(0,size)):decodeGatewayToShell(bytes.subarray(0,size))).toThrow();}
});
test("neutral MCP bridge refuses undeclared shell and approval discriminants",()=>{expect(new Ajv({strict:true}).validate(schema,invalid)).toBe(true);for(const row of invalid){const bytes=Buffer.from(row.hex,"hex");expect(()=>row.direction==="shell_to_gateway"?decodeShellToGateway(bytes):decodeGatewayToShell(bytes)).toThrow();}});
