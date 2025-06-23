# rsmear

Smear adjacent pixels in an image to make a mess. This is more just
a fun tool rather than something you should ever use for accurate
smearing effects for an image.

# usage

```
Usage: rsmear [OPTIONS] <INPUT_FILE>

Arguments:
  <INPUT_FILE>  input file for smearing

Options:
  -o, --output-file <OUTPUT_FILE>  output file name [default: output.png]
  -c, --chance <CHANCE>            chance of smear per pixel [default: 0.5]
      --s-height <S_HEIGHT>        max height of smear [default: 10]
      --s-width <S_WIDTH>          max width of smear [default: 10]
      --no-overflow                dont allow blending calculations to overflow
  -h, --help                       Print help
```
