# All For Test

本仓库是为了方便进行rcore的调试，主要做了以下几点工作：

1. 调整目录结构，取消ci-user目录，只保留user和os目录在同一层级下，user存放用户程序，os存放内核文件。
2. 简化Makefile，将三个Makefile简化为一个存放在os目录下，并且简化编译逻辑，帮助大家更方便地理解编译流程。
3. 针对vscode进行调试优化，实现开箱即用，并且实现内核和用户程序的同时调试，能够追踪代码从用户程序到内核代码。

## How to use

1. 如果使用vscode：

   ```bash
   git clone https://github.com/wusskk/rCore-Tutorial-Code-2024S.git
   git checkout ch3 #ch3为章节编号，可根据需要修改
   # 现在只需要在vscode中按F5启动调试
   ```
2. 如果使用命令行：

   ```bash
   git clone https://github.com/wusskk/rCore-Tutorial-Code-2024S.git
   git checkout ch3 #ch3为章节编号，可根据需要修改
   cd os
   make debug 
   # 另开一个新终端（下方载入符号文件部分仅适用于ch3）
   riscv64-unknown-elf-gdb -ex 'file os/target/riscv64gc-unknown-none-elf/debug/os' \
   -ex 'target remote localhost:15000' \
   -ex 'add-symbol-file user/build/dbg/commands.dbg'
   # 然后就能进行调试
   ```
3. 进行grade测试：

   ```bash
   cd os
   make test
   ```
4. 目录结构梳理：
仅列举重要文件，省略部分文件
   - .vscode/
      - tasks.json: 程序构建命令，是luanch.json的前置任务
      - luanch.json: 启动gdb，进行远程调试
   - check/
      - base.py
      - ch*.py grading的check脚本，通过读取程序输出进行判断
   - os/ 内核目录
      - src/ 源文件
      - Makefile 构建内核和用户程序
      - build.rs 构建内核的前置步骤，包括生成link_app.S和commands.gdb（为gdb加载用户程序的符号文件）
   - user/ 用户程序目录
   - src/
      - bin/
         - ch*.rs 用户程序的源文件
      - lib.rs 通过ecall指令封装系统调用

## 修改进展

* [X] ch3
* [X] ch4
* [X] ch5
* [ ] ch6(todo)
* [ ] ch7(todo)
* [ ] ch8(todo)

## 注意事项

1. 为了方便调试，所有文件编译均采用debug模式，若需要采用release模式调试，请将所有需要修改的地方改为release（主要为cargo命令和target文件路径）
2. 目前还不支持调试时跳转到.S文件的汇编代码
3. 有可能出现在vscode的集成终端中能够执行的命令，启动调试时输出：命令not found。原因是vscode本身的bug，暂无确切的解决方案，可以考虑使用绝对路径，并且注释掉Makefile中的环境检查语句。vscode的bug [issue链接](https://github.com/microsoft/vscode/issues/187955 "#187955")
4. 有问题可以微信群里问，或者提issue
5. 欢迎所有人对该项目进行优化（issue/pull request）
