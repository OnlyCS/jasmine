const TEST_INPUT: &'static str = "
type K = OtherType;

struct S {
	a: K,
	b: Vec<K>,
}

struct N(K);

fn f<L: K>(a: K, b:L) -> Z {}

enum E {
	A(K),
	B(K),
	C(K),
}

enum G<L: K> {
	A(L),
	B(L),
	C {
		a: L,
		b: L,
	},
	D,
}

trait T<K>: B {
	type L: K = SomeDefaultType;
	const C: Self::L = SomeDefaultValue;
	type D: K;
	const E: Self::D;

	fn f<L: K>(a: K, b:L) -> Z {}
	fn d<L: K>(a: K, b:L) -> Z;
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
