//! 🧮️ Spreadsheet formula calculation over the projected workbook: a pure parser, evaluator and recalculation that follow Excel's coercion, comparison and error rules.
//! A formula that sits on a reference cycle (or has no expression text) has no computed value, so its stored cache is left alone.

use crate::standards::v_ecma_376::subsets::base::schema::snapshot::{XlsxCellValue, XlsxWorkbook};
use crate::standards::v_ecma_376::subsets::base::schema::vocabulary::{column_index, XLSX_MAX_COLUMN, XLSX_MAX_ROW};
use std::cell::RefCell;
use std::cmp::Ordering;
use std::collections::{BTreeMap, HashMap, HashSet};

//#region 🔖️Result
/// 🧮️ One formula cell whose freshly computed cache differs from the stored one.
#[derive(Clone, Debug, PartialEq)]
pub struct XlsxRecalculatedCell {
    pub sheet: String,
    pub row: u32,
    pub col: u32,
    pub expr: String,
    pub cached: XlsxCellValue,
}

/// 🧮️ Computes every formula cell of `workbook` and returns those whose computed cache differs from the stored cache, in sheet then storage order.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn recalculate_workbook(workbook: &XlsxWorkbook) -> Vec<XlsxRecalculatedCell> {
    let calc = Calc::new(workbook);
    let mut changed = Vec::new();
    for (sheet_index, sheet) in workbook.sheets.iter().enumerate() {
        for cell in &sheet.cells {
            let XlsxCellValue::Formula { expr, cached } = &cell.value else { continue };
            let Ok(value) = calc.formula_value(sheet_index, cell.row, cell.col) else { continue };
            let computed = value.into_cached();
            if !calc.same_cache(cached.as_deref(), &computed) {
                changed.push(XlsxRecalculatedCell { sheet: sheet.name.clone(), row: cell.row, col: cell.col, expr: expr.clone(), cached: computed });
            }
        }
    }
    changed
}
//#endregion 🔖️Result

//#region 🔖️Values
#[derive(Clone, Debug, PartialEq)]
enum Val {
    Num(f64),
    Text(String),
    Bool(bool),
    Empty,
}

#[derive(Clone, Debug, PartialEq)]
enum Fail {
    Excel(&'static str),
    Unresolved,
}

type Eval<T> = Result<T, Fail>;

const DIV0: Fail = Fail::Excel("#DIV/0!");
const VALUE: Fail = Fail::Excel("#VALUE!");
const NAME: Fail = Fail::Excel("#NAME?");
const REF: Fail = Fail::Excel("#REF!");
const NUM: Fail = Fail::Excel("#NUM!");

impl Val {
    fn into_cached(self) -> XlsxCellValue {
        match self {
            Val::Num(number) => XlsxCellValue::Number(if number == 0.0 { 0.0 } else { number }),
            Val::Text(text) => XlsxCellValue::InlineString(text),
            Val::Bool(flag) => XlsxCellValue::Boolean(flag),
            Val::Empty => XlsxCellValue::Number(0.0),
        }
    }

    fn number(&self) -> Eval<f64> {
        match self {
            Val::Num(number) => Ok(*number),
            Val::Bool(flag) => Ok(f64::from(u8::from(*flag))),
            Val::Empty => Ok(0.0),
            Val::Text(text) => parse_number(text).ok_or(VALUE),
        }
    }

    fn text(&self) -> String {
        match self {
            Val::Num(number) => format_number(*number),
            Val::Text(text) => text.clone(),
            Val::Bool(flag) => if *flag { "TRUE" } else { "FALSE" }.into(),
            Val::Empty => String::new(),
        }
    }

