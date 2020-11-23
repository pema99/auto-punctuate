use enigo::*;
use clipboard::*;
use hotkey::*;
use percent_encoding::*;

const FRAGMENT: &AsciiSet = &CONTROLS.add(b' ').add(b'"').add(b'<').add(b'>').add(b'`');

fn main() {
    let mut hk = Listener::new();
    hk.register_hotkey(
        modifiers::CONTROL | modifiers::SHIFT,
        'Z' as u32,
        || {
            let mut enigo = Enigo::new();
            let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();

            enigo.key_sequence_parse("{+CTRL}ac{-CTRL}");
        
            let text = ctx.get_contents().unwrap();

            let response_bytes = std::process::Command::new("curl")
                .arg("-d")
                .arg(format!("text={}", utf8_percent_encode(&text, FRAGMENT).to_string()))
                .arg("http://bark.phon.ioc.ee/punctuator")
                .output()
                .expect("failed to execute process")
                .stdout;

            let punctuated = String::from_utf8_lossy(&response_bytes).trim().to_owned();

            ctx.set_contents(punctuated).unwrap();
        
            enigo.key_sequence_parse("{+CTRL}v{-CTRL}");
        },
    )
    .unwrap();

    hk.listen();
}
