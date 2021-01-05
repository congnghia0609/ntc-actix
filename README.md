# ntc-actix
ntc-actix is a WebServer template using Actix Web framework by Rust  

## Install Rust
```shell
# https://www.rust-lang.org/tools/install
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

#==> restart computer.

# check version
rustc --version
cargo --version
```

## Quick start
```shell
# Build
cargo build

# Run
cargo run

# Check for errors code
cargo check

# Building for Release
cargo build --release

# Remove generated artifacts
cargo clean

# Execute unit and integration tests of a package
cargo test

# Build documentation provided by all of your dependencies locally and open it in your browser
cargo doc --open
```

## Start MongoDB
```shell
sudo mongod --config /etc/mongod.conf
or
nohup sudo mongod --config /etc/mongod.conf >/dev/null 2>&1 &
```

## Call API User
### 1. Register: POST /user/register
```shell
curl -X POST -i 'http://127.0.0.1:8080/user/register' \
  -H "Content-Type: application/json" \
  --data '{
    "name": "name",
    "surname": "surname",
    "email": "user@email.com",
    "password": "password"
  }'
```

### 2. Login: POST /user/login
```shell
# Request
curl -X POST -i 'http://127.0.0.1:8080/user/login' \
  -H "Content-Type: application/json" \
  --data '{
    "email": "user@email.com",
    "password": "password"
  }'

# Response
{"message":"You have successfully logged in.","status":true,"token":"eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJzdWIiOiJ1c2VyQGVtYWlsLmNvbSIsImV4cCI6MTYwOTUzNTE1NH0.TMVWEsH2r_4FjE28S-jGmzh9gfX0pzY771dg2TotiJY"}
```

### 3. User Information: GET /user/userInformations
```shell
TOKEN=eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJzdWIiOiJ1c2VyQGVtYWlsLmNvbSIsImV4cCI6MTYwOTUzNTE1NH0.TMVWEsH2r_4FjE28S-jGmzh9gfX0pzY771dg2TotiJY
curl -X GET -H 'Content-Type: application/json' \
  -H 'Authorization: Bearer '$TOKEN \
  -i 'http://127.0.0.1:8080/user/userInformations'

or

curl -X GET -H 'Content-Type: application/json' \
  -H 'Authorization: Bearer eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJzdWIiOiJ1c2VyQGVtYWlsLmNvbSIsImV4cCI6MTYwOTUzNTE1NH0.TMVWEsH2r_4FjE28S-jGmzh9gfX0pzY771dg2TotiJY' \
  -i 'http://127.0.0.1:8080/user/userInformations'
```

### 4. ProtectedRoute: POST /user/protectedRoute
```shell
curl -X GET -H 'Content-Type: application/json' \
  -H 'Authorization: Bearer '$TOKEN \
  -i 'http://127.0.0.1:8080/user/protectedRoute'
```


## License
This code is under the [Apache License v2](https://www.apache.org/licenses/LICENSE-2.0).  