    fn boolean(&self) -> Eval<bool> {
        match self {
            Val::Num(number) => Ok(*number != 0.0),
            Val::Bool(flag) => Ok(*flag),
            Val::Empty => Ok(false),
            Val::Text(text) if text.eq_ignore_ascii_case("true") => Ok(true),
            Val::Text(text) if text.eq_ignore_ascii_case("false") => Ok(false),
            Val::Text(_) => Err(VALUE),
        }
    }
}

fn parse_number(text: &str) -> Option<f64> {
    let trimmed = text.trim();
    let first = trimmed.chars().next()?;
    if !(first.is_ascii_digit() || matches!(first, '-' | '+' | '.')) {
        return None;
    }
    trimmed.parse::<f64>().ok().filter(|number| number.is_finite())
}

fn format_number(number: f64) -> String {
    format!("{number}")
}

fn finite(number: f64) -> Eval<Val> {
    if number.is_finite() { Ok(Val::Num(number)) } else { Err(NUM) }
}
//#endregion 🔖️Values

//#region 🔖️Syntax
#[derive(Clone, Debug, PartialEq)]
struct Cell {
    sheet: Option<String>,
    row: u32,
    col: u32,
}

#[derive(Clone, Debug, PartialEq)]
enum Expr {
    Num(f64),
    Str(String),
    Bool(bool),
    Error(&'static str),
    Cell(Cell),
    Range(Cell, Cell),
    Name,
    Neg(Box<Expr>),
    Percent(Box<Expr>),
    Binary(char, Box<Expr>, Box<Expr>),
    Compare(&'static str, Box<Expr>, Box<Expr>),
    Call(String, Vec<Option<Expr>>),
}

const ERROR_LITERALS: [&str; 7] = ["#DIV/0!", "#N/A", "#NAME?", "#NULL!", "#NUM!", "#REF!", "#VALUE!"];

struct Parser {
    chars: Vec<char>,
    at: usize,
}

impl Parser {
    fn peek(&self) -> Option<char> {
        self.chars.get(self.at).copied()
    }

    fn skip(&mut self) {
        while self.peek().is_some_and(char::is_whitespace) {
            self.at += 1;
        }
    }

    fn eat(&mut self, text: &str) -> bool {
        self.skip();
        let wanted: Vec<char> = text.chars().collect();
        if self.chars[self.at..].starts_with(&wanted) {
            self.at += wanted.len();
            return true;
        }
        false
    }

    fn expression(&mut self) -> Option<Expr> {
        let mut left = self.concatenation()?;
        loop {
            let Some(operator) = ["<>", "<=", ">=", "=", "<", ">"].into_iter().find(|operator| self.eat(operator)) else { return Some(left) };
            let right = self.concatenation()?;
            left = Expr::Compare(operator, Box::new(left), Box::new(right));
        }
    }

    fn concatenation(&mut self) -> Option<Expr> {
        let mut left = self.additive()?;
        while self.eat("&") {
            left = Expr::Binary('&', Box::new(left), Box::new(self.additive()?));
        }
        Some(left)
    }

    fn additive(&mut self) -> Option<Expr> {
        let mut left = self.multiplicative()?;
        loop {
            let operator = if self.eat("+") { '+' } else if self.eat("-") { '-' } else { return Some(left) };
            left = Expr::Binary(operator, Box::new(left), Box::new(self.multiplicative()?));
        }
    }

    fn multiplicative(&mut self) -> Option<Expr> {
        let mut left = self.power()?;
        loop {
            let operator = if self.eat("*") { '*' } else if self.eat("/") { '/' } else { return Some(left) };
            left = Expr::Binary(operator, Box::new(left), Box::new(self.power()?));
        }
    }

    fn power(&mut self) -> Option<Expr> {
        let mut left = self.prefix()?;
        while self.eat("^") {
            left = Expr::Binary('^', Box::new(left), Box::new(self.prefix()?));
        }
        Some(left)
    }

    fn prefix(&mut self) -> Option<Expr> {
        if self.eat("-") {
            return Some(Expr::Neg(Box::new(self.prefix()?)));
        }
        if self.eat("+") {
            return self.prefix();
        }
        let mut operand = self.primary()?;
        while self.eat("%") {
            operand = Expr::Percent(Box::new(operand));
        }
        Some(operand)
    }

    fn primary(&mut self) -> Option<Expr> {
        self.skip();
        match self.peek()? {
            '(' => {
                self.at += 1;
                let inner = self.expression()?;
                self.eat(")").then_some(inner)
            }
            '"' => self.string().map(Expr::Str),
            '#' => {
                let rest: String = self.chars[self.at..].iter().collect();
                let literal = ERROR_LITERALS.into_iter().find(|literal| rest.to_ascii_uppercase().starts_with(literal))?;
                self.at += literal.len();
                Some(Expr::Error(literal))
            }
            '\'' => {
                let sheet = self.quoted_sheet()?;
                self.reference(Some(sheet))
            }
            '0'..='9' | '.' => self.number(),
            _ => self.word(),
        }
    }

    fn string(&mut self) -> Option<String> {
        self.at += 1;
        let mut text = String::new();
        loop {
            let character = self.peek()?;
            self.at += 1;
            if character != '"' {
                text.push(character);
            } else if self.peek() == Some('"') {
                self.at += 1;
                text.push('"');
            } else {
                return Some(text);
            }
        }
    }

    fn quoted_sheet(&mut self) -> Option<String> {
        self.at += 1;
        let mut name = String::new();
        loop {
            let character = self.peek()?;
            self.at += 1;
            if character != '\'' {
                name.push(character);
            } else if self.peek() == Some('\'') {
                self.at += 1;
                name.push('\'');
            } else {
                break;
            }
        }
        (self.peek() == Some('!')).then(|| self.at += 1).map(|()| name)
    }

    fn number(&mut self) -> Option<Expr> {
        let start = self.at;
        while self.peek().is_some_and(|c| c.is_ascii_digit() || c == '.') {
            self.at += 1;
        }
        if matches!(self.peek(), Some('e' | 'E')) {
            let mark = self.at;
            self.at += 1;
            if matches!(self.peek(), Some('+' | '-')) {
                self.at += 1;
            }
            if self.peek().is_some_and(|c| c.is_ascii_digit()) {
                while self.peek().is_some_and(|c| c.is_ascii_digit()) {
                    self.at += 1;
                }
            } else {
                self.at = mark;
            }
        }
        self.chars[start..self.at].iter().collect::<String>().parse().ok().map(Expr::Num)
    }

    fn run(&mut self) -> String {
        let start = self.at;
        while self.peek().is_some_and(|c| c.is_alphanumeric() || matches!(c, '_' | '.' | '$')) {
            self.at += 1;
        }
        self.chars[start..self.at].iter().collect()
    }

    fn word(&mut self) -> Option<Expr> {
        let word = self.run();
        if word.is_empty() {
            return None;
        }
        match self.peek() {
            Some('!') => {
                self.at += 1;
                self.reference(Some(word))
            }
            Some('(') => {
                self.at += 1;
                self.arguments().map(|arguments| Expr::Call(word.to_ascii_uppercase(), arguments))
            }
            _ => match cell_of(&word, None) {
                Some(cell) => self.range_from(cell),
                None if word.eq_ignore_ascii_case("TRUE") => Some(Expr::Bool(true)),
                None if word.eq_ignore_ascii_case("FALSE") => Some(Expr::Bool(false)),
                None => Some(Expr::Name),
            },
        }
    }

    fn reference(&mut self, sheet: Option<String>) -> Option<Expr> {
        let word = self.run();
        let cell = cell_of(&word, sheet)?;
        self.range_from(cell)
    }

    fn range_from(&mut self, first: Cell) -> Option<Expr> {
        if self.peek() != Some(':') {
            return Some(Expr::Cell(first));
        }
        self.at += 1;
        let word = self.run();
        Some(Expr::Range(first, cell_of(&word, None)?))
    }

    fn arguments(&mut self) -> Option<Vec<Option<Expr>>> {
        let mut arguments = Vec::new();
        if self.eat(")") {
            return Some(arguments);
        }
        loop {
            self.skip();
            arguments.push(if matches!(self.peek(), Some(',' | ')')) { None } else { Some(self.expression()?) });
            if self.eat(")") {
                return Some(arguments);
            }
            if !self.eat(",") {
                return None;
            }
        }
    }
}

fn cell_of(word: &str, sheet: Option<String>) -> Option<Cell> {
    let bare = word.replacen('$', "", 2);
    let letters = bare.trim_end_matches(|c: char| c.is_ascii_digit());
    let digits = &bare[letters.len()..];
    let col = column_index(letters).filter(|col| letters.len() <= 3 && *col <= XLSX_MAX_COLUMN)?;
    let row = digits.parse::<u32>().ok().filter(|row| (1..=XLSX_MAX_ROW).contains(row))?;
    Some(Cell { sheet, row, col })
}

fn parse_formula(text: &str) -> Option<Expr> {
    let mut parser = Parser { chars: text.chars().collect(), at: 0 };
    let expression = parser.expression()?;
    parser.skip();
    (parser.at == parser.chars.len()).then_some(expression)
}
//#endregion 🔖️Syntax

//#region 🔖️Evaluation
enum Operand {
    Value(Val),
    Area { sheet: usize, rows: (u32, u32), cols: (u32, u32) },
}

struct Calc<'a> {
    workbook: &'a XlsxWorkbook,
    cells: Vec<BTreeMap<(u32, u32), &'a XlsxCellValue>>,
    memo: RefCell<HashMap<(usize, u32, u32), Eval<Val>>>,
    visiting: RefCell<HashSet<(usize, u32, u32)>>,
}

impl<'a> Calc<'a> {
    fn new(workbook: &'a XlsxWorkbook) -> Self {
        let cells = workbook.sheets.iter().map(|sheet| sheet.cells.iter().map(|cell| ((cell.row, cell.col), &cell.value)).collect()).collect();
        Self { workbook, cells, memo: RefCell::default(), visiting: RefCell::default() }
    }

