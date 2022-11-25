# Money Parser

Rewrite of python based [money_parser](https://gitlab.com/intheflow/money_parser)

## Input formats

* [sgkb](https://www.sgkb.ch/)
* [easybank](https://www.easybank.at/) (currently needs preprocessing (encoding))
* [revolut](https://www.revolut.com/)
* [neon](https://www.neon-free.ch/)

## Output formats

* [homebank](https://homebank.free.fr/)


## Quickstart

* https://rustup.rs/
* `rustup install stable`
* `cargo build --release`
* executable in `target/release/money_parser_rust`


## Formats

# easybank

**NOTE:** easybank exports are iso-8859-1 encoded and start with a bom

To convert a easybank csv convert it to utf-8 and remove the bom:
```sh
TMPFILE=`mktemp`
iconv "$SOURCEFILE" -f ISO-8859-1 -o "$TMPFILE"
sed -i 's/\xef\xbb\xbf//' $TMPFILE
mv "$TMPFILE" "$SOURCEFILE"
```

# SGKB

**NOTE:** SGKB exports depend on your UI language, only english exports are supported
