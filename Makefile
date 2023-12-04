docker_compose = docker compose
docker_container = console

build: # Rebuild docker stack
	${docker_compose} build --build-arg UID=`id --user` --build-arg GID=`id --group`

up: # Start docker stack in detached mode
	${docker_compose} up --detach

ps: # Print docker stack status
	${docker_compose} ps

down: # Stop docker stack
	${docker_compose} down

ssh: # SSH into container for work
	${docker_compose} exec ${docker_container} /bin/bash
