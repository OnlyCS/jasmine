pub use crate::errors::*;
pub use libjasmine::prelude::*;
pub use proc_macro2::{Delimiter, TokenStream, TokenTree};
pub use std::collections::HashMap;
pub use std::iter::Peekable;
pub use std::sync::Arc;

macro_rules! bail {
    ($err:expr) => {
        return Err($err.into())
    };
}

macro_rules! expect_mac {
    (on $tree:expr, $expected:pat, $check:block, $ret:block) => {
        match $tree {
            $expected => {
                if !$check {
                    bail!(SyntaxError::ExpectWithCheck(
                        stringify!($expected),
                        stringify!($check)
                    ))
                } else {
                    $ret
                }
            }
            _ => bail!(SyntaxError::ExpectWithCheck(
                stringify!($expected),
                stringify!($check)
            )),
        }
    };

	(on $tree:expr, $expected:pat, ret $ret:block) => {
		expect!(on $tree, $expected, { true }, $ret)
	};

	(on $tree:expr, $expected:pat, chk $check:block) => {
		expect!(on $tree, $expected, $check, {})
	};

	(on $tree:expr, $expected:pat) => {
		expect!(on $tree, $expected, { true }, {})
	};

    ($tree:expr, $expected:pat, $check:block, $ret:block) => {
		#[allow(unused)]
		match $tree.peek() {
			Some($expected) => {
				if !$check {
					bail!(SyntaxError::ExpectWithCheck(
						stringify!($expected),
						stringify!($check)
					))
				} else {
					let Some($expected) = $tree.next() else { panic!() };
					$ret
				}
			}
			_ => bail!(SyntaxError::ExpectWithCheck(
				stringify!($expected),
				stringify!($check)
			)),
		}
	};

	($tree:expr, $expected:pat, ret $ret:block) => {
		expect!($tree, $expected, { true }, $ret)
	};

	($tree:expr, $expected:pat, chk $check:block) => {
		expect!($tree, $expected, $check, {})
	};

	($tree:expr, $expected:pat) => {
		expect!($tree, $expected, { true }, {})
	};
}

pub(crate) use bail;
pub(crate) use expect_mac as expect;