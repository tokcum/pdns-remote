Send one query after the other:

    while true; do echo "ETAD" | socat - UNIX-CONNECT:/tmp/pdns-rust.socket; sleep 1; done

Send three queries at once:

    while true; do (for x in 1, 1, 1; do echo hello; done) | socat - UNIX-CONNECT:/tmp/pdns-rust.socket; sleep 1; done