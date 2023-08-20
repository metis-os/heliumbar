use tokio::io::{AsyncBufReadExt, AsyncReadExt, BufReader};
pub async fn connect_socket() -> Option<tokio::net::UnixStream> {
    let uuid = std::env::var("HYPRLAND_INSTANCE_SIGNATURE");
    if let Err(err) = uuid {
        println!("{}", err);
        return None;
    }
    let path = format!("/tmp/hypr/{}/.socket2.sock", uuid.unwrap());
    let stream = tokio::net::UnixStream::connect(path).await;
    if let Err(err) = stream {
        println!("{}", err);
        return None;
    } //if

    return Some(stream.unwrap());
}

pub async fn listen(sender: glib::Sender<(String, String)>) {
    let stream = if let Some(stream) = connect_socket().await {
        stream
    } else {
        return;
    };

    let mut buffer = String::new();
    let mut reader = BufReader::new(stream);
    loop {
        while reader.read_line(&mut buffer).await.unwrap_or_default() > 0 {
            if let Some((action_name, action_value)) = buffer.split_once(">>") {
                sender.send((action_name.to_string(), action_value.to_string()));
            }
            buffer.clear();
        } //while
    } //loop
} //func
