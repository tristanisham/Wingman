import * as path from "@std/path";
import { render, CSS, KATEX_CSS } from "@deno/gfm"
import Handlebars from "npm:handlebars"
import { extract } from "@std/front-matter/any";
import { test } from "@std/front-matter/test";


export interface DirEntry {
    /** The file name of the entry. It is just the entity name and does not
     * include the full path. */
    name: string;
    /** The full path for the entry. */
    filePath: string;
    /** True if this is info for a regular file. Mutually exclusive to
     * `DirEntry.isDirectory` and `DirEntry.isSymlink`. */
    isFile: boolean;
    /** True if this is info for a regular directory. Mutually exclusive to
     * `DirEntry.isFile` and `DirEntry.isSymlink`. */
    isDirectory: boolean;
    /** True if this is info for a symlink. Mutually exclusive to
     * `DirEntry.isFile` and `DirEntry.isDirectory`. */
    isSymlink: boolean;
}

export async function* readDirRecursive(target: string): AsyncIterable<DirEntry> {
    for await (const entry of Deno.readDir(target)) {
        const fullPath = path.join(target, entry.name);

        yield {
            name: entry.name,
            filePath: fullPath,
            isFile: entry.isFile,
            isDirectory: entry.isDirectory,
            isSymlink: entry.isSymlink,
        }

        if (entry.isDirectory) {
            yield* readDirRecursive(fullPath);
        }

    }
}

Handlebars.registerPartial("nav", await Deno.readTextFile("./templates/partials/nav.hbs"))
const page_tmpl = Handlebars.compile(await Deno.readTextFile("./templates/page.hbs"))


export async function build(entry: DirEntry | string) {
    let filePath = "";
    if (typeof (entry) == "string") {
        filePath = entry;
        if (path.isAbsolute(filePath)) {
            filePath = path.relative("./www", filePath)
            filePath = path.join("./www", filePath)
        }

    } else {
        filePath = entry.filePath;
    }

    const topName = filePath.slice("www".length);
    const trimmedName = topName.slice(0, (topName.length) - path.extname(topName).length)
    const outPath = path.join("_site", `${trimmedName}.html`);
    const text = await Deno.readTextFile(filePath);
    if (!test(text)) {
        console.error("Blueprints require frontmatter to be rendered.")
        Deno.exit(1)
    }

    try {
        const frontmatter = extract(text);

        const body = render(frontmatter.body, { allowMath: true, baseUrl: "https://thebrief.wtf/" })
        const out_body = page_tmpl({
            meta: frontmatter.attrs,
            markdown: body,
            css: {
                GFM: CSS, KATEX: KATEX_CSS,
            }
        })

        await Deno.writeTextFile(outPath, out_body);

    } catch (err) {
        if (err instanceof TypeError) {
            console.error(err)
            Deno.exit(1);
        }

        throw err;
    }


}
