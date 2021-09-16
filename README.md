## Description

From [Wikipedia](<https://en.wikipedia.org/wiki/Polyglot_(computing)>):

> In computing, a polyglot is a computer program or script written in a valid form of multiple programming languages, which performs the same operations or output independent of the programming language used to compile or interpret it.

This repo contains a polyglot Hello World for Rust and C++.

## Running

```bash
~/polyglot_rscpp ~> g++ -x c++ polyglot.rs
~/polyglot_rscpp ~> ./a.out
Hello world
```

```bash
~/polyglot_rscpp ~> rustc polyglot.rs
~/polyglot_rscpp ~> ./polyglot
Hello world
```

Tested with `rustc` 1.54.0 and `g++` 9.3.0.

See also on [godbolt.org](https://godbolt.org/z/bafcb8j9q) and the [Rust Playground](https://play.rust-lang.org/?version=stable&mode=debug&edition=2018&gist=cbd0796f45da2f1ccb048bcdf2726735).

## Implementation

The easiest way to get started on a polyglot is to leverage differences in comment syntax. For example, `#define ...` will be a comment in many languages, but in C++ it'll be a macro definition, and once you have macros you can freely rewrite C++'s syntax—something to keep in mind next time you're writing C++ and wish you had all the expressiveness of bash scripts.

The problem here is that Rust and C++ have the same comment syntax: `/* */` for block comments and `//` for line comments. But there's a catch! One problem that some C++ developers find themselves running into while debugging is that block comments [_don't nest_](https://en.cppreference.com/w/cpp/comment). In particular:

```cpp
/* I am in a comment

/* I am still in a comment

*/ Yikes the outer comment got closed by that

*/ And this one is just dangling
```

But what about Rust? It turns out that nested comments [are supported](https://doc.rust-lang.org/reference/comments.html#non-doc-comments)! Let's try it.

```rust
/* I am in a comment

/* I am still in a comment

*/ This one closes the inner comment, I'm still in the outer one

*/ And this one closes the outer comment
```

The upshot of this is that we can now create a section of code visible to C++ but invisible to Rust. Let's use that to make `main` compatible with both languages:

```cpp
/*
/*
*/

#define fn int

*/

fn main() {
}
```

There's still one problem here—that dangling `*/`. The fix is as stupid as you might've guessed:

```cpp
/*
/*
*/

#define fn int

// */

fn main() {
}
```

All that's left now is to make it print in both languages. I ran into one more roadblock here: we can't just do `#define print! printf`, because macros and `!` don't seem to play well with each other. From a quick Google, we can print to `stdout` without macros in Rust like this:

```rust
use std::io::Write;

fn main() {
  std::io::stdout().write(b"Hello, world!\n").unwrap();
}
```

This nice thing about this is that it's mostly plausible C++ code, apart from that bytestring literal. Since we can define whatever C++ we want inside our comment, we can stub all of this out:

```cpp
namespace std {
    namespace io {
        // We need unwrap to be defined on a class somewhere
        class Unwrappable {
        public:
            void unwrap() {}
        };

        // stdout() will default construct this
        class stdout {
        public:
            Unwrappable write(const char *str) {
                std::cout << str;
                return Unwrappable();
            }
        };

        // Turn the Write trait import into a dummy typedef import
        using Write = int;
    }
}

// Turn that bytestring into a normal string
#define b

// Rust and C++ have slightly different syntax here
#define use using
```

And just like that, our Rust program is valid C++! You can check the full source in [polyglot.rs](./polyglot.rs) and try compiling both ways either locally or using the links above.

## Applications

Please don't.

## License

Free to use as long as you include a copy of this license and an implementation for one extra language in the same file.
