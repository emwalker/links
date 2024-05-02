# Recommendations

Prototype app to look at what a next iteration on search engines might look like.

I've long felt that the best things to read and listen to have come to me through other people.  Some people have great taste in music, and I end up liking most of the music they like.  Other people have a lot of insight into specific technical topics, such as databases, and I would be interested in any reading they might recommend for their areas of expertise.  But there are limitations to the scope of what I would find interesting from other people.  Someone with expertise about databases might not have tastes in music that agree with mine, and they might not know much about web development.  And someone whose tastes in music are very similar to mine might not know anything about databases or web development.  Is there a way to make use of people's strengths in a targeted way, on a topic-by-topic basis, without bringing in everything they might recommend?  This app tries to get the main moving parts of such a system in place in order to better understand the problem space.

In this project we're not focused on AI at the moment.  There might be a place for it later on, but the core of the system will be very much focused on the actions taken by individual users.

## Getting started

To get started with a development environment:

```sh
$ git clone git@github.com:emwalker/recommendations.git
$ cd recommendations
$ cd client
$ nvm use 20 # https://github.com/nvm-sh/nvm?tab=readme-ov-file#installing-and-updating
$ npm install
$ cd ..
$ make check
$ make setup
$ make dev
# Go to http://localhost:3000
```

To see what the compiled app looks like:

```sh
$ cd recommendations
$ make build
$ make start
# Go to http://localhost:3000
```

## License

MIT License
