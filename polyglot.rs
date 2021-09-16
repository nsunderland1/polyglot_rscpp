/*

/*


*/
#include <iostream>

#define fn int

namespace std {
    namespace io {
        class Unwrappable {
        public:
            void unwrap() {}
        };

        class stdout {
        public:
            Unwrappable write(const char *str) {
                std::cout << str;
                return Unwrappable();
            }
        };

        using Write = int;
    }
}

#define b

#define use using

// */

use std::io::Write;

fn main() {
  std::io::stdout().write(b"Hello world\n").unwrap();
}
