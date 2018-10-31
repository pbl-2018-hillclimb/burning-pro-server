FROM rust:1.30

WORKDIR /usr/src/burning-pro-server
COPY burning-pro-server burning-pro-server

RUN cargo install --debug --path ./burning-pro-server

EXPOSE 8080

VOLUME /data/db

ENV DATABASE_URL=/data/db/db.sqlite3

CMD ["burning-pro-server"]
