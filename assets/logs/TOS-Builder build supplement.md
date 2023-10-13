# 关系tos构建系统的一些补充

### 关于stlink的使用

需要多次重启还有upgrade，才能成功连接上去。。。

### 添加一些必要的辅助函数

##### 关于文件拷贝

拷贝的时候可以获取当前目录的目录名称。

### 关于模版引擎的渲染

尝试使用handlebars进行开发，这里还有handlerbars的文档

> Ref: https://github.com/sunng87/handlebars-rust
> Ref: https://handlebarsjs.com/guide/

### 关于定义多行字符串

好像是得用下面的东西来写。。。而且还需要定义成为pub才能被外部包访问
```rs
pub const TOS_CONFIG: &str = r#"

xxx

"#
```

##### vscode的rust没有提示信息，好像是bug，需要重启一下

关于设置rust的模版渲染。
```rs
#[test]
fn print_tos_header() {
    println!("{}", TOS_CONFIG);

    let mut reg = Handlebars::new();
    let app = App::default();
    let mut tos_header_file = File::create("tos_config.h");

    reg.register_template_string("tos_header", TOS_CONFIG);
    reg.render_to_write("tos_header", &app.tos_project_config, &mut tos_header_file);
}
```

##### 关于handlebars的用法

这里这里的几个参数使用的好像都没有问题，是成功导出了的。

```rs
#[test]
fn print_tos_header() {
    println!("{}", TOS_CONFIG);

    let mut reg = Handlebars::new();
    let app = App::default();
    let mut tos_header_file = File::create("tos_config.h").unwrap();

    reg.register_template_string("tos_header", TOS_CONFIG);
    reg.render_to_write("tos_header", &app.tos_project_config, &mut tos_header_file);
}
```

其次，这里成功到处的目录在项目的根目录（相对路径的情况下）。

### 使用xml-rs解析配置

确实是有对应的值的。
```bash
[OwnedAttribute { name: OwnedName { local_name: "Cclass", namespace: None, prefix: None }, value: "CMSIS" }
```

确实是可以读取值的，就是麻烦了一点。。。
```rs
        for e in parser {
            match e {
                Ok(XmlEvent::StartElement { name, attributes, .. }) => {
                    let vec = attributes.to_vec();
                    print!("{:?}", vec);
                    println!("{:spaces$}+{name}:{depth}", "", spaces = depth * 2);
                    depth += 1;

                    if name.local_name == "ScatterFile" {
                        inside_target_element = true;
                    }
                    current_element = name.local_name;
                }
                Ok(XmlEvent::EndElement { name }) => {
                    depth -= 1;
                    println!("{:spaces$}-{name}", "", spaces = depth * 2);
                    if name.local_name == "ScatterFile" {
                        inside_target_element = false;
                    }
                    current_element.clear();
                }
                Ok(XmlEvent::Characters(text)) if inside_target_element => {
                    println!("Element '{}' Value: {}", current_element, text);
                    // Element 'ScatterFile' Value: stm32wle5xx_flash.sct
                }
```

读取打印includepath的结果。
```bash
[]              +IncludePath:7
Element 'IncludePath' Value: ../Core/Inc;../Drivers/STM32WLxx_HAL_Driver/Inc;../Drivers/STM32WLxx_HAL_Driver/Inc/Legacy;../Drivers/CMSIS/Device/ST/STM32WLxx/Include;../Drivers/CMSIS/Include
```

### 关于xml文件的修改

大概就是使用一个简单的状态机来获取到唯一的元素：

