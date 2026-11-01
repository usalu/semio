//! 📚️ Canonical ADD2 executable syntax decoded once from the committed IFCJ resource `program.ifcj`, whose neutral form is `program.json` (the compiled program of `../compiler.ts`).
use super::ast::{ExpressAggregateEntry, ExpressCase, ExpressExpression, ExpressExpressionKind, ExpressFunction, ExpressGlobalRule, ExpressInverse, ExpressProgram, ExpressRule, ExpressStatement, ExpressStatementKind, ExpressVariable};
use semio_s_artifact_stdio_ifc_json_pack::{with_json_pack, Json, JsonPackError, Leaker};
use std::sync::OnceLock;

static PROGRAM: OnceLock<ExpressProgram> = OnceLock::new();
const RESOURCE: &[u8] = include_bytes!("program.ifcj");

struct Builder {
    leak: Leaker,
}

impl Builder {
    fn expression(&mut self, json: &Json<'_>) -> Result<ExpressExpression, JsonPackError> {
        let fields = json.object(&["kind", "offset", "text", "operator", "name", "operands", "entries"])?;
        let kind = match fields.require("kind")?.string()? {
            "integer" => ExpressExpressionKind::Integer,
            "real" => ExpressExpressionKind::Real,
            "string" => ExpressExpressionKind::String,
            "logical" => ExpressExpressionKind::Logical,
            "unknown" => ExpressExpressionKind::Unknown,
            "identifier" => ExpressExpressionKind::Identifier,
            "unary" => ExpressExpressionKind::Unary,
            "binary" => ExpressExpressionKind::Binary,
            "attribute" => ExpressExpressionKind::Attribute,
            "qualify" => ExpressExpressionKind::Qualify,
            "call" => ExpressExpressionKind::Call,
            "index" => ExpressExpressionKind::Index,
            "slice" => ExpressExpressionKind::Slice,
            "aggregate" => ExpressExpressionKind::Aggregate,
            "query" => ExpressExpressionKind::Query,
            "interval" => ExpressExpressionKind::Interval,
            _ => return Err(JsonPackError::Shape("expression kind")),
        };
        Ok(ExpressExpression {
            kind,
            offset: fields.require("offset")?.index()?,
            text: self.leak.optional(fields.get("text"))?,
            operator: self.leak.optional(fields.get("operator"))?,
            name: self.leak.optional(fields.get("name"))?,
            operands: self.expressions(fields.get("operands"))?,
            entries: self.entries(fields.get("entries"))?,
        })
    }