    fn shared(&self, index: usize) -> String {
        self.workbook.shared_strings.get(index).cloned().unwrap_or_else(|| format!("#{index}"))
    }

    fn same_cache(&self, stored: Option<&XlsxCellValue>, computed: &XlsxCellValue) -> bool {
        match (stored, computed) {
            (Some(XlsxCellValue::Number(left)), XlsxCellValue::Number(right)) => (left - right).abs() <= 1e-12 * left.abs().max(right.abs()).max(1.0),
            (Some(XlsxCellValue::SharedString(index)), XlsxCellValue::InlineString(text)) => self.shared(*index) == *text,
            (Some(stored), computed) => stored == computed,
            (None, _) => false,
        }
    }

    fn sheet_index(&self, name: &str) -> Eval<usize> {
        self.workbook.sheets.iter().position(|sheet| sheet.name.eq_ignore_ascii_case(name)).ok_or(REF)
    }

    fn formula_value(&self, sheet: usize, row: u32, col: u32) -> Eval<Val> {
        let key = (sheet, row, col);
        if let Some(done) = self.memo.borrow().get(&key) {
            return done.clone();
        }
        if !self.visiting.borrow_mut().insert(key) {
            return Err(Fail::Unresolved);
        }
        let result = match self.cells[sheet].get(&(row, col)) {
            Some(XlsxCellValue::Formula { expr, .. }) if !expr.trim().is_empty() => match parse_formula(expr) {
                Some(parsed) => self.scalar(sheet, &parsed),
                None => Err(VALUE),
            },
            _ => Err(Fail::Unresolved),
        };
        self.visiting.borrow_mut().remove(&key);
        self.memo.borrow_mut().insert(key, result.clone());
        result
    }

