# Rust一些简单的测试和配置

### 安装Rust

- https://www.rust-lang.org/zh-CN/tools/install

```bash
asklv@qdht-SYS-7049GP-TRT:~/Project/tsoc/rust-test$ curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
info: downloading installer

Welcome to Rust!

This will download and install the official compiler for the Rust
programming language, and its package manager, Cargo.

Rustup metadata and toolchains will be installed into the Rustup
home directory, located at:

  /home/asklv/.rustup

This can be modified with the RUSTUP_HOME environment variable.

The Cargo home directory is located at:

  /home/asklv/.cargo

This can be modified with the CARGO_HOME environment variable.

The cargo, rustc, rustup and other commands will be added to
Cargo's bin directory, located at:

  /home/asklv/.cargo/bin

This path will then be added to your PATH environment variable by
modifying the profile files located at:

  /home/asklv/.profile
  /home/asklv/.bashrc
  /home/asklv/.zshenv

You can uninstall at any time with rustup self uninstall and
these changes will be reverted.

Current installation options:


   default host triple: x86_64-unknown-linux-gnu
     default toolchain: stable (default)
               profile: default
  modify PATH variable: yes

1) Proceed with installation (default)
2) Customize installation
3) Cancel installation
>

info: profile set to 'default'
info: default host triple is x86_64-unknown-linux-gnu
info: syncing channel updates for 'stable-x86_64-unknown-linux-gnu'
781.5 KiB / 781.5 KiB (100 %) 271.4 KiB/s in  2s ETA:  0s
info: latest update on 2023-07-13, rust version 1.71.0 (8ede3aae2 2023-07-12)
info: downloading component 'cargo'
  7.0 MiB /   7.0 MiB (100 %) 416.0 KiB/s in 15s ETA:  0s
info: downloading component 'clippy'
  2.3 MiB /   2.3 MiB (100 %)   1.4 MiB/s in  1s ETA:  0s
info: downloading component 'rust-docs'
 13.6 MiB /  13.6 MiB (100 %) 908.8 KiB/s in 24s ETA:  0s
info: downloading component 'rust-std'
 25.4 MiB /  25.4 MiB (100 %)   2.3 MiB/s in 10s ETA:  0s
info: downloading component 'rustc'
 64.0 MiB /  64.0 MiB (100 %)   1.3 MiB/s in 52s ETA:  0s    
info: downloading component 'rustfmt'
  2.3 MiB /   2.3 MiB (100 %)   1.7 MiB/s in  1s ETA:  0s
info: installing component 'cargo'
info: installing component 'clippy'
info: installing component 'rust-docs'
 13.6 MiB /  13.6 MiB (100 %)   6.6 MiB/s in  1s ETA:  0s
info: installing component 'rust-std'
 25.4 MiB /  25.4 MiB (100 %)  10.1 MiB/s in  2s ETA:  0s
info: installing component 'rustc'
 64.0 MiB /  64.0 MiB (100 %)  11.7 MiB/s in  5s ETA:  0s
info: installing component 'rustfmt'
info: default toolchain set to 'stable-x86_64-unknown-linux-gnu'

  stable-x86_64-unknown-linux-gnu installed - rustc 1.71.0 (8ede3aae2 2023-07-12)


Rust is installed now. Great!

To get started you may need to restart your current shell.
This would reload your PATH environment variable to include
Cargo's bin directory ($HOME/.cargo/bin).

To configure your current shell, run:
source "$HOME/.cargo/env"
```

### 配置vscode

安装配置环境：

安装插件：用同样的方法再安装 rust-analyzer 和 Native Debug 两个扩展。

由于这个配置文件用的是toml，所以本地也安装了toml的插件`Better TOML`

### 下载依赖卡住了

因为 VSCODE 的下载太慢了，而且该下载构建还锁住了当前的项目，导致你无法在另一个地方再次进行构建。

