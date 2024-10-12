test:
	@cargo llvm-cov

build:
	@docker compose build

run: build
	@docker compose up

push: build
	@docker tag unai-server:latest asia-northeast1-docker.pkg.dev/unseo-chatbot/unai/chat-server:latest
	@docker push asia-northeast1-docker.pkg.dev/unseo-chatbot/unai/chat-server:latest

deploy: push
	@gcloud run deploy chat-server \
		--image=asia-northeast1-docker.pkg.dev/unseo-chatbot/unai/chat-server:latest \
		--region=asia-northeast1 \
		--project=unseo-chatbot
