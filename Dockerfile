FROM debian:bookworm

RUN apt update && apt upgrade -y

RUN apt install -y libwebkit2gtk-4.0-dev \
    build-essential \
    curl \
    wget \
    file \
    libssl-dev \
    libgtk-3-dev \
    libayatana-appindicator3-dev \
    clang \
    librsvg2-dev

RUN apt install -y nodejs npm

RUN apt install -y nsis lld llvm

# RUN curl -fsSL https://bun.sh/install | bash
RUN npm install -g bun

RUN curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh -s -- -y

# RUN echo 'source $HOME/.cargo/env' >> $HOME/.bashrc
ENV PATH="/root/.cargo/bin:${PATH}"

RUN rustup update

RUN rustup target add x86_64-pc-windows-msvc

RUN cargo install cargo-xwin

COPY . /source

WORKDIR /source

CMD [ "bash" ]

# bunx tauri build --runner cargo-xwin --target x86_64-pc-windows-msvc


