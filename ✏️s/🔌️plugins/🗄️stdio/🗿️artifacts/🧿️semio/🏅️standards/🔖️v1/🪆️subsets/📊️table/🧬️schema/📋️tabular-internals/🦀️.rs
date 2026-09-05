//! 🗃️ Column-oriented in-memory tables: continuous and categorical columns with names, missing values, selection, and CSV round-tripping.
//!
//! Moved verbatim from `🧰️framework/🔨️modules/🧮️math/📋️tabular` in ticket 26/08/12/
//! DISSOLVE-KERNELS-AND-MODULES-INTO-EVENT-SOURCED-ARTIFACTS wave M3c: Rust-only compute internals
//! backing `📊️table`'s inferences (no TS twin — this is algorithm, not boundary vocabulary).

// #region 🔖️Error
/// ⚠️ Fallible construction, lookup, and parsing errors for [`Table`] and [`CategoricalColumn`].
#[derive(Debug)]
pub enum TabularError {
    LengthMismatch { expected: usize, found: usize },
    DuplicateName(String),
    UnknownColumn(String),
    IndexOutOfBounds(usize),
    NotContinuous(String),
    NotCategorical(String),
    Csv { line: usize, message: String },
}

impl std::fmt::Display for TabularError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::LengthMismatch { expected, found } => write!(formatter, "column length {found} does not match row count {expected}"),
            Self::DuplicateName(name) => write!(formatter, "duplicate column name `{name}`"),
            Self::UnknownColumn(name) => write!(formatter, "no column named `{name}`"),
            Self::IndexOutOfBounds(index) => write!(formatter, "column index {index} out of bounds"),
            Self::NotContinuous(name) => write!(formatter, "column `{name}` is not continuous"),
            Self::NotCategorical(name) => write!(formatter, "column `{name}` is not categorical"),
            Self::Csv { line, message } => write!(formatter, "csv parse error at line {line}: {message}"),
        }
    }
}

impl std::error::Error for TabularError {}
// #endregion 🔖️Error

// #region 🔖️Categorical
/// 🚫️ Reserved categorical code marking a missing value (mirrors `f64::NAN` for continuous columns).
pub const MISSING_CODE: u32 = u32::MAX;

/// 🏷️ A categorical column: integer codes into a level table, with `MISSING_CODE` marking missing values.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
pub struct CategoricalColumn {
    levels: Vec<String>,
    codes: Vec<u32>,
}

