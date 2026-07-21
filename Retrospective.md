<!-- markdownlint-disable MD024 -->
# Retrospectives

## 2026-07-21

### What was our experience today?

* Nice with a small group
  * Larger group sometimes comes up with more ideas!
* Big language
  * Need to dig deeper to build real intuition

## 2026-07-15

### How could I imaging using today’s learning?

* Nice to remember that something exists, so you can google it later.
* Helps to come prepared - which I wasn't, today!

## 2026-07-14

### How could I imaging using today’s learning?

* I'll use “syntactic sugar” because it is fun to say!
* Need to practice where to put the generic in a signature
* LISP encoded hardware in its function name
* .into() checking ranges is super nice
* You can change something to mutable when passed by ownership
  * You have to specify this in the trait signature for pass by reference

## 2026-07-07

### What did I find interesting?

* Three is the sweet spot for mobbing!
  * Sometimes too much time between being active with larger groups
* Option and Result, better to default to match instead of if/else
  * Match conditions being expressions make them more powerful
  * If conditions are expressions, but are more verbose
  * Match is doing coverage checking, if/else does not
* Consistent pattern of Result and Option make library code more readable
  * Better to be "as expected"

## 2026-06-30

### What was a head-scratcher and what might be useful?

* Use into_iter() in the quiz was better than iter()
* Quiz2 would be better TDD if you could get test feedback on each branch
* Probably the hardest, so far...
  * Following the compiler hints could have kept me from learning.
  * Use the "rustc --explain <CODE>" to dig deeper
* Doing learning as a group and trying different approaches creates more learning

## 2026-06-23

### What are my reflections on today’s learnings?

* Appreciate making it work with a small group!
* .or_insert() and .or_default() are interesting functions
  * Allows you to condense things into a single line
* Interesting seeing the lint suppression to error when suppression is not doing anything
  * Allows you to remove stale suppressions
* Aliasing in modules was interesting, as well

## 2026-06-22

### What are my reflections on today’s learnings?

* Liked trying other ways to solve the problem
  * Gives more learning and good discussion on why things might be preferred
* Need to grab more food! Keep your brain working!
* From CGM Dev Learning... the section on Modules needs more examples
  * Modules can get confusing if not organized well
* Noticing as a beginner... using can require some complex syntax
  * But its safe!
* Across lots of language, be careful about what is constructed, replaced, etc
  * Read the documentation carefully!!!
* Like the organization of modules
* Good to be able to use HashMaps
  * Heapless crate makes this available for embedded systems!
  * Cool that we get to have nice things!

## 2026-06-10

### What do I think about enums?

* More unions than enums!
* Really awesome when paired with match expressions.
* Storage... check the AI article in the chat
  * Notionally: variant, union

### What do I think about the various strings?

* Size of the smart pointer was interesting:
  * String: Pointer, Length, Capacity
  * &str: Pointer, Length
* What we call &str:
  * String slice!
* Cool that &str can receive &str, string literals, and String.

## 2026-06-03

### What was neat in this session?

* Unit structs are fun
  * Not used for default values in match (switch) statements!
  * Zero element, zero size!
* Talking about compiler letting a mutable reference go out of scope
  * Compiler captures borrowing
* Structs, no classes
* ".." (struct update) syntax to assign default struct

## 2026-06-02

### What stood out today?

* Good job of using hints and one another!
* Learning about bindings
* Rust compiler messages are often very helpful, but not always...
* Cloning versus switching the order... whether we reuse or create anew
* "Notional Style Guide" conversations
* Digging in on what is really happening in memory
* Kindness for the person coming in late

### Is six too many?

* Great hearing everyone's thoughts
* Everyone got to go twice, which was good
* Seven or eight would probably need to split
* Nice for the non-Rust discussion to have everyone together

## 2026-05-27

### How did today's session go? What did I find interesting…

* Great getting reminded of the Rust syntax
* Appreciate exploring different options
  * Learning about the details of types, smart pointers, etc…
  * Lots of learning in the questions
* Nice to look at the low-level details
  * Used to it just working and not thinking about what is actually happening
* UTF encoding of one to four bytes is interesting and good to learn
* Lots of ways to initialize a vector
* 99_i32 notation for encoding type in a literal
  * 99L is the same in 'C'
  * <https://en.cppreference.com/cpp/language/integer_literal>

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
