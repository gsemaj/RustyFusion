import sys
import re

fname = sys.argv[1]
with open(fname, "r") as fi:
    code = fi.read()

regex_rules = [
    (
        r"#pragma pack\((\d+)\)",
        r"#[repr(packed(\1))]"
    ),
    (
        r"struct (\S+) {([\S\s]+?)};",
        r"""#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct \1 {\2}
impl FFPacket for \1 {}"""
    ),
    (
        r"uint(\d+)_t (\S+;)",
        r"u\1 \2"
    ),
    (
        r"int(\d+)_t (\S+;)",
        r"i\1 \2"
    ),
    (
        r"int (\S+;)",
        r"i32 \1"
    ),
    (
        r"char(\d+)_t (\S+;)",
        r"u\1 \2"
    ),
    (
        r"short (\S+;)",
        r"u16 \1"
    ),
    (
        r"float (\S+;)",
        r"f32 \1"
    ),
    (
        r"(\S+) (\S+)\[(\d+)\];",
        r"[\1; \3] \2;"
    ),
    (
        r"(\S+|\[.+\]) (\S+);",
        r"pub \2: \1,"
    ),
]

for rule in regex_rules:
    code = re.sub(rule[0], rule[1], code)

print(code)
with open(fname + "_rust", "w") as fo:
    fo.write(code)