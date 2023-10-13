# 关于tencentos插件的开发

整数类型（Integer）：i8、i16、i32、i64、i128，分别表示有符号的8位、16位、32位、64位和128位整数；u8、u16、u32、u64、u128，分别表示无符号的8位、16位、32位、64位和128位整数；isize 和 usize，表示与当前平台相关的有符号和无符号整数类型。

### 关于rust计算百分比

```bash
.percent(((app.download.current as f64 / app.download.total as f64) * 100.0) as u16)
```

注意，需要转换成为f64才能计算，否则回出现问题！！！

### 使用conten_length的单位

```bash
单位是字节（bytes）!!!
```

# 关于时间操作的步骤

使用的chrono库。

> Ref: https://zhuanlan.zhihu.com/p/622979623

### 下载好像成功了，还算比较快。。。

```bash
2023-10-06T17:28:17.125646+08:00 [DEBUG] (1) tosbuilder::utils::downloader: downloaded 297088514/335544320 cost(35 s), 
2023-10-06T17:28:17.125844+08:00 [DEBUG] (1) tosbuilder::utils::downloader: downloaded 297104898/335544320 cost(35 s), 
2023-10-06T17:28:17.126027+08:00 [DEBUG] (1) tosbuilder::utils::downloader: downloaded 297107905/335544320 cost(35 s), 
2023-10-06T17:28:17.126249+08:00 [DEBUG] (1) tosbuilder::utils::downloader: downloaded 297118082/335544320 cost(35 s), 
2023-10-06T17:28:17.126392+08:00 [INFO] download to path /Users/asklv/Projects/TSOC/TOS-Builder/v2.5.0.zip ok
2023-10-06T17:28:17.283781+08:00 [DEBUG] (1) tosbuilder::handler: Activate modules: TOSDownload(Version) Key event: KeyEvent { code: Char('c'), modifiers: CONTROL, kind: Press, state: NONE }
```

### 现在下载终于正常了啊

把数据串行化了，不过这样起码可以跑起来了，那个异步之类的还是有点问题的。。。