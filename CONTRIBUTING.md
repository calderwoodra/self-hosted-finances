### Setup
#### Installs
1. [Install Rust](https://doc.rust-lang.org/book/ch01-01-installation.html)
2. [Install cargo-watch](https://crates.io/crates/cargo-watch)
   1. `cd backend/ && cargo install watch && cd ..`
   2. `cd cli/ && cargo install watch && cd ..`
3. `brew install lefthook && lefthook install`
4. Setup PostgreSQL
   1. `brew install postgresql@13` 
   2. `echo 'export PATH="/usr/local/opt/postgresql@13/bin:$PATH"' >> ~/.bash_profile`
   3. `source ~/.bash_profile`
   4. `brew services start postgresql@13`
   5. `createuser -s -r db-finance-user`
5. Setup Diesel
   1. `cargo install diesel_cli --no-default-features --features postgres`
6. `cp .env.sample .env` and follow the instructions there 

### Running the web server
`make server`
