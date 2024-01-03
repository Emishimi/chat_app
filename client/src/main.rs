use tokio::net::TcpStream;
use tokio_tungstenite::{connect_async, tungstenite::protocol::Message};
use futures_util::{StreamExt, SinkExt};
use url::Url;

#[tokio::main]
async fn main() {
    // The URL of the WebSocket server
    let url = Url::parse("ws://localhost:8080/ws/").expect("Invalid WebSocket URL");

    // Connect to the server
    let (ws_stream, _) = connect_async(url)
        .await
        .expect("Failed to connect to WebSocket server");
    println!("WebSocket client connected");

    let (write, read) = ws_stream.split();

    // Sending a message to the WebSocket server
    let send_task = tokio::spawn(async move {
        let mut write = write;
        write
            .send(Message::Text("Hello WebSocket Server!".into()))
            .await
            .expect("Failed to send message");
        // You can add more message sending logic here
    });

    // Receiving messages from the WebSocket server
    let receive_task = tokio::spawn(async move {
        let mut read = read;
        read.for_each(|message| async {
            match message {
                Ok(msg) => println!("Received: {:?}", msg),
                Err(e) => eprintln!("Error: {:?}", e),
            }
        }).await;
    });

    // Wait for both tasks to complete
    let _ = tokio::try_join!(send_task, receive_task).unwrap();
}
