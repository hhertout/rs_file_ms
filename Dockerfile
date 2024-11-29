FROM rust

WORKDIR /app

COPY . .

RUN ["cargo", "run"]