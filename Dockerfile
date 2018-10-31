FROM rust:1.29

WORKDIR /usr/src/burning-pro-server
COPY burning-pro-server burning-pro-server

RUN cargo install --debug --path ./burning-pro-server

EXPOSE 8080

VOLUME /data/db
VOLUME /data/config

ENV DATABASE_URL=/data/db/db.sqlite3
ENV DOTENV=/data/config/env

CMD ["burning-pro-server"]
