import regex

# Written with Python 3.8
# Run from the project root to generate the JIS-to-unicode mappings file

def main():
    hex_pattern = regex.compile("0x([0-9a-fA-F]+)")
    unicode_pattern = regex.compile("U(\+[0-9a-fA-F]+)+")
    with open("jis/euc-jis-2004-std.txt", "r") as input:
        with open("src/jis213.rs", "w") as output:
            output.write("pub fn jis_to_utf8(code: u32) -> Option<&'static str> {\n")
            output.write("\tmatch code {\n")

            text = input.read()
            lines = text.split("\n")
            for line in lines:
                jis_match = hex_pattern.match(line)
                if not jis_match:
                    continue

                unicode_match = unicode_pattern.search(line)
                if not unicode_match:
                    continue

                jis = jis_match.groups()[0].zfill(6)
                
                escaped_parts = []
                
                for capture in unicode_match.captures(1):
                    escaped = f"\\u{{{capture[1:]}}}"
                    escaped_parts.append(escaped)

                unicode = "".join(escaped_parts)
                
                line = f"\t\t0x{jis} => Some(\"{unicode}\"),\n"
                output.write(line)
            
            output.write("\t\t_ => None,\n")
            output.write("\t}\n")
            output.write("}\n")
            

main()