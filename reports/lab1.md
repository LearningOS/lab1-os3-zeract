### 实现功能
本次实验实现了系统调用`sys_task_info`，查询当前正在执行的任务信息，包括任务状态、任务使用的系统调用次数、任务的总运行时长，在`TaskControlBlock`中添加需要记录的信息如任务使用各个系统调用的次数和任务开始时间，在每个系统调用执行的时候对当前的任务其系统调用次数加1，到调用`sys_task_info`的时候，通过`get_time_us()`计算当前任务的总运行时长。


### 简答作业

1.  RustSBI version 0.3.0-alpha.4
- [ERROR] [kernel] PageFault in application, bad addr = 0x0, bad instruction = 0x80400408, core dumped.

第一个程序存在页错误，访问了非法地址0x0，并向其中写入数据的指令

- [ERROR] [kernel] IllegalInstruction in application, core dumped.

第二个程序非法使用了sret指令

- [ERROR] [kernel] IllegalInstruction in application, core dumped.

第三个程序在U状态下使用了csrr指令
2.
	1. 刚进入`__restore`时，`a0`代表了内核栈的栈顶，`__restore`可以用于任务切换和时钟中断
	2. 这几行代码处理了`t0`、`t1`、`t2`、`sstatus`、`sepc`、`sscratch`这几个寄存器，`sstatus`表示当前当前CPU所处的特权级，`sepc`表示用户态该执行的下一条指令的地址，`sscratch`表示内核栈的栈顶地址，可用于之后的trap
	3. 跳过x2因为它已经改变，指向了内核栈，跳过x4是因为一般不会用到，所以无需保存
	4. sp指向用户栈栈顶，sscratch指向内核栈栈顶
	5. 状态转换发生在sret，在执行该指令后，`sstatus`会被更改，并跳转到`sepc`寄存器指向的那条指令
	6. sp指向内核栈栈顶，sscratch指向用户栈栈顶
	7. 从U态进入S态发生在call trap_handler这条指令