```bash
asklv@qdht-SYS-7049GP-TRT:~/Project/tsoc/rust-test/test-download$ cargo run
    Blocking waiting for file lock on package cache
```

增加下载速度，见前面内容
耐心等待持有锁的用户构建完成
- 强行停止正在构建的进程，例如杀掉 IDE 使用的 rust-analyzer 插件进程，然后删除 $HOME/.cargo/.package_cache 目录

> Ref: https://course.rs/first-try/slowly-downloading.html#%E4%B8%8B%E8%BD%BD%E5%8D%A1%E4%BD%8F

### 增加国内镜像

直接使用新注册服务来替代默认的

在 $HOME/.cargo/config.toml 添加以下内容：

首先，创建一个新的镜像源 [source.ustc]，然后将默认的 crates-io 替换成新的镜像源: replace-with = 'ustc'。

```toml
[source.crates-io]
replace-with = 'ustc'

[source.ustc]
registry = "git://mirrors.ustc.edu.cn/crates.io-index"
```

```bash
  Downloaded bitflags v1.3.2 (registry `ustc`)
  Downloaded cc v1.0.79 (registry `ustc`)
  Downloaded log v0.4.19 (registry `ustc`)
  Downloaded openssl-probe v0.1.5 (registry `ustc`)
  Downloaded percent-encoding v2.3.0 (registry `ustc`)
  Downloaded libssh2-sys v0.2.23 (registry `ustc`)
  Downloaded openssl-sys v0.9.90 (registry `ustc`)
  Downloaded pkg-config v0.3.27 (registry `ustc`)
  Downloaded form_urlencoded v1.2.0 (registry `ustc`)
  Downloaded libz-sys v1.1.9 (registry `ustc`)
  Downloaded idna v0.4.0 (registry `ustc`)
  Downloaded git2 v0.13.25 (registry `ustc`)
  Downloaded jobserver v0.1.26 (registry `ustc`)
  Downloaded libc v0.2.147 (registry `ustc`)
  Downloaded tinyvec v1.6.0 (registry `ustc`)
  Downloaded unicode-bidi v0.3.13 (registry `ustc`)
  Downloaded tinyvec_macros v0.1.1 (registry `ustc`)
  Downloaded libgit2-sys v0.12.26+1.3.0 (registry `ustc`)
  Downloaded unicode-normalization v0.1.22 (registry `ustc`)
  Downloaded url v2.4.0 (registry `ustc`)
  Downloaded 20 crates (6.1 MB) in 7.65s (largest was `libz-sys` at 2.4 MB)
   Compiling libc v0.2.147
   Compiling pkg-config v0.3.27
   Compiling tinyvec_macros v0.1.1
   Compiling percent-encoding v2.3.0
   Compiling unicode-bidi v0.3.13
   Compiling bitflags v1.3.2
   Compiling log v0.4.19
   Compiling openssl-probe v0.1.5
   Compiling tinyvec v1.6.0
   Compiling form_urlencoded v1.2.0
   Compiling unicode-normalization v0.1.22
   Compiling jobserver v0.1.26
   Compiling cc v1.0.79
   Compiling idna v0.4.0
   Compiling url v2.4.0
   Compiling openssl-sys v0.9.90
   Compiling libz-sys v1.1.9
   Compiling libssh2-sys v0.2.23
   Compiling libgit2-sys v0.12.26+1.3.0
   Compiling git2 v0.13.25
   Compiling test-download v0.1.0 (/home/asklv/Project/tsoc/rust-test/test-download)
error[E0283]: type annotations needed
   --> src/main.rs:30:12
    |
30  |     remote.fetch(&[], Some(&mut fetch_options), None)?;
    |            ^^^^^ --- type must be known at this point
    |            |
    |            cannot infer type of the type parameter `Str` declared on the method `fetch`
    |
    = note: cannot satisfy `_: AsRef<str>`
note: required by a bound in `git2::Remote::<'repo>::fetch`
   --> /home/asklv/.cargo/registry/src/mirrors.ustc.edu.cn-61ef6e0cd06fb9b8/git2-0.13.25/src/remote.rs:276:23
    |
