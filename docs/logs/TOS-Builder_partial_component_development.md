# 修复select的逻辑

### 切换当前路径的操作

```rs
// 这里可以切换当前路径到dir下面
set_current_dir(dir).unwrap();
```

### 使用`to_string_lossy`的用法

Cow 是 "clone on write" 的缩写，意味着在需要修改字符串时才会进行复制操作，否则会直接使用原始数据的借用。

使用 Cow<str> 类型可以在需要修改字符串时进行复制，而在不需要修改时直接使用原始数据的引用，从而提供了更高效的字符串处理方式。

### 关于路径的处理

这里==visit有一个`break`导致路径查找不全面==！！！

```rs
fn visit_dir(&mut self, path: &str) -> Result<(Vec<DirEntry>, Vec<DirEntry>), Box<dyn Error>> {
    let path = Path::new(path);
    let mut dir_entries = vec![];
    let mut file_entries = vec![];
    match path.is_dir() {
        true => {
            for entry in fs::read_dir(path)? {
                match entry {
                    Ok(entry) => {
                        let path = entry.path();
                        if path.is_dir() {
                            dir_entries.push(entry);
                            break;
```

### 可能可以通过RootDir来判断根目录

判断根目录是否可用。
```rs
pub fn as_os_str(self) -> &'a OsStr {
    match self {
        Component::Prefix(p) => p.as_os_str(),
        Component::RootDir => OsStr::new(MAIN_SEP_STR),
        Component::CurDir => OsStr::new("."),
        Component::ParentDir => OsStr::new(".."),
        Component::Normal(path) => path,
    }
}
```

### 这个根目录还是有点问题，后面再看看吧

==进入根目录之后，再进入后面的文件会出现错误！！！==

### 关于TOS中可用的配置信息

目前按照releases的信息提供这些配置

```rs
#[derive(Debug)]
pub struct TOSProjectConfig {
    pub path: String,
    pub version: TOSProjectVersion,
}

impl TOSProjectConfig {
    pub fn default() -> Self {
        Self {
            path: current_dir().unwrap().to_str().unwrap().to_string(),
            version: TOSProjectVersion::VERSION_2_5_0,
        }
    }
}

/// TOS project type.
#[derive(Debug, PartialEq)]
pub enum TOSProjectVersion {
    VERSION_2_5_0,
    VERSION_2_4_5,
    VERSION_2_1_0,
}

impl TOSProjectVersion {
    pub fn as_str(&self) -> &'static str {
        match self {
            TOSProjectVersion::VERSION_2_5_0 => "v2.5.0",
            TOSProjectVersion::VERSION_2_4_5 => "v2.4.5",
            TOSProjectVersion::VERSION_2_1_0 => "v2.1.0",
        }
    }

    pub fn convert_to_type(t: String) -> TOSProjectVersion {
        match t.as_str() {
            "v2.5.0" => TOSProjectVersion::VERSION_2_5_0,
            "v2.4.5" => TOSProjectVersion::VERSION_2_4_5,
            "v2.1.0" => TOSProjectVersion::VERSION_2_1_0,
            _ => TOSProjectVersion::VERSION_2_5_0,
        }
    }
}
```


### ==后面应该可以考虑直接反序列化配置到本地中，下次可以直接读取并且修改配置==

### 切换路由

需要使用match来保证路由稳定。

```rs
match app.routes.current {
    0 => {
        project_select::draw_page(app, frame, chunks[1]);
        match app.active_modules {
            ActiveModules::ProjectSelect(crate::app::ProjectSelect::Fs) | ActiveModules::ProjectSelect(crate::app::ProjectSelect::Kind) => {
            }
            _ => {
                // set default active module
                app.active_modules = ActiveModules::ProjectSelect(crate::app::ProjectSelect::Fs);
            }
        }
    }
    1 => {
        tos_download::draw_page(app, frame, chunks[1]);
        match app.active_modules {
            ActiveModules::TOSDownload(crate::app::TOSDownload::Fs) | ActiveModules::TOSDownload(crate::app::TOSDownload::Version) => {
            }
            _ => {
                // set default active module
                app.active_modules = ActiveModules::TOSDownload(crate::app::TOSDownload::Fs);
            }
        }
    }
```

