## Installing postgresql on WSL
https://learn.microsoft.com/en-us/windows/wsl/tutorials/wsl-database 

### Rust <-> Postgres connector
Define .env file with DATABASE_URL=postgres://username:password@localhost/mybotdb

### Open Postgres
sudo -u postgres psql

Create database: `CREATE DATABASE mybotdb;` and connect to this db: `sudo -u postgres psql -d mybotdb`

### Some commands
**Creating a User**

CREATE USER test_user WITH PASSWORD 'test_password';


### Migrations
These are sequential files, kind of like a log of every change made to our table. 

We can "add" a change by `sqlx migrate add some_descriptive_name` -> this will add a .sql file in migrations/timestamp_some_descriptive_name.sql

You can edit this .sql and write stuff, like CREATE TABLE messages;
Then when you run `sqlx migrate run`, it will apply each .sql file in migrations/ sequentially based on the timestamp
The sequence is written in a table called _sqlx_migrations