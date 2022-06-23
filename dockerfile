FROM node:18-buster

RUN apt-get update && apt-get install -y \
    curl \
    build-essential \
    libssl-dev \
    gradle \
    openjdk-17-jdk

RUN npm install -g pnpm pm2

RUN curl https://sh.rustup.rs -sSf | sh

RUN cargo install wasm-pack

COPY . /app
WORKDIR /app

RUN pnpm install --frozen-lockfile
RUN pnpm build

CMD ["./run.sh"]
