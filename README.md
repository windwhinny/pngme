# pngme
A png parser, implements of https://picklenerd.github.io/pngme_book/introduction.html

# Useage

```
Usage: pngme <COMMAND>

Commands:
  encode  
  remove  
  create  
  print   
  help    Print this message or the help of the given subcommand(s)

Options:
  -h, --help  Print help
```

## Encode
```
Usage: pngme encode <FILE_PATH> <CHUNK_TYPE> <MESSAGE> [OUTPUT_FILE]

Arguments:
  <FILE_PATH>    File to read from.
  <CHUNK_TYPE>   String
  <MESSAGE>      The data bytes appropriate to the chunk type, if any.
  [OUTPUT_FILE]  

Options:
  -h, --help  Print help
```

## Remove
```
Usage: pngme encode <FILE_PATH> <CHUNK_TYPE> <MESSAGE> [OUTPUT_FILE]

Arguments:
  <FILE_PATH>    File to read from.
  <CHUNK_TYPE>   String
  <MESSAGE>      The data bytes appropriate to the chunk type, if any.
  [OUTPUT_FILE]  

Options:
  -h, --help  Print help
```

## Print
```
Usage: pngme print [OPTIONS] <FILE_PATH>

Arguments:
  <FILE_PATH>  File to read from.

Options:
  -i, --index <INDEX>            print the full data at index
  -c, --chunk-type <CHUNK_TYPE>  print the full data with chunk type
  -s, --string                   print the full data as string
  -h, --help                     Print help
```

## Create
```
Usage: pngme create <FILE_PATH>

Arguments:
  <FILE_PATH>  File path to create.

Options:
  -h, --help  Print help
```
