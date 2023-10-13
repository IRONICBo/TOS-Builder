# 一些资料的信息查询

Mac上安装CubeMx

> Ref: https://www.baidu.com/link?url=KIw8zjwPP3hRZclKnpskKPQ_-IYyYEjq_KtTpuWOc4Liw1xfIC4Dbm5x7-_BhXtZ&wd=&eqid=8e1ade0d000034b70000000664a7d57e
> 
> Ref: https://blog.csdn.net/meteornk/article/details/130778390
> 
> Ref: https://zhuanlan.zhihu.com/p/414161114

### AT框架的配置

AT框架的介绍。
> Ref: https://cloud.tencent.com/edu/learning/course-2985-55251

### 大致构思

这些配置主要是存放在`#define`文件里面的。


1. 从零开始构建：
为了减少包的体积，这里的几个工程都是从web上面下载的，目前打算是托管在github上，然后从github上下载文件到某个目录中。（这里需要一些网络请求的库，下载文件）
MDK/IAR/GCC三个工程，然后里面的配置都是默认的，只需要从零开始构建即可。这里需要查看一下这个配置是不是只是修改了这些define常量的配置。（文件读取，文件解析等一些库）
2. 读取现有工程：
和上面的类似，可能得check一下这个项目是否合法，比如检查结构和版本号等。
3. 添加tencentos-tiny的构建库：
从github上面拉取代码，然后放到本地。然后针对不同的系统进行构建。（也是需要一些网络请求的库）
4. AT框架的引入和配置：
这个也是和前面的类似，代码已经拉下来了，只需要按照文件模版进行修改即可。