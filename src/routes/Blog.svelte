<script lang="ts">
    import SvelteMarkdown from "svelte-markdown";
    import LinkList from "../lib/LinkList.svelte";
    import { ComponentRenderConfig, Render, createRender } from "svelte-render";
    import { import_blog, list_blogs } from "../blogs/blog_getter";
    import { writable } from "svelte/store";

    enum PageType {
        Post,
        List,
        NotFound
    }

    export let blogId: string = null;
    let pageType = blogId ? PageType.Post : PageType.List;
    const divClass = () => pageType == PageType.NotFound ? "" : "blog";
    const dummy = () => null;

    let markdownSource = writable({ source: "# Loading Blog Article..." });
    let render: ComponentRenderConfig<any> = createRender(SvelteMarkdown, markdownSource);

    if (pageType == PageType.List)
        default_page().then(dummy)
    else if (pageType == PageType.Post)
        blog_page().then(dummy)
    

    async function default_page() {
        try {

            let blogIter = Object.values(await list_blogs());
            let blogs = [];

            for (const blog of blogIter) blogs.push(blog.meta);

            blogs = blogs.sort((a,b)=> a.ordering - b.ordering).map(b => [b.title, `/blog/${b.id}`, `by ${b.author}. ${b.description}`])
            markdownSource.set({source: ""});
            render = createRender(LinkList, { styles:"blog", header: "Blogs", links: blogs });
        } catch (err) {

            reportError(err);
            let source = `# Encountered an error.

It seems we've encountered an error:
<pre>
    <code>
        ${err.toString()}    
    </code>
</pre>
`;

            markdownSource.set({ source });
        }
    }

    async function blog_page() {
        if (blogId == null) return default_page();

        let { text, meta } = await import_blog(blogId);
        let source: string;
        if (meta.id == blogId)
            source = `# ${meta.title}\n## Written by ${meta.author}\n\n${text}`;
        else
            source = `# 404.
That page doesn't exist.`;

        markdownSource.set({ source });
    }
</script>

<main>
    <div class={divClass()}>
        <Render of={render} />
    </div>
</main>
