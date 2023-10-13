# 关于rust的序列化方式

这里给了关于序列化和反序列化引擎的支持。

 SerDe 是Serializer/Deserializer的简写。

添加序列化和反序列化的依赖。
```rs
cargo add serde --features derive
cargo add serde_json
cargo add serde_derive
```

> Ref: https://github.com/serde-rs/serde
> Ref: https://zhuanlan.zhihu.com/p/595712478