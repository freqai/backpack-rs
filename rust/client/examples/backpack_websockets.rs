#[macro_use]
extern crate tokio;
use backpack_client::websockets::*;
use backpack_client::ws_model::{WebsocketEvent, WebsocketEventUntag};
use futures::future::BoxFuture;

use std::sync::atomic::{AtomicBool};

use tokio::sync::mpsc::UnboundedSender;


#[tokio::main]
async fn main() {

    let (logger_tx, mut logger_rx) = tokio::sync::mpsc::unbounded_channel::<WebsocketEvent>();
    let (close_tx, mut close_rx) = tokio::sync::mpsc::unbounded_channel::<bool>();
    let wait_loop = tokio::spawn(async move {
        'hello: loop {
            select! {
                event = logger_rx.recv() => println!("{event:?}"),
                _ = close_rx.recv() => break 'hello
            }
        }
    });
    let streams: Vec<BoxFuture<'static, ()>> = vec![
        
    //Box::pin(book_ticker(logger_tx.clone())),
    Box::pin(order_update(logger_tx.clone())),
       
    ];

    for stream in streams {
        tokio::spawn(stream);
    }

    select! {
        _ = wait_loop => { println!("Finished!") }
        _ = tokio::signal::ctrl_c() => {
            println!("Closing websocket stream...");
            close_tx.send(true).unwrap();
            tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
        }
    }
}


#[allow(dead_code)]
async fn book_ticker(logger_tx: UnboundedSender<WebsocketEvent>) {
    let keep_running = AtomicBool::new(true);
    let book_ticker: String = book_ticker_stream("WIF_USDC");

    let mut web_socket: WebSockets<'_, WebsocketEventUntag> = WebSockets::new(|events: WebsocketEventUntag| {
        if let WebsocketEventUntag::WebsocketEvent(we) = &events {
            logger_tx.send(we.clone()).unwrap();
        }
        if let WebsocketEventUntag::BookTickerDataEvent(tick_event) = events {
            println!("{tick_event:?}")
        }
        Ok(())
    }, "yQ3SUCT9hfYEvj0y/myaVc7IEqB8VKRdDozqt3pFBQg=","uJOx7uP5LOn9FTFycUmSGQumwoCzOhgVlQnwvNyeOTU=" );

    web_socket.connect(&book_ticker).await.unwrap(); // check error
    if let Err(e) = web_socket.event_loop(&keep_running).await {
        println!("Error: {e}");
    }
    web_socket.disconnect().await.unwrap();
    println!("disconnected");
}


#[allow(dead_code)]
async fn order_update(logger_tx: UnboundedSender<WebsocketEvent>) {
    let keep_running = AtomicBool::new(true);
    let order_update: String = order_update_stream();

    let mut web_socket: WebSockets<'_, WebsocketEventUntag> = WebSockets::new(|events: WebsocketEventUntag| {
        if let WebsocketEventUntag::WebsocketEvent(we) = &events {
            logger_tx.send(we.clone()).unwrap();
        }
        if let WebsocketEventUntag::OrderDataEvent(tick_event) = events {
            println!("{tick_event:?}")
        }
        Ok(())
    }, "yQ3SUCT9hfYEvj0y/myaVc7IEqB8VKRdDozqt3pFBQg=","uJOx7uP5LOn9FTFycUmSGQumwoCzOhgVlQnwvNyeOTU=" );

    web_socket.connect(&order_update).await.unwrap(); // check error
    if let Err(e) = web_socket.event_loop(&keep_running).await {
        println!("Error: {e}");
    }
    web_socket.disconnect().await.unwrap();
    println!("disconnected");
}