use CreateProcessW::Command;

#[ctor::ctor]
fn ctor(){
    let child = Command::new("cmd.exe")
        .spawn()
        .expect("cmd failed to start");
}