    fn cell_value(&self, sheet: usize, row: u32, col: u32) -> Eval<Val> {
        Ok(match self.cells[sheet].get(&(row, col)) {
            None | Some(XlsxCellValue::Empty) => Val::Empty,
            Some(XlsxCellValue::Number(number)) => Val::Num(*number),
            Some(XlsxCellValue::SharedString(index)) => Val::Text(self.shared(*index)),
            Some(XlsxCellValue::InlineString(text)) => Val::Text(text.clone()),
            Some(XlsxCellValue::Boolean(flag)) => Val::Bool(*flag),
            Some(XlsxCellValue::Error(error)) => return Err(Fail::Excel(ERROR_LITERALS.into_iter().find(|literal| literal.eq_ignore_ascii_case(error)).unwrap_or("#VALUE!"))),
            Some(XlsxCellValue::Formula { .. }) => return self.formula_value(sheet, row, col),
        })
    }

    fn area_cells(&self, sheet: usize, rows: (u32, u32), cols: (u32, u32)) -> Vec<Eval<Val>> {
        let keys: Vec<(u32, u32)> = self.cells[sheet].range((rows.0, 0)..=(rows.1, u32::MAX)).map(|(key, _)| *key).filter(|(_, col)| (cols.0..=cols.1).contains(col)).collect();
        keys.into_iter().map(|(row, col)| self.cell_value(sheet, row, col)).collect()
    }

