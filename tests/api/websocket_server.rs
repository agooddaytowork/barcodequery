use tungstenite::connect;
use url::Url;

use crate::helpers::spawn_app;

#[tokio::test(flavor = "multi_thread")]
async fn websocket_endpoint_works() -> anyhow::Result<()> {
    let app = spawn_app().await;
    let web_socket_url = format!("ws://{}/ws", app.address);
    let (mut socket, response) =
        connect(Url::parse(&web_socket_url).unwrap()).expect("Can't connect");

    println!("Connected to the server");
    println!("Response HTTP code: {}", response.status());
    println!("Response contains the following headers:");
    for (ref header, _value) in response.headers() {
        println!("* {}", header);
    }

    assert!(!response.status().is_server_error()); // not server error
    socket.close(None).unwrap();

    Ok(())
}
