use super::*;

#[test]
fn carrier_product_matrix_matches_the_production_snapshot_codec() {
    let texts = ["", "a", "a\nb", "a\rb", "a\r", "a\r\nb"];
    for line_ending in [LineEnding::Lf, LineEnding::CrLf] {
        for trailing_newline in [false, true] {
            for first in texts {
                for second in texts {
                    for lines in [Vec::new(), vec![first.to_string()], vec![first.to_string(), second.to_string()]] {
                        let snapshot = TxtSnapshot { lines, trailing_newline, line_ending, ..Default::default() };
                        let production_round_trip = TxtSnapshot::from_body(&snapshot.to_body()) == snapshot;
                        assert_eq!(native_snapshot_error(&snapshot).is_none(), production_round_trip, "{snapshot:?}");
                    }
                }
            }
        }
    }
}