    fn operand(&self, home: usize, expr: &Expr) -> Eval<Operand> {
        let area = |first: &Cell, last: &Cell| -> Eval<Operand> {
            let sheet = first.sheet.as_deref().map_or(Ok(home), |name| self.sheet_index(name))?;
            Ok(Operand::Area { sheet, rows: (first.row.min(last.row), first.row.max(last.row)), cols: (first.col.min(last.col), first.col.max(last.col)) })
        };
        match expr {
            Expr::Cell(cell) => area(cell, cell),
            Expr::Range(first, last) => area(first, &Cell { sheet: first.sheet.clone(), ..last.clone() }),
            other => self.scalar(home, other).map(Operand::Value),
        }
    }

    fn value_of(&self, operand: Operand) -> Eval<Val> {
        match operand {
            Operand::Value(value) => Ok(value),
            Operand::Area { sheet, rows: (row, last_row), cols: (col, last_col) } if row == last_row && col == last_col => self.cell_value(sheet, row, col),
            Operand::Area { .. } => Err(VALUE),
        }
    }

    fn scalar(&self, home: usize, expr: &Expr) -> Eval<Val> {
        let number = |expr: &Expr| self.scalar(home, expr).and_then(|value| value.number());
        match expr {
            Expr::Num(number) => Ok(Val::Num(*number)),
            Expr::Str(text) => Ok(Val::Text(text.clone())),
            Expr::Bool(flag) => Ok(Val::Bool(*flag)),
            Expr::Error(error) => Err(Fail::Excel(error)),
            Expr::Name => Err(NAME),
            Expr::Cell(_) | Expr::Range(..) => self.operand(home, expr).and_then(|operand| self.value_of(operand)),
            Expr::Neg(inner) => finite(-number(inner)?),
            Expr::Percent(inner) => finite(number(inner)? / 100.0),
            Expr::Binary('&', left, right) => Ok(Val::Text(self.scalar(home, left)?.text() + &self.scalar(home, right)?.text())),
            Expr::Binary(operator, left, right) => {
                let (left, right) = (number(left)?, number(right)?);
                match operator {
                    '+' => finite(left + right),
                    '-' => finite(left - right),
                    '*' => finite(left * right),
                    '/' if right == 0.0 => Err(DIV0),
                    '/' => finite(left / right),
                    _ if left == 0.0 && right == 0.0 => Err(NUM),
                    _ => finite(left.powf(right)),
                }
            }
            Expr::Compare(operator, left, right) => {
                let ordering = compare(&self.scalar(home, left)?, &self.scalar(home, right)?);
                Ok(Val::Bool(match *operator {
                    "=" => ordering == Ordering::Equal,
                    "<>" => ordering != Ordering::Equal,
                    "<" => ordering == Ordering::Less,
                    ">" => ordering == Ordering::Greater,
                    "<=" => ordering != Ordering::Greater,
                    _ => ordering != Ordering::Less,
                }))
            }
            Expr::Call(name, arguments) => self.call(home, name, arguments),
        }
    }

    fn argument(&self, home: usize, arguments: &[Option<Expr>], position: usize) -> Eval<Val> {
        match arguments.get(position) {
            Some(Some(expr)) => self.scalar(home, expr),
            Some(None) => Ok(Val::Empty),
            None => Err(VALUE),
        }
    }