```rs
for e in parser {
    // Targets -> Target -> TargetOption -> TargetArmAds -> ArmAdsMisc -> Cads -> VariousControls -> IncludePath
    // Find IncludePath
    match e {
        Ok(XmlEvent::StartElement { name, attributes, .. }) => {
            println!("------ Level {}", targets_level);
            match name.local_name.as_str() {
                "Targets" => targets_level += 1,
                "Target" => targets_level += 1,
                "TargetOption" => targets_level += 1,
                "TargetArmAds" => targets_level += 1,
                "ArmAdsMisc" => targets_level += 1,
                "Cads" => targets_level += 1,
                "VariousControls" => targets_level += 1,
                "IncludePath" => targets_level += 1,
                _ => {}
            }
        }
        Ok(XmlEvent::EndElement { name }) => {
            match name.local_name.as_str() {
                "Targets" => targets_level -= 1,
                "Target" => targets_level -= 1,
                "TargetOption" => targets_level -= 1,
                "TargetArmAds" => targets_level -= 1,
                "ArmAdsMisc" => targets_level -= 1,
                "Cads" => targets_level -= 1,
                "VariousControls" => targets_level -= 1,
                "IncludePath" => targets_level -= 1,
                _ => {}
            }
        }
        Ok(XmlEvent::Characters(text)) if targets_level == 7 => {
            println!("Element '{}' Value: {} Level {}", current_element, text, targets_level);
            // Element 'ScatterFile' Value: stm32wle5xx_flash.sct
        }
        Err(e) => {
            eprintln!("Error: {e}");
            break;
        }
        // There's more: https://docs.rs/xml-rs/latest/xml/reader/enum.XmlEvent.html
        _ => {}
    }
}
```

### 更新了，尝试使用传入数组的方式结合状态机完成节点查找

```rs
pub fn find_element_value<'a>(
    reader: BufReader<File>,
    target_path: &[&str],
) -> String {
    let mut target_index = 0;
    let mut include_path_value = String::new();
    let parser = EventReader::new(reader);
    
    for event in parser {
        match event {
            Ok(XmlEvent::StartElement { name, .. }) => {
                if target_index < target_path.len() && name.local_name == target_path[target_index] {
                    println!("------ Level {} - target_path {} - name {}", target_index, target_path[target_index],  name.local_name );
                    target_index += 1;
                }
            }
            Ok(XmlEvent::EndElement { name }) => {
                if target_index > 0 && name.local_name == target_path[target_index - 1] {
                    println!("------ Exit Level {} - target_path {} - name {}", target_index, target_path[target_index], name.local_name );
                    target_index -= 1;
                }
            }
            Ok(XmlEvent::Characters(text)) => {
                // println!("------ Level {} - {}", target_index, text);
                if target_index == target_path.len() {
                    include_path_value = text;
                    break;
                }
            }
            _ => {}
        }
    }

    include_path_value
}
```

```rs
running 1 test
------ Level 0 - target_path Targets - name Targets
------ Level 1 - target_path Target - name Target
------ Level 2 - target_path TargetOption - name TargetOption
------ Level 3 - target_path TargetArmAds - name TargetArmAds
------ Level 4 - target_path Cads - name Cads
------ Level 5 - target_path VariousControls - name VariousControls
------ Level 6 - target_path IncludePath - name IncludePath
IncludePath: ../Core/Inc;../Drivers/STM32WLxx_HAL_Driver/Inc;../Drivers/STM32WLxx_HAL_Driver/Inc/Legacy;../Drivers/CMSIS/Device/ST/STM32WLxx/Include;../Drivers/CMSIS/Include
test utils::xml_helper::tests::test_get_include_path ... ok
```

##### 一些其他的xml库

- xml-rs - 这是 Rust 中最受欢迎的 XML 解析库之一，它提供了一组简单的 API 来读取和操作 XML 文档。
- sxd-document - 这个库提供了一个 DOM 模型，可用于读取和操作 XML 文档。
- quick-xml - 这个库提供了一个高性能的 SAX 解析器，它可以快速地解析大型 XML 文件。
- roxmltree - 这个库提供了一个简单的 DOM 模型，可用于读取和操作 XML 文档。它还支持 XPath 查询和修改操作。

这些库都很棒，具体使用哪个取决于您的需求和偏好。如果您需要高性能的解析器，可以考虑使用 quick-xml。如果您更喜欢 DOM 模型，可以使用 sxd-document 或 roxmltree。如果您需要一个简单的 API 来读取和操作 XML 文档，可以考虑使用 xml-rs。

> Ref: https://juejin.cn/s/rust%20lang%20xml%20parser

##### 官方提供的读写案例

是将reader读取之后，然后重新初始化一下xml writer，最后再写入。

> Ref: https://github.com/netvl/xml-rs/blob/master/examples/rewrite.rs

这里有Options，需要使用Some包裹变量。
```rs
/// Denotes an end of an XML element.
EndElement {
    /// Optional qualified name of the element.
    ///
    /// If `None`, then it is assumed that the element name should be the last valid one.
    /// If `Some` and element names tracking is enabled, then the writer will check it for
    /// correctness.
    name: Option<Name<'a>>,
},
```