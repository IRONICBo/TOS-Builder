#ifndef _TOS_CONFIG_H_
#define  _TOS_CONFIG_H_

#include "{{TOS_CFG_HEADER_INCLUDE}}"	// 目标芯片头文件，用户需要根据情况更改

#define TOS_CFG_TASK_PRIO_MAX           {{ TOS_CFG_TASK_PRIO_MAX }} 	// 配置TencentOS tiny默认支持的最大优先级数量

#define TOS_CFG_ROUND_ROBIN_EN          {{ TOS_CFG_ROUND_ROBIN_EN }}		// 配置TencentOS tiny的内核是否开启时间片轮转

#define TOS_CFG_OBJECT_VERIFY_EN        {{ TOS_CFG_OBJECT_VERIFY_EN }}	    // 配置TencentOS tiny是否校验指针合法

#define TOS_CFG_TASK_DYNAMIC_CREATE_EN  {{ TOS_CFG_TASK_DYNAMIC_CREATE_EN }}		// TencentOS tiny 动态任务创建功能宏

#define TOS_CFG_EVENT_EN                {{ TOS_CFG_EVENT_EN }}		// TencentOS tiny 事件模块功能宏

#define TOS_CFG_MMBLK_EN                {{ TOS_CFG_MMBLK_EN }}		// 配置TencentOS tiny是否开启内存块管理模块

#define TOS_CFG_MMHEAP_EN               {{ TOS_CFG_MMHEAP_EN }}		// 配置TencentOS tiny是否开启动态内存模块

#define TOS_CFG_MMHEAP_DEFAULT_POOL_EN  {{ TOS_CFG_MMHEAP_DEFAULT_POOL_EN }}		// TencentOS tiny 默认动态内存池功能宏

#define TOS_CFG_MMHEAP_DEFAULT_POOL_SIZE {{ TOS_CFG_MMHEAP_DEFAULT_POOL_SIZE }}	// 配置TencentOS tiny默认动态内存池大小

#define TOS_CFG_MUTEX_EN                {{ TOS_CFG_MUTEX_EN }}		// 配置TencentOS tiny是否开启互斥锁模块

#define TOS_CFG_MESSAGE_QUEUE_EN        {{ TOS_CFG_MESSAGE_QUEUE_EN }}		// 配置TencentOS tiny是否开启消息队列模块

#define TOS_CFG_MAIL_QUEUE_EN           {{ TOS_CFG_MAIL_QUEUE_EN }}		// 配置TencentOS tiny是否开启消息邮箱模块

#define TOS_CFG_PRIORITY_MESSAGE_QUEUE_EN	{{ TOS_CFG_PRIORITY_MESSAGE_QUEUE_EN }}	// 配置TencentOS tiny是否开启优先级消息队列模块

#define TOS_CFG_PRIORITY_MAIL_QUEUE_EN	{{ TOS_CFG_PRIORITY_MAIL_QUEUE_EN }}		// 配置TencentOS tiny是否开启优先级消息邮箱模块

#define TOS_CFG_TIMER_EN                {{ TOS_CFG_TIMER_EN }}		// 配置TencentOS tiny是否开启软件定时器模块

#define TOS_CFG_PWR_MGR_EN              {{ TOS_CFG_PWR_MGR_EN }}		// 配置TencentOS tiny是否开启外设电源管理模块

#define TOS_CFG_TICKLESS_EN             {{ TOS_CFG_TICKLESS_EN }}		// 配置Tickless 低功耗模块开关

#define TOS_CFG_SEM_EN                  {{ TOS_CFG_SEM_EN }}		// 配置TencentOS tiny是否开启信号量模块

#define TOS_CFG_TASK_STACK_DRAUGHT_DEPTH_DETACT_EN      {{ TOS_CFG_TASK_STACK_DRAUGHT_DEPTH_DETACT_EN }}	// 配置TencentOS tiny是否开启任务栈深度检测

#define TOS_CFG_FAULT_BACKTRACE_EN      {{ TOS_CFG_FAULT_BACKTRACE_EN }}		// 配置TencentOS tiny是否开启异常栈回溯功能

#define TOS_CFG_IDLE_TASK_STK_SIZE      {{ TOS_CFG_IDLE_TASK_STK_SIZE }}	    // 配置TencentOS tiny空闲任务栈大小

#define TOS_CFG_CPU_TICK_PER_SECOND     {{ TOS_CFG_CPU_TICK_PER_SECOND }}	    // 配置TencentOS tiny的tick频率

#define TOS_CFG_CPU_CLOCK               (SystemCoreClock)	            // 配置TencentOS tiny CPU频率

#define TOS_CFG_TIMER_AS_PROC           {{ TOS_CFG_TIMER_AS_PROC }}		// 配置是否将TIMER配置成函数模式

#endif