<script lang="ts">
    import SvelteMarkdown from "svelte-markdown";
    import { import_blog, list_blogs } from "../blogs/blog_getter";

    export let blogId = null;

    let markdown = "# Loading Blog Article...";
    let divClass = "blog"

    async function default_page(): Promise<string> {
        try {
            let blogIter = Object.values(await list_blogs());
            console.log(blogIter);
            let blogs = [];
            for (const blog of blogIter)
                blogs.push(blog.meta)
            console.log(blogs);
            return `# Blogs
`.concat(blogs.map(b => ` - [${b.title}](/blog/${b.id}) by ${b.author}. ${b.description}`).join('\n'));
        } catch (err) {
            reportError(err);
            return `
# Encountered an error.

It seems we've encountered an error:
<pre>
    <code>
        ${err.toString()}    
    </code>
</pre>
`;
        }
    }
    
    (async (blogId) => {
        if (blogId == null) return (markdown = await default_page());

        let { text, meta } = await import_blog(blogId);
        if (meta.id == blogId)
            markdown = `# ${meta.title}\n## Written by ${meta.author}\n\n${text}`;
        else {
            markdown = `# 404.
That page doesn't exist.`
        }
    })(blogId);
</script>

<main>
    <div class={divClass}>
        <SvelteMarkdown source={markdown} />
    </div>
</main>
