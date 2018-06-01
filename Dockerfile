FROM rust:latest

RUN apt-get update; \
    apt-get install -y --no-install-recommends \
        libgtk-3-dev \
        ; \
    rm -rf /var/lib/apt/lists/*;
