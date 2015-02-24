extern crate rustecla;

pub fn main() {
    let gl = rustecla::new_gl(1024, 2048);

    println!("Hi there");
    let prompt = "$ ";
    loop {
        println!("Made it here");
        let input = rustecla::get_line(gl, prompt);
        println!("{}",input);
        if input == "exit\n".to_string() {
            break;
        } else if input == "clear\n".to_string() {
            rustecla::clear(gl);
        }
    }
//    rustecla::query_char(gl, "Really quit? [Y/n] ", 'y');
    rustecla::del_gl(gl);
}
