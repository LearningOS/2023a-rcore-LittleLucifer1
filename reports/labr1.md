#### 编程作业

1. 第一个作业很简单，就是跟着文档的步骤，一步步扩展`easy-fs-fuse`，使得它可以生成同时包含 Rust 和 C 用户程序的镜像。只是文档中有一个指令的错误，刚开始看的时候一脸蒙B，不过仔细想想，应该是作者的笔误。

2. 第二个作业是修改内核中的代码。这里讲一讲我自己的心路历程。

   一看到实验概述中说，最好完成ch6。我本来打算进入一个新的stage，没想到还得看stage2中的代码。不过也好，之前的阶段二时间过于仓促，没有对操作系统有个完整的了解。不过没想到，对于刚学操作系统的我来说，这个文件系统有点点复杂。

   好在花了大概3天的时间，我终于把所有的理论知识和代码的逻辑都弄明白了。之后，开开心心的进入lab1。在大概看完整个文档的指引后，大概对于如何编译c文件和全国操作系统大赛的初赛难度有了一个较为清楚的了解。到了第二个实验了，我尝试着跑了一下程序，结果发现了`Incorrect argc`，在`hello.c`文件中发现了这个错误的信息。不过这个是为什么呢？我尝试着将`argc != 1`的判断条件改为`argc == 1 or 2 or 0`。结果发现都报错。说明这个`argc`压根就没接收到合适的值。应该是解析命令行参数的地方出了问题。所以，我又开始看ch7了:cry:。结合整个的rcore的代码和文档中小心的特意提示。原来应该是在内核态到用户态传入的a0寄存器值有问题。

   解决思路：rcore中解析命令行的栈布局其实是不符合文档的规范的。而在lab1中的文档中则给出了正确的栈布局。同时，rcore中的rust程序在解析这个参数`main(argc, argv)`时，得到的是内核态的a0 和 a1，而两个c语言程序则是利用`sp`。而这个`sp`指向在`rcore`中指向的是栈顶，但是在c程序的栈结构布局中，其实是栈尾。因此，答案也就出来了。我们只需要改变rcore中这部分的栈结构布局的代码就好了。

   于是，我修改了`task.rs`中的文件

   原代码

   ```rust
   // // push arguments on user stack
           // user_sp -= (args.len() + 1) * core::mem::size_of::<usize>();
           // let argv_base = user_sp;
           // let mut argv: Vec<_> = (0..=args.len())
           //     .map(|arg| {
           //         translated_refmut(
           //             memory_set.token(),
           //             (argv_base + arg * core::mem::size_of::<usize>()) as *mut usize,
           //         )
           //     })
           //     .collect();
           // *argv[args.len()] = 0;
           // for i in 0..args.len() {
           //     user_sp -= args[i].len() + 1;
           //     *argv[i] = user_sp;
           //     let mut p = user_sp;
           //     for c in args[i].as_bytes() {
           //         *translated_refmut(memory_set.token(), p as *mut u8) = *c;
           //         p += 1;
           //     }
           //     *translated_refmut(memory_set.token(), p as *mut u8) = 0;
           // }
           // // make the user_sp aligned to 8B for k210 platform
           // user_sp -= user_sp % core::mem::size_of::<usize>();
   
   ```

   现代码

   ```rust
   let mut argv_addr: Vec<usize> = Vec::new();
           
           for i in 0..args.len() {
               user_sp -= args[i].len() + 1;
               
               let mut p = user_sp;
               argv_addr.push(user_sp);
               for c in args[i].as_bytes() {
                   *translated_refmut(memory_set.token(), p as *mut u8) = *c;
                   p += 1;
               }
               *translated_refmut(memory_set.token(), p as *mut u8) = 0;
           }
   
           user_sp -= (args.len() + 1) * core::mem::size_of::<usize>();
           let argv_base = user_sp;
           let mut argv: Vec<_> = (0..=args.len())
               .map(|arg| {
                   translated_refmut(
                       memory_set.token(), 
                       (argv_base + arg * core::mem::size_of::<usize>()) as *mut usize,)
               })
               .collect();
           for i in 0..args.len() {
               *argv[i] = argv_addr[i];
           }
           *argv[args.len()] = 0;
           user_sp -= core::mem::size_of::<usize>();
           *translated_refmut(memory_set.token(), user_sp as *mut usize) = args.len();
   
   ```

   这样`sp`指向了栈头，第一个8字节的数是`argc`，之后若干字节就是指向`argv`字符串的指针。

#### 问答题

+ elf 文件和 bin 文件有什么区别？

  elf文件实际上是一个可执行文件，以ELF魔数开头的描述文件空间布局和信息的文件。而bin文件，实际上是对ELF文件做了一个阉割。所以只是包括了数据部分的内容。没有关于整个文件的说明信息。实际上，也正如我们所预料，`file ch6_file0.elf`可以看到有`ELF 64-bit LSB executable`的字样，而bin文件，就不算是一个文件，将其反汇编放入一个文件中，可以看到程序的汇编代码。