### Create Container
 ```docker build -t my-rust-app .```


### Running a container
```
docker run -e DISCORD_API_TOKEN=yuhfsaf -e DATABASE_URL=postgresql://postgres:1234@host.docker.internal:5432/mybotdb my-rust-app
```
This runs container my-rust-app, passing in env variables. 

```host.docker.internal``` is a special DNS name. It represents the host's IP. It tells Docker that dont look for localhost insdie the VM, look for the host machine. In Linux, we can add --network host to mean the host's network set.