impl CategoricalColumn {
    /// 🏷️ Builds a column from string labels; `""` encodes a missing value, other labels are assigned
    /// levels in first-seen order.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn from_labels(labels: &[&str]) -> Self {
        let mut levels: Vec<String> = Vec::new();
        let mut index: std::collections::HashMap<&str, u32> = std::collections::HashMap::new();
        let mut codes = Vec::with_capacity(labels.len());
        for &label in labels {
            if label.is_empty() {
                codes.push(MISSING_CODE);
                continue;
            }
            let code = *index.entry(label).or_insert_with(|| {
                levels.push(label.to_string());
                (levels.len() - 1) as u32
            });
            codes.push(code);
        }
        Self { levels, codes }
    }

    /// 🏷️ Builds a column from already-encoded parts, validating every non-missing code is in range.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn from_parts(levels: Vec<String>, codes: Vec<u32>) -> Result<Self, TabularError> {
        for &code in &codes {
            if code != MISSING_CODE && code as usize >= levels.len() {
                return Err(TabularError::IndexOutOfBounds(code as usize));
            }
        }
        Ok(Self { levels, codes })
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn len(&self) -> usize {
        self.codes.len()
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn is_empty(&self) -> bool {
        self.codes.is_empty()
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn n_levels(&self) -> usize {
        self.levels.len()
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn codes(&self) -> &[u32] {
        &self.codes
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn levels(&self) -> &[String] {
        &self.levels
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn level(&self, code: u32) -> Option<&str> {
        self.levels.get(code as usize).map(String::as_str)
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn code_of(&self, label: &str) -> Option<u32> {
        self.levels.iter().position(|l| l == label).map(|i| i as u32)
    }

    /// 🔢️ Per-level occurrence counts, missing values excluded.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn counts(&self) -> Vec<usize> {
        let mut counts = vec![0usize; self.levels.len()];
        for &code in &self.codes {
            if code != MISSING_CODE {
                counts[code as usize] += 1;
            }
        }
        counts
    }

    /// 🔢️ One-hot encoding, one `Vec<f64>` per row; a missing row is filled with `NaN` indicators.
    /// When `drop_first`, level `0`'s indicator column is omitted (the usual reference-level encoding).
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn one_hot(&self, drop_first: bool) -> Vec<Vec<f64>> {
        let skip = usize::from(drop_first);
        let width = self.levels.len().saturating_sub(skip);
        self.codes
            .iter()
            .map(|&code| {
                if code == MISSING_CODE {
                    vec![f64::NAN; width]
                } else {
                    let mut row = vec![0.0; width];
                    let code = code as usize;
                    if code >= skip {
                        row[code - skip] = 1.0;
                    }
                    row
                }
            })
            .collect()
    }
}
// #endregion 🔖️Categorical

// #region 🔖️Column
/// 📦️ One table column: either a continuous `f64` series (missing = `NaN`) or a [`CategoricalColumn`].
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
pub enum Column {
    Continuous(Vec<f64>),
    Categorical(CategoricalColumn),
}

impl Column {
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn len(&self) -> usize {
        match self {
            Self::Continuous(values) => values.len(),
            Self::Categorical(column) => column.len(),
        }
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn is_missing_at(&self, row: usize) -> bool {
        match self {
            Self::Continuous(values) => values[row].is_nan(),
            Self::Categorical(column) => column.codes[row] == MISSING_CODE,
        }
    }
}
// #endregion 🔖️Column

// #region 🔖️Table
/// 🗃️ A column-oriented, named, row-aligned dataset.
#[derive(Clone, Debug, PartialEq, Default, value_derive::ToValue, value_derive::FromValue)]
pub struct Table {
    names: Vec<String>,
    columns: Vec<Column>,
    rows: usize,
}

impl Table {
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn new() -> Self {
        Self::default()
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn n_rows(&self) -> usize {
        self.rows
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn n_cols(&self) -> usize {
        self.columns.len()
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn names(&self) -> &[String] {
        &self.names
    }

    /// 🗃️ Appends a column, returning its new index. The first pushed column fixes the table's row
    /// count; every later column must match it.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn push_column(&mut self, name: &str, column: Column) -> Result<usize, TabularError> {
        if self.names.iter().any(|existing| existing == name) {
            return Err(TabularError::DuplicateName(name.to_string()));
        }
        if self.columns.is_empty() {
            self.rows = column.len();
        } else if column.len() != self.rows {
            return Err(TabularError::LengthMismatch { expected: self.rows, found: column.len() });
        }
        self.names.push(name.to_string());
        self.columns.push(column);
        Ok(self.columns.len() - 1)
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn push_continuous(&mut self, name: &str, values: Vec<f64>) -> Result<usize, TabularError> {
        self.push_column(name, Column::Continuous(values))
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn push_categorical(&mut self, name: &str, labels: &[&str]) -> Result<usize, TabularError> {
        self.push_column(name, Column::Categorical(CategoricalColumn::from_labels(labels)))
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn column_index(&self, name: &str) -> Result<usize, TabularError> {
        self.names.iter().position(|n| n == name).ok_or_else(|| TabularError::UnknownColumn(name.to_string()))
    }

    /// 🔎️ Infallible variant of [`Table::column_index`].
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn index_of(&self, name: &str) -> Option<usize> {
        self.names.iter().position(|n| n == name)
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn column(&self, index: usize) -> Result<&Column, TabularError> {
        self.columns.get(index).ok_or(TabularError::IndexOutOfBounds(index))
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn continuous(&self, index: usize) -> Result<&[f64], TabularError> {
        match self.column(index)? {
            Column::Continuous(values) => Ok(values),
            Column::Categorical(_) => Err(TabularError::NotContinuous(self.names[index].clone())),
        }
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn categorical(&self, index: usize) -> Result<&CategoricalColumn, TabularError> {
        match self.column(index)? {
            Column::Categorical(column) => Ok(column),
            Column::Continuous(_) => Err(TabularError::NotCategorical(self.names[index].clone())),
        }
    }

    /// 🔀️ Projects the table to the given columns (order preserved, repeats allowed).
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn select_columns(&self, indices: &[usize]) -> Result<Table, TabularError> {
        let mut out = Table::new();
        for &index in indices {
            let column = self.column(index)?.clone();
            out.push_column(&self.names[index], column)?;
        }
        Ok(out)
    }

    /// 🔀️ Gathers rows by index (repeats allowed — needed for bootstrap resampling).
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn select_rows(&self, indices: &[usize]) -> Result<Table, TabularError> {
        for &row in indices {
            if row >= self.rows {
                return Err(TabularError::IndexOutOfBounds(row));
            }
        }
        let mut out = Table::new();
        for (name, column) in self.names.iter().zip(self.columns.iter()) {
            let gathered = match column {
                Column::Continuous(values) => Column::Continuous(indices.iter().map(|&row| values[row]).collect()),
                Column::Categorical(cat) => Column::Categorical(CategoricalColumn { levels: cat.levels.clone(), codes: indices.iter().map(|&row| cat.codes[row]).collect() }),
            };
            out.push_column(name, gathered)?;
        }
        Ok(out)
    }

    /// ✅️ Row indices, ascending, where none of the given columns is missing.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn complete_rows(&self, columns: &[usize]) -> Result<Vec<usize>, TabularError> {
        for &index in columns {
            if index >= self.columns.len() {
                return Err(TabularError::IndexOutOfBounds(index));
            }
        }
        Ok((0..self.rows).filter(|&row| columns.iter().all(|&index| !self.columns[index].is_missing_at(row))).collect())
    }

    /// ✅️ `select_rows(complete_rows(columns))` — the complete-case sub-table.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn drop_missing(&self, columns: &[usize]) -> Result<Table, TabularError> {
        let rows = self.complete_rows(columns)?;
        self.select_rows(&rows)
    }

    /// 🏗️ Builds a table from parallel-named `f64` column vectors.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn from_f64_columns(names: Vec<String>, columns: Vec<Vec<f64>>) -> Result<Table, TabularError> {
        let mut out = Table::new();
        for (name, values) in names.into_iter().zip(columns) {
            out.push_continuous(&name, values)?;
        }
        Ok(out)
    }

    /// 🏗️ Builds a table from parallel-named `(codes, level_names)` categorical column pairs.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn from_categorical_columns(names: Vec<String>, columns: Vec<(Vec<u32>, Vec<String>)>) -> Result<Table, TabularError> {
        let mut out = Table::new();
        for (name, (codes, levels)) in names.into_iter().zip(columns) {
            let column = CategoricalColumn::from_parts(levels, codes)?;
            out.push_column(&name, Column::Categorical(column))?;
        }
        Ok(out)
    }
}
// #endregion 🔖️Table

// #region 🔖️Csv
/// ⚙️ CSV dialect options for [`Table::parse_csv`]/[`Table::to_csv`].
#[derive(Clone, Copy, Debug)]
pub struct CsvOptions {
    pub delimiter: char,
    pub has_header: bool,
}

impl Default for CsvOptions {
    fn default() -> Self {
        Self { delimiter: ',', has_header: true }
    }
}

/// 📄️ Splits `text` into rows of raw string fields via a single-pass RFC-4180-subset state machine:
/// quoted fields, doubled-quote escaping, delimiters/newlines inside quotes, and CRLF/LF line endings.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn split_csv_fields(text: &str, delimiter: char) -> Vec<Vec<String>> {
    let mut rows = Vec::new();
    let mut row: Vec<String> = Vec::new();
    let mut field = String::new();
    let mut in_quotes = false;
    let mut chars = text.chars().peekable();
    let mut row_has_content = false;
    while let Some(c) = chars.next() {
        if in_quotes {
            if c == '"' {
                if chars.peek() == Some(&'"') {
                    field.push('"');
                    chars.next();
                } else {
                    in_quotes = false;
                }
            } else {
                field.push(c);
            }
            continue;
        }
        if c == '"' {
            in_quotes = true;
            row_has_content = true;
        } else if c == delimiter {
            row.push(std::mem::take(&mut field));
            row_has_content = true;
        } else if c == '\r' {
            continue;
        } else if c == '\n' {
            row.push(std::mem::take(&mut field));
            rows.push(std::mem::take(&mut row));
            row_has_content = false;
        } else {
            field.push(c);
            row_has_content = true;
        }
    }
    if row_has_content || !field.is_empty() || !row.is_empty() {
        row.push(field);
        rows.push(row);
    }
    rows
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn is_missing_token(field: &str) -> bool {
    field.is_empty() || field.eq_ignore_ascii_case("na") || field.eq_ignore_ascii_case("nan")
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn quote_csv_field(field: &str, delimiter: char) -> String {
    if field.contains(delimiter) || field.contains('"') || field.contains('\n') {
        format!("\"{}\"", field.replace('"', "\"\""))
    } else {
        field.to_string()
    }
}

impl Table {
    /// 📄️ Parses CSV text, inferring each column's type: `Continuous` if every non-missing field
    /// parses as `f64`, `Categorical` otherwise. Missing tokens are empty fields, `NA`, and `NaN`
    /// (case-insensitive).
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn parse_csv(text: &str, options: CsvOptions) -> Result<Table, TabularError> {
        let rows = split_csv_fields(text, options.delimiter);
        let mut rows = rows.into_iter();
        let first = rows.next();
        let Some(first) = first else {
            return Ok(Table::new());
        };
        let n_cols = first.len();
        let (names, data_rows): (Vec<String>, Vec<Vec<String>>) = if options.has_header { (first, rows.collect()) } else { ((0..n_cols).map(|i| format!("c{i}")).collect(), std::iter::once(first).chain(rows).collect()) };
        for (line, row) in data_rows.iter().enumerate() {
            if row.len() != n_cols {
                return Err(TabularError::Csv { line: line + 1, message: format!("expected {n_cols} fields, found {}", row.len()) });
            }
        }
        let mut table = Table::new();
        for (col_index, name) in names.iter().enumerate() {
            let field_of = |row: &Vec<String>| row[col_index].trim().to_string();
            let all_numeric = data_rows.iter().all(|row| {
                let field = field_of(row);
                is_missing_token(&field) || field.parse::<f64>().is_ok()
            });
            if all_numeric {
                let values = data_rows
                    .iter()
                    .map(|row| {
                        let field = field_of(row);
                        if is_missing_token(&field) {
                            f64::NAN
                        } else {
                            field.parse::<f64>().expect("validated above")
                        }
                    })
                    .collect();
                table.push_continuous(name, values)?;
            } else {
                let labels: Vec<String> = data_rows.iter().map(field_of).collect();
                let refs: Vec<&str> = labels.iter().map(String::as_str).collect();
                table.push_categorical(name, &refs)?;
            }
        }
        Ok(table)
    }

    /// 📄️ Serializes to CSV text: missing values become empty fields, floats use Rust's shortest
    /// round-trip `Display`, and fields containing the delimiter/quote/newline are quoted.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn to_csv(&self, options: CsvOptions) -> String {
        let mut out = String::new();
        if options.has_header {
            let header: Vec<String> = self.names.iter().map(|name| quote_csv_field(name, options.delimiter)).collect();
            out.push_str(&header.join(&options.delimiter.to_string()));
            out.push('\n');
        }
        for row in 0..self.rows {
            let fields: Vec<String> = self
                .columns
                .iter()
                .map(|column| match column {
                    Column::Continuous(values) => {
                        if values[row].is_nan() {
                            String::new()
                        } else {
                            values[row].to_string()
                        }
                    }
                    Column::Categorical(cat) => cat.level(cat.codes[row]).map(|label| quote_csv_field(label, options.delimiter)).unwrap_or_default(),
                })
                .collect();
            out.push_str(&fields.join(&options.delimiter.to_string()));
            out.push('\n');
        }
        out
    }
}
// #endregion 🔖️Csv

// #region 🔖️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tabular_error_contract_is_owned_and_stable() {
        let errors = [
            (TabularError::LengthMismatch { expected: 4, found: 2 }, "column length 2 does not match row count 4".to_string()),
            (TabularError::DuplicateName("x".into()), "duplicate column name `x`".to_string()),
            (TabularError::UnknownColumn("x".into()), "no column named `x`".to_string()),
            (TabularError::IndexOutOfBounds(8), "column index 8 out of bounds".to_string()),
            (TabularError::NotContinuous("x".into()), "column `x` is not continuous".to_string()),
            (TabularError::NotCategorical("x".into()), "column `x` is not categorical".to_string()),
            (TabularError::Csv { line: 3, message: "bad quote".into() }, "csv parse error at line 3: bad quote".to_string()),
        ];
        for (error, message) in errors {
            assert_eq!(error.to_string(), message);
            assert!(std::error::Error::source(&error).is_none());
        }
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn nan_aware_eq(a: &[f64], b: &[f64]) -> bool {
        a.len() == b.len() && a.iter().zip(b).all(|(x, y)| (x.is_nan() && y.is_nan()) || (x - y).abs() < 1e-12)
    }

    // #region 🔖️CategoricalTests
    #[semio_framework_async_macros::async_test]
    async fn categorical_from_labels_assigns_first_seen_order() {
        let col = CategoricalColumn::from_labels(&["b", "a", "b", "", "c"]);
        assert_eq!(col.levels(), &["b".to_string(), "a".to_string(), "c".to_string()]);
        assert_eq!(col.codes(), &[0, 1, 0, MISSING_CODE, 2]);
    }

    #[semio_framework_async_macros::async_test]
    async fn categorical_counts_exclude_missing() {
        let col = CategoricalColumn::from_labels(&["a", "b", "a", ""]);
        assert_eq!(col.counts(), vec![2, 1]);
    }

    #[semio_framework_async_macros::async_test]
    async fn categorical_one_hot_matches_hand_matrix() {
        let col = CategoricalColumn::from_labels(&["a", "b", "c", ""]);
        let full = col.one_hot(false);
        assert_eq!(full[0], vec![1.0, 0.0, 0.0]);
        assert_eq!(full[1], vec![0.0, 1.0, 0.0]);
        assert_eq!(full[2], vec![0.0, 0.0, 1.0]);
        assert!(full[3].iter().all(|v| v.is_nan()));

        let dropped = col.one_hot(true);
        assert_eq!(dropped[0], vec![0.0, 0.0]);
        assert_eq!(dropped[1], vec![1.0, 0.0]);
        assert_eq!(dropped[2], vec![0.0, 1.0]);
        assert!(dropped[3].iter().all(|v| v.is_nan()));
    }

    #[semio_framework_async_macros::async_test]
    async fn categorical_from_parts_rejects_out_of_range_code() {
        assert!(CategoricalColumn::from_parts(vec!["a".to_string()], vec![5]).is_err());
    }
    // #endregion 🔖️CategoricalTests

    // #region 🔖️TableTests
    #[semio_framework_async_macros::async_test]
    async fn push_column_length_mismatch_errors() {
        let mut table = Table::new();
        table.push_continuous("x", vec![1.0, 2.0, 3.0]).unwrap();
        let err = table.push_continuous("y", vec![1.0, 2.0]).unwrap_err();
        assert!(matches!(err, TabularError::LengthMismatch { expected: 3, found: 2 }));
    }

    #[semio_framework_async_macros::async_test]
    async fn push_column_duplicate_name_errors() {
        let mut table = Table::new();
        table.push_continuous("x", vec![1.0]).unwrap();
        assert!(matches!(table.push_continuous("x", vec![2.0]), Err(TabularError::DuplicateName(_))));
    }

    #[semio_framework_async_macros::async_test]
    async fn column_index_unknown_errors() {
        let table = Table::new();
        assert!(matches!(table.column_index("missing"), Err(TabularError::UnknownColumn(_))));
    }

    #[semio_framework_async_macros::async_test]
    async fn continuous_and_categorical_type_errors() {
        let mut table = Table::new();
        table.push_continuous("x", vec![1.0]).unwrap();
        table.push_categorical("y", &["a"]).unwrap();
        assert!(matches!(table.categorical(0), Err(TabularError::NotCategorical(_))));
        assert!(matches!(table.continuous(1), Err(TabularError::NotContinuous(_))));
    }

    #[semio_framework_async_macros::async_test]
    async fn complete_rows_and_drop_missing() {
        let mut table = Table::new();
        table.push_continuous("x", vec![1.0, f64::NAN, 3.0, 4.0, f64::NAN]).unwrap();
        table.push_categorical("y", &["a", "b", "", "c", "d"]).unwrap();
        let complete = table.complete_rows(&[0, 1]).unwrap();
        assert_eq!(complete, vec![0, 3]);
        let dropped = table.drop_missing(&[0, 1]).unwrap();
        assert_eq!(dropped.n_rows(), 2);
        assert!(nan_aware_eq(dropped.continuous(0).unwrap(), &[1.0, 4.0]));
    }

    #[semio_framework_async_macros::async_test]
    async fn select_rows_allows_repetition_for_bootstrap() {
        let mut table = Table::new();
        table.push_continuous("x", vec![10.0, 20.0, 30.0]).unwrap();
        let resampled = table.select_rows(&[2, 2, 0]).unwrap();
        assert!(nan_aware_eq(resampled.continuous(0).unwrap(), &[30.0, 30.0, 10.0]));
    }

    #[semio_framework_async_macros::async_test]
    async fn select_columns_projects_subset() {
        let mut table = Table::new();
        table.push_continuous("x", vec![1.0, 2.0]).unwrap();
        table.push_continuous("y", vec![3.0, 4.0]).unwrap();
        let projected = table.select_columns(&[1]).unwrap();
        assert_eq!(projected.names(), &["y".to_string()]);
        assert!(nan_aware_eq(projected.continuous(0).unwrap(), &[3.0, 4.0]));
    }
    // #endregion 🔖️TableTests

    // #region 🔖️CsvTests
    #[semio_framework_async_macros::async_test]
    async fn csv_round_trip_with_missing_values() {
        let mut table = Table::new();
        table.push_continuous("x", vec![1.5, f64::NAN, 3.0]).unwrap();
        table.push_categorical("y", &["a", "b", ""]).unwrap();
        let csv = table.to_csv(CsvOptions::default());
        let parsed = Table::parse_csv(&csv, CsvOptions::default()).unwrap();
        assert_eq!(parsed.names(), table.names());
        assert!(nan_aware_eq(parsed.continuous(0).unwrap(), table.continuous(0).unwrap()));
        assert_eq!(parsed.categorical(1).unwrap().codes(), table.categorical(1).unwrap().codes());
    }

    #[semio_framework_async_macros::async_test]
    async fn csv_parses_quoted_field_with_embedded_delimiter_and_escaped_quote() {
        let text = "name,note\na,\"x, \"\"y\"\"\"\n";
        let table = Table::parse_csv(text, CsvOptions::default()).unwrap();
        let note = table.categorical(1).unwrap();
        assert_eq!(note.level(note.codes()[0]).unwrap(), "x, \"y\"");
    }

    #[semio_framework_async_macros::async_test]
    async fn csv_type_inference_continuous_with_blank_is_nan() {
        let text = "x\n1\n2\n\n4\n";
        let table = Table::parse_csv(text, CsvOptions::default()).unwrap();
        let values = table.continuous(0).unwrap();
        assert!(nan_aware_eq(values, &[1.0, 2.0, f64::NAN, 4.0]));
    }

    #[semio_framework_async_macros::async_test]
    async fn csv_type_inference_any_nonnumeric_is_categorical() {
        let text = "x\n1\nfoo\n3\n";
        let table = Table::parse_csv(text, CsvOptions::default()).unwrap();
        assert!(table.categorical(0).is_ok());
    }

    #[semio_framework_async_macros::async_test]
    async fn csv_headerless_synthesizes_names() {
        let text = "1,a\n2,b\n";
        let table = Table::parse_csv(text, CsvOptions { has_header: false, ..Default::default() }).unwrap();
        assert_eq!(table.names(), &["c0".to_string(), "c1".to_string()]);
    }
    // #endregion 🔖️CsvTests

    // #region 🔖️SerdeTests
    #[semio_framework_async_macros::async_test]
    async fn table_json_round_trip() {
        // No NaN in the continuous column here: serde_json has no JSON representation for NaN
        // (it serializes to `null`, which `f64`'s Deserialize then rejects), so JSON round-tripping
        // is a documented non-goal for missing continuous values — categorical missingness (an
        // integer sentinel) round-trips fine and is covered below instead.
        let mut table = Table::new();
        table.push_continuous("x", vec![1.0, 2.5]).unwrap();
        table.push_categorical("y", &["a", ""]).unwrap();
        let json = serde_json::to_string(&table).unwrap();
        let back: Table = serde_json::from_str(&json).unwrap();
        assert_eq!(back.names(), table.names());
        assert!(nan_aware_eq(back.continuous(0).unwrap(), table.continuous(0).unwrap()));
        assert_eq!(back.categorical(1).unwrap().codes(), table.categorical(1).unwrap().codes());
    }
    // #endregion 🔖️SerdeTests
}
// #endregion 🔖️Tests
