use super::{execute_numeric_fixed, Value};

fn input(kind: &str, value: &str) -> Value {
    match kind {
        "i32" => Value::I32(value.parse().unwrap()),
        "i64" => Value::I64(value.parse().unwrap()),
        "f32" => Value::f32(value.parse().unwrap()),
        "f64" => Value::f64(value.parse().unwrap()),
        _ => panic!("unknown value type {kind}"),
    }
}

#[test]
fn authored_scalar_results() {
    for row in include_str!("🔣️.tsv").lines().skip(1) {
        let fields: Vec<_> = row.split('\t').collect();
        let opcode = u8::from_str_radix(fields[0], 16).unwrap();
        let operands: Vec<_> = fields[3].split(',').map(|v| input(fields[1], v)).collect();
        let actual = execute_numeric_fixed(opcode, &operands);
        if fields[4] == "trap" {
            assert!(actual.is_err(), "{row}: {actual:?}");
        } else {
            assert_eq!(actual.unwrap(), input(fields[2], fields[4]), "{row}");
        }
    }
}
