arch ?= x86_64
kernel := build/boot/kernel-$(arch).bin
iso := build/boot/ggos-$(arch).iso

linker_script := boot/$(arch)/linker.ld
grub_cfg := boot/$(arch)/grub.cfg
assembly_source_files := $(wildcard boot/$(arch)/*.asm)
assembly_object_files := $(patsubst boot/$(arch)/%.asm, \
	build/boot/$(arch)/%.o, $(assembly_source_files))

.PHONY: all clean run iso

all: $(kernel)

clean:
	@rm -r build/boot

run: $(iso)
	@qemu-system-x86_64 -cdrom $(iso)

iso: $(iso)

$(iso): $(kernel) $(grub_cfg)
	@mkdir -p build/boot/isofiles/boot/grub
	@cp $(kernel) build/boot/isofiles/boot/kernel.bin
	@cp $(grub_cfg) build/boot/isofiles/boot/grub
	@grub-mkrescue  -d /usr/lib/grub/i386-pc -o $(iso) build/boot/isofiles 2> /dev/null
	@rm -r build/boot/isofiles

$(kernel): $(assembly_object_files) $(linker_script)
	@ld -n -T $(linker_script)  -o $(kernel) $(assembly_object_files)

# compile assembly files
build/boot/$(arch)/%.o: boot/$(arch)/%.asm
	@mkdir -p $(shell dirname $@)
	@nasm -felf64 $< -o $@