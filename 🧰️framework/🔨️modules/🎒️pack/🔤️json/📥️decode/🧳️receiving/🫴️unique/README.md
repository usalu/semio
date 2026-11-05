# Unique Original JSON Output

`OriginalJsonOutputReceiving<T>` borrows the actual immutable output range retained by its transport and captures one complete typed value. It preserves the original parser, DSL value, source admission and typed receiver through every bounded step and cancellation. Each step must receive the same range from a current, validated guest-memory borrow. The range identity is data; the containing Source and runtime frontier provide authority and prevent concurrent guest execution.

`OriginalJsonOutputSource<T>` owns the actual decoder loan beside that receiver. Construct its allocation-free declaration inside the Source-native payload before startup. `receive_step` and `take_step` return the complete producer outcome and any receiving refusal. The containing payload must retain each refusal until its owned data has been retired. Readiness means parser and typed-receiver cleanup completed and the unique typed capture is ready.

A caller may use `T = Result<TurnResult, ()>` for the full kernel poll result, including UI, messages, effects and lifecycle fields. After capture, the containing instance must acknowledge its actual serialized output and drive its real deallocation frontier, then execute and settle the funded resident actor-output retirement. These receiving APIs perform neither guest acknowledgement nor guest execution.

Retire a captured typed value with `close_owned_step` before closing this decoder. `close_step` first returns raw producer cleanup outcomes and then the actual Source return step. Cancellation preserves those same owners and uses the original retirement context. `terminal_is_empty` requires both the receiver and its decoder Source to be empty.

The adjacent neutral corpus is checked independently with strict Ajv and SQLite. Rust laws additionally compare Serde output, observe physical heap receipts, test foreign borrowed ranges and cancel every declared prefix under real Original Source turns. Current native execution status belongs to the active ticket evidence.
