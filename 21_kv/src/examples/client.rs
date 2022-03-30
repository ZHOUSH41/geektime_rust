use anyhow::Result;
use async_prost::AsyncProstStream;
use futures::prelude::*;
use kv::{CommandRequest, CommandResponse};
use tokio::net::TcpStream;
use tracing::info;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let addr = "127.0.0.1:9527";
    // 连接服务器
    let stream = TcpStream::connect(addr).await?;
    // 创建一个 AsyncProstStream 来处理 TCP Frame
    let mut client = AsyncProstStream::<_, CommandResponse, CommandRequest, _>::from(stream);

    // 生成一个 HSET 命令
    let cmd = CommandRequest::new_hget("table1", "hello", "world".into());

    // 发送HSET命令
    client.send(cmd).await?;
    if let Some(Ok(data)) = client.next().await {
        info!("Got response {:?}", data);
    }    

    Ok(())
}