    fn numbers(&self, home: usize, arguments: &[Option<Expr>]) -> Eval<Vec<f64>> {
        let mut numbers = Vec::new();
        for argument in arguments {
            match argument.as_ref().map(|expr| self.operand(home, expr)).transpose()? {
                None => numbers.push(0.0),
                Some(Operand::Value(value)) => numbers.push(value.number()?),
                Some(Operand::Area { sheet, rows, cols }) => {
                    for value in self.area_cells(sheet, rows, cols) {
                        if let Val::Num(number) = value? {
                            numbers.push(number);
                        }
                    }
                }
            }
        }
        Ok(numbers)
    }

    fn texts(&self, home: usize, arguments: &[Option<Expr>]) -> Eval<String> {
        let mut joined = String::new();
        for argument in arguments {
            match argument.as_ref().map(|expr| self.operand(home, expr)).transpose()? {
                None => {}
                Some(Operand::Value(value)) => joined += &value.text(),
                Some(Operand::Area { sheet, rows, cols }) => {
                    for value in self.area_cells(sheet, rows, cols) {
                        joined += &value?.text();
                    }
                }
            }
        }
        Ok(joined)
    }

    fn booleans(&self, home: usize, arguments: &[Option<Expr>]) -> Eval<Vec<bool>> {
        let mut flags = Vec::new();
        for argument in arguments {
            match argument.as_ref().map(|expr| self.operand(home, expr)).transpose()? {
                None => {}
                Some(Operand::Value(value)) => flags.push(value.boolean()?),
                Some(Operand::Area { sheet, rows, cols }) => {
                    for value in self.area_cells(sheet, rows, cols) {
                        match value? {
                            Val::Num(number) => flags.push(number != 0.0),
                            Val::Bool(flag) => flags.push(flag),
                            Val::Text(_) | Val::Empty => {}
                        }
                    }
                }
            }
        }
        if flags.is_empty() { Err(VALUE) } else { Ok(flags) }
    }

    fn count(&self, home: usize, arguments: &[Option<Expr>], all: bool) -> Eval<Val> {
        let mut count = 0usize;
        for argument in arguments {
            match argument.as_ref().map(|expr| self.operand(home, expr)).transpose() {
                Ok(None) => count += usize::from(all),
                Ok(Some(Operand::Value(value))) => count += usize::from(all || value.number().is_ok()),
                Ok(Some(Operand::Area { sheet, rows, cols })) => {
                    let values = self.area_cells(sheet, rows, cols);
                    if values.contains(&Err(Fail::Unresolved)) {
                        return Err(Fail::Unresolved);
                    }
                    count += values.into_iter().filter(|value| match value {
                        Ok(Val::Num(_)) => true,
                        Ok(Val::Empty) => false,
                        _ => all,
                    }).count();
                }
                Err(Fail::Excel(_)) => count += usize::from(all),
                Err(Fail::Unresolved) => return Err(Fail::Unresolved),
            }
        }
        Ok(Val::Num(count as f64))
    }

