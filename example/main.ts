import { parseArgs } from "@std/cli/parse_args";
import { extname } from "@std/path";
import * as parser from "./models/parser.ts";
import { Application } from "@oak/oak/application";
import { blue, bold, cyan, green, yellow } from "@std/fmt/colors";
import { exists } from "@std/fs/exists";

// Learn more at https://deno.land/manual/examples/module_metadata#concepts
if (import.meta.main) {
  const args: { init: boolean, build: string, serve: number, watch: boolean } = parseArgs(Deno.args);
  if (!exists("./www") || !exists("./_site") ) {
    console.warn("Gnat required an existing ./www and ./_site directory.")
    console.log(`Please run again with the ${yellow("--init")} flag passed.`)
  }
    if (args.init) {
      await Deno.mkdir("./www", { recursive: true });
      await Deno.mkdir("./_site", { recursive: true })
      await Deno.writeTextFile(".gitignore", "_site/", { append: true })
    } else if (args.build) {
      for await (const entry of parser.readDirRecursive("./www")) {
        if (extname(entry.filePath) !== ".md") {
          continue;
        }

        await parser.build(entry)

      }
    } else if (args.watch) {
      const watcher = Deno.watchFs("www/", { recursive: true });
      for await (const event of watcher) {
        console.log(`[%s] %s`, green(event.kind), event.paths[0]);

        for (const path of event.paths) {
          console.log(`[%s] %s`, blue("build"), event.paths[0]);
          await parser.build(path)
        }

      }

      watcher.close();
    } else if (args.serve) {
      const app = new Application();

      // Logger
      app.use(async (context, next) => {
        await next();
        const rt = context.response.headers.get("X-Response-Time");
        console.log(
          `${green(context.request.method)} ${cyan(context.request.url.pathname)} - ${bold(
            String(rt),
          )
          }`,
        );
      });

      // Response Time
      app.use(async (context, next) => {
        const start = Date.now();
        await next();
        const ms = Date.now() - start;
        context.response.headers.set("X-Response-Time", `${ms}ms`);
      });


      app.use(async (context) => {
        await context.send({
          root: `${Deno.cwd()}/_site`,
          index: "index.html",
        })
      })

      console.log(`http://localhost:${args.serve || 3000}`);
      await app.listen({ port: args.serve || 3000 });
    }
}
