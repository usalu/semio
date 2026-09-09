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
    let json = dsl::json::to_json_string(&table);
    let back: Table = dsl::json::from_json_str(&json).unwrap();
    assert_eq!(back.names(), table.names());
    assert!(nan_aware_eq(back.continuous(0).unwrap(), table.continuous(0).unwrap()));
    assert_eq!(back.categorical(1).unwrap().codes(), table.categorical(1).unwrap().codes());
}
// #endregion 🔖️SerdeTests
