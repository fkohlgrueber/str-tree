#[derive(Debug, PartialEq)]
pub struct StrTree<'a> {
    pub elmt: &'a str,
    pub children: Vec<StrTree<'a>>,
}

impl<'a> StrTree<'a> {
    pub const fn new(elmt: &'a str, children: Vec<StrTree<'a>>) -> Self {
        StrTree { elmt, children }
    }
}

#[allow(unused_macros)]
macro_rules! str_tree_simple {
    ($name:expr => $($children:expr),+ $(,)? ) => {
        StrTree::new($name, vec!($($children,)+))
    };
    ($name:expr) => {
        StrTree::new($name, vec!())
    };
}

#[macro_export]
macro_rules! str_tree {
    // starting point
    ( $e:expr $( => { $($rek:tt)* } )? ) => {
        StrTree::new($e, str_tree!(@LIST {} {$($($rek)*)?}))
    };

    // recursive case
    (@LIST {$($done:expr),*} { $e:expr => {$($rek:tt)*} $(, $($tail:tt)* )? } ) => {
        str_tree!(@LIST { $($done,)* StrTree::new($e, str_tree!(@LIST {} {$($rek)*})) } {$($($tail)*)?} )
    };

    // simple case
    (@LIST {$($done:expr),*} { $e:expr $(, $($tail:tt)* )? } ) => {
        str_tree!(@LIST { $($done,)* StrTree::new($e, vec!()) } {$($($tail)*)?} )
    };

    // end of recursion
    (@LIST {$($done:expr),*} {} ) => {
        vec!( $($done),* )
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    fn gen_example_structure() -> StrTree<'static> {
        // Example structure
        // a
        //   b
        //   c
        //     d
        //       e
        //   f

        StrTree::new(
            "a",
            vec![
                StrTree::new("b", vec![]),
                StrTree::new(
                    "c",
                    vec![StrTree::new("d", vec![StrTree::new("e", vec![])])],
                ),
                StrTree::new("f", vec![]),
            ],
        )
    }

    #[test]
    fn test_macro_simple() {
        // simple flat case
        let st = str_tree_simple!("a");
        assert_eq!(st, StrTree::new("a", vec!()));

        // trailing comma
        let st = str_tree_simple!("a" => str_tree_simple!("b"), );
        assert_eq!(st, StrTree::new("a", vec!(StrTree::new("b", vec!()))));

        // complete example

        let exp = gen_example_structure();

        let macro_1 = str_tree_simple!("a" =>
            str_tree_simple!("b"),
            str_tree_simple!("c" =>
                str_tree_simple!("d" =>
                    str_tree_simple!("e")
                )),
            str_tree_simple!("f")
        );

        assert_eq!(exp, macro_1);
    }

    #[test]
    fn test_macro() {
        // simple flat case
        let st = str_tree!("a");
        assert_eq!(st, StrTree::new("a", vec!()));

        // empty braces
        let st = str_tree!("a" => {});
        assert_eq!(st, StrTree::new("a", vec!()));

        // no trailing comma
        let st = str_tree!("a" => { "b" });
        assert_eq!(st, StrTree::new("a", vec!(StrTree::new("b", vec!()))));

        // trailing comma
        let st = str_tree!("a" => { "b", "c", });
        assert_eq!(
            st,
            StrTree::new(
                "a",
                vec!(StrTree::new("b", vec!()), StrTree::new("c", vec!()),)
            )
        );

        // complete example

        let exp = gen_example_structure();

        let macro_2 = str_tree!(
            "a" => {
            "b",
            "c" => {
                "d" => {
                    "e",
                },
            },
            "f",
            }
        );

        assert_eq!(exp, macro_2);
    }
}
