# tail-log

Watch log file that on linux os.

免登陆查看 Linux 上的日志. 通过 websocket 实时显示日志内容.

## quickstart

server:

```sh
git clone --depth=1 https://github.com/xiaoyureed/tail-log
cd tail-log
cargo build --release
./target/release/tail-log <log file path>
```

client: 克隆源码, 浏览器打开 ws_client.html 即可查看日志输出 (若没有内容, 那需要在本地启动一个静态服务器然后在服务器中打开查看)

