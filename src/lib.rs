

#[derive(Debug, PartialEq)]
pub struct StrTree<'a> {
    elmt: &'a str,
    children: Vec<StrTree<'a>>
}

impl<'a> StrTree<'a> {
    pub const fn new(elmt: &'a str, children: Vec<StrTree<'a>>) -> Self {
        StrTree {
            elmt, children
        }
    }
}

#[macro_export]
macro_rules! str_tree_1 {
    ($name:expr => $($children:expr),+ $(,)? ) => {
        StrTree::new($name, vec!($($children,)+))
    };
    ($name:expr) => {
        StrTree::new($name, vec!())
    };
}

#[macro_export]
macro_rules! str_tree_2 {
    // starting point
    ( $e:expr $( => { $($rek:tt)* } )? ) => {
        StrTree::new($e, str_tree_2!(@LIST {} {$($($rek)*)?}))
    };

    // recursive case
    (@LIST {$($done:expr),*} { $e:expr => {$($rek:tt)*} $(, $($tail:tt)* )? } ) => {
        str_tree_2!(@LIST { $($done,)* StrTree::new($e, str_tree_2!(@LIST {} {$($rek)*})) } {$($($tail)*)?} )
    };

    // simple case
    (@LIST {$($done:expr),*} { $e:expr $(, $($tail:tt)* )? } ) => {
        str_tree_2!(@LIST { $($done,)* StrTree::new($e, vec!()) } {$($($tail)*)?} )
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
        
        StrTree::new("a", vec!(
            StrTree::new("b", vec!()),
            StrTree::new("c", vec!(
                StrTree::new("d", vec!(
                    StrTree::new("e", vec!())
                ))
            )),
            StrTree::new("f", vec!()),
        ))
    }
    

    #[test]
    fn test_macro_1() {
        // simple flat case
        let st = str_tree_1!("a");
        assert_eq!(st, StrTree::new("a", vec!()));

        // trailing comma
        let st = str_tree_1!("a" => str_tree_1!("b"), );
        assert_eq!(st, StrTree::new("a", vec!(StrTree::new("b", vec!()))));

        // complete example
        
        let exp = gen_example_structure();
        
        let macro_1 = str_tree_1!("a" => 
            str_tree_1!("b"),
            str_tree_1!("c" => 
                str_tree_1!("d" => 
                    str_tree_1!("e")
                )),
            str_tree_1!("f")
        );

        assert_eq!(exp, macro_1);

    }

    #[test]
    fn test_macro_2() {
        // simple flat case
        let st = str_tree_2!("a");
        assert_eq!(st, StrTree::new("a", vec!()));

        // empty braces
        let st = str_tree_2!("a" => {});
        assert_eq!(st, StrTree::new("a", vec!()));

        // no trailing comma
        let st = str_tree_2!("a" => { "b" });
        assert_eq!(st, StrTree::new("a", vec!(StrTree::new("b", vec!()))));

        // trailing comma
        let st = str_tree_2!("a" => { "b", "c", });
        assert_eq!(st, StrTree::new("a", vec!(
            StrTree::new("b", vec!()),
            StrTree::new("c", vec!()),
        )));

        // complete example
        
        let exp = gen_example_structure();

        let macro_2 = str_tree_2!(
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

