import re

# Written with Python 3.8
# Run from the project root to generate the JIS-to-unicode mappings file

# Mappings file comes from here:
# https://github.com/unicode-org/icu/blob/main/icu4c/source/data/mappings/jisx-212.ucm

pattern = re.compile(r"<U([\dA-F]{4})> \\x([\dA-F]{2})\\x([\dA-F]{2})")
with open("./jis/jisx-212.ucm", "r") as input:
    with open("./src/jis212.rs", "w") as output:
        output.write("pub fn jis212_to_utf8(code: u16) -> Option<char> {\n")
        output.write("\tmatch code {\n")

        text = input.read()
        matches = pattern.findall(text)
        for match in matches:
            unicode_raw = match[0]
            jis_raw = "".join(match[1:])
            variant = f"\t\t0x{jis_raw} => Some('\\u{{{unicode_raw}}}'),\n"
            output.write(variant)

        output.write("\t\t_ => None,\n")
        output.write("\t}\n")
        output.write("}\n")