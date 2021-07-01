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

- Rust variants, available in the `kradical_static` crate.
- UTF-8 variants of the original formats are available under `assets/outputs`. 
    - `krad_utf8.txt` follows the same format as the original `kradfile`. Each line contains the following:
        - The kanji
        - A colon
        - Each of constituent radicals separated by spaces
    - `radk_utf8.txt` differs from the original `radkfile` and instead mirrors the `kradfile` format. Each line contains the following:
        - The radical
        - The number of strokes in the radical
        - A colon
        - Each of the kanji that contain the radical separated by spaces

Below is an example invocation. For more information, use `cargo run -- --help`. 

`kradical_converter radk unicode --inputs .\assets\edrdg_files\radkfile .\assets\edrdg_files\radkfile2 --output .\assets\outputs\radk_utf8.txt`


### kradical_static

Rust files containing the parsed contents of the radical decompositions. If you need to work with the radical decompositions but don't specifically need to do any parsing work, these can simply be imported as-is. The source radical decompositions are updated infrequently so it is unlikely that these are out of date, but please submit a PR if you notice there are fresh edits available. 


## Notes on the original formats

Working out how to convert the original files to something more usable was more difficult than I would have anticipated, so I just want to take this space to document the formats for anyone else who might need to work with these files in the future. 


### Encoding

In general, ASCII characters can be treated as-is when they appear in comments, colon delimiters, and whitespace. This is true of any of the JIS encodings, each of whose Japanese characters occupy a range of bytes that doesn't conflict with ASCII. However, I found that simply applying an encoding conversion to these files was unsuccessful. None of the Japanese codecs offered in Rust's [encoding](https://docs.rs/encoding/0.2.33/encoding/codec/japanese/index.html) crate could translate a file to UTF-8 without errors. The characters being decomposed also come from different JIS character sets depending on the file, and the radicals may be in a different encoding from the kanji. This makes things a bit annoying to deal with. I've broken out below what encodings worked for me in different parts of each file. Please also read the comments in each of the files for additional grammar details. To be able to inspect the non-ASCII bytes easily, I recommend my [ascii_hexdump](https://github.com/tim-harding/ascii_hexdump) project.


#### JIS X 0208

These characters are always two bytes long. I personally used a JIS X 0213 to Unicode conversion table because it seems to be compatible and it was easier to find a reference for it. 


##### Radicals

Unfortunately, since the JIS X 0208 character set is limited, the authors in some cases had to use characters that contained the radical they wanted instead of the radical itself. They recommend a number of Unicode characters that better represent the radicals they wanted. However, some of the suggestions appear to be incorrect and others that are used by the WWWJDIC server are missing. Below are the replacements used by this library. Thanks to [Jisho](https://jisho.org/#radical) for some excellent alternate characters they found. 

- 化 -> ⺅
- 个 -> 𠆢
- 并 -> 丷
- 刈 -> ⺉
- 込 -> ⻌
- 尚 -> ⺌
- 忙 -> ⺖
- 扎 -> 扌
- 汁 -> ⺡
- 犯 -> ⺨
- 艾 -> ⺾
- 邦 -> ⻏
- 阡 -> ⻙
- 阡 -> ⻖
- 老 -> ⺹
- 杰 -> ⺣
- 礼 -> ⺭
- 疔 -> ⽧
- 禹 -> ⽱
- 初 -> ⻂
- 買 -> ⺲
- 滴 -> 啇
- 乞 -> 𠂉


#### JIS X 0212

Where they stand alone, these characters are three bytes long. The JIS X 0212 to Unicode conversion table I found does not seem to work on these. Instead, the EUC-JP codec works. JIS X 0212 is also used for the kanji in the `radkfile`, but each character is not necessarily 3 bytes long and they aren't whitespace delimited. Just take those entire lines and run them through the EUC-JP codec rather than trying to split out individual characters by hand.  


### `kradfile`

Each pair of two bytes is in JIS X 0208. 

```text
�� : �� �� ��
```


### `kradfile2`

The first three bytes, the kanji, are from JIS X 0212. The remaining two-byte radical characters are in JIS X 0208. 

```text
��� : �� �� ��
```


### `radkfile` and `radkfile2`

The two bytes identifying the radical are in JIS X 0208. Glyph alternate representations are four characters in the set \[0-9A-Z\]. These are given as JIS X 0212 characters in hexadecimal, but unlike the other JIS X 0212 characters used in these files, the EUC-JP codec does _not_ work. You must use the JIS X 0212 to Unicode conversion table instead. If the alternate representation does not match the hexadecimal form, it corresponds to an image from the WWWJDIC server, for example [js02](http://nihongo.monash.edu/gif212/js02.png).

```text
# No alternate representation
$ �� 1
# Glyph alternate representation
$ �� 3 4A6D
# Image alternate representation
$ �� 2 js01
```

The lines following radical identifiers are in JIS X 0212.


```text
�����������������...
```


## License

In accordance with the [EDRDG license statement](http://www.edrdg.org/edrdg/licence.html), this project is distributed under the [Attribution-ShareAlike 3.0 Unported](https://creativecommons.org/licenses/by-sa/3.0/legalcode) license. The files included under `assets/edrdg_files` were downloaded from the [Monash Nihongo FTP Archive](http://ftp.edrdg.org/pub/Nihongo/00INDEX.html#dic_fil) and are the property of EDRDG.