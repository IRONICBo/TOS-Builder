# cargo启动失败问题

Blocking waiting for file lock on package cache

1.出现问题的场景：

下载create出现timeout,之后更换了国内源下载create成功后,再次build或run时出现这个问题。

2.解决办法：

删除 ~/.cargo/.package-cache ，然后再次build或run
```bash
rm -f  ~/.cargo/.package-cache 
```

> Ref: https://zhuanlan.zhihu.com/p/543720330?utm_id=0