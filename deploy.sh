#!/bin/sh


# usage : ssh -t [remote host] sudo [path to script]

image_name="burning-pro-server-image"
container_name="burning-pro-server"
volume_host="/srv/burning-pro-server"
volume_container="/data"
run_opt="-itd --rm -v ${volume_host}/db:${volume_container}/db -v ${volume_host}/config:${volume_container}/config -p 8080:8080"

# move to repository and get latest files
cd "$(dirname "$(readlink -f "$0")")"
git pull --ff-only

# build
docker build -t ${image_name} .

# stop old container (automatically removed by --rm option)
for container_id in $(docker ps -aq --filter "name=${container_name}");
do
    docker stop ${container_id}
done

# restart
docker run ${run_opt} --name ${container_name} ${image_name}
