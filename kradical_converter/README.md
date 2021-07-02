# Kradical Converter

[![LICENSE](https://img.shields.io/crates/l/kradical_converter)](https://crates.io/crates/kradical_converter)
[![Crates.io Version](https://img.shields.io/crates/v/kradical_converter)](https://crates.io/crates/kradical_converter)

A binary for converting the original JIS-encoded files to other formats. It is also able to combine multiple of these files. Below is an example invocation. For more information, use `cargo run -- --help`. 

`kradical_converter radk unicode --inputs .\assets\edrdg_files\radkfile .\assets\edrdg_files\radkfile2 --output .\assets\outputs\radk_utf8.txt`


## License

These binaries are distributed under [GNU General Public License v3.0](https://choosealicense.com/licenses/gpl-3.0/). Note that the EDRDG files are distributed under [different terms](http://www.edrdg.org/edrdg/licence.html).