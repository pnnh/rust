extern crate cc;

fn main() {
    cc::Build::new()
        .cpp(false)
        .file("src/math.c")
        .compile("calculator");
}