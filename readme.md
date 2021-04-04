# QuickDeploy
### A rocket-based clone of [Miniserve](https://github.com/svenstaro/miniserve/)

It's basically an inferior version of Miniserve because it lacks an in-browser file navigator -- something that Miniserve has, and does well.

## Usage
`qdp <DIRECTORY> <MOUNT POINT>?`

## Examples:
`qdp .`
(Serves current directory and mounts the server at localhost:8000/)

`qdp /tmp/folder /navigator`
(Serves `/tmp/folder` and mounts the server at `localhost:8000/navigator`)

## In-browser usage
Navigate to `localhost:8000/<MOUNT POINT>`. If there is an index.html file in your local directory, that will be served, otherwise, you will get a 404 screen. To navigate to a file, simply type `localhost:8000/<MOUNT POINT>/<FILE NAME>`.