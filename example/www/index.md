---
title: Wingman
desc: A website engine designed to get you published
---

# Welcome to Wingman

Wingman is a website engine and generator written in Rust. With Wingman, you can create a new Markdown-based website in seconds, customize every inch with vanilla HTML, CSS, and JavaScript, and serve or build your site for the world.

Here is what an initial Wingman project's directory looks like.

```bash
example/
    _site
        index.html
        static
           page.css
    templates
        page.hbs
        partials
           nav.hbs
    www
       index.md
       static
           page.css

8 directories, 11 files
```

You can create a new project anytime by running `wingman init`. The `-f` flag forces Wingman to overwrite already existing files. If there are files already in your target directory, try: `wingman init -f`.

There are three important directories in every Wingman project. The `_site` out dir, this is where Wingman write your production builds. You can serve the root of this folder to see your website. Wingman comes with a built-in development web server.

```bash
wingman serve --port 3030
# --port is optional. Without it, Wingman will default to port 3030.
wingman serve
```

The `templates` dir hosts your site's template files. Modify these to modify the core structure of your site. Be warned. 
