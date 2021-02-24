# Wingman
## Static site generator and theming engine
Github hosts the nightly build. Stable is on [Crates](https://crates.io/crates/wingman). | [Dev-blog (built with Wingman)](https://wingman-rs.neocities.org/)

### Commands
* ```help``` generates a list of help commands and their short codes
* ```new``` generates a new seed.json file. 
* ```build``` builds static site off of seed.json and provided directory

### Thoughts and Methods
**Wingman** exists to provide a quick way to create functional light weight static websites. Eventually, Wingman will support custom themeing and multiple modes, but currently, it only creates (responds to) the html blog type. 

#### Who is Wingman for?
Wingman is for anyone looking to create a quick one-page static site without having to worry about the fat that holds the modern web back. Namely _@imports_ and _JavaScript_. These technologies are often uneccessary for build a fast, easily accessabile, and responsive website, and are impossible to utilize to their most bloated potential on protocols like Gemini and Gopher, which this application intends to support in the future. 

If you're new to web development, or simply want to generate the most bare-bones site possible with minimal typing, this is the tool for you.

### How to use Wingman
Wingman is designed to build off common mark, and in the future might utilize a new parser for additional functionality. Write your markdown posts in the ```/posts``` directory and run ```wingman build``` for your index.html to generate. While **Wingman** is in Alpha, you might want to reformat the file quickly in your prefered code editor. It shoud still render fine regardless if you decide to do this however.

Then, take your ```index.html``` file and upload it to a host like neocities, Netlify, or Github Pages.

## Future Wishlist Featues
* Custom directories
* Gopher support
* Gemini support
* full on web-app hosting using Actix for html 5
* Less.js CSS Compilation support

**THIS APPLICATION IS IN ALPHA, AND ITS CARGO.IO FORK WILL ONLY CONTAIN STABLE RELEASES OF THE APPLICATION. FOR NIGHLY BUILDS, GITHUB IS THE PLACE**
