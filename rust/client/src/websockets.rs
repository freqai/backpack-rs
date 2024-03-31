use std::sync::atomic::{AtomicBool, Ordering};
use futures::{SinkExt, StreamExt};
 
use serde::Deserialize;
use serde_json::from_str;
use tokio::net::TcpStream;
use tokio_tungstenite::tungstenite::handshake::client::Response;
use tokio_tungstenite::tungstenite::Message;
use tokio_tungstenite::WebSocketStream;
use tokio_tungstenite::{connect_async, MaybeTlsStream};
use url::Url;

use crate::error::*;
use base64::{engine::general_purpose::STANDARD, Engine};
use ed25519_dalek::{Signature, Signer, SigningKey};
use serde::Serialize;

#[derive(Serialize, Deserialize)]
struct Subscription {
    method: String,
    params: Vec<String>,
    signature: Vec<String>,
}


#[derive(Serialize, Deserialize)]
struct SubscriptionPub {
    method: String,
    params: Vec<String>,
}


const SIGNING_WINDOW: u32 = 5000;


pub fn order_update_stream() -> &'static str { "account.orderUpdatesd" }
pub fn book_ticker_stream(symbol: &str) -> String { format!("bookTicker.{symbol}") }


pub struct WebSockets<'a, WE> {
    pub socket: Option<(WebSocketStream<MaybeTlsStream<TcpStream>>, Response)>,
     
    handler: Box<dyn FnMut(WE) -> Result<()> + 'a + Send>,
    api_key: &'a str,
    api_secret: &'a str,
}

impl<'a, WE: serde::de::DeserializeOwned> WebSockets<'a, WE> {
    /// New websocket holder with default configuration
    /// # Examples
    /// see examples/binance_websockets.rs
    pub fn new<Callback>(handler: Callback, api_key: &'a str, api_secret: &'a str) -> WebSockets<'a, WE>
    where
        Callback: FnMut(WE) -> Result<()> + 'a + Send,
    {
        Self::new_with_options(handler, api_key , api_secret )
    }

    /// New websocket holder with provided configuration
    /// # Examples
    /// see examples/binance_websockets.rs
    pub fn new_with_options<Callback>(handler: Callback, api_key: &'a str, api_secret: &'a str) -> WebSockets<'a, WE>
    where
        Callback: FnMut(WE) -> Result<()> + 'a + Send,
    {
        WebSockets {
            socket: None,
            handler: Box::new(handler),
            api_key:api_key,
            api_secret:api_secret,
        }
    }

    fn sign(&self, endpoint: &str) -> Result<Vec<String>> {
        let instruction = "subscribe";

        if endpoint == "account.orderUpdate" {

            

            let timestamp = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)?
                .as_millis();

        
            let api_secret = STANDARD
                .decode(self.api_secret)?
                .try_into()
                .map_err(|_| Error::SecretKey)?;
        
            let signer = SigningKey::from_bytes(&api_secret);
            let verifier = signer.verifying_key();

            let mut signee = format!("instruction={instruction}");
        
            signee.push_str(&format!("&method=SUBSCRIBE"));
            signee.push_str(&format!("&params={endpoint}"));
            signee.push_str(&format!("&timestamp={timestamp}&window={SIGNING_WINDOW}"));
            tracing::debug!("signee: {}", signee);

            let signature: Signature = signer.sign(signee.as_bytes());
            let signature = STANDARD.encode(signature.to_bytes());

        return  Ok(vec![
                self.api_key.to_string(),
                signature,
                timestamp.to_string(),
                SIGNING_WINDOW.to_string(),
            ]);
            
        }

       return  Ok(vec![]);

    }

    
    /// Connect to a websocket endpoint
    pub async fn connect(&mut self, endpoint: &str) -> Result<()> {
        let wss: String = format!("{}", "wss://ws.backpack.exchange");
        let url = Url::parse(&wss)?;

        self.handle_connect(url,endpoint).await
    } 
    async fn handle_connect(&mut self, url: Url,endpoint: &str) -> Result<()> {

        match connect_async(url).await {
            Ok(answer) => {
                

                self.socket = Some(answer);
                
                let mut message:String = "".to_string();

                if endpoint == "account.orderUpdate" {

                 
                    // 创建订阅消息
                    let subscribe_message = Subscription {
                        method: "SUBSCRIBE".to_string(),
                        params: vec![endpoint.to_string()],
                        signature:self.sign(endpoint).unwrap(),
                    };
                      message = serde_json::to_string(&subscribe_message)?;

                } else {
                    let subscribe_message =  SubscriptionPub{
                        method: "SUBSCRIBE".to_string(),
                        params: vec![endpoint.to_string()],
                    };
                      message = serde_json::to_string(&subscribe_message)?;

                }

 
                // 发送订阅消息
                if let Some(ref mut socketw) = self.socket {

                    println!("send message :{}",message);

                    socketw.0.send(Message::Text(message)).await?;

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
                        println!("receive message :{}",msg.as_str());

                        let event: WE = from_str(msg.as_str())?;
                        println!("from_str");

                        (self.handler)(event)?;
                        println!("handler");

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
