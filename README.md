# Kradical

Utilities for working with the [Electronic Dictionary Research and Development Group](https://www.edrdg.org/) (EDRDG) [radical decomposition](https://www.edrdg.org/krad/kradinf.html) files.


## Crates

### jis

Lookup tables for converting JIS X 0212 and JIS X 0213 characters into UTF-8. These files are generated using the Python scripts in `assets/jis`.


### kradical_parsing

Parsers for the following file types. For each, the first provides mappings for the set of characters in the JIS X 0208 standard, the second provides mappings for the additional characters in the JIS X 0212 standard. The contents is converted from the original mixed JIS X 0208, JIS X 0212, and EUC-JP encoding into UTF-8. 


#### `kradfile` and `kradfile2`

Gives the consituent radicals for a given kanji. 


#### `radkfile` and `radkfile2`

Lists the kanji that include a given radical. This is an inverted mapping from the `kradfile`. 


### kradical_converter    

A binary for converting the original JIS-encoded files to other formats. It is also able to combine multiple of these files. The outputs of this program are provided with the repository:

- UTF-8 variants of the original formats are available under `assets/outputs`. 
    - `krad_utf8.txt` follows the same format as the original `kradfile`. Each line contains the following:
        - The kanji
        - A colon
        - Each of constituent radicals separated by spaces
    - `radk_utf8.txt` is adapted from the original format at my discretion for ease of use. Each line contains the following:
        - The radical
        - The number of strokes in the radical
        - Optionally, one of the following:
            - The tag `alt_image(NAME)`, where `NAME` is the name of an image file used by the [WWWJDIC](http://nihongo.monash.edu/cgi-bin/wwwjdic?1C) server as a better representation of the radical. This is likely of limited utility but included for completeness. 
            - The tag `alt_glyph(GLYPH)`, where `GLYPH` is an alternative glyph for the radical. In general, this should be used wherever available. The original authors where limited to what was available in the JIS X 0208 character set to represent each radical, but with UTF-8 we can do much better. 
- Rust variants, available in the `kradical_static` crate.

Below is an example invocation. For more information, use `cargo run -- --help`. 

`cargo run -- radk unicode --inputs .\assets\edrdg_files\radkfile .\assets\edrdg_files\radkfile2 --output .\assets\outputs\radk_utf8.txt`


### kradical_static

Rust files containing the parsed contents of the radical decompositions. If you need to work with the radical decompositions but don't specifically need to do any parsing work, these can simply be imported as-is. The source radical decompositions are updated infrequently so it is unlikely that these are out of date, but please submit a PR if you notice there are fresh edits available. 


## Notes on the original formats

Working out how to convert the original files to something more usable was more difficult than I would have anticipated, so I just want to take this space to document the formats for anyone else who might need to work with these files in the future. 

### Encoding

In general, ASCII characters can be treated as-is when they appear in comments, colon delimiters, and whitespace. This is true of any of the JIS encodings, each of whose Japanese characters occupy a range of bytes that doesn't conflict with ASCII. However, I found that simply applying an encoding conversion to these files was unsuccessful. None of the Japanese codecs offered in Rust's [encoding](https://docs.rs/encoding/0.2.33/encoding/codec/japanese/index.html) crate could translate a file to UTF-8 without errors. The characters being decomposed also come from different JIS character sets depending on the file, and the radicals may be in a different encoding from the kanji. This makes things a bit annoying to deal with. I've broken out below what encodings worked for me in different parts of each file. For characters in JIS X 0208, I personally used JIS X 0213 because it seems to be compatible and it was easier to find a conversion reference for it. 


### `kradfile`

Each pair of two bytes is in JIS X 0208. 

```text
�� : �� �� ��
```


### `kradfile2`

The first the bytes, the radical, are 

```text
��� : �� �� ��
```


## License

In accordance with the [EDRDG license statement](http://www.edrdg.org/edrdg/licence.html), this project is distributed under the [Attribution-ShareAlike 3.0 Unported](https://creativecommons.org/licenses/by-sa/3.0/legalcode) license. The files included under `assets/edrdg_files` were downloaded from the [Monash Nihongo FTP Archive](http://ftp.edrdg.org/pub/Nihongo/00INDEX.html#dic_fil) and are the property of EDRDG.