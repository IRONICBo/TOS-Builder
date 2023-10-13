# 包名和包文件的组织

### 无法定义和组织包文件？

需要在顶层定义一下子module的位置，同时还需要在下面一层组织module。

```rs
// main.rs

mod xxx;
mod xxx;
```

```rs
// 子目录文件

mod xxx; // 导出
```

### 无法引用子目录的文件

- 在lib.rs中引用导出这些modules，用于后续使用
```rs
// Pages.
mod pages;

// Handlers.
mod handlers;
```

然后在其他目录中就可以使用`use xxx`来导入模块了。

### 几个基本模块

```rs
/// Tab key.
Tab,
/// Shift + Tab key.
BackTab,
```

按键映射，可以使用的方式。

### 布局方式

```rs
let chunks = Layout::default()
    .direction(Direction::Vertical)
    .margin(1)
    .constraints(
        [
            Constraint::Percentage(10), // header
            Constraint::Percentage(89), // body
            Constraint::Percentage(1), // footer
        ]
        .as_ref(),
    )
    .split(frame.size());
```
好像是从上往下压缩的，所以最下面的分配1也可以显示的。

### 创建的流程

选择创建的好的MDK文件路径。

- 创建的操作都是在最后执行的，前面的都是配置界面
- 