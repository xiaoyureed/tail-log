use std::thread;
use websocket::sync::Server;
use websocket::OwnedMessage;
use std::io::BufReader;
use std::process::Command;
use std::process::Stdio;
use std::io::prelude::*;

fn main() {
    // read from cli
    let mut args = std::env::args();
    args.next().unwrap();

    let log_path = args.next().unwrap();

    let server = Server::bind("127.0.0.1:2794").unwrap();
    for request in server.filter_map(Result::ok) {
        let log_path = log_path.clone();
        // Spawn a new thread for each connection.
        thread::spawn(move || {
            if !request.protocols().contains(&"rust-websocket".to_string()) {
                request.reject().unwrap();
                return;
            }

            let mut client = request.use_protocol("rust-websocket").accept().unwrap();

            let ip = client.peer_addr().unwrap();

            println!("Connection from {}", ip);

            let message = OwnedMessage::Text("**************tail 工具**********************".to_string());
            client.send_message(&message).unwrap();

            let child = Command::new("tail")
                .args(&["-f", log_path.as_str()])
                .stdin(Stdio::piped())
                .stdout(Stdio::piped())
                .spawn()
                .expect("failed tail command");

            let stdout = child.stdout.expect("error of stdout");
            let mut buf_reader = BufReader::new(stdout);
            let mut line = String::new();
            loop {
                buf_reader.read_line(&mut line).expect("error of read line");
                // write!(std::io::stdout(), "{}", &line.as_str()).expect("error of write");
                let msg = OwnedMessage::Text(line.clone());
                client.send_message(&msg).unwrap();
                line.drain(..);
            }


            // let (mut receiver, mut sender) = client.split().unwrap();

            // let path = receiver.incoming_messages().next().unwrap();

            // for message in receiver.incoming_messages() {
            //     let message = message.unwrap();

            //     match message {
            //         OwnedMessage::Close(_) => {
            //             let message = OwnedMessage::Close(None);
            //             sender.send_message(&message).unwrap();
            //             println!("Client {} disconnected", ip);
            //             return;
            //         }
            //         OwnedMessage::Ping(ping) => {
            //             let message = OwnedMessage::Pong(ping);
            //             sender.send_message(&message).unwrap();
            //         }
            //         _ => sender.send_message(&message).unwrap(),
            //     }
            // }

            

        });
    }
}
