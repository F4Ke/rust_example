# RUSTY

## Config

The project is dockerized.

## Docker

[to finish]
`docker-compose up`

## Native install

But if you want to have it natively:
install rust
<https://www.rust-lang.org/tools/install>

Database is running from docker,
we have a `.env`
all env variable should be prefixed by `APP_SERVER_` , or `APP_DATABASE_` 

### Hot reloading

install cargo-watch 
`cargo install cargo-watch`

then you can run
`cargo watch -x run`

### First install

0. `cargo install diesel_cli --no-default-features --features postgres` 
1. `cargo build`
2. `source .env`
3. `diesel setup`

#### lpq lib error

if you have the following error on mac os 
`= note: ld: library not found for -lpq`

use: `brew install libiconv` to resolve it

if it doenst fix it: you can

```
brew install libpq
brew link --force libpq
```

remember to export variables at the end of the script, then reload the bash;
you should have 
```
export PATH="/opt/homebrew/opt/libpq/bin:$PATH"
export PQ_LIB_DIR="$(brew --prefix libpq)/lib"
export LDFLAGS="-L/opt/homebrew/opt/libpq/lib"
export CPPFLAGS="-I/opt/homebrew/opt/libpq/include"
```

then `cargo clean`
and retry the install

## Database

we use postgres from the docker-compose file
with that we use the `Diesel` orm : https://diesel.rs/guides/getting-started


### Migrations 

migration path : `./migrations`

generate a migration file 
`diesel migration generate create_books`
this will create the `up` and `down` files

run a migration `diesel migration run`


## Routes

the API is prefixed by `/api/`
front end `/books`

## Tests

run `cargo test`
