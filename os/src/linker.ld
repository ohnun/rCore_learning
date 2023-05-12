OUTPUT_ARCH(riscv)
ENTRY(_start)
BASE_ADDRESS = 0x80200000;

SECTIONS
{
	. = BASE_ADDRESS;
	skernel = .;

	stext = .;
	.text : {
		*(.text.entry)
		*(.text .text.*)
	}

	. = ALIGN(4K);
	etext = .;
	srodata = .;
	.rodata : {
		*(.data .data.*)
		*(.sdata .sdata.*)
	}

	. = ALIGN(4K);
	ebss = .;
	ekernel = .;

	/DISCARD/ : {
		*(.en_frame)
	}
}
