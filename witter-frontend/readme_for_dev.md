
### 遇到的问题

> `cargo make watch` 执行后会不停地执行 build，尽管并没有文件改动

这是因为，watch 了 build 的结果，而 build 会产生新的文件，这又触发了 watch 。。。  

解决办法是，只 watch 源文件，在`Makefile.toml`中修改:

`watch = {watch = ["./src"]}`

