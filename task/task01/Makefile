# Task01:: Run um with codex.um, and extract embennded um program

all: download run-codex prepare-input-file run-codex-with-input cut-codex-output

download:
	curl -O http://www.boundvariable.org/codex.umz

run-codex:
	 cargo run --release --bin um -- codex.umz

# output:
# self-check succeeded!
# enter decryption key:
# decrypting...
# ok
# LOADING: 9876543210

#  == CBV ARCHIVE ==
#     VOLUME ID 9

#  Choose a command:

#  p) dump UM data
#  x) exit


# decryption key: `(\b.bb)(\v.vv)06FHPVboundvarHRAk`

prepare-input-file:
	echo -E '(\b.bb)(\v.vv)06FHPVboundvarHRAk' > decryption.key
	: # p) dump UM data
	echo p >> decryption.key

run-codex-with-input:
	cat decryption.key | cargo run --release --bin um -- codex.umz > codex.output


# > % wc codex.output
# >    24864 1134301 16165199 codex.output

# > % % gmd5sum codex.output
# > 93f44c887997037211d7b3c6b3f9dfd3  codex.output

# > hexdump -C codex.output

# ```
# 00000000  73 65 6c 66 2d 63 68 65  63 6b 20 73 75 63 63 65  |self-check succe|
# 00000010  65 64 65 64 21 0a 65 6e  74 65 72 20 64 65 63 72  |eded!.enter decr|
# 00000020  79 70 74 69 6f 6e 20 6b  65 79 3a 0a 64 65 63 72  |yption key:.decr|
# 00000030  79 70 74 69 6e 67 2e 2e  2e 0a 6f 6b 0a 4c 4f 41  |ypting....ok.LOA|
# 00000040  44 49 4e 47 3a 20 39 38  37 36 35 34 33 32 31 30  |DING: 9876543210|
# 00000050  0a 0a 20 3d 3d 20 43 42  56 20 41 52 43 48 49 56  |.. == CBV ARCHIV|
# 00000060  45 20 3d 3d 0a 20 20 20  20 56 4f 4c 55 4d 45 20  |E ==.    VOLUME |
# 00000070  49 44 20 39 0a 0a 20 43  68 6f 6f 73 65 20 61 20  |ID 9.. Choose a |
# 00000080  63 6f 6d 6d 61 6e 64 3a  0a 0a 20 70 29 20 64 75  |command:.. p) du|
# 00000090  6d 70 20 55 4d 20 64 61  74 61 0a 20 78 29 20 65  |mp UM data. x) e|
# 000000a0  78 69 74 0a 0a 3f 20 0a  3f 20 55 4d 20 70 72 6f  |xit..? .? UM pro|
# 000000b0  67 72 61 6d 20 66 6f 6c  6c 6f 77 73 20 63 6f 6c  |gram follows col|
# 000000c0  6f 6e 3a d0 00 10 8f c0  00 00 30 00 00 00 00 00  |on:.......0.....|
# 000000d0  00 00 00 00 00 00 00 00  00 00 00 00 00 00 00 00  |................|
# ```

cut-codex-output:
	tail -c +$((0xc0 + 4)) codex.output > main.um

# > % gmd5sum main.um
# > 3ad5b7637df1c5f713689abcba2efff3  main.um
