#### 编程作业

*作业一：* 这里根据文档的指引修改内核。简要的说一说遇到的一些小问题。作业的目的是引入一个内核栈的初始化函数。实验文档的指引是以ch8为基础，而我的实验基于ch7，所以会略有不同。我修改了`task.rs`中关于之前手动初始化栈的代码（`exec`函数中），利用模块中的`init_stack`函数进行初始化栈。同时，ch7`makefile`中的`gdbclient`指令有些问题，之前没有发现，导致遇到一些问题。

*作业二：*这里会遇到三个`syscall`的系统调用指令。受文档中`通过 Manual Page 添加 Syscall`内容的启发，我并不需要实现每个`syscall`完整定义的功能，可能只需要实现部分即可。

+ `syscall_ioctl` ： 这个指令是用来控制设备的。文档中的相关描述有点复杂，猜测不用实现具体的功能，如果遇到`bug`再处理。事实上，确实直接返回0即可。

+ `syscall_writev：` 这个指令用来处理多个缓冲区的写入，功能与`write`类似。我先尝试了一下直接使用`write`的系统调用，结果发现程序会无法正常的运行。估计这个就需要我们自己实现。根据手册的规定，我修改了`syscall`文件夹下的`fs.rs`中的代码。

  ```rust
  /// 添加相关的数据结构
  pub struct Iovec {
      iov_base: *const u8,
      iov_len: usize,
  }
  /// 处理多个缓冲区
  pub fn sys_writev(fd: usize, mut iov: *const Iovec, iovcnt: usize) -> isize {
  		......
  for _ in 0..iovcnt {
              let ptr_iov = translated_ref(token, iov);
              let ptr_base:*const u8 = ptr_iov.iov_base;
              let ptr_len: usize = ptr_iov.iov_len;
              file.write(UserBuffer::new(translated_byte_buffer(token, ptr_base, ptr_len)));
              total_size += ptr_len;
              unsafe{
                  iov = iov.add(1);
              }
              // file.write(UserBuffer::new(translated_byte_buffer(token, buf, len))) as isize
          }
  }
  ```

+ `syscall_exit_group:` 这个的功能我们简化直接使用`exit`的系统调用。

解决完三个系统调用之后，成功的得到了预期的结果。

![image-20231115112135616](/home/user/2023a-rcore-LittleLucifer1/reports/image-20231115112135616.png
)

#### 问答作业

其中的 `options` 参数分别有哪些可能？

- `WNOHANG`：如果没有可等待的子进程立即可用，则立即返回，而不会阻塞。
- `WUNTRACED`：也等待已经暂停的子进程的状态变化。
- `WCONTINUED`：也等待已经继续执行的子进程的状态变化。
- `WSTOPPED`：与 `WUNTRACED` 相同，但已被标记为 `WIFSTOPPED` 的子进程被视为可等待的。
- `WEXITED`：只等待已经退出的子进程的状态变化。
- `WNOWAIT`：返回子进程的状态信息，但不等待子进程的状态变化。
- `WEDXIT`：在兼容性模式下等待已经退出的子进程的状态变化。
- `WCONTINUED`：在兼容性模式下等待已经继续执行的子进程的状态变化。

用`int`的32个bit如何表示？

可以使用二进制表示，而如果要判断是什么类型，则可以使用掩码。