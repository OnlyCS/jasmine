const TEST_INPUT: &'static str = "
type K = __unsafe_java_int;
struct MyStruct<J> {
	other: __unsafe_java_int,
	another: J,
}
";

fn main() {
    match parser::parse(TEST_INPUT) {
        Ok(program) => {
            println!("{:#?}", program);
        }
        Err(e) => {
            panic!("{}", e)
        }
    }
}
