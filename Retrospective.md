# Retrospectives

## 2026-05-26

### What did I observe today?

* Iterators are more like math; end up more clear
* Phil’s typing is suspect...
* Looking at array slices shows there is interesting memory ownership under the hood
  * Excited to learn more!
* Y’all did well without the pre-reading (except for John)
  * Thank you for doing the reading, John!
* Enjoyed the smaller group
  * Pretty relaxed
  * Good fit for not doing the reading!
* Nice to be working with Vectors, and having more data types
  * Embedded engineers can have nice things
  * What will work across all the targets that we deal with
* Adding a range or span type to C would be so nice!
  * Would fix a lot of security issues
* A lot more fun doing it as a group!

## 2026-05-20

### What stood out today?

* This ain’t C
  * Statements versus expressions is something to think about
  * It isn't crazy-different, like Perl or equivalent
  * Not too far off... more easy that transitioning to Javascript
* Still forgetting the syntax, as I'm not doing it consistently
* Rustlings is just a sampler plate of the languageA
* Worked a
* John‘s question about return versus no-return/expression
* Drew’s question about why Rust doesn't panic on an overflow
  * You can enable this behavior, as desired

## 2026-05-19

### What did I find interesting?

* Statement vs expression
  * Comparison with Javascript and C
* Starting to appreciate why someone says Javascript is Lisp-like
* Like that Rust doesn't need Yoda-isms to avoid assignment instead of comparison
* C is very string and file oriented
  * No concept of modules
  * Macros are string manipulation
  * Modules and Macros are more natural in Rust
* C isn't going away anytime soon, due to total weight of existing
  * Unix epoch is going to be a challenge
  * C starts out small/simple, but gets hard with all the other things you need to know
  * Undefined behavior is unacceptable in a safety critical system

## 2026-05-13

### What did I find interesting today?

* Learning about shadowing
  * C also allows shadowing, although only in inner scopes
* Data is immutable by default
* Compiler is very helpful
  * Super-clean
* Is 'let' the only way to create data?
  * Several to create data: let, const, static
    * This is about scope
* Are there other types
  * Many data types: u8, i8, f32, ... u128, i128, bool

## 2026-05-12

### What worked well?

* LiveShare worked well- once closed and reopened
* Teamwork correcting Phil's spelling
* Mobbing is nice and relaxed
* Good brain break from the workday
* Tried some different experiments to see different paths

### What was different or interesting?

* What is the benefit of shadowing?
  * There is a readme recommending against shadowing
  * You can block this with Clippy (the Rust linter)
* Macros and constants have types
  * Macros in Rust are hygienic
* Like the term syntactic sugar
* Traits and such can get hard to read

## 2026-05-11

### What worked well today?

* I was able to clone and make a change to the repo (davidtdc)

## end
