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
var __spreadArray = (this && this.__spreadArray) || function (to, from, pack) {
    if (pack || arguments.length === 2) for (var i = 0, l = from.length, ar; i < l; i++) {
        if (ar || !(i in from)) {
            if (!ar) ar = Array.prototype.slice.call(from, 0, i);
            ar[i] = from[i];
        }
    }
    return to.concat(ar || Array.prototype.slice.call(from));
};
Object.defineProperty(exports, "__esModule", { value: true });
exports.registerTests1 = registerTests1;
function registerTests1(vitest, dependencies, source) {
    return __awaiter(this, void 0, void 0, function () {
        var ACTOR_INSTANCE_LIFECYCLE_MAXIMUM_BYTES, actorInstanceCapturedReceiptMatches, actorInstanceCloseReceiptMatches, actorInstanceLifecycleReceiptEquals, actorInstanceLifetimeEquals, decodeActorInstanceLifecycle, encodeActorInstanceLifecycle, it, expect;
        var _this = this;
        return __generator(this, function (_a) {
            ACTOR_INSTANCE_LIFECYCLE_MAXIMUM_BYTES = dependencies.ACTOR_INSTANCE_LIFECYCLE_MAXIMUM_BYTES, actorInstanceCapturedReceiptMatches = dependencies.actorInstanceCapturedReceiptMatches, actorInstanceCloseReceiptMatches = dependencies.actorInstanceCloseReceiptMatches, actorInstanceLifecycleReceiptEquals = dependencies.actorInstanceLifecycleReceiptEquals, actorInstanceLifetimeEquals = dependencies.actorInstanceLifetimeEquals, decodeActorInstanceLifecycle = dependencies.decodeActorInstanceLifecycle, encodeActorInstanceLifecycle = dependencies.encodeActorInstanceLifecycle;
            it = vitest.it, expect = vitest.expect;
            it("actor instance close fault publication fixture preserves watchdog and terminal-outcome precedence", function () { return __awaiter(_this, void 0, void 0, function () {
                var readFileSync, Ajv, fixture, schema, validate, name, module, greater, _i, _a, row, _b, _c, row, _d, start, preflight, finish, entered, _e, _f, row;
                return __generator(this, function (_g) {
                    switch (_g.label) {
                        case 0: return [4 /*yield*/, Promise.resolve().then(function () { return require("node:fs"); })];
                        case 1:
                            readFileSync = (_g.sent()).readFileSync;
                            return [4 /*yield*/, Promise.resolve().then(function () { return require("ajv"); })];
                        case 2:
                            Ajv = (_g.sent()).default;
                            fixture = JSON.parse(readFileSync(new URL("./🚨️fault.fixture.json", source.url), "utf8"));
                            schema = JSON.parse(readFileSync(new URL("./🧬️schema/🔣️.json", source.url), "utf8"));
                            validate = new Ajv({ strict: true }).addSchema(schema).getSchema("".concat(schema.$id, "#/$defs/CloseFaultFixture"));
                            expect(validate(fixture)).toBe(true);
                            expect(validate(__assign(__assign({}, fixture), { extra: true }))).toBe(false);
                            expect(validate(__assign(__assign({}, fixture), { callbackLimitUs: 8001 }))).toBe(false);
                            expect(validate(__assign(__assign({}, fixture), { owners: __assign(__assign({}, fixture.owners), { forgottenPayloads: 1 }) }))).toBe(false);
                            name = "lodash-es/gte.js";
                            return [4 /*yield*/, Promise.resolve("".concat(name)).then(function (s) { return require(s); })];
                        case 3:
                            module = _g.sent();
                            greater = module && typeof module === "object" ? Reflect.get(module, "default") : null;
                            if (typeof greater !== "function")
                                throw new Error("invalid independent elapsed comparison oracle");
                            for (_i = 0, _a = fixture.callbacks; _i < _a.length; _i++) {
                                row = _a[_i];
                                expect(row.candidate !== "fault" && greater(row.elapsedUs, fixture.callbackLimitUs) ? "deadline-yield" : row.candidate).toBe(row.published);
                            }
                            for (_b = 0, _c = fixture.clocks; _b < _c.length; _b++) {
                                row = _c[_b];
                                _d = row.samples, start = _d[0], preflight = _d[1], finish = _d[2];
                                entered = typeof start === "number" && typeof preflight === "number" && preflight >= start;
                                expect(entered).toBe(row.workEntered);
                                expect(!entered || typeof finish !== "number" || finish < preflight ? "fault" : greater(finish - start, fixture.callbackLimitUs) ? "deadline-yield" : "complete").toBe(row.published);
                            }
                            for (_e = 0, _f = fixture.terminalPump; _e < _f.length; _e++) {
                                row = _f[_e];
                                expect(row.faulted ? "fault" : row.blocked ? "external-wait" : row.complete ? "complete" : "ready").toBe(row.status);
                            }
                            return [2 /*return*/];
                    }
                });
            }); });
            it("actor instance close native value fixture accounts exact descendant text and independent cloned structure", function () { return __awaiter(_this, void 0, void 0, function () {
                var readFileSync, Ajv, fixture, schema, validate, name, module, clone, _loop_1, _i, _a, row;
                return __generator(this, function (_b) {
                    switch (_b.label) {
                        case 0: return [4 /*yield*/, Promise.resolve().then(function () { return require("node:fs"); })];
                        case 1:
                            readFileSync = (_b.sent()).readFileSync;
                            return [4 /*yield*/, Promise.resolve().then(function () { return require("ajv"); })];
                        case 2:
                            Ajv = (_b.sent()).default;
                            fixture = JSON.parse(readFileSync(new URL("../../🖱️ui/🧬️contract/♻️retirement/🧫️fixtures/🔣️.json", source.url), "utf8"));
                            schema = JSON.parse(readFileSync(new URL("../../🖱️ui/🧬️contract/♻️retirement/🧬️schema/🔣️.json", source.url), "utf8"));
                            validate = new Ajv({ strict: true }).compile(schema);
                            expect(validate(fixture)).toBe(true);
                            expect(validate(__assign(__assign({}, fixture), { extra: true }))).toBe(false);
                            expect(validate(__assign(__assign({}, fixture), { grants: [0, 4096] }))).toBe(false);
                            expect(validate(__assign(__assign({}, fixture), { ownership: __assign(__assign({}, fixture.ownership), { nestedRootTerminalBeforeDescendants: true }) }))).toBe(false);
                            name = "lodash-es/cloneDeepWith.js";
                            return [4 /*yield*/, Promise.resolve("".concat(name)).then(function (s) { return require(s); })];
                        case 3:
                            module = _b.sent();
                            clone = module && typeof module === "object" ? Reflect.get(module, "default") : null;
                            if (typeof clone !== "function")
                                throw new Error("invalid independent descendant traversal oracle");
                            _loop_1 = function (row) {
                                var measured = { textBytes: 0, pages: 0, collections: 0 };
                                var stack = [row.value];
                                while (stack.length) {
                                    var value = stack.pop();
                                    if (typeof value === "string")
                                        measured.textBytes += new TextEncoder().encode(value).length;
                                    else if (value && typeof value === "object") {
                                        var entries = Object.entries(value);
                                        measured.pages += entries.length;
                                        measured.collections += Number(entries.length !== 0);
                                        for (var _c = 0, entries_1 = entries; _c < entries_1.length; _c++) {
                                            var _d = entries_1[_c], key = _d[0], child = _d[1];
                                            if (!Array.isArray(value))
                                                measured.textBytes += new TextEncoder().encode(key).length;
                                            stack.push(child);
                                        }
                                    }
                                }
                                var oracle = { textBytes: 0, pages: 0, collections: 0 };
                                var copied = clone(row.value, function (value, key, parent) {
                                    if (parent && typeof parent === "object" && !Array.isArray(parent))
                                        oracle.textBytes += Buffer.byteLength(String(key), "utf8");
                                    if (typeof value === "string")
                                        oracle.textBytes += Buffer.byteLength(value, "utf8");
                                    else if (value && typeof value === "object") {
                                        var size = Object.keys(value).length;
                                        oracle.pages += size;
                                        oracle.collections += Number(size !== 0);
                                    }
                                    return undefined;
                                });
                                expect(copied).toEqual(row.value);
                                expect(measured).toEqual(oracle);
                                expect(measured).toEqual({ textBytes: row.textBytes, pages: row.pages, collections: row.collections });
                            };
                            for (_i = 0, _a = fixture.cases; _i < _a.length; _i++) {
                                row = _a[_i];
                                _loop_1(row);
                            }
                            return [2 /*return*/];
                    }
                });
            }); });
            it("actor instance close fixed-list fixture preserves ordered payload handoff", function () { return __awaiter(_this, void 0, void 0, function () {
                var readFileSync, Ajv, fixture, schema, validate, moduleName, module, reverse, _i, _a, row, remaining, moved;
                return __generator(this, function (_b) {
                    switch (_b.label) {
                        case 0: return [4 /*yield*/, Promise.resolve().then(function () { return require("node:fs"); })];
                        case 1:
                            readFileSync = (_b.sent()).readFileSync;
                            return [4 /*yield*/, Promise.resolve().then(function () { return require("ajv"); })];
                        case 2:
                            Ajv = (_b.sent()).default;
                            fixture = JSON.parse(readFileSync(new URL("../../🖱️ui/🧬️contract/♻️retirement/📋️list/🧫️fixtures/🔣️.json", source.url), "utf8"));
                            schema = JSON.parse(readFileSync(new URL("../../🖱️ui/🧬️contract/♻️retirement/📋️list/🧬️schema/🔣️.json", source.url), "utf8"));
                            validate = new Ajv({ strict: true }).compile(schema);
                            expect(validate(fixture)).toBe(true);
                            expect(validate(__assign(__assign({}, fixture), { capacity: 5 }))).toBe(false);
                            expect(validate(__assign(__assign({}, fixture), { extra: true }))).toBe(false);
                            expect(validate(__assign(__assign({}, fixture), { ownership: __assign(__assign({}, fixture.ownership), { releaseWithPayloadAccepted: true }) }))).toBe(false);
                            expect(validate(__assign(__assign({}, fixture), { cases: __spreadArray(__spreadArray([], fixture.cases, true), [{ name: "overflow", values: [1, 2, 3, 4, 5], popped: [] }], false) }))).toBe(false);
                            moduleName = "lodash-es/reverse.js";
                            return [4 /*yield*/, Promise.resolve("".concat(moduleName)).then(function (s) { return require(s); })];
                        case 3:
                            module = _b.sent();
                            reverse = module && typeof module === "object" ? Reflect.get(module, "default") : null;
                            if (typeof reverse !== "function")
                                throw new Error("invalid independent sequence oracle");
                            for (_i = 0, _a = fixture.cases; _i < _a.length; _i++) {
                                row = _a[_i];
                                remaining = __spreadArray([], row.values, true);
                                moved = [];
                                while (remaining.length)
                                    moved.push(remaining.pop());
                                expect(moved).toEqual(row.popped);
                                expect(reverse(__spreadArray([], row.values, true))).toEqual(row.popped);
                                expect(JSON.parse(JSON.stringify(row.values))).toEqual(row.values);
                            }
                            return [2 /*return*/];
                    }
                });
            }); });
            it("actor typed descendant fixture covers the exact component and patch rosters", function () { return __awaiter(_this, void 0, void 0, function () {
                var readFileSync, Ajv, fixture, schema, validate, encode, bytes, moduleName, module, pairs, pending, oracleBytes, value, entries, _i, entries_2, entry, components, validateComponents, enumFields, valueFields, semanticBytes, _a, _b, row;
                return __generator(this, function (_c) {
                    switch (_c.label) {
                        case 0: return [4 /*yield*/, Promise.resolve().then(function () { return require("node:fs"); })];
                        case 1:
                            readFileSync = (_c.sent()).readFileSync;
                            return [4 /*yield*/, Promise.resolve().then(function () { return require("ajv"); })];
                        case 2:
                            Ajv = (_c.sent()).default;
                            fixture = JSON.parse(readFileSync(new URL("../../🖱️ui/🧬️contract/♻️retirement/🌳️typed/🧫️fixtures/🔣️.json", source.url), "utf8"));
                            schema = JSON.parse(readFileSync(new URL("../../🖱️ui/🧬️contract/♻️retirement/🌳️typed/🧬️schema/🔣️.json", source.url), "utf8"));
                            validate = new Ajv({ strict: true }).addSchema(schema).getSchema("".concat(schema.$id, "#/$defs/TypedFixture"));
                            expect(validate(fixture)).toBe(true);
                            expect(validate(__assign(__assign({}, fixture), { componentVariants: fixture.componentVariants.slice(1) }))).toBe(false);
                            expect(validate(__assign(__assign({}, fixture), { patchVariants: __spreadArray(__spreadArray([], fixture.patchVariants, true), ["invented"], false) }))).toBe(false);
                            expect(validate(__assign(__assign({}, fixture), { ownership: __assign(__assign({}, fixture.ownership), { arenaContentionAdvances: true }) }))).toBe(false);
                            expect(validate(__assign(__assign({}, fixture), { document: __assign(__assign({}, fixture.document), { terminalDescendantsRetired: false }) }))).toBe(false);
                            encode = new TextEncoder();
                            bytes = function (value) {
                                return typeof value === "string"
                                    ? encode.encode(value).length
                                    : Array.isArray(value)
                                        ? value.reduce(function (sum, child) { return sum + bytes(child); }, 0)
                                        : value && typeof value === "object"
                                            ? Object.entries(value).reduce(function (sum, _a) {
                                                var key = _a[0], child = _a[1];
                                                return sum + encode.encode(key).length + bytes(child);
                                            }, 0)
                                            : 0;
                            };
                            moduleName = "lodash-es/toPairs.js";
                            return [4 /*yield*/, Promise.resolve("".concat(moduleName)).then(function (s) { return require(s); })];
                        case 3:
                            module = _c.sent();
                            pairs = module && typeof module === "object" ? Reflect.get(module, "default") : null;
                            if (typeof pairs !== "function")
                                throw new Error("invalid independent object traversal oracle");
                            pending = [fixture.document.value];
                            oracleBytes = 0;
                            while (pending.length) {
                                value = pending.pop();
                                if (typeof value === "string")
                                    oracleBytes += Buffer.byteLength(value);
                                else if (Array.isArray(value))
                                    pending.push.apply(pending, value);
                                else if (value && typeof value === "object") {
                                    entries = pairs(value);
                                    if (!Array.isArray(entries))
                                        throw new Error("invalid independent object entries");
                                    for (_i = 0, entries_2 = entries; _i < entries_2.length; _i++) {
                                        entry = entries_2[_i];
                                        if (!Array.isArray(entry) || entry.length !== 2 || typeof entry[0] !== "string")
                                            throw new Error("invalid independent object pair");
                                        oracleBytes += Buffer.byteLength(entry[0]);
                                        pending.push(entry[1]);
                                    }
                                }
                            }
                            expect(bytes(fixture.document.value)).toBe(fixture.document.valueTextBytes);
                            expect(oracleBytes).toBe(fixture.document.valueTextBytes);
                            components = JSON.parse(readFileSync(new URL("../../🖱️ui/🧬️contract/♻️retirement/🌳️typed/🧩️components.json", source.url), "utf8"));
                            validateComponents = new Ajv({ strict: true }).addSchema(schema).getSchema("".concat(schema.$id, "#/$defs/Components"));
                            expect(validateComponents(components)).toBe(true);
                            expect(validateComponents(__assign(__assign({}, components), { cases: components.cases.slice(1) }))).toBe(false);
                            expect(validateComponents(__assign(__assign({}, components), { cases: components.cases.map(function (row, index) { return (index ? row : __assign(__assign({}, row), { component: __assign(__assign({}, row.component), { extra: 1 }) })); }) }))).toBe(false);
                            expect(components.cases.map(function (row) { return row.component.type; })).toEqual(fixture.componentVariants);
                            enumFields = new Set(["type", "role", "kind", "trigger", "placement"]);
                            valueFields = new Set(["props", "args", "input", "dragData", "dataAttributes"]);
                            semanticBytes = function (value, key, raw) {
                                if (key === void 0) { key = ""; }
                                if (raw === void 0) { raw = false; }
                                if (typeof value === "string")
                                    return !raw && enumFields.has(key) ? 0 : Buffer.byteLength(value);
                                if (Array.isArray(value))
                                    return !raw && key === "bytes" ? value.length : value.reduce(function (sum, child) { return sum + semanticBytes(child, "", raw); }, 0);
                                if (!value || typeof value !== "object")
                                    return 0;
                                var entries = pairs(value);
                                if (!Array.isArray(entries))
                                    throw new Error("invalid typed component oracle");
                                return entries.reduce(function (sum, entry) {
                                    if (!Array.isArray(entry) || entry.length !== 2 || typeof entry[0] !== "string")
                                        throw new Error("invalid typed component pair");
                                    return sum + (raw ? Buffer.byteLength(entry[0]) : 0) + semanticBytes(entry[1], entry[0], raw || valueFields.has(entry[0]));
                                }, 0);
                            };
                            for (_a = 0, _b = components.cases; _a < _b.length; _a++) {
                                row = _b[_a];
                                expect(semanticBytes(row.component), row.component.type).toBe(row.bytes);
                            }
                            return [2 /*return*/];
                    }
                });
            }); });
            it("actor arena handback fixture preserves exact fair obligations across word boundaries", function () { return __awaiter(_this, void 0, void 0, function () {
                var readFileSync, Ajv, fixture, schema, validate, counter, independent, carry, index, next, counts, _i, _a, owner, moduleName, module, groupBy, groups, _b, counts_1, _c, slot, count, group, order, cursor, slots, slot, count;
                var _d, _e;
                return __generator(this, function (_f) {
                    switch (_f.label) {
                        case 0: return [4 /*yield*/, Promise.resolve().then(function () { return require("node:fs"); })];
                        case 1:
                            readFileSync = (_f.sent()).readFileSync;
                            return [4 /*yield*/, Promise.resolve().then(function () { return require("ajv"); })];
                        case 2:
                            Ajv = (_f.sent()).default;
                            fixture = JSON.parse(readFileSync(new URL("../../🖱️ui/🧬️contract/♻️retirement/📮️handback/🧫️fixtures/🔣️.json", source.url), "utf8"));
                            schema = JSON.parse(readFileSync(new URL("../../🖱️ui/🧬️contract/♻️retirement/📮️handback/🧬️schema/🔣️.json", source.url), "utf8"));
                            validate = new Ajv({ strict: true }).compile(schema);
                            expect(validate(fixture)).toBe(true);
                            expect(validate(__assign(__assign({}, fixture), { slots: 255 }))).toBe(false);
                            expect(validate(__assign(__assign({}, fixture), { expectedOrder: fixture.expectedOrder.slice(1) }))).toBe(false);
                            expect(validate(__assign(__assign({}, fixture), { emptyReadyBitConsumesNewOwner: true }))).toBe(false);
                            expect(validate(__assign(__assign({}, fixture), { rejectedObligationRetained: false }))).toBe(false);
                            expect(validate(__assign(__assign({}, fixture), { aliasCounter: __assign(__assign({}, fixture.aliasCounter), { afterReturn: "0" }) }))).toBe(false);
                            counter = fixture.aliasCounter;
                            independent = Buffer.alloc(8);
                            independent.writeBigUInt64LE(BigInt(counter.before));
                            carry = 1;
                            for (index = 0; index < independent.length; index++) {
                                next = independent[index] + carry;
                                independent[index] = next & 255;
                                carry = next >>> 8;
                            }
                            expect(independent.readBigUInt64LE().toString()).toBe(counter.afterReturn);
                            expect((BigInt(counter.afterReturn) - 1n).toString()).toBe(counter.afterConsume);
                            expect((Math.pow(2n, 64n) - 1n).toString()).toBe(counter.maximum);
                            counts = new Map();
                            for (_i = 0, _a = fixture.obligations; _i < _a.length; _i++) {
                                owner = _a[_i];
                                counts.set(owner.slot, ((_d = counts.get(owner.slot)) !== null && _d !== void 0 ? _d : 0) + 1);
                            }
                            moduleName = "lodash-es/groupBy.js";
                            return [4 /*yield*/, Promise.resolve("".concat(moduleName)).then(function (s) { return require(s); })];
                        case 3:
                            module = _f.sent();
                            groupBy = module && typeof module === "object" ? Reflect.get(module, "default") : null;
                            if (typeof groupBy !== "function")
                                throw new Error("invalid independent handback grouping oracle");
                            groups = groupBy(fixture.obligations, "slot");
                            if (!groups || typeof groups !== "object")
                                throw new Error("invalid independent handback groups");
                            for (_b = 0, counts_1 = counts; _b < counts_1.length; _b++) {
                                _c = counts_1[_b], slot = _c[0], count = _c[1];
                                group = Reflect.get(groups, String(slot));
                                if (!Array.isArray(group))
                                    throw new Error("invalid independent handback group");
                                expect(group.length).toBe(count);
                            }
                            order = [];
                            cursor = fixture.start;
                            while (counts.size) {
                                slots = __spreadArray([], counts.keys(), true).sort(function (a, b) { return a - b; });
                                slot = (_e = slots.find(function (slot) { return slot >= cursor; })) !== null && _e !== void 0 ? _e : slots[0];
                                count = counts.get(slot);
                                if (count === 1)
                                    counts.delete(slot);
                                else
                                    counts.set(slot, count - 1);
                                order.push(slot);
                                cursor = (slot + 1) % fixture.slots;
                            }
                            expect(order).toEqual(fixture.expectedOrder);
                            return [2 /*return*/];
                    }
                });
            }); });
            it("actor patch storage separates physical placement from semantic retirement grants", function () { return __awaiter(_this, void 0, void 0, function () {
                var readFileSync, Ajv, fixture, schema, validate, _i, _a, invalid, chunkModule, module, chunk, pages, native;
                return __generator(this, function (_b) {
                    switch (_b.label) {
                        case 0: return [4 /*yield*/, Promise.resolve().then(function () { return require("node:fs"); })];
                        case 1:
                            readFileSync = (_b.sent()).readFileSync;
                            return [4 /*yield*/, Promise.resolve().then(function () { return require("ajv"); })];
                        case 2:
                            Ajv = (_b.sent()).default;
                            fixture = JSON.parse(readFileSync(new URL("../../🖱️ui/🧬️contract/♻️retirement/🩹️patch/🧫️fixtures/🔣️.json", source.url), "utf8"));
                            schema = JSON.parse(readFileSync(new URL("../../🖱️ui/🧬️contract/♻️retirement/🩹️patch/🧬️schema/🔣️.json", source.url), "utf8"));
                            validate = new Ajv({ strict: true }).compile(schema);
                            expect(validate(fixture)).toBe(true);
                            for (_i = 0, _a = [
                                __assign(__assign({}, fixture), { logicalCapacity: 128 }),
                                __assign(__assign({}, fixture), { placedBytes: [0, 4096, fixture.native64.operationBytes - 1, fixture.native64.operationBytes, fixture.native64.operationBytes] }),
                                __assign(__assign({}, fixture), { allocationBeforeAdmission: true }),
                                __assign(__assign({}, fixture), { emptyPageStillCharged: false }),
                                __assign(__assign({}, fixture), { cancelMovesPayload: true }),
                                __assign(__assign({}, fixture), { unplaced: __assign(__assign({}, fixture.unplaced), { allocationBytes: fixture.native64.operationBytes }) }),
                            ]; _i < _a.length; _i++) {
                                invalid = _a[_i];
                                expect(validate(invalid)).toBe(false);
                            }
                            chunkModule = "lodash-es/chunk.js";
                            return [4 /*yield*/, Promise.resolve("".concat(chunkModule)).then(function (s) { return require(s); })];
                        case 3:
                            module = _b.sent();
                            chunk = module && typeof module === "object" ? Reflect.get(module, "default") : null;
                            if (typeof chunk !== "function")
                                throw new Error("invalid independent patch paging oracle");
                            pages = chunk(fixture.operations, 1);
                            if (!Array.isArray(pages) || !pages.every(Array.isArray))
                                throw new Error("invalid independent patch pages");
                            expect(pages.map(function (page) { return page.length; })).toEqual(fixture.pageLengths);
                            expect(pages
                                .slice()
                                .reverse()
                                .map(function (page) { return page[0].id; })).toEqual(fixture.retirementOrder);
                            native = fixture.native64;
                            expect(fixture.logicalCapacity * native.descriptorBytes).toBe(native.directoryBytes);
                            expect(native.directoryBytes + native.operationBytes).toBe(native.firstBackingBytes);
                            expect(fixture.placementGrants.map(function (grant) { return (grant >= native.operationBytes ? native.operationBytes : 0); })).toEqual(fixture.placedBytes);
                            expect(native.directoryBytes).toBeLessThanOrEqual(fixture.physicalGrant);
                            expect(native.firstPayloadBytes).toBeLessThanOrEqual(fixture.physicalGrant);
                            expect(native.operationBytes).toBeGreaterThan(Math.max.apply(Math, fixture.semanticGrants));
                            expect(Buffer.byteLength("é".repeat(256))).toBe(fixture.unplaced.semanticBytes);
                            expect(new TextEncoder().encode("é".repeat(256)).length).toBe(fixture.unplaced.semanticBytes);
                            return [2 /*return*/];
                    }
                });
            }); });
            it("actor instance close wire matches strict shared fixtures and an independent LEB128 encoder", function () { return __awaiter(_this, void 0, void 0, function () {
                var readFileSync, Ajv, fixture, schema, ajv, validateFixture, validate, _i, _a, invalid, dropModule, importedDrop, drop, oracleModule, imported, oracle, encoder, u64, _loop_2, _b, _c, row, _loop_3, _d, _e, bytes, _loop_4, _f, _g, activationGeneration, _loop_5, _h, _j, guestLifetime;
                return __generator(this, function (_k) {
                    switch (_k.label) {
                        case 0: return [4 /*yield*/, Promise.resolve().then(function () { return require("node:fs"); })];
                        case 1:
                            readFileSync = (_k.sent()).readFileSync;
                            return [4 /*yield*/, Promise.resolve().then(function () { return require("ajv"); })];
                        case 2:
                            Ajv = (_k.sent()).default;
                            fixture = JSON.parse(readFileSync(new URL("./🧫️fixtures/🔣️.json", source.url), "utf8"));
                            schema = JSON.parse(readFileSync(new URL("./🧬️schema/🔣️.json", source.url), "utf8"));
                            ajv = new Ajv({ strict: true });
                            ajv.addSchema(JSON.parse(readFileSync(new URL("../../🌱️value/🧬️schema/🔣️.json", source.url), "utf8")));
                            ajv.addSchema(schema);
                            validateFixture = ajv.getSchema("".concat(schema.$id, "#/$defs/LifetimeFixture"));
                            expect(validateFixture(fixture), JSON.stringify(validateFixture.errors)).toBe(true);
                            expect(schema.$defs.LifetimeFixture.properties.turnResults.items.properties.hex.maxLength).toBe(fixture.turnResults[0].hex.length + 2 * ACTOR_INSTANCE_LIFECYCLE_MAXIMUM_BYTES);
                            validate = ajv.getSchema("".concat(schema.$id, "#/$defs/Lifetime"));
                            for (_i = 0, _a = ["0", "-1", "01", "18446744073709551616"]; _i < _a.length; _i++) {
                                invalid = _a[_i];
                                expect(validate(__assign(__assign({}, fixture.vectors[0].value), { activationGeneration: invalid }))).toBe(false);
                                expect(validate(__assign(__assign({}, fixture.vectors[1].value), { lifetime: __assign(__assign({}, fixture.vectors[1].value.lifetime), { guestLifetime: invalid }) }))).toBe(false);
                            }
                            expect(validate(__assign(__assign({}, fixture.vectors[0].value), { unexpected: true }))).toBe(false);
                            dropModule = "lodash-es/drop.js";
                            return [4 /*yield*/, Promise.resolve("".concat(dropModule)).then(function (s) { return require(s); })];
                        case 3:
                            importedDrop = _k.sent();
                            if (!importedDrop || typeof importedDrop !== "object")
                                throw new Error("invalid ordered owner oracle module");
                            drop = Reflect.get(importedDrop, "default");
                            if (typeof drop !== "function")
                                throw new Error("invalid ordered owner oracle interface");
                            expect(fixture.publishedClose.complete.map(function (_value, index) { return drop(fixture.publishedClose.owners, index + 1); })).toEqual(fixture.publishedClose.remaining);
                            expect(fixture.publishedClose.remaining.map(function (_value, index) { return index >= fixture.publishedClose.owners.length; })).toEqual(fixture.publishedClose.complete);
                            oracleModule = "@webassemblyjs/leb128/lib/leb.js";
                            return [4 /*yield*/, Promise.resolve("".concat(oracleModule)).then(function (s) { return require(s); })];
                        case 4:
                            imported = _k.sent();
                            if (!imported || typeof imported !== "object")
                                throw new Error("invalid LEB128 oracle module");
                            oracle = Reflect.get(imported, "default");
                            if (!oracle || typeof oracle !== "object")
                                throw new Error("invalid LEB128 oracle interface");
                            encoder = Reflect.get(oracle, "encodeUIntBuffer");
                            if (typeof encoder !== "function")
                                throw new Error("missing LEB128 oracle encoder");
                            u64 = function (value) {
                                var input = Buffer.alloc(8);
                                input.writeBigUInt64LE(value);
                                var output = encoder(input);
                                if (!(output instanceof Uint8Array))
                                    throw new Error("invalid LEB128 oracle bytes");
                                return Array.from(output);
                            };
                            _loop_2 = function (row) {
                                var value = JSON.parse(JSON.stringify(row.value), function (key, field) { return (["activationGeneration", "guestLifetime", "closeGeneration"].includes(key) ? BigInt(field) : field); });
                                var bytes = encodeActorInstanceLifecycle(value);
                                var body = value.kind === "ack" ? value.receipt : value;
                                var tag = value.kind === "ack" ? { captured: 5, accepted: 6, retired: 7 }[value.receipt.kind] : { open: 0, captured: 1, close: 2, accepted: 3, retired: 4 }[value.kind];
                                var expected = body.kind === "open"
                                    ? __spreadArray(__spreadArray(__spreadArray([tag], u64(body.activationGeneration), true), u64(BigInt(body.instanceId)), true), u64(BigInt(body.requestSequence)), true) : __spreadArray(__spreadArray(__spreadArray(__spreadArray(__spreadArray([tag], u64(body.lifetime.activationGeneration), true), u64(BigInt(body.lifetime.instanceId)), true), u64(body.lifetime.guestLifetime), true), u64(BigInt(body.requestSequence)), true), ("closeGeneration" in body ? u64(body.closeGeneration) : []), true);
                                expect(Array.from(bytes)).toEqual(expected);
                                expect(Buffer.from(bytes).toString("hex")).toBe(row.hex);
                                expect(bytes.length).toBeLessThanOrEqual(ACTOR_INSTANCE_LIFECYCLE_MAXIMUM_BYTES);
                                expect(decodeActorInstanceLifecycle(bytes)).toEqual(value);
                                expect(function () { return decodeActorInstanceLifecycle(Uint8Array.from(__spreadArray(__spreadArray([], bytes, true), [0], false))); }).toThrow();
                                var _loop_6 = function (length_1) {
                                    expect(function () { return decodeActorInstanceLifecycle(bytes.subarray(0, length_1)); }).toThrow();
                                };
                                for (var length_1 = 0; length_1 < bytes.length; length_1 += 1) {
                                    _loop_6(length_1);
                                }
                            };
                            for (_b = 0, _c = fixture.vectors; _b < _c.length; _b++) {
                                row = _c[_b];
                                _loop_2(row);
                            }
                            _loop_3 = function (bytes) {
                                expect(function () { return decodeActorInstanceLifecycle(Uint8Array.from(bytes)); }).toThrow();
                            };
                            for (_d = 0, _e = [
                                [8, 1, 7, 9],
                                [0, 0, 7, 9],
                                [0, 0x81, 0, 7, 9],
                                __spreadArray(__spreadArray([0], Array(10).fill(255), true), [7, 9], false),
                                [0, 1, 7, 0],
                                [1, 1, 7, 0, 9],
                                [3, 1, 7, 13, 9, 0],
                                __spreadArray([0, 1, 7], u64(9007199254740992n), true),
                            ]; _d < _e.length; _d++) {
                                bytes = _e[_d];
                                _loop_3(bytes);
                            }
                            _loop_4 = function (activationGeneration) {
                                expect(function () { return encodeActorInstanceLifecycle({ kind: "open", activationGeneration: activationGeneration, instanceId: 7, requestSequence: 8 }); }).toThrow();
                            };
                            for (_f = 0, _g = [0n, -1n, 18446744073709551616n, 1, "1"]; _f < _g.length; _f++) {
                                activationGeneration = _g[_f];
                                _loop_4(activationGeneration);
                            }
                            _loop_5 = function (guestLifetime) {
                                expect(function () { return encodeActorInstanceLifecycle({ kind: "captured", lifetime: { activationGeneration: 1n, instanceId: 7, guestLifetime: guestLifetime }, requestSequence: 8 }); }).toThrow();
                            };
                            for (_h = 0, _j = [0n, -1n, 18446744073709551616n, 13, "13"]; _h < _j.length; _h++) {
                                guestLifetime = _j[_h];
                                _loop_5(guestLifetime);
                            }
                            expect(function () { return encodeActorInstanceLifecycle({ kind: "ack", receipt: { kind: "open", activationGeneration: 1n, instanceId: 7, requestSequence: 8 } }); }).toThrow();
                            return [2 /*return*/];
                    }
                });
            }); });
            it("actor instance close receipts reject reused IDs and premature terminal messages", function () { return __awaiter(_this, void 0, void 0, function () {
                var readFileSync, fixture, prior, current, request, accepted, open, captured;
                return __generator(this, function (_a) {
                    switch (_a.label) {
                        case 0: return [4 /*yield*/, Promise.resolve().then(function () { return require("node:fs"); })];
                        case 1:
                            readFileSync = (_a.sent()).readFileSync;
                            fixture = JSON.parse(readFileSync(new URL("./🧫️fixtures/🔣️.json", source.url), "utf8"));
                            prior = __assign(__assign({}, fixture.reopen.prior), { activationGeneration: BigInt(fixture.reopen.prior.activationGeneration), guestLifetime: BigInt(fixture.reopen.prior.guestLifetime) });
                            current = __assign(__assign({}, fixture.reopen.current), { activationGeneration: BigInt(fixture.reopen.current.activationGeneration), guestLifetime: BigInt(fixture.reopen.current.guestLifetime) });
                            request = { kind: "close", lifetime: current, requestSequence: 9 };
                            accepted = { kind: "accepted", lifetime: current, requestSequence: 9, closeGeneration: 13n };
                            expect(actorInstanceLifetimeEquals(prior, current)).toBe(fixture.reopen.oldRequestAccepted);
                            expect(actorInstanceCloseReceiptMatches(request, null, __assign(__assign({}, accepted), { lifetime: prior }))).toBe(fixture.reopen.oldReceiptAccepted);
                            expect(actorInstanceCloseReceiptMatches(request, null, __assign(__assign({}, accepted), { kind: "retired" }))).toBe(false);
                            expect(actorInstanceCloseReceiptMatches(request, null, accepted)).toBe(true);
                            expect(actorInstanceCloseReceiptMatches(request, accepted, __assign(__assign({}, accepted), { kind: "retired" }))).toBe(true);
                            expect(actorInstanceCloseReceiptMatches(request, accepted, __assign(__assign({}, accepted), { kind: "retired", closeGeneration: 12n }))).toBe(false);
                            expect(actorInstanceCloseReceiptMatches(request, accepted, __assign(__assign({}, accepted), { kind: "retired", requestSequence: 8 }))).toBe(false);
                            open = { kind: "open", activationGeneration: current.activationGeneration, instanceId: current.instanceId, requestSequence: 8 };
                            captured = { kind: "captured", lifetime: current, requestSequence: 8 };
                            expect(actorInstanceCapturedReceiptMatches(open, captured)).toBe(true);
                            expect(actorInstanceCapturedReceiptMatches(open, accepted)).toBe(false);
                            expect(actorInstanceCapturedReceiptMatches(open, __assign(__assign({}, captured), { requestSequence: 7 }))).toBe(false);
                            expect(actorInstanceCapturedReceiptMatches(open, __assign(__assign({}, captured), { lifetime: __assign(__assign({}, current), { activationGeneration: current.activationGeneration + 1n }) }))).toBe(false);
                            expect(actorInstanceLifecycleReceiptEquals(captured, __assign({}, captured))).toBe(true);
                            expect(actorInstanceLifecycleReceiptEquals(captured, __assign(__assign({}, captured), { lifetime: prior }))).toBe(false);
                            expect(actorInstanceLifecycleReceiptEquals(accepted, __assign(__assign({}, accepted), { kind: "retired" }))).toBe(false);
                            expect(actorInstanceLifecycleReceiptEquals(accepted, __assign(__assign({}, accepted), { closeGeneration: 14n }))).toBe(false);
                            return [2 /*return*/];
                    }
                });
            }); });
            it("actor instance close worker activation preserves the new generation across a delayed dispose", function () { return __awaiter(_this, void 0, void 0, function () {
                var readFileSync, materializeSpecifier, shardWorkerSource, fixture, prior, current, Worker, pending, worker, moduleUrl, send, _a, _b, _c, _d, _e, _f, _g;
                return __generator(this, function (_h) {
                    switch (_h.label) {
                        case 0: return [4 /*yield*/, Promise.resolve().then(function () { return require("node:fs"); })];
                        case 1:
                            readFileSync = (_h.sent()).readFileSync;
                            materializeSpecifier = new URL("../../../🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/🏗️materialization/🟦️.ts", source.url).href;
                            return [4 /*yield*/, Promise.resolve("".concat(/* @vite-ignore */ materializeSpecifier)).then(function (s) { return require(s); })];
                        case 2:
                            shardWorkerSource = (_h.sent()).shardWorkerSource;
                            fixture = JSON.parse(readFileSync(new URL("./🧫️fixtures/🔣️.json", source.url), "utf8"));
                            prior = BigInt(fixture.reopen.prior.activationGeneration);
                            current = prior + 1n;
                            return [4 /*yield*/, Promise.resolve().then(function () { return require("node:worker_threads"); })];
                        case 3:
                            Worker = (_h.sent()).Worker;
                            pending = new Map();
                            worker = new Worker("const { parentPort } = require('node:worker_threads'); const self = { postMessage: value => parentPort.postMessage(value), addEventListener: (_, callback) => parentPort.on('message', data => callback({ data })) }; const WebAssembly = { Suspending: function(){}, promising: function(){} };\n" +
                                shardWorkerSource(), { eval: true });
                            worker.on("message", function (message) {
                                if (message.kind !== "result" || !message.requestId)
                                    return;
                                var waiting = pending.get(message.requestId);
                                if (waiting) {
                                    pending.delete(message.requestId);
                                    waiting.resolve(message);
                                }
                            });
                            worker.on("error", function (error) {
                                for (var _i = 0, _a = pending.values(); _i < _a.length; _i++) {
                                    var waiting = _a[_i];
                                    waiting.reject(error);
                                }
                                pending.clear();
                            });
                            moduleUrl = "data:text/javascript," + encodeURIComponent("export async function createActorApi(actorId, activationGeneration) { return { poll: async () => ({ actorId, activationGeneration }) }; }");
                            send = function (data, requestId) {
                                return new Promise(function (resolve, reject) {
                                    pending.set(requestId, { resolve: resolve, reject: reject });
                                    worker.postMessage(data);
                                });
                            };
                            _h.label = 4;
                        case 4:
                            _h.trys.push([4, , 12, 14]);
                            _a = expect;
                            return [4 /*yield*/, send({ kind: "activate", requestId: "a1", actorId: "same", activationGeneration: prior, moduleUrl: moduleUrl, assets: [] }, "a1")];
                        case 5:
                            _a.apply(void 0, [_h.sent()]).toMatchObject({ ok: true });
                            _b = expect;
                            return [4 /*yield*/, send({ kind: "turn", requestId: "t1", actorId: "same", activationGeneration: prior, events: [], budget: {} }, "t1")];
                        case 6:
                            _b.apply(void 0, [(_h.sent()).value]).toEqual({ actorId: "same", activationGeneration: prior });
                            worker.postMessage({ kind: "dispose", actorId: "same", activationGeneration: prior });
                            _c = expect;
                            return [4 /*yield*/, send({ kind: "activate", requestId: "a2", actorId: "same", activationGeneration: current, moduleUrl: moduleUrl, assets: [] }, "a2")];
                        case 7:
                            _c.apply(void 0, [_h.sent()]).toMatchObject({ ok: true });
                            worker.postMessage({ kind: "dispose", actorId: "same", activationGeneration: prior });
                            _d = expect;
                            return [4 /*yield*/, send({ kind: "turn", requestId: "t2", actorId: "same", activationGeneration: current, events: [], budget: {} }, "t2")];
                        case 8:
                            _d.apply(void 0, [(_h.sent()).value]).toEqual({ actorId: "same", activationGeneration: current });
                            _e = expect;
                            return [4 /*yield*/, send({ kind: "frame", requestId: "f-old", actorId: "same", activationGeneration: prior, frame: { kind: "Register", actor: "same" } }, "f-old")];
                        case 9:
                            _e.apply(void 0, [(_h.sent()).error]).toMatch(/actor-lifecycle\.activation-mismatch/);
                            _f = expect;
                            return [4 /*yield*/, send({ kind: "frame", requestId: "f-current", actorId: "same", activationGeneration: current, frame: { kind: "Register", actor: "same" } }, "f-current")];
                        case 10:
                            _f.apply(void 0, [_h.sent()]).toMatchObject({ ok: true });
                            _g = expect;
                            return [4 /*yield*/, send({ kind: "activate", requestId: "old", actorId: "old", activationGeneration: prior, moduleUrl: moduleUrl, assets: [] }, "old")];
                        case 11:
                            _g.apply(void 0, [(_h.sent()).ok]).toBe(false);
                            worker.postMessage({ kind: "dispose", actorId: "same", activationGeneration: current });
                            return [3 /*break*/, 14];
                        case 12: return [4 /*yield*/, worker.terminate()];
                        case 13:
                            _h.sent();
                            return [7 /*endfinally*/];
                        case 14: return [2 /*return*/];
                    }
                });
            }); });
            return [2 /*return*/];
        });
    });
}
