# Wingman
## Static site generator and theming engine
Github hosts the nightly build. Stable is on [Crates](https://crates.io/crates/wingman). | [Dev-blog (built with Wingman)](https://wingman-rs.neocities.org/)

### Commands & Flags
* ```--help``` generates a list of help commands, what they do, and their short codes
* ```new``` generates a new seed.json file. 
    * ```--blog``` generates a new seed.json with type: blog
    * ```post``` generates a new post of seed.json type: blog | ```wingman new post```
* ```build``` builds static site off of seed.json and provides directory, index.html, and styles.css

### Thoughts and Methods
**Wingman** exists to provide a quick way to create functional light weight static websites. Wingman supports custom themeing and will support multiple modes, but currently, it only creates (responds to) the html blog type. 

#### Who is Wingman for?
Wingman is for anyone looking to create a quick one-page static site without having to worry about the fat that holds the modern web back. Namely _@imports_ and _JavaScript_. These technologies are often uneccessary for build a fast, easily accessabile, and responsive website, and are impossible to utilize to their most bloated potential on protocols like Gemini and Gopher, which this application intends to support in the future. 

If you're new to web development, or simply want to generate the most bare-bones site possible with minimal typing, this is the tool for you.

### How to use Wingman
1) Make a directory for your blog, cd into it, and run *wingman* new. 

```
mkdir blog
cd blog
wingman new

```

2) In seed.json set "type" to "blog or run *wingman new --blog*

```
wingman new --blog

```

3) Build the initial directory structure

```
wingman build

```
This will generate the program's file structure.

```
./bin/
  /posts/
   - markdown-example.md

```

The markdown file here is just an example file, and can be deleted. If no files are in posts/ ```wingman b``` will generate the example file. Delete that file when you're ready to write your own posts. 

4) Write a post and publish

Run:

```wingman new post```

Edit the new file generated in posts and then run ```wingman b``` to generate an index.html and style sheet. And that's your entire site. Upload those two files and your blog will be ready! You can link to posts by linking to their ID. Each post is given an ID starting at 0 from oldest to newest in the current system.
So, your first post can be linked to as example.com/#0

Then, take your ```index.html``` and ```styles.css``` file and upload it to a host like Neocities, Netlify, or Github Pages.

## Future Wishlist Featues
* Custom directories
* Gopher support
* Gemini support
* full on web-app hosting using Actix for html 5


**THIS APPLICATION IS IN ALPHA, AND ITS CARGO.IO FORK WILL ONLY CONTAIN STABLE RELEASES OF THE APPLICATION. FOR NIGHLY BUILDS, GITHUB IS THE PLACE**
