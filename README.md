# virtual-worker

This is a Rust app that simulates order completion. It is meant to be used in conjunction with the makeline-service to simulate a real-world order processing scenario.

## Running the app locally

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install)
- [Docker](https://docs.docker.com/get-docker/)
- [Docker Compose](https://docs.docker.com/compose/install/)

### Running the app

This app relies on the makeline-service and the mongodb instance running. A docker-compose file is provided to make this easy. The docker-compose file will also start the RabbitMQ, MongoDB, order-service, and virtual-customer services to simulate incoming orders. 

To run the necessary services, clone the repo, open a terminal, and navigate to the `virtual-worker` directory. Then run the following command:

```bash
docker compose up
```

With the services running, open a new terminal and navigate to the `virtual-worker` directory. Then run the following command:

```bash
export MAKELINE_SERVICE_URL=http://localhost:3001
export ORDERS_PER_HOUR=3600
cargo run
```

The `MAKELINE_SERVICE_URL` environment variable is used to tell the virtual worker where to send the order completion messages. The `ORDERS_PER_HOUR` environment variable is used to tell the virtual worker how many orders to complete per hour. We'll set it to `3600`, which is one order per second.

When the app is running, you should see output similar to the following:

```text
Orders to process per hour: 3600
Order processing interval: 1 seconds
Sleep duration between orders: 1s
Order 86827 completed at 29.06ms with status 2
Order 52881 completed at 1.04s with status 2
Order 54344 completed at 2.06s with status 2
```
