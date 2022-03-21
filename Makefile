WOT_BOT_ID="ghcr.io/chipp/wot_bot"

install:
	docker-compose down || true
	docker image rm -f $(WOT_BOT_ID)
	docker pull $(WOT_BOT_ID)
	docker-compose up -d

action_deploy:
	make install
