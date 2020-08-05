# SimpleProfanityFilter

A simple profanity filter implemented in Rust.

Currently the filter is just converting everything to lower case and comparing to the list at [RobertJGabriel/Google-profanity-words](https://github.com/RobertJGabriel/Google-profanity-words)

Don't actually use this one when there's better Rust versions out there which are on crates.io ([kaikalii/censor](https://github.com/kaikalii/censor) for example)

## Downloading

```bash
git clone https://github.com/JamesFator/SimpleProfanityFilter.git
cd SimpleProfanityFilter
cargo build
```

## Usage
1. Include the crate.
2. Instantiate new instance of the filter.
3. Invoke the `filter` method on a str and the output will be a filtered copy.
```rust
extern crate simple_profanity_filter;
use simple_profanity_filter::ProfanityFilter;

let filter = ProfanityFilter::new();
let filtered_word = filter.filter("Hello world!");
```

## Testing
There's a few tests set up to test what this library can and cannot handle right now.
```bash
cargo test
```

## Example
If you want to play around with the library, there's an example binary you can run. Once started, type a sentence, press `Return` and the binary will print the filtered sentence. If you wish to quit, interrupt with a `Ctrl-c`.
```bash
cargo run --example cli
```

## License
[MIT](https://choosealicense.com/licenses/mit/)
