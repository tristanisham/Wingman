
## How to modify seed.json
```
{
            //add variables here:
            "title": "{%} - WPLQ!,
            //ALL CAP keys are reserved by WPLQ. 
            //ROOT sets the index directory of your site and is used by the transcopiler
            "ROOT": "./"
            //Keys in PAGES will be created into files. keys with values that are arrays will create a directory and files inside said directory.
            "PAGES": {
                "/about": ./about.html,
                "/posts: [
                    "/hello-world.html",
                    "/this-is-my-story.html",
                    "/why-javascript-is-evil.html"
                ]
            },
        }
```