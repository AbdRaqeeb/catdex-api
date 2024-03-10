## Catdex API
This is an API built using `actix_web`, `postgres` and `Rust`.


### Requirements
- [] Postgres database
- [] Rust Runtime


### Usage
```bash
    cd catdex-api

    # install packages
    cargo build

    # add database to terminal env
    export DATABASE_URL=postgres://{{database-user}}:{{database-password}}@localhost/{{database-name}}
```

### SSL Certificates
> SSL certificates required because the server is served on HTTPS

```bash
    # install openssl
    sudo apt install openssl

    # Generate certificate
    openssl req -x509 -newkey rsa:4096 keyout key.pem out cert.pem days 365 sha256 subj "/CN=localhost"

    # geneate password less key
    openssl rsa -in key.pem -out key-no-password.pem

    #install required headers for OS
    sudo apt install libssl-dev
```

### Start Application
```bash
    # start server
    cargo run
```

### API
```bash
    # Get Cats
    curl https://localhost:5050/api/cats

    # Get Single Cat
    curl https://localhost:8080/api/cats/1

    # Create cat. NOTE image should be a path to an existing image file
    curl -F "name=Persian" -F "image=@persian.jpg" https://localhost:5050/api/cats
```
