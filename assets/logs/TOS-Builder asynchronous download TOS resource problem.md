# 关于tokio运行时的开发逻辑

#[tokio::main] 属性告诉编译器将标记的函数作为程序的主函数，并使用 tokio 运行时来管理异步任务的执行。

```rs
#[tokio::main]
pub async fn download(url: &str, tx: &Sender<String>) -> std::result::Result<(), failure::Error> {
    let resp = reqwest::get(url).await?.text().await?;
    tx.send(resp)?;
    Ok(())
}

#[tokio::main]
pub async fn download_as_bytes(
    url: &str,
    tx: &Sender<bytes::Bytes>,
) -> std::result::Result<(), failure::Error> {
    let resp = reqwest::get(url).await?.bytes().await?;
    tx.send(resp)?;
    Ok(())
}
```

### 在函数前面加上`#[tokio::main]`就可以执行这个函数了！！！

这里有日志
```bash
2023-10-03T10:57:44.642374+08:00 [INFO] Current config path: /Users/asklv/Projects/TSOC/TOS-Builder
2023-10-03T10:57:44.642561+08:00 [INFO] Current config version: v2.5.0
2023-10-03T10:57:44.644+08:00 [INFO] url https://github.com/OpenAtomFoundation/TencentOS-tiny/releases/tag/v2.5.0
2023-10-03T10:57:44.647668+08:00 [DEBUG] (1) reqwest::connect: starting new connection: https://github.com/
```

这里打印的也是OK的啊，怎么没有看到文件呢？？？
```bash
2023-10-03T11:07:12.948929+08:00 [DEBUG] (1) reqwest::connect: starting new connection: https://fanyi-cdn.cdn.bcebos.com/
2023-10-03T11:07:13.01827+08:00 [INFO] resp 200 OK
2023-10-03T11:07:13.018729+08:00 [INFO] asdasd
```

emmm好像是路径的问题。。。
```bash
2023-10-03T11:10:54.547536+08:00 [DEBUG] (1) reqwest::connect: starting new connection: https://fanyi-cdn.cdn.bcebos.com/
2023-10-03T11:10:54.619252+08:00 [INFO] content_length 4610
2023-10-03T11:10:54.619489+08:00 [INFO] download to path /Users/asklv/Projects/TSOC/TOS-Builder1.jpg
2023-10-03T11:10:54.620084+08:00 [INFO] asdasd
```

下载之后就可以了。

##### 关于rust的to_string_lossy的方法

to_string_lossy 方法会将输入的字符串引用转换为 Cow<str> 类型，如果字符串包含有效的 UTF-8 数据，则返回一个对原始字符串的借用；如果字符串包含非 UTF-8 数据，则会自动将其转换为有效的 UTF-8 数据，并返回一个新的 String。

##### reqwest中下载进度的查询

```rs
fn download_file(url: &str, path: &str) -> io::Result<()> {
    let client = Client::new();
    let mut res = client.get(url)
        .send()?;

    // 获取文件大小
    let len = res.content_length()
        .unwrap_or(0);

    // 创建进度条
    let pb = ProgressBar::new(len);
    pb.set_style(ProgressStyle::default_bar()
        .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {bytes}/{total_bytes} ({eta})")
        .progress_chars("#>-"));

    // 创建文件
    let mut file = File::create(path)?;

    // 读取响应并更新进度条
    let mut downloaded = 0;
    let mut buf = [0; 8192];
    while let Some(chunk) = res.chunk().unwrap() {
        downloaded += chunk.len();
        pb.set_position(downloaded as u64);
        file.write_all(chunk)?;
    }

    pb.finish_with_message("downloaded");

    Ok(())
}
```

> Ref: https://juejin.cn/s/rust%20reqwest%20progress%20bar

##### 目前下载日志

```bash
2023-10-03T12:13:33.152297+08:00 [INFO] downloaded 19515529/0
2023-10-03T12:13:33.315723+08:00 [INFO] downloaded 19531913/0
2023-10-03T12:13:33.558204+08:00 [INFO] downloaded 19548297/0
2023-10-03T12:13:33.729454+08:00 [INFO] downloaded 19564681/0
2023-10-03T12:13:33.780331+08:00 [INFO] downloaded 19567681/0
2023-10-03T12:13:33.981003+08:00 [INFO] downloaded 19584058/0
2023-10-03T12:13:34.139716+08:00 [INFO] downloaded 19600442/0
```

好像是可以的了，不过这里的头部是`application/zip`不太好获取到文件总体大小，所以可能只能显示动态进度条。

不过使用wget好像也获取不到。。。