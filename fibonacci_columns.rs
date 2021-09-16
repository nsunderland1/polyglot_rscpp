/* /**/ #include <iostream>                                      // */
/*/**/ uint32_t fibonacci(uint32_t n)                            // */ fn fibonacci(mut n: u32) -> u32
                                        {
                                           if (n < 2) { return n; }
/*/**/     uint32_t                                              // */ let mut               
                                           f0 = 0;
/*/**/     uint32_t                                              // */ let mut 
                                           f1 = 1;
/*/**/     uint32_t                                              // */ let mut  
                                           temp = 0;
                                           while (n > 1) {
                                               temp = f1;
                                               f1 = f0 + f1;
                                               f0 = temp;
                                               n = n - 1;
                                           }

                                           return f1;
                                        }

/* /**/ int                                                      // */ fn
                                        main() {
/* /**/     std::cout <<                                         // */ println!("{}",
                                            fibonacci(10)
/* /**/               << std::endl                               // */ )
                                            ;
                                        }