276 |     pub fn fetch<Str: AsRef<str> + crate::IntoCString + Clone>(
    |                       ^^^^^^^^^^ required by this bound in `Remote::<'repo>::fetch`
help: consider specifying the generic argument
    |
30  |     remote.fetch::<Str>(&[], Some(&mut fetch_options), None)?;
    |                 +++++++

For more information about this error, try `rustc --explain E0283`.
error: could not compile `test-download` (bin "test-download") due to previous error
```

### 显示信息报错

```rs
error[E0283]: type annotations needed
   --> src/main.rs:30:12
    |
30  |     remote.fetch(&[], Some(&mut fetch_options), None)?;
    |            ^^^^^ --- type must be known at this point
    |            |
    |            cannot infer type of the type parameter `Str` declared on the method `fetch`
    |
    = note: cannot satisfy `_: AsRef<str>`
note: required by a bound in `git2::Remote::<'repo>::fetch`
   --> /home/asklv/.cargo/registry/src/mirrors.ustc.edu.cn-61ef6e0cd06fb9b8/git2-0.13.25/src/remote.rs:276:23
    |
276 |     pub fn fetch<Str: AsRef<str> + crate::IntoCString + Clone>(
    |                       ^^^^^^^^^^ required by this bound in `Remote::<'repo>::fetch`
help: consider specifying the generic argument
    |
30  |     remote.fetch::<Str>(&[], Some(&mut fetch_options), None)?;
    |                 +++++++
```

这个错误是由于Git2-rs库在fetch方法上定义了一个泛型参数Str，它需要满足AsRef<str> trait的约束。但是在你的代码中，调用fetch方法时没有指定泛型参数的具体类型，导致编译器无法推断出Str的类型。

要解决这个问题，你需要在调用fetch方法时显式地指定泛型参数的类型。根据代码逻辑，这里泛型参数Str应该是&str类型。

### 在这里面测试了文件下载和tui之类的

```bash
/home/asklv/Project/tsoc/rust-test
```

- `test-zip`解压也是可以的，但是注意这个会覆盖当前的目录

### 动态遍历目录树的库

- 就是用的walkdir的方式

### 支持命令行编译的一些参考资料

这里给出的是keil的命令行。

> Ref: https://developer.arm.com/documentation/101407/0538/Command-Line

这里给出的是IAR的命令行构建方式。

> Ref: https://www.iar.com/knowledge/support/technical-notes/general/build-from-the-command-line/

或者都使用CMake的方式来构建，这种的也可以。

> Ref: https://zhuanlan.zhihu.com/p/508740829

> Ref: https://github.com/IARSystems/cmake-tutorial

这里的部分是将系统更改为Makefile的方式，使用ArmCC和Armlink工具进行编译和链接：

Keil使用ArmCC编译源文件，使用Armlink链接目录文件生成efl文件，然后调用fromelf转换为bin文件。
那么ArmCC和Armlink如何使用呢？Keil提供一种方法来展示如何使用ArmCC和Armlink。

> Ref: https://blog.csdn.net/feihe0755/article/details/127329778

构建系统支持将Keil工程生成makefile的工程。下面还给出了repo，真是一个大善人。

> Ref: https://github.com/Qrpucp/Keil2Makefile
> Ref: https://blog.csdn.net/weixin_45467056/article/details/123564509

生成各种版本的工程，通过python

> Ref: https://github.com/project-generator/project_generator

几种命令行编译的方式。

> Ref: https://www.cnblogs.com/memorypro/p/9562919.html

最后就是可以参考CLion的方式，这个编辑可以声称CMakeLists文件啊，也可以考虑都转换为CMake的工程，那个生成makefile的就先不管了。

> Ref: https://zhuanlan.zhihu.com/p/160183640