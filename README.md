Send one query after the other:

    while true; do echo "ETAD" | socat - UNIX-CONNECT:/tmp/pdns-rust.socket; sleep 1; done

Send three queries at once:

    while true; do (for x in 1, 1, 1; do echo hello; done) | socat - UNIX-CONNECT:/tmp/pdns-rust.socket; sleep 1; done


# Integration testing

Start container:

    docker-compose up -d

Create a zone and some records:

    docker exec pdns pdnsutil create-zone example.com ns1.example.com
    docker exec pdns pdnsutil add-record example.com '' MX '25 mail.example.com'
    docker exec pdns pdnsutil add-record example.com. www A 192.0.2.1


Test:

    dig -p 1053 example.com MX @127.0.0.1
    dig -p 1053 a www.example.com @127.0.0.1



Nach Durchstarten von pdns-remote nicht vergessen:

    sudo chown 953:953 pdns-remote.sock
