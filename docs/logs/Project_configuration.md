# 支持命令行编译的一些参考资料

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

### 添加cmake基本工程

移植gcc版本动配置。

windows下面配置之后，需要添加到`Path`的目录下面！！！

按照文档配置就行，目前就直接使用的默认配置。

### 配置安装STLINK Utility

> Ref: https://zhuanlan.zhihu.com/p/448660584

### 关于tencentos的配置，应该还有什么架构需要选择，比如armv7a之类的！！！然后拷贝对应的路径到文件下面！！！

### 编译下载有问题！！！显示`PendSV_Handler`已经被定义了

好像是tos里面有实现一个`PendSV_Handler`，所以需要自己另外使用一个PendSV_Handler！！！把之前的函数前面写入`__weak`的关键字就可以编译通过！！！

### 对比了一下两个工程

好像是在`<Group>`标签下面添加配置，把路径写进去就可以！！！

emmm，确认了一下，好像只需要改`uvprojx`的配置就行了啊啊啊啊啊啊！！！其他的东西在编译的时候可以自动更新！！！

### rust中解析和配置xml文件

==看起来直接硬编码也可以的！！！就是找到位置然后替换一下！！！==

在Rust中，有几个流行的库可用于解析和处理XML文件。其中一个常用的库是xml-rs。xml-rs是一个高性能的XML解析器，它提供了简单而灵活的API来处理XML数据。

要使用xml-rs库，您需要在Rust项目的Cargo.toml文件中添加以下依赖项：

```toml
[dependencies]
xml-rs = "0.8"
```
然后，您可以使用以下代码来解析XML文件并添加XML节点：

```rs
use std::fs::File;
use std::io::BufReader;
use xml::reader::{EventReader, XmlEvent};
use xml::writer::{EmitterConfig, EventWriter};

fn main() {
    // 打开要解析的XML文件
    let file = File::open("path/to/your/xml/file.xml").expect("Failed to open XML file");
    let file = BufReader::new(file);

    // 创建事件读取器
    let parser = EventReader::new(file);

    // 创建事件写入器
    let mut writer = EmitterConfig::new()
        .perform_indent(true)
        .create_writer(std::io::stdout());

    // 遍历XML事件
    for event in parser {
        match event {
            Ok(XmlEvent::StartElement { name, attributes, .. }) => {
                // 处理开始元素事件
                // 在这里可以添加逻辑来检查要添加节点的位置，并在需要时添加节点
                writer
                    .write(XmlEvent::StartElement { name, attributes })
                    .expect("Failed to write XML event");
            }
            Ok(XmlEvent::EndElement { name }) => {
                // 处理结束元素事件
                writer
                    .write(XmlEvent::EndElement { name })
                    .expect("Failed to write XML event");
            }
            Ok(event) => {
                // 处理其他类型的事件
                writer.write(event).expect("Failed to write XML event");
            }
            Err(e) => {
                // 处理解析错误
                eprintln!("Error: {}", e);
                break;
            }
        }
    }
}
```
在上面的代码中，您需要将"path/to/your/xml/file.xml"替换为实际的XML文件路径。代码将打开XML文件并使用EventReader逐个读取XML事件。根据需要，您可以在处理开始元素事件时添加逻辑来插入新的XML节点。然后，使用EventWriter将处理后的XML事件写回XML文件或输出到控制台。

请注意，上述代码仅提供了一个基本示例，您可能需要根据您的具体需求进行适当的修改和扩展。还可以参考xml-rs库的文档以获取更多有关使用该库的详细信息和示例。

### 感觉还是直接管配置文件吧

就是把配置和头文件添加进去，然后添加链接文件位置！！！

### 关于gcc部分的移植

好像就是在配置里面添加`C_SOURCES`和`C_INCLUDES`的路径就可以！！！

### 移植TOS的报错IAR Error：Fatal Error[Pe1696]: cannot open source file “core_cm0plus.h“

这里的配置有点问题。。。

> Ref: https://blog.csdn.net/m0_65220915/article/details/131570378

### 配置安装IAR

> Ref: https://blog.csdn.net/qq_56527127/article/details/119815640

##### 添加IAR的配置

好像是和keil的差不多，头文件的在options里面添加。
```xml
<options>
<name></name>
<state>xxxx</state>
</options>
```

然后其他的配置也是类似的，就是在`xml`文件里面配置一下就行。

在那个`<group>`里面配置就行。

```xml
<group>
<name></name>
<file>
    <name></name>
</file>
</group>
```

### 添加架构配置和驱动配置

### 注意gcc下面的是gcc！！！其他的是armcc！！！

==makefile中wildcard的理解， 明确表示通配符==

wildcard用来明确表示通配符。因为在 Makefile 里,变量实质上就是 C/C++ 中的宏,也就是说,如果一个表达式如 objs = *.o ,则 objs 的值就是 *.o ,而不是表示所有的 .o 文件。若果要使用通配符,那么就要使用 wildcard 来声明 * 这个符号,使 * 符号具有通配符的功能。

### 注意！！！文档位置不一样！！！

gcc => gcc
mdk => armcc
iar => iccarm

### 下载好像出了问题！！！

好像是这里的有点问题

注意这里需要添加过滤！！！否则会自动刷新到别的界面去了！！！
```rs
match app.routes.current {
    0 => {
        project_select::draw_page(app, frame, chunks[1]);
        match app.active_modules {
            ActiveModules::ProjectSelect(crate::app::ProjectSelect::Fs) | ActiveModules::ProjectSelect(crate::app::ProjectSelect::Kind) | ActiveModules::ProjectSelect(crate::app::ProjectSelect::Arch) => {
            }
            _ => {
                // set default active module
                app.active_modules = ActiveModules::ProjectSelect(crate::app::ProjectSelect::Fs);
            }
        }
    }
```