use hello::ThreadPool;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::time::Duration;
use std::{fs, thread};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let pool = ThreadPool::new(4);

    //    // incoming 返回 TcpStream 的迭代器，stream 代表一个客户端和服务端之间打开的 connection
    //    // connection 代表客户端连接服务端、服务端生成响应以及服务端关系连接的全部请求/响应过程
    //    for stream in listener.incoming() {
    //        let stream = stream.unwrap();
    //
    //        pool.execute(|| handle_connection(stream));
    //    }

    // show case for graceful shutting down after 2 requests
    for stream in listener.incoming().take(2) {
        let stream = stream.unwrap();

        pool.execute(|| handle_connection(stream));
    }

    println!("Shutting down.")
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 512];

    stream.read(&mut buffer).unwrap();
    // println!("Request: {}", String::from_utf8_lossy(&buffer[..]));

    // b"" 字节字符串语法将其转换为字节字符串 &str -> &[u8,]
    let get = b"GET / HTTP/1.1\r\n";
    let sleep = b"GET /sleep HTTP/1.1\r\n";

    let (status_line, filename) = if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK\r\n\r\n", "hello.html")
    } else if buffer.starts_with(sleep) {
        thread::sleep(Duration::from_secs(5));
        ("HTTP/1.1 200 OK\r\n\r\n", "hello.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND\r\n\r\n", "404.html")
    };

    let contents = fs::read_to_string(filename).unwrap();

    let response = format!("{}{}", status_line, contents);

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}

// HTTP 是一个基于文本的协议
// 一个请求有如下格式：
// Method Request-URI HTTP-VERSION CRLF
// headers CRLF
// message-body

// 第一行叫做 request line
// 请求函数 统一资源标识符(URI) HTTP客户端版本 CRLF序列（代表回车和换行 carriage return line feed, \r\n）
// 下面这个请求实体从 Host: 开始的都是 headers，GET请求没有 body

// GET / HTTP/1.1
// Host: 127.0.0.1:7878
// Connection: keep-alive
// Cache-Control: max-age=0
// Upgrade-Insecure-Requests: 1
// User-Agent: Mozilla/5.0 (Macintosh; Intel Mac OS X 10_14_6) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/76.0.3809.132 Safari/537.36
// Sec-Fetch-Mode: navigate
// Accept: text/html,application/xhtml+xml,application/xml;q=0.9,image/webp,image/apng,*//*//*;q=0.8,application/signed-exchange;v=b3
// Sec-Fetch-Site: cross-site
// Accept-Encoding: gzip, deflate, br
// Accept-Language: zh-CN,zh;q=0.9,en;q=0.8

// 缩写响应
// 一个响应有如下格式：
// HTTP-Version Status-Code Reason-Phrase CRLF
// headers CRLF
// message-body

// 第一行叫做 status line
// HTTP版本 一个数字状态码 一个描述之前状态码的文本原因短语，CRLF 序列之后是任意 header，另一个 CRLF 序列，和响应的 body
// HTTP/1.1 200 OK\r\n\r\n
