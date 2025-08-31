# Kradical

Utilities for working with the [Electronic Dictionary Research and Development Group](https://www.edrdg.org/) (EDRDG) [radical decomposition](https://www.edrdg.org/krad/kradinf.html) files.


## Usage

### Converting EDRDG Files

The `kradical_converter` tool can convert EDRDG radical files to various formats. Use multiple `--inputs` flags to process multiple files together.

The JSON and UTF8 txt files are created under `/assets/outputs/` with the below commands.

#### Create JSON
With output extension `.json` and `json` option:
```bash
# Generate krad.json
cargo run -p kradical_converter -- --inputs ./assets/edrdg_files/kradfile --inputs ./assets/edrdg_files/kradfile2 --output ./assets/outputs/krad.json krad json

# Generate radk.json
cargo run -p kradical_converter -- --inputs ./assets/edrdg_files/radkfile --inputs ./assets/edrdg_files/radkfile2 --output ./assets/outputs/radk.json radk json
```

#### Create UTF-8 txt
With output extension `.txt` and `unicode` option:
```bash
# Generate krad_utf8.txt (kanji → radicals mapping)
cargo run -p kradical_converter -- --inputs ./assets/edrdg_files/kradfile --inputs ./assets/edrdg_files/kradfile2 --output ./assets/outputs/krad_utf8.txt krad unicode

# Generate radk_utf8.txt (radical → kanji mapping)
cargo run -p kradical_converter -- --inputs ./assets/edrdg_files/radkfile --inputs ./assets/edrdg_files/radkfile2 --output ./assets/outputs/radk_utf8.txt radk unicode
```

#### Create Rust static files for `kradical_static`
With output extension `.rs` and `rust` option:
```bash
# Generate decompositions.rs (for kradical_static crate)
cargo run -p kradical_converter -- --inputs ./assets/edrdg_files/kradfile --inputs ./assets/edrdg_files/kradfile2 --output ./kradical_static/src/decompositions.rs krad rust

# Generate memberships.rs (for kradical_static crate)
cargo run -p kradical_converter -- --inputs ./assets/edrdg_files/radkfile --inputs ./assets/edrdg_files/radkfile2 --output ./kradical_static/src/memberships.rs radk rust
```

#### Executable usage
1. First, build the release version of the converter:

```bash
cargo build --release
```

2. Then replace `cargo run -p kradical_converter -- --inputs` with `./target/release/kradical_converter --inputs` in the above commands.
    - On Windows also replace `/` with `\` for the path.


## Crates

More information about each crate included with the project in the associated readme files:

- [JIS](kradical_jis/README.md)
- [Parsing](kradical_parsing/README.md)
- [Converter](kradical_converter/README.md)
- [Static](kradical_static/README.md)


## Other

Included with this project under `assets/outputs` are several UTF-8-encoded variants of the source files in a more convenient format.

- `krad_utf8.txt` follows the same format as the original `kradfile`. Each line contains the following:
    - The kanji
    - A colon
    - Each of constituent radicals separated by spaces
- `radk_utf8.txt` differs from the original `radkfile` and instead mirrors the `kradfile` format. Each line contains the following:
    - The radical
    - The number of strokes in the radical
    - A colon
    - Each of the kanji that contain the radical separated by spaces


## License

In accordance with the [EDRDG license statement](http://www.edrdg.org/edrdg/licence.html), this project is distributed under the [Attribution-ShareAlike 3.0 Unported](https://creativecommons.org/licenses/by-sa/3.0/legalcode) license. The files included under `assets/edrdg_files` were downloaded from the [Monash Nihongo FTP Archive](http://ftp.edrdg.org/pub/Nihongo/00INDEX.html#dic_fil) and are the property of EDRDG.

JIS X 0212 conversion tables are distributed under the [Unicode license](http://www.unicode.org/copyright.html).