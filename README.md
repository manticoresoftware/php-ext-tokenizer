# php-ext-tokenizer

This extension enables the use of the tokenizer library from HuggingFace, allowing you to tokenize any text with a provided JSON file.

## How to Use

Here's an example of how to use it:

```php
<?php
use Manticore\Ext\Tokenizer;
// Using the BERT tokenizer.json file
$tokenizer = Tokenizer::create("tokenizer.json");
var_dump($tokenizer->tokenize("Hello world"));
var_dump($tokenizer->encode("Hello world"));
```

This will display the tokenized and encoded results for the given configuration and text:

```text
array(2) {
  [0]=>
  string(5) "hello"
  [1]=>
  string(5) "world"
}

array(2) {
  [0]=>
  int(7592)
  [1]=>
  int(2088)
}
```

## How to Build

You need to have `cargo` installed to build.

```bash
cargo build --release
```

After the build is complete, you can use the extension with PHP as usual.

```bash
$ php -d extension=target/release/libphp_ext_tokenizer.so -r 'var_dump(class_exists("Manticore\Ext\Tokenizer"));'
bool(true)
```
