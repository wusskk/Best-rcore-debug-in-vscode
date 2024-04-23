# ouput directory
TARGET := riscv64gc-unknown-none-elf
MODE := debug
TARGET_DIR := target/$(TARGET)/$(MODE)

# where is sbi
BOOTLOADER := ../bootloader/rustsbi-qemu.bin

# user build
BUILD_DIR := ../user/build
APPS = $(wildcard ../user/src/bin/*.rs) 									# all user programs
BINS = $(patsubst ../user/src/bin/%.rs, ../user/$(TARGET_DIR)/%, $(APPS)) 	# all user binaries
OBJCOPY := rust-objcopy --binary-architecture=riscv64

# randomize
RAND := $(shell awk 'BEGIN{srand();printf("%d", 65536*rand())}')

# lab number and chapter number
LAB = 3
CHAPTER = 5

check_env:
	@echo "Checking environment..."
	@which rust-objcopy > /dev/null || (echo "Please install rust-objcopy" && exit 1)
	@which qemu-system-riscv64 > /dev/null || (echo "Please install qemu-system-riscv64" && exit 1)
	@which python3 > /dev/null || (echo "Please install python3" && exit 1)
	@which riscv64-unknown-elf-gdb > /dev/null || (echo "Please install riscv64-unknown-elf-gdb" && exit 1)
	@echo "Environment check passed."

user_clean:
	cd ../user && cargo clean
	rm -rf $(BUILD_DIR)

clean: user_clean
	cargo clean

randomize:
	find ../user/src/bin -name "*.rs" | xargs -I {} sh -c 'sed -i.bak 's/OK/OK$(RAND)/g' {} && rm -rf {}.bak'
	find ../user/src/bin -name "*.rs" | xargs -I {} sh -c 'sed -i.bak 's/passed/passed$(RAND)/g' {} && rm -rf {}.bak'
	find ../check -name "*.py" | xargs -I {} sh -c 'sed -i.bak 's/OK/OK$(RAND)/g' {} && rm -rf {}.bak'
	find ../check -name "*.py" | xargs -I {} sh -c 'sed -i.bak 's/passed/passed$(RAND)/g' {} && rm -rf {}.bak'

user_build: check_env clean randomize
	mkdir -p $(BUILD_DIR)/bin/
	mkdir -p $(BUILD_DIR)/dbg/
	mkdir -p $(BUILD_DIR)/elf/
# build user programs
	cd ../user && cargo build
# copy user binaries and get debug info(symbol file)
	$(foreach bin, $(BINS), \
		$(OBJCOPY) --strip-all $(bin) -O binary $(BUILD_DIR)/bin/$(notdir $(bin)).bin; \
		$(OBJCOPY) --only-keep-debug $(bin) $(BUILD_DIR)/dbg/$(notdir $(bin)).dbg; \
		cp $(bin) $(BUILD_DIR)/elf/$(notdir $(bin)).elf;)	
	cp -f ../user/build/elf/ch$(CHAPTER)_usertest.elf ../user/build/elf/ch$(CHAPTER)b_initproc.elf

kernel: user_build
	cargo build

run: kernel
	timeout --foreground 30s qemu-system-riscv64 \
		-machine virt \
		-nographic \
		-bios $(BOOTLOADER) \
		-kernel $(TARGET_DIR)/os

test: user_build
	make run | tee stdout-ch$(CHAPTER)
# check the output
	python3 ../check/ch$(CHAPTER).py < stdout-ch$(CHAPTER)
# check the report(you can comment it out if you don't want to check the report)
	@for i in $(shell seq $(LAB)); do \
		if ! [ -f ../reports/lab$$i.pdf -o -f ../reports/lab$$i.md ]; then \
			echo "Report for lab$$i needed. Add your report to reports/lab$$i.pdf or reports/lab$$i.md" ; \
			exit 1 ; \
		else \
			echo "Report for lab$$i found." ; \
		fi; \
	done


debug: kernel
	@echo ".*Now run 'gdb' in another window."
	qemu-system-riscv64 \
	-machine virt \
	-nographic \
	-bios $(BOOTLOADER) \
	-kernel $(TARGET_DIR)/os \
	-S -gdb tcp::15000

.PHONY: kernel clean run test debug
