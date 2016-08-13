Fix OpenSSL problem on MacOS

    $ brew install openssl
    $ C_INCLUDE_PATH=$(brew --prefix openssl)/include cargo build
