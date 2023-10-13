# 关于Tencentos tiny的进一步配置

### tencentos tiny的项目结构解析

参考的文档地址：
> Ref: https://github.com/OpenAtomFoundation/TencentOS-tiny/blob/master/doc/09.Code_Directories.md

### AT框架部分架构

- net
    - at		TencentOS tiny为串口类通信模组提供的AT框架实现层
    - lora_module_wrapper		TencentOS tiny为串口类LoraWAN模块提供的移植框架
    - lwip		Lwip协议实现源码及适配层
    - sal_module_wrapper		TencentOS tiny为串口类网络模块（wifi gprs）提供的socket移植框架
    - socket_wrapper		标准BSD Socket接口实现
    - tencent_firmware_module_wrapper		TencentOS tiny提供的腾讯定制模组移植框架

### 移植AT框架和模组

这里也有一些移植其他信息的步骤，包括移植内核的操作。
> Ref: https://cloud.tencent.com/document/product/1081/48381emmm

这里是github上面移植的步骤
> Ref: https://github.com/OpenAtomFoundation/TencentOS-tiny/blob/master/doc/27.AT_Firmware_and_SAL_Firmware_User_Guide.md#4-at%E6%A1%86%E6%9E%B6sal%E6%A1%86%E6%9E%B6%E7%A7%BB%E6%A4%8D%E6%96%B9%E6%B3%95