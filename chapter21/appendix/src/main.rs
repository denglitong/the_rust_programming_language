/// as
/// break
/// const
/// continue
/// crate
/// dyn
/// else
/// enum
/// extern
/// false
/// fn
/// for
/// if
/// impl
/// in
/// let
/// loop
/// match
/// mod
/// move
/// mut
/// pub
/// ref
/// return
/// Self
/// self
/// static
/// struct
/// trait
/// true
/// type
/// unsafe
/// use
/// where
/// while

/// abstract
/// async
/// become
/// box
/// do
/// final
/// macro
/// override
/// priv
/// try
/// typeof
/// unsized
/// virtual
/// yield
///
/// !

// 原始字符串允许你使用任何单词作为标识符，即使是关键字
fn r#match() {
    println!("fn declare match");
}

fn main() {
    //! 定义和调用的地方都需要带上 r#
    /**/
    /*!
    inside comment
    */
    /**
    outside comment
     */
    r#match();

    for _i in 0..10 {
        do_something();
    }
}

fn do_something() {}

// 栈上 Copy, 堆上 Clone
