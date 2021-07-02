# Notes on the original formats

Working out how to convert the original files to something more usable was more difficult than I would have anticipated, so I just want to take this space to document the formats for anyone else who might need to work with these files in the future. 


## Encoding

In general, ASCII characters can be treated as-is when they appear in comments, colon delimiters, and whitespace. This is true of any of the JIS encodings, each of whose Japanese characters occupy a range of bytes that doesn't conflict with ASCII. However, I found that simply applying an encoding conversion to these files was unsuccessful. None of the Japanese codecs offered in Rust's [encoding](https://docs.rs/encoding/0.2.33/encoding/codec/japanese/index.html) crate could translate a file to UTF-8 without errors. The characters being decomposed also come from different JIS character sets depending on the file, and the radicals may be in a different encoding from the kanji. This makes things a bit annoying to deal with. I've broken out below what encodings worked for me in different parts of each file. Please also read the comments in each of the files for additional grammar details. To be able to inspect the non-ASCII bytes easily, I recommend my [ascii_hexdump](https://github.com/tim-harding/ascii_hexdump) project.


### JIS X 0208

These characters are always two bytes long. I personally used a JIS X 0213 to Unicode conversion table because it seems to be compatible and it was easier to find a reference for it. 


#### Radicals

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


### JIS X 0212

Where they stand alone, these characters are three bytes long. The JIS X 0212 to Unicode conversion table I found does not seem to work on these. Instead, the EUC-JP codec works. JIS X 0212 is also used for the kanji in the `radkfile`, but each character is not necessarily 3 bytes long and they aren't whitespace delimited. Just take those entire lines and run them through the EUC-JP codec rather than trying to split out individual characters by hand.  


## `kradfile`

Each pair of two bytes is in JIS X 0208. 

```text
�� : �� �� ��
```


## `kradfile2`

The first three bytes, the kanji, are from JIS X 0212. The remaining two-byte radical characters are in JIS X 0208. 

```text
��� : �� �� ��
```


## `radkfile` and `radkfile2`

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