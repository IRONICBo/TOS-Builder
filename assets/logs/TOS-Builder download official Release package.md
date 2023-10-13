# 关于TOS的实际逻辑开发

### TOS的Release文件下载，（以及检测目录是否正确下载）

中间可能需要什么进度条之类的。

##### 测试打印日志

```bash
ActiveModules::TOSDownload(crate::app::TOSDownload::Version) => {
    // Start download and download to the config path
    println!("Start download")
}
```

测试日志显示，可以使用库来实现。有env_logger、simple_logger、simplelog，感觉simplelog会比较适合。

env_logger可能会更强大一点吧，但是目前还是先用着`simplelog = "^0.12.0"`

emmm感觉有点太老了，不太好使，还是老实用`env_logger`吧。

##### 添加信息。。。

emmm，好像用simple log也可以hhh，主要是可以打印到文件里面去，主要是要引入`log`顶层库！！！

其他模块要用的话，就是使用
```rs
use log::info;
```

然后在`main.rs`中配置模块
```rs
#[macro_use]
extern crate log;
```

然后直接使用`info!`之类的就可以了。。。

暂存一下：
```rs
use simplelog::{ConfigBuilder, LevelFilter, WriteLogger, Config};
use std::fs::File;

pub fn init_logger() {
    // Set default config
    let config = Config::default();

    // Create config file
    let file_name = "tos_builder.log";
    let log_file = File::create(file_name).expect("Cannot create log file");

    // TODO: Set dynamic log level
    WriteLogger::init(LevelFilter::Info, config, log_file).expect("Cannot init logger");
}
```

> 关于时间戳的格式：Rfc2822 和 Rfc3339 是两种不同的日期和时间格式标准。Rfc2822，也称为"Internet Message Format"，定义了一种用于表示电子邮件消息的日期和时间格式。Day, DD Mon YYYY HH:MM:SS ±ZZZZ Rfc3339，也称为"Date and Time on the Internet: Timestamps"，是一种更现代化和严格的日期和时间格式标准。它的日期和时间格式如下：YYYY-MM-DDTHH:MM:SS±hh:mm

啊啊啊啊其实可以的！！！ simplelog！！！
```bash
2023-10-03T00:23:54.663352+08:00 [DEBUG] (1) tosbuilder::handler: Activate modules: ProjectSelect(Fs) Key event: KeyEvent { code: Char('d'), modifiers: NONE, kind: Press, state: NONE }
```

debug模式自动会写入文件位置和行号码！！！非常便捷！！！

好像还是不太行啊啊啊啊，这里的target只有标准输入输出流，没有文件的。。。不过应该可以强制写入文件，框架不支持就先不管了。

关于一些rust库的选型：
> Ref: https://blog.csdn.net/s_lisheng/article/details/78250340

还有高度可配置的log库，目前还是以实现为主，这里反正都是宏可以后期更换的，先用着simplelog吧。
> Ref: https://github.com/estk/log4rs

##### 关于下载部分的实现

这里涉及到了异步函数的编写，需要注意一下～

```bash
调用 download_tos 函数，并使用 await 关键字等待异步函数的结果。
```

### 弹出弹窗，用于修改key value的值

目前弹窗是有的，主要是修改输入数据然后保存，目前暂定都是字符类型的吧。

### 配置文件的缓存，存储成为json文件之类的

默认会读取当前目录下面的`config.json`文件，然后会反序列化配置。

### 下载文件到本地

使用`reqwest`的get方法请求文件，并且下载到本地文件夹下。

> Ref: https://juejin.cn/s/rust%20reqwest%20%E4%B8%8B%E8%BD%BD%E6%96%87%E4%BB%B6
