//! 🚦️ Exact owned sample words and contiguous relational positions.
use crate::standards::v6_0::subsets::document::io::sqlite::snapshot::tests::*;
#[test]fn owned_word_precision_and_sample_ordinals_survive_projection(){let source=fixture();let restored=restore(&project(&source)).unwrap();assert_eq!(restored.ifds[1].blocks[0].samples,vec![TiffWord64{lo:32769,hi:0},TiffWord64{lo:65535,hi:0}]);}
#[test]fn sample_ordinals_must_be_contiguous(){let mut database=project(&fixture());database.table_mut("tiff_sample").unwrap().rows[1].values[2]=SqliteValue::Integer(4);assert!(restore(&database).is_err());}
