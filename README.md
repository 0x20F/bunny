<h1 align="center">bunny</h1>

<p align="center">Smart bookmarking tool, running custom commands to open urls from a browser url bar</p>


# What?
"Smart bookmarking"? What? I had the same reaction. Imagine it as a very fast, and elegant (if you so wish) way of entering urls into your browser. Instead of writing `reddit.com/r/programmerhumor`, you could be writing `rd r programmerhumor` and be redirected to your entertainment of choice a lot easier.

This was inspired by the following [article by facebook](https://developers.facebook.com/blog/post/2020/06/03/build-smart-bookmarking-tool-rust-rocket/)


# How to use
* Clone the repository (not on crates.io yet)
* Build it
* Run the server
* Create a custom search engine for your browser and point it towards the server

The server looks for a file that contains all the bookmarks inside the home directory (`~/bookmarks.toml`)


# Writing bookmarks (`bookmarks.toml`)
The engine is simple, it's all written in a `toml` format for readability

* The bookmarks file is made of `books`, they can have any *name* you choose, an *alias*, and a *default* url:
```rust
[twitter]
alias = "tw"
default = "https://twitter.com"
```

* Each `book` in the file has `pages`. Each page has a *name*, a *prefix*, and a *url*:
```rust
[twitter]
alias = "tw"
default = "https://twitter.com"

[twitter.pages]
search = { prefix = "NONE", url = "https://twitter.com/search?q={encoded}" }
profile = { prefix = "@", url = "https://twitter.com/{raw}" }
```

* Each `url` can contain special keys that handle the data you pass to the command.

# Keys

Consider the following command: `tw rust lang`. Here are the keys and what they do with the given data. The `prefix` gets stripped away and we are left to handle `rust lang`
  - `{default}` - will be replaced with the default url of the `book` => `https://twitter.com` 
  - `{encoded}` - will url encode the data => `rust%20lang`
  - `{raw}`     - will pass in the raw data without encoding it => `rust lang`
  - `{0}`       - will pass in the first *segment* of the data => `rust`
  - `{1}`       - will pass in the second *segment* of the data => `lang`
    * There are up to `4` total segments at the moment, because it felt like more were just too many. These can be used to create more customised commands, such as, a reddit command:
    ```rust
    [reddit]
    alias = "rd"
    default = "https://reddit.com"
    
    [reddit.pages]
    whatever = { prefix = "NONE", url = "https://reddit.com/{0}/{1} }
    ```
    * The above can be used as `rd r programmerhumor` to go to a subreddit or as `rd u programmerhumor` to go to a user 
    
# Prefixes

The url prefix can be whatever you want it to be, it is used to differentiate between each command. You could have `-s` when searching for something, or full on `search` if shortcuts aren't your thing. 

**You dont even need spaces between the prefix and command**. 

`-sheyooo` will be split into `-s heyooo` if the prefix is defined as `-s`. 

There's also one special prefix, `NONE`, which means the command will not expect a prefix, and encode the url with all the given data. 



### Full examples can be seen in the [example file](./example/bookmarks.example.toml)
