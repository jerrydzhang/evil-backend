# evil-backend
## Setup
`docker-compose up -d` run postgres container  
`cargo install cargo-watch` install cargo-watch  
`cargo install diesel_cli --no-default-features --features postgres`  install diesel  
`diesel migration run` run migrations  
`cargo watch -x run` run server  
`stripe listen --forward-to localhost:8080/api/stripe_webhooks` listen for stripe webhooks
