use serde::{Deserialize, Serialize};
use std::env;
use std::thread;
use std::time::{Duration, Instant};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let order_service_url =
        env::var("MAKELINE_SERVICE_URL").unwrap_or_else(|_| "http://localhost:3001".to_string());

    let orders_per_hour: u64 = env::var("ORDERS_PER_HOUR")
        .unwrap_or_else(|_| "1".to_string())
        .parse()
        .unwrap_or(1);

    if orders_per_hour == 0 {
        println!("Please set the ORDERS_PER_HOUR environment variable");
        std::process::exit(1);
    }

    println!("Orders to process per hour: {}", orders_per_hour);

    let minutes: f64 = 60.0;
    let seconds: f64 = 60.0;

    // calculate the time between orders in seconds
    let order_processing_interval: f64 = (minutes / (orders_per_hour as f64)) * seconds;
    println!(
        "Order processing interval: {} seconds",
        order_processing_interval
    );

    // time to sleep between orders
    let sleep_duration = Duration::from_secs_f64(order_processing_interval);
    println!("Sleep duration between orders: {:?}", sleep_duration);

    // keep track of how long we've been running
    let start_time = Instant::now();

    let client = reqwest::blocking::Client::new();
    loop {
        let orders = client
            .get(format!("{}/order/fetch", order_service_url))
            .send()
            .and_then(|response| response.error_for_status())
            .and_then(|response| response.text());
        let orders = match orders {
            Ok(body) => match serde_json::from_str::<Option<Vec<Order>>>(&body) {
                Ok(orders) => orders.unwrap_or_default(),
                Err(err) => {
                    eprintln!("Invalid order response: {}", err);
                    Vec::new()
                }
            },
            Err(err) => {
                eprintln!("Failed to fetch orders: {}", err);
                Vec::new()
            }
        };
        if orders.is_empty() {
            println!("No orders to process");
            thread::sleep(sleep_duration);
            continue;
        }
        for mut order in orders {
            order.status = COMPLETE;
            let response = client
                .put(format!("{}/order", order_service_url))
                .header("Content-Type", "application/json")
                .body(serde_json::to_string(&order)?)
                .send()
                .and_then(|response| response.error_for_status());
            match response {
                Ok(_) => println!(
                    "Order {} completed at {:.2?} with status {}",
                    order.order_id,
                    start_time.elapsed(),
                    order.status
                ),
                Err(err) => eprintln!("Failed to complete order {}: {}", order.order_id, err),
            }
            thread::sleep(sleep_duration);
        }
    }
}

const COMPLETE: u32 = 2;

#[derive(Debug, Deserialize, Serialize)]
struct Order {
    #[serde(rename = "orderId")]
    order_id: String,
    #[serde(rename = "customerId")]
    customer_id: String,
    items: Vec<Item>,
    status: u32,
}

#[derive(Debug, Deserialize, Serialize)]
struct Item {
    #[serde(rename = "productId")]
    product_id: u32,
    quantity: u32,
    price: f32,
}
