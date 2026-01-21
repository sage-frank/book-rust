
use std::pin::{Pin,pin};
use std::time::Duration;
use trpl;

fn main() {
    // trpl::run(async {
    //     let handler = trpl::spawn_task(async {
    //         for i in 1..10 {
    //             println!("hi number {i} form the first task");
    //             trpl::sleep(Duration::from_millis(500)).await;
    //         }
    //     });
    //
    //     for i in 1..5 {
    //         println!("hi number {i} form the second task");
    //         trpl::sleep(Duration::from_millis(500)).await;
    //     }
    //
    //     handler.await.unwrap();
    // })

    // trpl::run(
    //
    //     async {
    //
    //         let fut1 = async {
    //             for i in 1..10 {
    //                 println!("hi number {i} from the first task");
    //                 trpl::sleep(Duration::from_millis(500)).await;
    //             }
    //         };
    //
    //         let fut2 = async {
    //         for i in 1..5 {
    //             println!("hi number {i} from the second task");
    //             trpl::sleep(Duration::from_millis(500)).await;
    //         }
    //         };
    //
    //
    //         trpl::join(fut1, fut2).await;
    //
    //     }
    //
    //
    // )
    // block_on
    trpl::run(async {
        let (tx, mut rx) = trpl::channel();

        let tx1 = tx.clone();

        let tx_fu1 = pin!(async move {
            let vals = vec![
                String::from("hi"),
                String::from("from"),
                String::from("the"),
                String::from("future"),
            ];

            for val in vals {
                tx.send(val).unwrap();
                trpl::sleep(Duration::from_millis(500)).await;
            }
            // drop(tx) ;
        });

        let tx_fut2 = pin!( async {
            while let Some(value) = rx.recv().await {
                println!("got {value}");
            }
        });

        let tx_fu2 = pin!( async move {
            let vals = vec![
                String::from("hi"),
                String::from("from"),
                String::from("the"),
                String::from("future"),
            ];

            for val in vals {
                tx1.send(val).unwrap();
                trpl::sleep(Duration::from_millis(500)).await;
            }
            // drop(tx) ;
        });
        // trpl::join3(tx_fu1, tx_fut2,tx_fu2).await;

        //trpl::join!(tx_fut2, tx_fu2, tx_fu1);
        // let futures = vec![tx_fut2, tx_fu2,tx_fu1];

        // let futures: Vec<Pin<Box<dyn Future<Output = ()>>>> =
        //     vec![Box::pin(tx_fu1), Box::pin(tx_fu2), Box::pin(tx_fut2)];

        let futures :Vec<Pin<&mut dyn Future<Output=()>>> = vec![
            tx_fu1,tx_fu2,tx_fut2
        ];

        trpl::join_all(futures).await;
    })
}
