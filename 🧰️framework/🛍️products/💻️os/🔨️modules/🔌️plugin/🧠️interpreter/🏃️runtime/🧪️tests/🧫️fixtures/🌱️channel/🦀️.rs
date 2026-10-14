//! 🌱️ The synthetic current Owned ABI exposes only the real channel result and traps unrelated operations.

use super::OwnedSemioExport;
pub fn owned_channel_guest(channel: u32) -> Vec<u8> {
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
    let answer = channel.to_string().into_bytes();
    let (types, bodies): (Vec<u8>, Vec<Vec<u8>>) = OwnedSemioExport::ALL
        .iter()
        .map(|export| match export {
            OwnedSemioExport::Allocate => (0u8, vec![0x00, 0x41, 0x00, 0x0b]),
            OwnedSemioExport::Deallocate => (1, vec![0x00, 0x0b]),
            OwnedSemioExport::ChannelVersion => {
                let mut body = vec![0x00, 0x42];
                sleb(((answer.len() as i64) << 32) | 1024, &mut body);
                body.push(0x0b);
                (2, body)
            }
            OwnedSemioExport::Checkpoint | OwnedSemioExport::Describe => (2, vec![0x00, 0x00, 0x0b]),
            OwnedSemioExport::StepJob|OwnedSemioExport::ReadJobOutput|OwnedSemioExport::AcknowledgeJobOutput|OwnedSemioExport::CancelJob=>(4,vec![0x00,0x00,0x0b]),
            OwnedSemioExport::CloseJobs=>(5,vec![0x00,0x00,0x0b]),
            OwnedSemioExport::BorrowJobOutput=>(6,vec![0x00,0x00,0x0b]),
            OwnedSemioExport::AcknowledgeJobOutputChunk=>(7,vec![0x00,0x00,0x0b]),
            _ => (3, vec![0x00, 0x00, 0x0b]),
        })
        .unzip();
    let mut core = b"\0asm\x01\0\0\0".to_vec();
    let mut signatures=vec![8,0x60,1,0x7f,1,0x7f,0x60,2,0x7f,0x7f,0,0x60,0,1,0x7e,0x60,2,0x7f,0x7f,1,0x7e];
    for count in [12,11,5,14]{signatures.extend([0x60,count]);signatures.extend(std::iter::repeat_n(0x7e,count as usize));signatures.extend([1,0x7e]);}
    section(1,signatures,&mut core);
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
        uleb(index as u64, &mut exports);
    }
    section(7, exports, &mut core);
    let mut code = Vec::new();
    uleb(bodies.len() as u64, &mut code);
    for body in &bodies {
        uleb(body.len() as u64, &mut code);
        code.extend(body);
    }
    section(10, code, &mut core);
    let mut data = vec![1, 0x00, 0x41];
    sleb(1024, &mut data);
    data.push(0x0b);
    uleb(answer.len() as u64, &mut data);
    data.extend(&answer);
    section(11, data, &mut core);
    let mut component = b"\0asm\x0d\0\x01\0".to_vec();
    section(1, core, &mut component);
    component
}
