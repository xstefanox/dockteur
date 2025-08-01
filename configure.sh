# TODO remove

xh GET http://localhost:$(docker compose port toxiproxy 8474 | cut -d':' -f2)/proxies

docker compose exec toxiproxy /toxiproxy-cli create --listen 0.0.0.0:9001 --upstream whoami:80 whoami
docker compose exec toxiproxy /toxiproxy-cli toxic add --toxicName wtimeout --upstream --type timeout --attribute timeout=2000 whoami
docker compose exec toxiproxy /toxiproxy-cli toxic add --toxicName wlat --upstream --type latency --attribute latency=2000 whoami

docker compose exec toxiproxy /toxiproxy-cli toxic remove whoami
docker compose exec toxiproxy /toxiproxy-cli inspect whoami
