### rust-blogspot-auth0

## Instalation step-by-step API

- git clone -b edo https://github.com/padangpanda/rust-blogspot-auth0.git
- cd rust-blogspot-auth0
- buat file .env di folder sejajar /src dan cargo.toml, lalu masukkan = 
  - DATABASE_URL=postgres://"username postgres":"password postgres"@localhost/auth0_demo?sslmode=disable
  - TOKEN_KEY="bebas token key apa aja"
- jalankan di terminal = cargo install diesel_cli --no-default-features --features postgres
- tambahan setting untuk linux, install lewat terminal = sudo apt-get install libpq-dev
  - refer dokumentasi = https://github.com/diesel-rs/diesel/blob/master/guide_drafts/backend_installation.md
- tambahan setting untuk windows = https://www.yodiw.com/install-rust-diesel-in-windows-10-and-fix-issue/
- untuk lebih amannya jalankan = cargo build (proses compile agak lama, mohon bersabar)
- setelah semua step diatas sudah tidak ada error, jalankan di terminal = diesel migration run
  - step tambahan = check di GUI postgres masing2 apakah table "Users" sudah terbuat, dan terdapat kolom "email, password, name"
- untuk menjalankan hot reload di terminal = cargo watch -x run (bila belom bisa menjalankan cargo watch, bisa install lewat command "cargo install cargo-watch")
- selamat API sudah berjalan di local anda

### Endpoint API
## POST /register

Request Body
```json
{
"email": "blabla@email.com",
"password": "password minimal 8, huruf kecil, huruf besar, nomer",
"username": "username"
}
```

Response OK 201
```json
{
"message": "You are sucessfully registered.",
"data": "registered email"
}
```

Response error Bad_Request 400 (validate field email dan password)
```json
{
"message": "validate error",
"data": ""
}
```

Response error Bad_Request 400 (validate unique email)
```json
{
"message": "Email is already used.",
"data": ""
}
```

Response error Internal_Server_Error 500
```json
{
"message": "Internal Server Error",
"data": ""
}
```
## POST /login

Request Body
```json
{
"email": "blabla@email.com",
"password": "password minimal 8, huruf kecil, huruf besar, nomer"
}
```

Response OK 200
```json
{
"email": "blabla@email.com",
"username": "username",
"token": "token jwt"
}
```

Response error Unauthorized 401
```json
{
"message": "Wrong username or password, please try again",
"data": ""
}
```

Response error Internal_Server_Error 500
```json
{
"message": "Internal Server Error",
"data": ""
}
```