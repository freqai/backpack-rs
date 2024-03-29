use std::sync::atomic::{AtomicBool, Ordering};

use futures::StreamExt;
use serde_json::from_str;
use tokio::net::TcpStream;
use tokio_tungstenite::tungstenite::handshake::client::Response;
use tokio_tungstenite::tungstenite::Message;
use tokio_tungstenite::WebSocketStream;
use tokio_tungstenite::{connect_async, MaybeTlsStream};
use url::Url;

use crate::config::Config;
use crate::errors::*;


#[derive(Serialize, Deserialize)]
struct Subscription {
    method: String,
    params: Vec<String>,
    signature: Vec<String>,
}



pub fn order_update_stream() -> &'static str { "account.orderUpdatesd" }
pub fn book_ticker_stream(symbol: &str) -> String { format!("bookTicker.{symbol}") }


pub struct WebSockets<'a, WE> {
    pub socket: Option<(WebSocketStream<MaybeTlsStream<TcpStream>>, Response)>,
    handler: Box<dyn FnMut(WE) -> Result<()> + 'a + Send>,
    conf: Config,
}

impl<'a, WE: serde::de::DeserializeOwned> WebSockets<'a, WE> {
    /// New websocket holder with default configuration
    /// # Examples
    /// see examples/binance_websockets.rs
    pub fn new<Callback>(handler: Callback) -> WebSockets<'a, WE>
    where
        Callback: FnMut(WE) -> Result<()> + 'a + Send,
    {
        Self::new_with_options(handler, Config::default())
    }

    /// New websocket holder with provided configuration
    /// # Examples
    /// see examples/binance_websockets.rs
    pub fn new_with_options<Callback>(handler: Callback, conf: Config) -> WebSockets<'a, WE>
    where
        Callback: FnMut(WE) -> Result<()> + 'a + Send,
    {
        WebSockets {
            socket: None,
            handler: Box::new(handler),
            conf,
        }
    }

    
    /// Connect to a websocket endpoint
    pub async fn connect(&mut self, endpoint: &str) -> Result<()> {
        let wss: String = format!("{}", "wss://ws.backpack.exchange");
        let url = Url::parse(&wss)?;

        self.handle_connect(url).await
    } 
    async fn handle_connect(&mut self, url: Url) -> Result<()> {
        match connect_async(url).await {
            Ok(answer) => {
                self.socket = Some(answer);

                // 创建订阅消息
                let subscribe_message = Subscription {
                    method: "SUBSCRIBE".to_string(),
                    params: vec!["stream".to_string()],
                    signature: vec![
                        "<verifying key>".to_string(),
                        "<signature>".to_string(),
                        "<timestamp>".to_string(),
                        "<window>".to_string(),
                    ],
                };

                // 序列化订阅消息为JSON字符串
                let message = serde_json::to_string(&subscribe_message)?;

                // 发送订阅消息
                if let Some(ref mut socket) = self.socket {
                    socket.send(Message::Text(message)).await?;
                }



                
                Ok(())
            }
            Err(e) => Err(Error::Msg(format!("Error during handshake {e}"))),
        }
    }

   

    /// Disconnect from the endpoint
    pub async fn disconnect(&mut self) -> Result<()> {
        if let Some(ref mut socket) = self.socket {
            socket.0.close(None).await?;
            Ok(())
        } else {
            Err(Error::Msg("Not able to close the connection".to_string()))
        }
    }

    pub fn socket(&self) -> &Option<(WebSocketStream<MaybeTlsStream<TcpStream>>, Response)> { &self.socket }

    pub async fn event_loop(&mut self, running: &AtomicBool) -> Result<()> {
        while running.load(Ordering::Relaxed) {
            if let Some((ref mut socket, _)) = self.socket {
                // TODO: return error instead of panic?
                let message = socket.next().await.unwrap()?;

                match message {
                    Message::Text(msg) => {
                        if msg.is_empty() {
                            return Ok(());
                        }
                        let event: WE = from_str(msg.as_str())?;
                        (self.handler)(event)?;
                    }
                    Message::Ping(_) | Message::Pong(_) | Message::Binary(_) | Message::Frame(_) => {}
                    Message::Close(e) => {
                        return Err(Error::Msg(format!("Disconnected {e:?}")));
                    }
                }
            }
        }
        Ok(())
    }
}
