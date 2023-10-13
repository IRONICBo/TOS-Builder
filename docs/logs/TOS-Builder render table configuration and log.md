# 关于表格渲染重新组织了一下

### 直接用的解析json好像不太方便啊

不如直接把配置名直接存储一下，这样方便一点，还不用解析并且效率会比较高。

这个是之前的版本。
```rs
let json_value = serde_json::to_value(&app.tos_header_config).unwrap();
if let Value::Object(map) = json_value {
    for (key, value) in map.iter() {
        // TODO: render config table
        // println!("Field: {}, Value: {:?}", key, value);
        if let Value::Object(inner_map) = value {
            for (key, value) in inner_map.iter() {
                // println!("Field: {}, Value: {:?}", key, value);
                match value {
                    Value::String(s) => {
                        println!("Field: {}, String Value: {:?}", key, s);
                    }
                    Value::Bool(b) => {
                        println!("Field: {}, Bool Value: {:?}", key, b);
                    }
                    _ => {}
                }
            }
        }
    }
}
```

### 类型转换的问题

```rs
impl From<BoolValue> for [String; 3] {
    fn from(s: BoolValue) -> Self {
        [s.key, s.value.to_string(), s.comment]
    }
}
```

这个转换好像不太好用，感觉还是写一个方法用来转换吧。

- as 语法实现的类型转换只能用于基本类型之前都相互转换，如果基本类型转非基本类型就会编译错误。
- From Trait 一般用于非基本类型的转换（比较复杂的转换)，比如String::from 函数
- tryFrom 和From trait 很相似，不同点就是tryFrom 返回值是result 类型（包含错误信息)

> Ref: https://blog.csdn.net/guiyiba/article/details/131304784

### 添加example

感觉还是应该添加一个默认的example，这样子容易测试一点hhh，不然没办法直接用。

