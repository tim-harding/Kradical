# Kradical

Utilities for working with the [Electronic Dictionary Research and Development Group](https://www.edrdg.org/) (EDRDG) [radical decomposition](https://www.edrdg.org/krad/kradinf.html) files.


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