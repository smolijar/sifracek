# Sifracek

```bash
$ sifracek encode "HELLOWORLD"
```

```markdown
[Morse]: Standard morse-code encoding with dots and dashes
...././.-../.-../---/.--/---/.-./.-../-..

[Reverse]: Reverses a string
DLROWOLLEH

[Snail]: Curl input in spiral from center outward to a square plane
  O R L D
  W H E
  O L L
```


```
Usage: sifracek <COMMAND> [OPTIONS] <INPUT>

Commands:
  encode  Encode input with selected algo (or all available)
  decode  Decode input with selected algo (or all available)
  help    Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help information
  -V, --version  Print version information
  
Usage: sifracek decode [OPTIONS] <INPUT>

Arguments:
  <INPUT>  

Options:
  -a, --algo <ALGO>  [possible values: morse, rev, snail]
  -h, --help         Print help information
```


## Roadmap

 - [ ] Specific options for decoders (e.g. `--dot="_/\_" --dash="_/ˇˇˇˇˇ\_"`)
 - [ ] [Caesar cipher](https://en.wikipedia.org/wiki/Caesar_cipher)
 - [ ] [SMS cypher](https://www.dcode.fr/multitap-abc-cipher)
 - [ ] Autogenerate README examples on build
 - [ ] Snail decode