### 使用popup来实现弹出层的效果

应该是要使用`Clear`的组件来实现

> Ref: https://docs.rs/ratatui/latest/ratatui/widgets/struct.Clear.html

### 用户输入，应该要使用类似`userinput`的效果

应该要使用类似popup + userinput完成配置操作。

还有一些check操作，应该也可以用上。

那些开关量的话，直接使用space进行选择或者是反向选择吧，输入感觉会有问题。

然后统一都使用enter进行选中，然后space进行保存。

### 关于配置的修改

应该可以使用table来完成列表渲染，然后这里显示也是可以超出页面滚动的。

然后使用enter进入修改，然后使用space保存。

开关量的话，直接使用space进行选择和反向选择吧。

> Ref: https://github.com/ratatui-org/ratatui/tree/main/examples#table

### 这个panic应该也可以用上，出问题的时候显示一下

> Ref: https://github.com/ratatui-org/ratatui/tree/main/examples#panic

### 关于Popup部分

主要是用到了这个Clear属性，这个会清除这部分的数据，然后再次绘制图像。

```rs
f.render_widget(Clear, area); //this clears out the background
f.render_widget(block, area); // 重新绘画
```

### 关于rust的

在 Rust 中，saturating_sub 是一个针对数值类型的方法，用于执行饱和减法（saturating subtraction）操作。它用于在执行减法运算时，确保结果不会溢出并保持在类型的取值范围内。

当执行常规的减法运算时，如果结果超出了数值类型的表示范围，通常会发生溢出。而 saturating_sub 方法会在结果溢出时，返回类型的最大或最小值，而不是引发 panic 或产生不确定的结果。

### 这个生命周期好像有点问题

还是在这里定义一下吧，这样子会比较好做一点。
```rs
pub fn get_input_block<'a>(app: &App, title: &str, current: &str) -> Paragraph<'a> {
    Paragraph::new(current)
        .style(match app.input.input_mode {
            InputMode::Normal => Style::default(),
            InputMode::Editing => Style::default().fg(Color::Yellow),
        })
        .block(Block::default().borders(Borders::ALL).title(title))
}
```

现在好像没有问题了，所有的借用都得定义生命周期。
```rs
pub fn get_input_block<'a>(app: &'a App, title: &'a str, current: &'a str) -> Paragraph<'a> {
    Paragraph::new(current.clone())
        .style(match app.input.input_mode {
            InputMode::Normal => Style::default(),
            InputMode::Editing => Style::default().fg(Color::Yellow),
        })
        .block(Block::default().borders(Borders::ALL).title(title.clone()))
}
```

### 关于弹出层的使用

目前和input结合在一起了，暂时还是可以使用的，但是后面应该还是得修改修改呜呜呜。

```rs
pub fn draw_input_popup<B: Backend>(app: &mut App, frame: &mut Frame<'_, B>) {
    let size = frame.size();
    let block = input::get_input_block(app, "Test Input", app.input.input.as_str());
    let area = centered_rect(60, 10, size);

    match app.input.input_mode {
        InputMode::Normal =>
            {}

        InputMode::Editing => {
            // Make the cursor visible and ask ratatui to put it at the specified coordinates after
            // rendering
            frame.set_cursor(
                // Draw the cursor at the current position in the input field.
                // This position is can be controlled via the left and right arrow key
                area.x + app.input.cursor_position as u16 + 1,
                // Move one line down, from the border to the input line
                area.y + 1,
            )
        }
    }

    frame.render_widget(Clear, area);
    frame.render_widget(block, area);
}
```