# evil-backend
## Setup
`docker-compose up -d` run postgres container  
`cargo install cargo-watch` install cargo-watch  
`cargo install diesel_cli --no-default-features --features postgres`  install diesel  
`diesel migration run` run migrations  
`cargo watch -x run` run server
## TODO
- [ ] Someway to add to inventory
- Process transactions
    - [x] Checkout session
    - [ ] Decrement inventory
    - [ ] Check over how payments are processed
- more stuff...
- [ ] Secure API
- [ ] Pen test