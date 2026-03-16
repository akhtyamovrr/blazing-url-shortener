# Blazing Url shortener

This project aim is to learn how to implement web service that works with classic industrial infrastructure (Postgres, Kafka, gRPC, ...).

It is implemented step by step - from naive implementation to more complex one.

In the beginning it just interacts with local DB without any transactions control. Later I plan to implement such patterns as `Transactional Outbox` and `Unit of Work` to solve enterprise-like problems. 

To run this project, use `cargo run --release`. It expects that you already have PG running on 5432 port with database name `test-db`. Create table `links` with two text columns: `id` and `full_url`. 


Run Postgres using `docker run --name some-postgres -e POSTGRES_PASSWORD=mysecretpassword -p 5432:5432 -d postgres`

Check tests coverage (install `llvm-cov` to run it)

```
cargo llvm-cov --open
```

See file `todo.txt` with plans for project improvements and expected features. 

