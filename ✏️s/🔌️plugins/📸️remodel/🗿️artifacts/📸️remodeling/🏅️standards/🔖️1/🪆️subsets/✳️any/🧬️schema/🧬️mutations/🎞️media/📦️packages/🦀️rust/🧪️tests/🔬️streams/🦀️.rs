//! 🧪️ The committed media mutation vectors `streams`: one implementation case per fixture bundle, each checked against its vector by the shared law library.

extern crate semio_framework_os_kernel as protocol;

#[path = "../../../../../🧪️tests/🔬️vectors/🦀️.rs"]
mod vectors;

#[path = "../../../../../🌱create-stream/🧪️tests/🎞️adds/🦀️.rs"]
mod create_stream_tests_adds_an_unbound_2b2373;
#[path = "../../../../../🌱create-stream/🧪️tests/🎥️adds/🦀️.rs"]
mod create_stream_tests_adds_stream_c_458900;
#[path = "../../../../../🌱create-stream/🧪️tests/👻️rejects/🦀️.rs"]
mod create_stream_tests_rejects_a_stream_aac5c2;
#[path = "../../../../../🌱create-stream/🧪️tests/🔂️rejects-a-6b58da/🦀️.rs"]
mod create_stream_tests_rejects_a_6b58da;
#[path = "../../../../../🌱create-stream/🧪️tests/🛰️adds/🦀️.rs"]
mod create_stream_tests_adds_a_third_61fb5d;
#[path = "../../../../../🪓delete-stream/🧪️tests/⏮️removes/🦀️.rs"]
mod delete_stream_tests_removes_the_first_c0fc2a;
#[path = "../../../../../🪓delete-stream/🧪️tests/⛓️refuses/🦀️.rs"]
mod delete_stream_tests_refuses_to_remove_422a37;
#[path = "../../../../../🪓delete-stream/🧪️tests/🚫️refuses/🦀️.rs"]
mod delete_stream_tests_refuses_to_3c20ff;
#[path = "../../../../../🪓delete-stream/🧪️tests/🪓removes/🦀️.rs"]
mod delete_stream_tests_removes_the_spare_556d1d;
#[path = "../../../../../⏱️change-stream-sync/🧪️tests/⏱️retimes/🦀️.rs"]
mod change_stream_sync_tests_retimes_the_50dd75;
#[path = "../../../../../⏱️change-stream-sync/🧪️tests/⏱️shifts/🦀️.rs"]
mod change_stream_sync_tests_shifts_stream_a_5b442c;
#[path = "../../../../../⏱️change-stream-sync/🧪️tests/🔁️warns/🦀️.rs"]
mod change_stream_sync_tests_warns_that_the_a98c13;
#[path = "../../../../../⏱️change-stream-sync/🧪️tests/🚫️refuses/🦀️.rs"]
mod change_stream_sync_tests_refuses_to_8095d3;
#[path = "../../../../../➕add-stream-frame/🧪️tests/🎞️appends/🦀️.rs"]
mod add_stream_frame_tests_appends_a_third_8ac259;
#[path = "../../../../../➕add-stream-frame/🧪️tests/🎞️appends2/🦀️.rs"]
mod add_stream_frame_tests_appends_an_0c2164;
#[path = "../../../../../➕add-stream-frame/🧪️tests/🎬️refuses/🦀️.rs"]
mod add_stream_frame_tests_refuses_a_frame_81beea;
#[path = "../../../../../➕add-stream-frame/🧪️tests/🔁️warns/🦀️.rs"]
mod add_stream_frame_tests_warns_that_the_1e8abe;
#[path = "../../../../../➕add-stream-frame/🧪️tests/🚫️refuses/🦀️.rs"]
mod add_stream_frame_tests_refuses_to_c93e98;
#[path = "../../../../../➖remove-stream-frame/🧪️tests/⏮️drops/🦀️.rs"]
mod remove_stream_frame_tests_drops_the_first_d98a0f;
#[path = "../../../../../➖remove-stream-frame/🧪️tests/✂️drops/🦀️.rs"]
mod remove_stream_frame_tests_drops_the_middle_2d6d53;
#[path = "../../../../../➖remove-stream-frame/🧪️tests/🚫️refuses/🦀️.rs"]
mod remove_stream_frame_tests_refuses_a_frame_e7c374;
#[path = "../../../../../➖remove-stream-frame/🧪️tests/🚫️removes/🦀️.rs"]
mod remove_stream_frame_tests_removes_the_last_304bdf;
#[path = "../../../../../🔁replace-stream/🧪️tests/🎥️reingests/🦀️.rs"]
mod replace_stream_tests_reingests_the_311c32;
#[path = "../../../../../🔁replace-stream/🧪️tests/📼️attaches/🦀️.rs"]
mod replace_stream_tests_attaches_a_607df8;
#[path = "../../../../../🔁replace-stream/🧪️tests/🚫️refuses/🦀️.rs"]
mod replace_stream_tests_refuses_to_f7f40d;
#[path = "../../../../../🔁replace-stream/🧪️tests/🧹️clears/🦀️.rs"]
mod replace_stream_tests_clears_the_video_143f2b;
