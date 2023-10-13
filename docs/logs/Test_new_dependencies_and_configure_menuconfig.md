# 添加更多依赖用于增强功能

### 关于生成Makefile或者CMake的构建

rust中使用模版生成对应的代码。

如果你需要处理复杂的模板渲染需求，并且希望拥有更多高级的模板功能，tera是一个很好的选择。但如果你更偏向于简单、易用且类型安全的模板库，askama可能更适合你。

# 类似menuconfig的功能

### RT-thread中也提供了make menuconfig

可以根据这个的实现来参考配置TOS。

> Ref: https://github.com/RT-Thread/rt-thread/blob/master/README_zh.md

里面还有使用qemu仿真的部分。

> Ref: https://github.com/RT-Thread/rt-thread/blob/master/documentation/quick-start/quick_start_qemu/quick_start_qemu_linux.md

# 可以参考的rtthread做法

使用`menuconfig`和`scons`进行构建。

> Ref: https://github.com/IRONICBo/rt-thread/tree/master/tools

### menuconfig配置的做法

主要是`menuconfig`和`Kconfig`的配置和解析

> Ref: https://zhuanlan.zhihu.com/p/517418914