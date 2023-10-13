# rtthread板级BSP构建

Scons配置的方式。

> Ref: https://blog.csdn.net/qq_41053994/article/details/128616894

### 或者可以看一下其他的构建系统？XMake之类的

好像XMake比较快，而且比CMake好。

但是scons感觉还是去掉吧，不是针对c/c++的专有编译系统。

### 大致查看一下RTThread的构建工具流程

大概就是scons管理编译生成，然后menuconfig管理配置信息。

### tos组件包的管理

> Ref: https://blog.csdn.net/weixin_43772810/article/details/125523892

### KConfig的介绍

主要是能够适配Windows下面的开发：

> Ref: https://blog.csdn.net/qq_33229007/article/details/129340204

Kconfig用来配置内核，它就是各种配置界面的源文件，内核的配置工具读取各个Kconfig文件，生成配置界面供开发人员配置内核，最后生成配置文件.config。

主要包含下面的一些配置：
1. config条目(entry)
config是关键字，表示一个配置选项的开始；紧跟着的TMPFS_POSIX_ACL是配置选项的名称，省略了前缀"CONFIG_"
    bool表示变量类型，即"CONFIG_ TMPFS_POSIX_ACL "的类型，有5种类型：bool、tristate、string、hex和int，其中tristate和string是基本的类型
2. menu条目
   menu条目用于生成菜单，其格式如下：
3. choice条目
choice条目将多个类似的配置选项组合在一起，供用户单选或多选，这不同于menu条目
4. comment条目
comment条目用于定义一些帮助信息，出现在界面的第一行
5.source条目
source条目用于读取另一个Kconfig文件


> Ref: https://blog.csdn.net/qianxuedegushi/article/details/113249179

### 这里有代码可以将嵌入式工程互转

代码生成器，大概看了一下就是一些字符串转换还有一些模版的生成和替换。

> Ref: https://github.com/project-generator/project_generator