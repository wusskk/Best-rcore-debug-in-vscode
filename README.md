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
   -ex 'add-symbol-file user/build/dbg/ch3_sleep.dbg' \
   -ex 'add-symbol-file user/build/dbg/ch3_sleep1.dbg' \
   -ex 'add-symbol-file user/build/dbg/ch3_task.dbg'
   # 然后就能进行调试，注意增加的符号文件仅适用于ch3，其他分支需要载入的符号文件不同
   ```
3. 进行grade测试：

   ```bash
   cd os
   make test
   ```

## 修改进展

* [X] ch3
* [ ] ch4(todo)
* [ ] ch5(todo)
* [ ] ch6(todo)
* [ ] ch7(todo)
* [ ] ch8(todo)