    fn expressions(&mut self, json: Option<&Json<'_>>) -> Result<&'static [ExpressExpression], JsonPackError> {
        let Some(json) = json else { return Ok(&[]) };
        Ok(Leaker::slice(json.array()?.iter().map(|item| self.expression(item)).collect::<Result<Vec<_>, _>>()?))
    }

    fn shared(&mut self, json: &Json<'_>) -> Result<&'static ExpressExpression, JsonPackError> {
        Ok(Box::leak(Box::new(self.expression(json)?)))
    }

    fn optional_shared(&mut self, json: Option<&Json<'_>>) -> Result<Option<&'static ExpressExpression>, JsonPackError> {
        json.map(|item| self.shared(item)).transpose()
    }

    fn entries(&mut self, json: Option<&Json<'_>>) -> Result<&'static [ExpressAggregateEntry], JsonPackError> {
        let Some(json) = json else { return Ok(&[]) };
        let mut rows = Vec::new();
        for item in json.array()? {
            let fields = item.object(&["value", "repeat"])?;
            rows.push(ExpressAggregateEntry { value: self.shared(fields.require("value")?)?, repeat: self.optional_shared(fields.get("repeat"))? });
        }
        Ok(Leaker::slice(rows))
    }

    fn statements(&mut self, json: Option<&Json<'_>>) -> Result<&'static [ExpressStatement], JsonPackError> {
        let Some(json) = json else { return Ok(&[]) };
        Ok(Leaker::slice(json.array()?.iter().map(|item| self.statement(item)).collect::<Result<Vec<_>, _>>()?))
    }

    fn statement(&mut self, json: &Json<'_>) -> Result<ExpressStatement, JsonPackError> {
        let fields = json.object(&["kind", "offset", "name", "expressions", "body", "otherwise", "cases", "while", "until"])?;
        let kind = match fields.require("kind")?.string()? {
            "assign" => ExpressStatementKind::Assign,
            "return" => ExpressStatementKind::Return,
            "if" => ExpressStatementKind::If,
            "case" => ExpressStatementKind::Case,
            "repeat" => ExpressStatementKind::Repeat,
            "block" => ExpressStatementKind::Block,
            "escape" => ExpressStatementKind::Escape,
            "skip" => ExpressStatementKind::Skip,
            "call" => ExpressStatementKind::Call,
            "null" => ExpressStatementKind::Null,
            _ => return Err(JsonPackError::Shape("statement kind")),
        };
        let mut cases = Vec::new();
        if let Some(rows) = fields.get("cases") {
            for row in rows.array()? {
                let case = row.object(&["labels", "body"])?;
                cases.push(ExpressCase { labels: self.expressions(case.get("labels"))?, body: self.statements(case.get("body"))? });
            }
        }
        Ok(ExpressStatement {
            kind,
            offset: fields.require("offset")?.index()?,
            name: self.leak.optional(fields.get("name"))?,
            expressions: self.expressions(fields.get("expressions"))?,
            body: self.statements(fields.get("body"))?,
            otherwise: self.statements(fields.get("otherwise"))?,
            cases: Leaker::slice(cases),
            while_condition: self.optional_shared(fields.get("while"))?,
            until_condition: self.optional_shared(fields.get("until"))?,
        })
    }

    fn variables(&mut self, json: Option<&Json<'_>>) -> Result<&'static [ExpressVariable], JsonPackError> {
        let Some(json) = json else { return Ok(&[]) };
        let mut rows = Vec::new();
        for item in json.array()? {
            let fields = item.object(&["names", "type", "initial"])?;
            rows.push(ExpressVariable { names: self.leak.texts(fields.get("names"))?, type_source: self.leak.text(fields.require("type")?.string()?), initial: self.optional_shared(fields.get("initial"))? });
        }
        Ok(Leaker::slice(rows))
    }

    fn rules(&mut self, json: Option<&Json<'_>>) -> Result<&'static [ExpressRule], JsonPackError> {
        let Some(json) = json else { return Ok(&[]) };
        let mut rows = Vec::new();
        for item in json.array()? {
            let fields = item.object(&["owner", "label", "expression"])?;
            rows.push(ExpressRule { owner: self.leak.text(fields.require("owner")?.string()?), label: self.leak.text(fields.require("label")?.string()?), expression: self.shared(fields.require("expression")?)? });
        }
        Ok(Leaker::slice(rows))
    }

    fn functions(&mut self, json: &Json<'_>) -> Result<&'static [ExpressFunction], JsonPackError> {
        let mut rows = Vec::new();
        for item in json.array()? {
            let fields = item.object(&["name", "parameters", "resultType", "locals", "body"])?;
            rows.push(ExpressFunction {
                name: self.leak.text(fields.require("name")?.string()?),
                parameters: self.variables(fields.get("parameters"))?,
                result_type: self.leak.text(fields.require("resultType")?.string()?),
                locals: self.variables(fields.get("locals"))?,
                body: self.statements(fields.get("body"))?,
            });
        }
        Ok(Leaker::slice(rows))
    }

    fn global_rules(&mut self, json: &Json<'_>) -> Result<&'static [ExpressGlobalRule], JsonPackError> {
        let mut rows = Vec::new();
        for item in json.array()? {
            let fields = item.object(&["name", "populations", "locals", "body", "rules"])?;
            rows.push(ExpressGlobalRule {
                name: self.leak.text(fields.require("name")?.string()?),
                populations: self.leak.texts(fields.get("populations"))?,
                locals: self.variables(fields.get("locals"))?,
                body: self.statements(fields.get("body"))?,
                rules: self.rules(fields.get("rules"))?,
            });
        }
        Ok(Leaker::slice(rows))
    }

    fn inverse(&mut self, json: &Json<'_>) -> Result<&'static [ExpressInverse], JsonPackError> {
        let mut rows = Vec::new();
        for item in json.array()? {
            let fields = item.object(&["owner", "name", "type", "attribute", "aggregate", "lower", "upper"])?;
            rows.push(ExpressInverse {
                owner: self.leak.text(fields.require("owner")?.string()?),
                name: self.leak.text(fields.require("name")?.string()?),
                entity_type: self.leak.text(fields.require("type")?.string()?),
                attribute: self.leak.text(fields.require("attribute")?.string()?),
                aggregate: fields.get("aggregate").map(Json::boolean).transpose()?.unwrap_or(false),
                lower: self.optional_shared(fields.get("lower"))?,
                upper: self.optional_shared(fields.get("upper"))?,
            });
        }
        Ok(Leaker::slice(rows))
    }

    fn program(&mut self, json: &Json<'_>) -> Result<ExpressProgram, JsonPackError> {
        let fields = json.object(&["schema", "functions", "globalRules", "rules", "derived", "unique", "bounds", "inverse"])?;
        Ok(ExpressProgram {
            schema: self.leak.text(fields.require("schema")?.string()?),
            functions: self.functions(fields.require("functions")?)?,
            global_rules: self.global_rules(fields.require("globalRules")?)?,
            rules: self.rules(Some(fields.require("rules")?))?,
            derived: self.rules(Some(fields.require("derived")?))?,
            unique: self.rules(Some(fields.require("unique")?))?,
            bounds: self.rules(Some(fields.require("bounds")?))?,
            inverse: self.inverse(fields.require("inverse")?)?,
        })
    }
}

/// 📥️ Decode a committed program resource into immutable statics; every refusal is a closed cause.
pub fn decode_program(pack: &[u8]) -> Result<ExpressProgram, JsonPackError> {
    with_json_pack(pack, |root| Builder { leak: Leaker::default() }.program(root))
}

/// 📖️ Borrow the complete precompiled normative syntax; the first call decodes the embedded resource once, which the per-resource laws prove cannot be refused (a refusal is a corrupted build artifact).
pub fn ifc4x3_express_program() -> &'static ExpressProgram {
    PROGRAM.get_or_init(|| match decode_program(RESOURCE) {
        Ok(program) => program,
        Err(error) => panic!("embedded IFC4x3 program resource refused: {error:?}"),
    })
}
