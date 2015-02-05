extern crate rustecla;

pub fn main() {
    let gl = rustecla::new_gl(1024, 2048);

    let prompt = "$ ";
    loop {
        let input = rustecla::get_line(gl, prompt);
        println!("{}",input);
        if input == "exit\n".to_string() {
            break;
        }
    }
}
