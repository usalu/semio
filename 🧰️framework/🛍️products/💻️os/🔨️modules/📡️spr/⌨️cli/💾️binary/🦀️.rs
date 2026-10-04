//! ⌨️ Native command callers own finite admission, progress and optional stdin cancellation.
fn main(){
 use semio_framework_os_kernel::os_pack::control::{CancelToken,admit_command_transport,configure_stdin_cancellation,print_command_progress,print_command_input_progress,CommandContext};
 let mut args:Vec<String>=std::env::args().skip(1).collect();let cancellation=CancelToken::root_now();
 let context=match admit_command_transport(semio_framework_os_kernel::os_spr::ProtocolLimits::default().max_total_alloc,cancellation.clone(),print_command_progress){Ok(context)=>context,Err(error)=>{eprintln!("transport admission: {error}");std::process::exit(1)}};
 let command=match CommandContext::try_new(context,semio_framework_os_kernel::os_spr::ProtocolLimits::default().max_total_alloc,cancellation.clone(),print_command_input_progress){Ok(command)=>command,Err(error)=>{eprintln!("input admission: {error}");std::process::exit(1)}};
 if let Err(error)=configure_stdin_cancellation(&mut args,command.transport(),cancellation){eprintln!("cancel input: {error}");std::process::exit(1)}
 std::process::exit(semio_framework_async::block_on(semio_framework_os_kernel::os_spr::cli::main_impl(&args,&command)));
}
