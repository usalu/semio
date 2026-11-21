//! 🌱️ The synthetic current Owned ABI exposes only the real channel result and traps unrelated operations.
use semio_plugin_interpreter::OwnedSemioExport;

pub fn owned_channel_guest(channel: u32) -> Vec<u8> {
    owned_guest(channel,false,0,None)
}

/// 🪞️ The input export echoes its exact borrowed wire for independent transport laws.
pub fn owned_input_echo_guest(channel:u32)->Vec<u8>{owned_guest(channel,true,0,None)}

/// 🕰️ Startup crosses the actual original clock import before returning the current channel.
pub fn owned_clock_guest(channel:u32,binding:u64)->Vec<u8>{owned_guest(channel,false,0,Some(binding))}

/// 📥️ The actual input handback fixture.
pub fn owned_input_receipt_guest(channel:u32)->Vec<u8>{owned_guest(channel,false,3,None)}

/// ⏳️ The synthetic pending receiver retains its borrowed input through every actual invocation.
pub fn owned_input_pending_guest(channel:u32)->Vec<u8>{owned_guest(channel,false,4,None)}

/// 🔁️ Poll lends output and actual receipts.
pub fn owned_poll_guest(channel:u32)->Vec<u8>{owned_guest(channel,false,5,None)}

/// 🕰️ The cleanup export uses its actual budget clock binding through the host import.
pub fn owned_poll_clock_guest(channel:u32)->Vec<u8>{owned_guest(channel,false,6,None)}

/// ⚠️ A genuine failed Input receipt and true SAF1 acknowledgment exercise retained-wire cleanup fences.
pub fn owned_input_failure_guest(channel:u32)->Vec<u8>{owned_guest(channel,false,7,None)}

