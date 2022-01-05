# godot-server-buildpack
A buildpack to run your exported Godot game on Heroku with [Godot server](https://godotengine.org/download/server) build. 
Useful for making a Dedicated Server.

## Requirements
* A Heroku account and [Heroku CLI](https://devcenter.heroku.com/articles/heroku-cli) (if you haven't that already).
* ngrok account (for tcp tunnel). *might be replaced in the future*
* Linux exported Godot game (how to [export your game](https://docs.godotengine.org/en/stable/getting_started/workflow/export/exporting_projects.html)).
## Setup
1. After installing Heroku CLI and succefully exported your game, open up the terminal in the export output directory then run the following commands:
```bash
heroku login
```
2. Create a new Heroku app:
```bash
git init
heroku apps:create app_name
```
3. add this buildpack to your app using this command:
```bash
heroku buildpacks:add https://github.com/Abdera7mane/godot-server-buildpack/
```
4. Now we need to add some configuration variables (possible in the dashboard):
```bash
heroku config:set GODOT_VERSION="x.x.x"
heroku config:set SERVER_PORT="port_number"
heroku config:set NGROK_API_TOKEN="ngrok_api_token"
```
> GODOT_VERSION: Version of godot used to create the game.  
> SERVER_PORT: the port number which the game server is listening to.  
> NGROK_API_TOKEN: ngrok account [auth token](https://dashboard.ngrok.com/auth/your-authtoken).  
4. In the project directory create `Godotpack`file with **no extension** which will contain `path/to/project/main_package.pck`.
5. add & commit the changes:
```bash
git add .
git commit -m "finnished setup"
```
6. push to heroku repository:
```bash
git push heroku master
```
You made it, if no error occurs during the last step you should get an ip address to connect to your game server in your app website  at`app_name.herokuapp.com`.
