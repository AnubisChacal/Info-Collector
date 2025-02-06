# 🚀 Info Collector

Este projeto é um coletor de informações sobre a máquina Windows escrito em Rust. Ele obtém o IP, hostname e nome de usuário do sistema e os envia via requisição HTTP POST para um servidor.

## 📌 Requisitos

- 🦀 Rust instalado ([Instalar Rust](https://www.rust-lang.org/learn/get-started))
- 📦 Cargo para compilação e execução

## 📥 Instalação

Clone este repositório:

```sh
git clone https://github.com/AnubisChacal/Info-Collector.git
```

## ⚙️ Compilação e Execução

Compile e execute o programa com:

```sh
cargo build --release
```

Ou, para execução direta:

```sh
cargo run
```

## 🔍 Funcionamento

O programa realiza os seguintes passos:

1. 🖧 Obtém o IP da máquina a partir do `ipconfig`.
2. 💻 Obtém o hostname com `hostname`.
3. 👤 Obtém o nome do usuário com `whoami`.
4. 📡 Envia esses dados para `http://IP:PORT/` via uma requisição HTTP POST.

## 📂 Estrutura do Código

- `📜 MachineInfo`: Estrutura para armazenar as informações coletadas.
- `⚙️ run_command(cmd, args)`: Função genérica para executar comandos do Windows.
- `🌐 get_ip()`, `🖥️ get_hostname()`, `👤 get_username()`: Funções para capturar os dados do sistema.
- `📬 send_post_request(data)`: Envia os dados capturados para o servidor remoto.
- `🎯 main()`: Função principal que orquestra todo o processo.