fn owned_guest(channel:u32,input_echo:bool,input_receipt:u8,clock:Option<u64>)->Vec<u8>{
    fn uleb(mut value: u64, output: &mut Vec<u8>) {
        loop {
            let byte = (value & 0x7f) as u8;
            value >>= 7;
            output.push(if value == 0 { byte } else { byte | 0x80 });
            if value == 0 {
                return;
            }
        }
    }
    fn sleb(mut value: i64, output: &mut Vec<u8>) {
        loop {
            let byte = (value & 0x7f) as u8;
            value >>= 7;
            let done = (value == 0 && byte & 0x40 == 0) || (value == -1 && byte & 0x40 != 0);
            output.push(if done { byte } else { byte | 0x80 });
            if done {
                return;
            }
        }
    }
    fn section(id: u8, payload: Vec<u8>, output: &mut Vec<u8>) {
        output.push(id);
        uleb(payload.len() as u64, output);
        output.extend(payload);
    }
    
    const ESCROW: u32 = 3200;
    const REVISION: u32 = ESCROW + 32;
    const ACKNOWLEDGED: u32 = ESCROW + 40;
    const HELD_POINTER: u32 = 3280;
    const HELD_LENGTH: u32 = 3284;
    const BOOTSTRAP_RETURNED: u32 = 3288;
    const BOOTSTRAP: u32 = 4000;
    const TRANSITION: u32 = 4400;
    const CONTEXT_CLOSE: u32 = 4700;

    fn address(body: &mut Vec<u8>, pointer: u32) {
        body.push(0x41);
        sleb(i64::from(pointer), body);
    }
    fn store_u32(body: &mut Vec<u8>, pointer: u32, value: u32) {
        address(body, pointer);
        address(body, value);
        body.extend([0x36, 2, 0]);
    }
    fn load_u32(body: &mut Vec<u8>, pointer: u32) {
        address(body, pointer);
        body.extend([0x28, 2, 0]);
    }
    fn retain_header(body: &mut Vec<u8>, pointer: u32, length: u32) {
        store_u32(body, HELD_POINTER, pointer);
        store_u32(body, HELD_LENGTH, length);
    }
    fn publish_header(body: &mut Vec<u8>, pointer: u32, length: u32) {
        address(body, REVISION);
        address(body, REVISION);
        body.extend([0x29, 3, 0, 0x42, 1, 0x7c, 0x37, 3, 0]);
        retain_header(body, pointer, length);
    }
    fn echo_arguments(body: &mut Vec<u8>, pointer: u32, first_argument: u8, words: u8) {
        for index in 0..words {
            address(body, pointer);
            body.extend([0x20, first_argument + index, 0x37, 3]);
            uleb(8 + u64::from(index) * 8, body);
        }
    }
    fn packed(body: &mut Vec<u8>, pointer: u32, length: u32) {
        body.push(0x42);
        sleb((i64::from(length) << 32) | i64::from(pointer), body);
    }
    fn data_segment(data: &mut Vec<u8>, pointer: u32, bytes: &[u8]) {
        data.push(0);
        address(data, pointer);
        data.push(0x0b);
        uleb(bytes.len() as u64, data);
        data.extend(bytes);
    }

    let answer = channel.to_string().into_bytes();
    let (types, bodies): (Vec<u8>, Vec<Vec<u8>>) = OwnedSemioExport::ALL
        .iter()
        .map(|export| match export {
            OwnedSemioExport::Allocate if input_echo||input_receipt!=0 => (0u8, vec![0x00,0x41,0xc0,0x00,0x0b]),
            OwnedSemioExport::Allocate => (0u8, vec![0x00, 0x41, 0x00, 0x0b]),
            OwnedSemioExport::Deallocate if input_receipt == 5 => {
                let mut body = vec![0];
                load_u32(&mut body, HELD_POINTER);
                body.extend([0x20, 0, 0x46]);
                load_u32(&mut body, HELD_LENGTH);
                body.extend([0x20, 1, 0x46, 0x71, 0x04, 0x40]);
                body.extend([0x20, 0]);
                address(&mut body, BOOTSTRAP);
                body.extend([0x46, 0x04, 0x40]);
                store_u32(&mut body, BOOTSTRAP_RETURNED, 1);
                body.push(0x05);
                address(&mut body, ACKNOWLEDGED);
                address(&mut body, REVISION);
                body.extend([0x29, 3, 0, 0x37, 3, 0, 0x0b]);
                retain_header(&mut body, 0, 0);
                body.extend([0x0b, 0x0b]);
                (1, body)
            }
            OwnedSemioExport::Deallocate => (1, vec![0x00, 0x0b]),
            OwnedSemioExport::ChannelVersion => {
                let mut body = vec![0x00];
                if let Some(binding)=clock{body.push(0x42);sleb(binding as i64,&mut body);body.push(0x41);sleb(64,&mut body);body.extend([0x10,0]);}
                body.push(0x42);
                sleb(((answer.len() as i64) << 32) | 1024, &mut body);
                body.push(0x0b);
                (2, body)
            }
            OwnedSemioExport::AdmitActorContext if input_receipt == 5 => {
                let mut body = vec![0];
                load_u32(&mut body, BOOTSTRAP_RETURNED);
                body.extend([0x04, 0x7e]);
                echo_arguments(&mut body, TRANSITION + 8, 0, 11);
                publish_header(&mut body, TRANSITION, 200);
                packed(&mut body, TRANSITION, 200);
                body.push(0x05);
                echo_arguments(&mut body, BOOTSTRAP, 0, 19);
                retain_header(&mut body, BOOTSTRAP, 304);
                packed(&mut body, BOOTSTRAP, 304);
                body.extend([0x0b, 0x0b]);
                (5, body)
            }
            OwnedSemioExport::CloseActorContext if input_receipt == 5 => {
                let mut body = vec![0];
                echo_arguments(&mut body, CONTEXT_CLOSE, 0, 11);
                publish_header(&mut body, CONTEXT_CLOSE, 144);
                packed(&mut body, CONTEXT_CLOSE, 144);
                body.push(0x0b);
                (5, body)
            }
            OwnedSemioExport::AdmitActorInput if input_receipt!=0=>{let mut body=vec![0];if input_receipt==7{body.extend([0x20,20,0x50,0x45,0x04,0x40,0x41]);sleb(2048,&mut body);body.extend([0x41,4,0x3a,0,4,0x41]);sleb(2048,&mut body);body.extend([0x41,2,0x3a,0,5,0x0b]);}for index in 0..11{body.push(0x41);sleb(2048,&mut body);body.extend([0x20,index+2,0x37,3,8+index*8]);}if input_receipt==5{publish_header(&mut body,2048,144);}packed(&mut body,2048,144);body.push(0x0b);(9,body)},
            OwnedSemioExport::AdmitActorInput if input_echo => (9,vec![0x00,0x20,0x01,0xad,0x42,0x20,0x86,0x20,0x00,0xad,0x84,0x0b]),
            OwnedSemioExport::AdmitActorInput=>(9,vec![0x00,0x42,0,0x0b]),
            OwnedSemioExport::Poll if input_receipt>=5=>{let mut body=vec![0];for index in 0..11{body.push(0x41);sleb(2800,&mut body);body.extend([0x20,index,0x37,3,8+index*8]);}if input_receipt==5{publish_header(&mut body,2800,152);}packed(&mut body,2800,152);body.push(0x0b);(5,body)},
            OwnedSemioExport::CloseActorOutput if input_receipt>=5=>{let mut body=vec![0];if input_receipt==6{body.extend([0x20,3,0x41]);sleb(3008,&mut body);body.extend([0x10,0]);}for index in 0..11{body.push(0x41);sleb(2560,&mut body);body.extend([0x20,index,0x37,3,8+index*8]);}if input_receipt==5{publish_header(&mut body,2560,144);}packed(&mut body,2560,144);body.push(0x0b);(5,body)},
            
            OwnedSemioExport::AcknowledgeActorFailure if input_receipt==7=>{let mut body=vec![0];for index in 0..11{body.push(0x41);sleb(3600,&mut body);body.extend([0x20,index+2,0x37,3,16+index*8]);}for index in 0..2{body.push(0x41);sleb(3600,&mut body);body.extend([0x20,index,0x37,3]);uleb(152+u64::from(index)*8,&mut body);}body.push(0x42);sleb((184i64<<32)|3600,&mut body);body.push(0x0b);(10,body)},
            OwnedSemioExport::PublishActorFailure|OwnedSemioExport::ReadActorFailure|OwnedSemioExport::AcknowledgeActorFailure=>(10,vec![0x00,0x00,0x0b]),
            OwnedSemioExport::AcknowledgeActorFailureChunk=>(11,vec![0x00,0x42,0x00,0x0b]),
            OwnedSemioExport::BorrowActorFailure=>(12,vec![0x00,0x00,0x0b]),
            OwnedSemioExport::StartJob|OwnedSemioExport::StepJob|OwnedSemioExport::ReadJobOutput|OwnedSemioExport::AcknowledgeJobOutput|OwnedSemioExport::CancelJob=>(4,vec![0x00,0x00,0x0b]),
            OwnedSemioExport::CloseJobs=>(5,vec![0x00,0x00,0x0b]),
            OwnedSemioExport::BorrowJobOutput=>(6,vec![0x00,0x00,0x0b]),
            OwnedSemioExport::AcknowledgeJobOutputChunk=>(7,vec![0x00,0x00,0x0b]),
            OwnedSemioExport::Checkpoint | OwnedSemioExport::Describe => (2, vec![0x00, 0x00, 0x0b]),
            OwnedSemioExport::Restore | OwnedSemioExport::PackSchemaHash | OwnedSemioExport::Genesis
             | OwnedSemioExport::PrintMirror | OwnedSemioExport::ApplyOps | OwnedSemioExport::ReplayEnvelopes
             | OwnedSemioExport::SqliteSchema | OwnedSemioExport::SqliteExport | OwnedSemioExport::SqliteImport
             | OwnedSemioExport::SqliteRetirement | OwnedSemioExport::SqliteClose
             | OwnedSemioExport::SqliteTakeFile | OwnedSemioExport::SqliteTakePayload => (3, vec![0x00, 0x00, 0x0b]),
            OwnedSemioExport::Poll | OwnedSemioExport::AdmitActorContext
             | OwnedSemioExport::CloseActorContext | OwnedSemioExport::CloseActorOutput => (5, vec![0x00, 0x00, 0x0b]),
        })
        .unzip();
    let mut core = b"\0asm\x01\0\0\0".to_vec();
    let mut signatures=vec![13,0x60,1,0x7f,1,0x7f,0x60,2,0x7f,0x7f,0,0x60,0,1,0x7e,0x60,2,0x7f,0x7f,1,0x7e];
    for count in [20,19,5,22]{signatures.extend([0x60,count]);signatures.extend(std::iter::repeat_n(0x7e,count as usize));signatures.extend([1,0x7e]);}
    signatures.extend([0x60,2,0x7e,0x7f,0,0x60,21,0x7f,0x7f]);signatures.extend(std::iter::repeat_n(0x7e,19));signatures.extend([1,0x7e]);
    for count in [21,23,6]{signatures.extend([0x60,count]);signatures.extend(std::iter::repeat_n(0x7e,count as usize));signatures.extend([1,0x7e]);}
    section(1,signatures,&mut core);
    if clock.is_some()||input_receipt==6{let module="semio:framework/pure@1.0.0";let name="operation-clock-us";let mut imports=vec![1];uleb(module.len()as u64,&mut imports);imports.extend(module.as_bytes());uleb(name.len()as u64,&mut imports);imports.extend(name.as_bytes());imports.extend([0,8]);section(2,imports,&mut core);}
    let mut functions = Vec::new();
    uleb(types.len() as u64, &mut functions);
    functions.extend(&types);
    section(3, functions, &mut core);
    section(5, vec![1, 0x00, 1], &mut core);
    let mut exports = Vec::new();
    uleb(OwnedSemioExport::ALL.len() as u64 + 1, &mut exports);
    exports.push(6);
    exports.extend(b"memory");
    exports.extend([2, 0]);
    for (index, export) in OwnedSemioExport::ALL.iter().enumerate() {
        uleb(export.core_name().len() as u64, &mut exports);
        exports.extend(export.core_name().as_bytes());
        exports.push(0);
        uleb(index as u64+u64::from(clock.is_some()||input_receipt==6), &mut exports);
    }
    section(7, exports, &mut core);
    let mut code = Vec::new();
    uleb(bodies.len() as u64, &mut code);
    for body in &bodies {
        uleb(body.len() as u64, &mut code);
        code.extend(body);
    }
    section(10, code, &mut core);
    let receipt_segments=if input_receipt==7{6}else if input_receipt==5{8}else if input_receipt>=5{5}else if input_receipt!=0{2}else{0};
    let mut data=vec![receipt_segments+1,0,0x41];
    sleb(1024,&mut data);
    data.push(0x0b);
    uleb(answer.len() as u64,&mut data);
    data.extend(answer);
    if input_receipt!=0{let escrow=[128,12,0,0,48,0,0,0];data.extend([0,0x41]);sleb(2048,&mut data);data.push(0x0b);uleb(144,&mut data);data.extend(b"SAC3");data.extend([if input_receipt==7{7}else{4},if input_receipt==7{0}else if input_receipt>=5{3}else{input_receipt&3},0,0]);data.resize(data.len()+128,0);data.extend(escrow);if input_receipt>=5{data.extend([0,0x41]);sleb(2400,&mut data);data.push(0x0b);uleb(4,&mut data);data.extend(b"true");data.extend([0,0x41]);sleb(2560,&mut data);data.push(0x0b);uleb(144,&mut data);data.extend(b"SAC3");data.extend([2,1,0,0]);data.resize(data.len()+128,0);data.extend(escrow);data.extend([0,0x41]);sleb(2800,&mut data);data.push(0x0b);uleb(152,&mut data);data.extend(b"SAC3");data.extend([5,3,0,0]);data.resize(data.len()+128,0);data.extend(escrow);data.extend(2400u32.to_le_bytes());data.extend(4u32.to_le_bytes());}data.extend([0,0x41]);sleb(3200,&mut data);data.push(0x0b);uleb(48,&mut data);data.extend(b"SRE1");data.extend([0;4]);for v in [101u64,1,202,u64::from(input_receipt!=5),0]{data.extend(v.to_le_bytes());}if input_receipt==7{data.extend([0,0x41]);sleb(3600,&mut data);data.push(0x0b);uleb(184,&mut data);data.extend(b"SAF1");data.extend([4,1,0,0]);data.extend(b"SAC3");data.extend([3,0,0,0]);data.resize(data.len()+128,0);data.extend(escrow);data.resize(data.len()+32,0);}}
    if input_receipt == 5 {
        let mut bootstrap = b"SAB1\x01\x00\x30\x01".to_vec();
        bootstrap.resize(160, 0);
        for word in [38u64, 304, 0, 0, 38, 0, 0, 0, 0, 0, 1, 2, 0, 0, 1, 1, 0, 0] {
            bootstrap.extend(word.to_le_bytes());
        }
        data_segment(&mut data, BOOTSTRAP, &bootstrap);
        let mut transition = b"SBT1\x01\x00\xc8\x00SAC3\x01\x01\x00\x00".to_vec();
        transition.resize(144, 0);
        transition.extend(ESCROW.to_le_bytes());
        transition.extend(48u32.to_le_bytes());
        for word in [1u64, 1, 2, 0, 0, 1] {
            transition.extend(word.to_le_bytes());
        }
        data_segment(&mut data, TRANSITION, &transition);
        let mut closed = b"SAC3\x06\x00\x00\x00".to_vec();
        closed.resize(136, 0);
        closed.extend(ESCROW.to_le_bytes());
        closed.extend(48u32.to_le_bytes());
        data_segment(&mut data, CONTEXT_CLOSE, &closed);
    }
    section(11,data,&mut core);
    let mut component = b"\0asm\x0d\0\x01\0".to_vec();
    section(1, core, &mut component);
    component
}

