# Links

Prototype app to look at what a next iteration on search engines might look like.

I've long felt that the best things to read and listen to have come to me through other people.  Some people have great taste in music, and I end up liking most of the music they like.  Other people have a lot of insight into specific technical topics, such as databases, and I would be interested in any reading they might recommend for their areas of expertise.

But there are limitations to what I find interesting from other people.  Someone with expertise in databases might not have tastes in music that agree with mine, and they might not know much about web development.  And someone whose tastes in music are very similar to mine might not know anything about databases or web development.  Is there a way to make use of people's insights in a targeted way, on a user-by-user and topic-by-topic basis, without placing undue emphasis on everything they might recommend?  This app tries to put in place the main moving parts of such a system in order to better understand the question.

In this project we're not focused on AI at the moment.  There might be a place for it later on, but the core of the system will be very much focused on the actions taken by individual users.

## Screenshots

It's all a mockup, and nothing in the app is real at this point.

![Topic page](https://github.com/emwalker/links/assets/760949/df188504-1107-4b20-8379-1974b9f6d670)

## Getting started

To get started with a development environment:

```sh
$ git clone git@github.com:emwalker/links.git
$ cd links
$ cd client
$ nvm use # https://github.com/nvm-sh/nvm?tab=readme-ov-file#installing-and-updating
$ npm install
$ cd ..
$ make check
$ make setup
$ make dev
# Go to http://localhost:3000
```

To see what the production app looks like:

```sh
$ cd links
$ make build
$ make start
# Go to http://localhost:3000
```

## License

MIT License
