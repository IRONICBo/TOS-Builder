# TOSbuilder快速开发

### 可参考的文件浏览器仓库

> Ref: https://github.com/sayanarijit/xplr

### 关于rustfmt的用法

可以参考的用法，在crate下面新增一个`.rustfmt.toml`之类的配置，然后直接运行：

在命令行输入 rustfmt filename即可直接格式化对应的Rust源文件，也可以直接输入 cargo fmt格式化整个crate。

> Ref: https://blog.csdn.net/qq_45090256/article/details/125476170

### 关于文件浏览和文件传输

还有下面的标注栏之类的。

> Ref: https://github.com/veeso/termscp.git

### 解释一下rust这部分的用法

这段代码使用了一个闭包作为参数传递给 terminal.draw() 方法。闭包的定义开始于 |f|，表示闭包接收一个名为 f 的参数。在闭包的内部，执行了一些绘制操作。

```rs
terminal.draw(|f| {
let size = f.size();
let block = Block::default()
.title("Block")
.borders(Borders::ALL);
f.render_widget(block, size);
})?;
```

### 这里给了一些效果展示

给出了一些例子，可以参考并且选择需要的组件。

> Ref: https://github.com/ratatui-org/ratatui/tree/main/examples