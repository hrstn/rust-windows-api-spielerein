use dll_syringe::{Syringe, process::OwnedProcess};
use std::env;

fn print_msg(msg: &str, ok: u32){
    match ok{
        0=>println!("[+] {}",msg),
        1=>println!("[-] {}",msg),
        _=>println!("[~] {}",msg),
    }
}
fn get_shit_done(application: &str, dll_path: &str){
    print_msg(&format!("injection the dll in: {:?}",application.to_string()).to_string(),0);
    let target_process = OwnedProcess::find_first_by_name(application.to_string()).unwrap();
    let syringe = Syringe::for_process(target_process);
    let injected_payload = syringe.inject(dll_path).unwrap();
    syringe.eject(injected_payload).unwrap();
}
fn main(){

    let args: Vec<String> = env::args().collect();

    print_msg("r0st dll injection demo",0);

    if args.len() >= 3 {
        let argv1 = &args[1];
        let argv2 = &args[2];
        get_shit_done(argv1,argv2);
    } else {
        print_msg("usage: program.exe <application_name> <dll_path>",1);
    }
}