    fn call(&self, home: usize, name: &str, arguments: &[Option<Expr>]) -> Eval<Val> {
        let arity = |minimum: usize, maximum: usize| if (minimum..=maximum).contains(&arguments.len()) { Ok(()) } else { Err(VALUE) };
        let text = |position: usize| self.argument(home, arguments, position).map(|value| value.text());
        match name {
            "SUM" => finite(self.numbers(home, arguments)?.iter().sum()),
            "PRODUCT" => {
                let numbers = self.numbers(home, arguments)?;
                finite(if numbers.is_empty() { 0.0 } else { numbers.iter().product() })
            }
            "AVERAGE" => {
                let numbers = self.numbers(home, arguments)?;
                if numbers.is_empty() { Err(DIV0) } else { finite(numbers.iter().sum::<f64>() / numbers.len() as f64) }
            }
            "MIN" => Ok(Val::Num(self.numbers(home, arguments)?.into_iter().reduce(f64::min).unwrap_or(0.0))),
            "MAX" => Ok(Val::Num(self.numbers(home, arguments)?.into_iter().reduce(f64::max).unwrap_or(0.0))),
            "COUNT" => self.count(home, arguments, false),
            "COUNTA" => self.count(home, arguments, true),
            "IF" => {
                arity(2, 3)?;
                match (self.argument(home, arguments, 0)?.boolean()?, arguments.get(2)) {
                    (true, _) => self.argument(home, arguments, 1),
                    (false, Some(_)) => self.argument(home, arguments, 2),
                    (false, None) => Ok(Val::Bool(false)),
                }
            }
            "IFERROR" => {
                arity(2, 2)?;
                match self.argument(home, arguments, 0) {
                    Err(Fail::Excel(_)) => self.argument(home, arguments, 1),
                    other => other,
                }
            }
            "AND" => Ok(Val::Bool(self.booleans(home, arguments)?.into_iter().all(|flag| flag))),
            "OR" => Ok(Val::Bool(self.booleans(home, arguments)?.into_iter().any(|flag| flag))),
            "NOT" => {
                arity(1, 1)?;
                Ok(Val::Bool(!self.argument(home, arguments, 0)?.boolean()?))
            }
            "ABS" => {
                arity(1, 1)?;
                Ok(Val::Num(self.argument(home, arguments, 0)?.number()?.abs()))
            }
            "ROUND" => {
                arity(2, 2)?;
                Ok(Val::Num(round_half_away(self.argument(home, arguments, 0)?.number()?, self.argument(home, arguments, 1)?.number()?.trunc())))
            }
            "LEN" => {
                arity(1, 1)?;
                Ok(Val::Num(text(0)?.chars().count() as f64))
            }
            "UPPER" => {
                arity(1, 1)?;
                Ok(Val::Text(text(0)?.to_uppercase()))
            }
            "LOWER" => {
                arity(1, 1)?;
                Ok(Val::Text(text(0)?.to_lowercase()))
            }
            "CONCAT" | "CONCATENATE" => self.texts(home, arguments).map(Val::Text),
            _ => Err(NAME),
        }
    }
}

fn compare(left: &Val, right: &Val) -> Ordering {
    let rank = |value: &Val| match value {
        Val::Num(_) | Val::Empty => 0,
        Val::Text(_) => 1,
        Val::Bool(_) => 2,
    };
    let (left, right) = match (left, right) {
        (Val::Empty, Val::Text(_)) => (Val::Text(String::new()), right.clone()),
        (Val::Text(_), Val::Empty) => (left.clone(), Val::Text(String::new())),
        (Val::Empty, Val::Bool(_)) => (Val::Bool(false), right.clone()),
        (Val::Bool(_), Val::Empty) => (left.clone(), Val::Bool(false)),
        _ => (left.clone(), right.clone()),
    };
    rank(&left).cmp(&rank(&right)).then_with(|| match (&left, &right) {
        (Val::Text(a), Val::Text(b)) => a.to_lowercase().cmp(&b.to_lowercase()),
        (Val::Bool(a), Val::Bool(b)) => a.cmp(b),
        _ => left.number().unwrap_or(0.0).partial_cmp(&right.number().unwrap_or(0.0)).unwrap_or(Ordering::Equal),
    })
}

fn round_half_away(number: f64, digits: f64) -> f64 {
    if number == 0.0 || !number.is_finite() {
        return number;
    }
    let scientific = format!("{:e}", number.abs());
    let (mantissa, exponent) = scientific.split_once('e').unwrap_or((&scientific, "0"));
    let digits_text: String = mantissa.chars().filter(char::is_ascii_digit).collect();
    let point = exponent.parse::<i64>().unwrap_or(0) + 1;
    let keep = point + digits as i64;
    if keep >= digits_text.len() as i64 {
        return number;
    }
    let kept: u128 = if keep <= 0 { 0 } else { digits_text[..keep as usize].parse().unwrap_or(0) };
    let up = keep >= 0 && digits_text.as_bytes()[keep as usize] >= b'5';
    let magnitude = format!("{}e{}", kept + u128::from(up), -(digits as i64)).parse::<f64>().unwrap_or(0.0);
    magnitude.copysign(number)
}
//#endregion 🔖️Evaluation

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
