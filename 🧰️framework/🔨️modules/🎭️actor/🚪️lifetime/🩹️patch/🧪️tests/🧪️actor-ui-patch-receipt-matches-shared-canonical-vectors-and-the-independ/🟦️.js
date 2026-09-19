"use strict";
var __assign = (this && this.__assign) || function () {
    __assign = Object.assign || function(t) {
        for (var s, i = 1, n = arguments.length; i < n; i++) {
            s = arguments[i];
            for (var p in s) if (Object.prototype.hasOwnProperty.call(s, p))
                t[p] = s[p];
        }
        return t;
    };
    return __assign.apply(this, arguments);
};
var __awaiter = (this && this.__awaiter) || function (thisArg, _arguments, P, generator) {
    function adopt(value) { return value instanceof P ? value : new P(function (resolve) { resolve(value); }); }
    return new (P || (P = Promise))(function (resolve, reject) {
        function fulfilled(value) { try { step(generator.next(value)); } catch (e) { reject(e); } }
        function rejected(value) { try { step(generator["throw"](value)); } catch (e) { reject(e); } }
        function step(result) { result.done ? resolve(result.value) : adopt(result.value).then(fulfilled, rejected); }
        step((generator = generator.apply(thisArg, _arguments || [])).next());
    });
};
var __generator = (this && this.__generator) || function (thisArg, body) {
    var _ = { label: 0, sent: function() { if (t[0] & 1) throw t[1]; return t[1]; }, trys: [], ops: [] }, f, y, t, g = Object.create((typeof Iterator === "function" ? Iterator : Object).prototype);
    return g.next = verb(0), g["throw"] = verb(1), g["return"] = verb(2), typeof Symbol === "function" && (g[Symbol.iterator] = function() { return this; }), g;
    function verb(n) { return function (v) { return step([n, v]); }; }
    function step(op) {
        if (f) throw new TypeError("Generator is already executing.");
        while (g && (g = 0, op[0] && (_ = 0)), _) try {
            if (f = 1, y && (t = op[0] & 2 ? y["return"] : op[0] ? y["throw"] || ((t = y["return"]) && t.call(y), 0) : y.next) && !(t = t.call(y, op[1])).done) return t;
            if (y = 0, t) op = [op[0] & 2, t.value];
            switch (op[0]) {
                case 0: case 1: t = op; break;
                case 4: _.label++; return { value: op[1], done: false };
                case 5: _.label++; y = op[1]; op = [0]; continue;
                case 7: op = _.ops.pop(); _.trys.pop(); continue;
                default:
                    if (!(t = _.trys, t = t.length > 0 && t[t.length - 1]) && (op[0] === 6 || op[0] === 2)) { _ = 0; continue; }
                    if (op[0] === 3 && (!t || (op[1] > t[0] && op[1] < t[3]))) { _.label = op[1]; break; }
                    if (op[0] === 6 && _.label < t[1]) { _.label = t[1]; t = op; break; }
                    if (t && _.label < t[2]) { _.label = t[2]; _.ops.push(op); break; }
                    if (t[2]) _.ops.pop();
                    _.trys.pop(); continue;
            }
            op = body.call(thisArg, _);
        } catch (e) { op = [6, e]; y = 0; } finally { f = t = 0; }
        if (op[0] & 5) throw op[1]; return { value: op[0] ? op[1] : void 0, done: true };
    }
};
Object.defineProperty(exports, "__esModule", { value: true });
exports.registerTests1 = registerTests1;
function registerTests1(vitest, dependencies, source) {
    return __awaiter(this, void 0, void 0, function () {
        var ACTOR_UI_PATCH_RECEIPT_MAXIMUM_BYTES, actorUiPatchReceiptEquals, decodeActorUiPatchReceipt, encodeActorUiPatchReceipt, validateActorUiPatchPairing, it, expect;
        var _this = this;
        return __generator(this, function (_a) {
            ACTOR_UI_PATCH_RECEIPT_MAXIMUM_BYTES = dependencies.ACTOR_UI_PATCH_RECEIPT_MAXIMUM_BYTES, actorUiPatchReceiptEquals = dependencies.actorUiPatchReceiptEquals, decodeActorUiPatchReceipt = dependencies.decodeActorUiPatchReceipt, encodeActorUiPatchReceipt = dependencies.encodeActorUiPatchReceipt, validateActorUiPatchPairing = dependencies.validateActorUiPatchPairing;
            it = vitest.it, expect = vitest.expect;
            it("actor UI patch receipt matches shared canonical vectors and the independent LEB128 encoder", function () { return __awaiter(_this, void 0, void 0, function () {
                var readFileSync, Ajv, fixture, schema, lifetimeSchema, valueSchema, validate, moduleName, module, oracle, encode, _loop_1, _i, _a, row, _loop_2, _b, _c, hex;
                var _d;
                return __generator(this, function (_e) {
                    switch (_e.label) {
                        case 0: return [4 /*yield*/, Promise.resolve().then(function () { return require("node:fs"); })];
                        case 1:
                            readFileSync = (_e.sent()).readFileSync;
                            return [4 /*yield*/, Promise.resolve().then(function () { return require("ajv"); })];
                        case 2:
                            Ajv = (_e.sent()).default;
                            fixture = JSON.parse(readFileSync(new URL("./🧫️fixtures/🔣️.json", source.url), "utf8"));
                            schema = JSON.parse(readFileSync(new URL("./🧬️schema/🔣️.json", source.url), "utf8"));
                            lifetimeSchema = JSON.parse(readFileSync(new URL("../🧬️schema/🔣️.json", source.url), "utf8"));
                            valueSchema = JSON.parse(readFileSync(new URL("../../../🌱️value/🧬️schema/🔣️.json", source.url), "utf8"));
                            validate = new Ajv({ strict: true }).addSchema(valueSchema).addSchema(lifetimeSchema).addSchema(schema).getSchema("".concat(schema.$id, "#/$defs/PatchFixture"));
                            expect(validate(fixture)).toBe(true);
                            expect(validate(__assign(__assign({}, fixture), { maximumBytes: 36 }))).toBe(false);
                            moduleName = "@webassemblyjs/leb128/lib/leb.js";
                            return [4 /*yield*/, Promise.resolve("".concat(moduleName)).then(function (s) { return require(s); })];
                        case 3:
                            module = _e.sent();
                            oracle = module && typeof module === "object" ? (_d = Reflect.get(module, "default")) !== null && _d !== void 0 ? _d : module : null;
                            encode = oracle && typeof oracle === "object" ? Reflect.get(oracle, "encodeUIntBuffer") : null;
                            if (typeof encode !== "function")
                                throw new Error("invalid independent LEB128 encoder");
                            expect(ACTOR_UI_PATCH_RECEIPT_MAXIMUM_BYTES).toBe(fixture.maximumBytes);
                            _loop_1 = function (row) {
                                var receipt = {
                                    lifetime: { activationGeneration: BigInt(row.value.lifetime.activationGeneration), instanceId: row.value.lifetime.instanceId, guestLifetime: BigInt(row.value.lifetime.guestLifetime) },
                                    patchSequence: BigInt(row.value.patchSequence),
                                };
                                var bytes = encodeActorUiPatchReceipt(receipt);
                                expect(Buffer.from(bytes).toString("hex")).toBe(row.hex);
                                var independent = [receipt.lifetime.activationGeneration, BigInt(receipt.lifetime.instanceId), receipt.lifetime.guestLifetime, receipt.patchSequence].map(function (value) {
                                    var input = Buffer.alloc(8);
                                    input.writeBigUInt64LE(value);
                                    return Buffer.from(encode(input));
                                });
                                expect(Buffer.concat(independent).toString("hex")).toBe(row.hex);
                                expect(decodeActorUiPatchReceipt(bytes)).toEqual(receipt);
                                var _loop_3 = function (prefix) {
                                    expect(function () { return decodeActorUiPatchReceipt(bytes.subarray(0, prefix)); }).toThrow();
                                };
                                for (var prefix = 0; prefix < bytes.length; prefix += 1) {
                                    _loop_3(prefix);
                                }
                            };
                            for (_i = 0, _a = fixture.vectors; _i < _a.length; _i++) {
                                row = _a[_i];
                                _loop_1(row);
                            }
                            _loop_2 = function (hex) {
                                expect(function () { return decodeActorUiPatchReceipt(Buffer.from(hex, "hex")); }).toThrow();
                            };
                            for (_b = 0, _c = fixture.invalidHex; _b < _c.length; _b++) {
                                hex = _c[_b];
                                _loop_2(hex);
                            }
                            expect(function () { return decodeActorUiPatchReceipt(new Uint8Array(36)); }).toThrow();
                            return [2 /*return*/];
                    }
                });
            }); });
            it("actor UI patch receipt rejects invalid authority and enforces exact zero or one patch pairing", function () { return __awaiter(_this, void 0, void 0, function () {
                var readFileSync, fixture, receipt, invalidCounters, _loop_4, _i, invalidCounters_1, value, _loop_5, _a, _b, instanceId, _loop_6, _c, _d, row, _loop_7, _e, _f, count;
                return __generator(this, function (_g) {
                    switch (_g.label) {
                        case 0: return [4 /*yield*/, Promise.resolve().then(function () { return require("node:fs"); })];
                        case 1:
                            readFileSync = (_g.sent()).readFileSync;
                            fixture = JSON.parse(readFileSync(new URL("./🧫️fixtures/🔣️.json", source.url), "utf8"));
                            receipt = { lifetime: { activationGeneration: 41n, instanceId: 7, guestLifetime: 13n }, patchSequence: 51n };
                            invalidCounters = [0n, -1n, 0x10000000000000000n, 1, "1", null, undefined];
                            _loop_4 = function (value) {
                                expect(function () { return encodeActorUiPatchReceipt(__assign(__assign({}, receipt), { patchSequence: value })); }).toThrow();
                                expect(function () { return encodeActorUiPatchReceipt(__assign(__assign({}, receipt), { lifetime: __assign(__assign({}, receipt.lifetime), { activationGeneration: value }) })); }).toThrow();
                                expect(function () { return encodeActorUiPatchReceipt(__assign(__assign({}, receipt), { lifetime: __assign(__assign({}, receipt.lifetime), { guestLifetime: value }) })); }).toThrow();
                            };
                            for (_i = 0, invalidCounters_1 = invalidCounters; _i < invalidCounters_1.length; _i++) {
                                value = invalidCounters_1[_i];
                                _loop_4(value);
                            }
                            _loop_5 = function (instanceId) {
                                expect(function () { return encodeActorUiPatchReceipt(__assign(__assign({}, receipt), { lifetime: __assign(__assign({}, receipt.lifetime), { instanceId: instanceId }) })); }).toThrow();
                            };
                            for (_a = 0, _b = [-1, 0x100000000, 1.5, NaN]; _a < _b.length; _a++) {
                                instanceId = _b[_a];
                                _loop_5(instanceId);
                            }
                            expect(actorUiPatchReceiptEquals(receipt, decodeActorUiPatchReceipt(encodeActorUiPatchReceipt(receipt)))).toBe(true);
                            expect(actorUiPatchReceiptEquals(receipt, __assign(__assign({}, receipt), { lifetime: __assign(__assign({}, receipt.lifetime), { guestLifetime: 14n }) }))).toBe(fixture.feedback.oldGuestAccepted);
                            expect(actorUiPatchReceiptEquals(receipt, __assign(__assign({}, receipt), { patchSequence: 52n }))).toBe(fixture.feedback.oldSequenceAccepted);
                            _loop_6 = function (row) {
                                var validate = function () { return validateActorUiPatchPairing(row.patchCount, row.hasReceipt ? receipt : null); };
                                if (row.accepted)
                                    expect(validate).not.toThrow();
                                else
                                    expect(validate).toThrow();
                            };
                            for (_c = 0, _d = fixture.pairing; _c < _d.length; _c++) {
                                row = _d[_c];
                                _loop_6(row);
                            }
                            _loop_7 = function (count) {
                                expect(function () { return validateActorUiPatchPairing(count, receipt); }).toThrow();
                            };
                            for (_e = 0, _f = [-1, 1.5, NaN, Infinity]; _e < _f.length; _e++) {
                                count = _f[_e];
                                _loop_7(count);
                            }
                            expect(function () { return validateActorUiPatchPairing(1, __assign(__assign({}, receipt), { patchSequence: 0n })); }).toThrow();
                            return [2 /*return*/];
                    }
                });
            }); });
            return [2 /*return*/];
        });
    });
}
