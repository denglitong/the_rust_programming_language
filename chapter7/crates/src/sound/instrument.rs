// 模块
// pub 关键字只是使得模块共有可以访问到，但并不使其内容也是共有的
pub mod woodwind {
    pub fn clarinet() {
        println!("clarinet");
        // 使用 super，类似于文件系统中的 .. 开头，表示从父模块开始而不是当前模块
        super::breathe_in();
    }
}

fn breathe_in() {
    println!("breathe_in")
}

// 对于没有 pub 关键字的项，当你从当前模块向"下"看时是私有的，不过当你向"上"看时是共有的
// 再一次想象一下文件系统：如果你没有某个目录的权限，则无法从父目录中查看其内容
// 如果有该目录的权限，则可以查看其中的目录和任何父目录
mod voice {}
