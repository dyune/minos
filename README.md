Learning about operating systems (McGill's ECSE 427) this fall semester was quite fun, especially with the topics of process management, virtual memory management, and persistent storage. 
I found it to be quite a fascinating class as it really explained the black magic that runs our computers. Actually, for this course, a main method of evaluation was implementing a toy OS
that called upon topics we learnt in class such as manual memory management, process scheduling, and demand paging. There are some topics we haven't implemented but learnt about, 
such as disk operations, file systems and demand paging combined with segmentation or multi-level pages. 

With this in mind, I thought about expanding this assignment to implement these cool new features I never got to touch on. But, then I remembered how hard it was
to parse a string with C and how many times I ended up getting `SIGSEGV`! 

So, like any good programmer in 2025, I decided to re-write it in Rust because I find the language interesting and want to learn it, and I'd rather fight the borrow checker than
to get another segfault :D

Features (to come):
* File System implementation (hefty one)
* Allocating variables in heap and stack for programs
* Multi-level page (jury's still out on this one)
