<script lang="ts">
    import SvelteMarkdown from "svelte-markdown";
    import LinkList from "../lib/LinkList.svelte";
    import { ComponentRenderConfig, Render, createRender } from "svelte-render";
    import { import_blog, list_blogs } from "../blogs/blog_getter";
    import { writable, type Unsubscriber } from "svelte/store";
    import MarkdownCodeSpan from "../lib/markdown/MarkdownCodeSpan.svelte";
    import MarkdownCodeBlock from "../lib/markdown/MarkdownCodeBlock.svelte";
    
    import 'highlight.js/styles/github-dark.min.css'
    import hljs from 'highlight.js/lib/core';

    import javascript from 'highlight.js/lib/languages/javascript';
    import json from 'highlight.js/lib/languages/json';
    import css from 'highlight.js/lib/languages/css';
    import toml from 'highlight.js/lib/languages/ini';
    import svelte from 'highlight.js/lib/languages/xml';
    import rust from "highlight.js/lib/languages/rust";
    import sh from "highlight.js/lib/languages/shell";
    hljs.registerLanguage("js", javascript)
    hljs.registerLanguage("ts", javascript)
    hljs.registerLanguage("json", json)
    hljs.registerLanguage("css", css)
    hljs.registerLanguage("toml", toml)
    hljs.registerLanguage("svelte", svelte)
    hljs.registerLanguage("rs", rust)
    hljs.registerLanguage("sh", sh)

    enum PageType {
        Post,
        List,
        NotFound
    }

    export let blogId: string = null;
    let pageType = blogId ? PageType.Post : PageType.List;
    const divClass = () => pageType == PageType.NotFound ? "" : "blog";
    const dummy = () => null;

    let markdownSource = writable({ source: "# Loading Blog Article...", renderers: { codespan: MarkdownCodeSpan, code: MarkdownCodeBlock } });
    let unsubs: Unsubscriber[] = [];
    let render: ComponentRenderConfig<any> = createRender(SvelteMarkdown, markdownSource);

    unsubs.push(markdownSource.subscribe(
        _ => setTimeout(() => hljs.highlightAll(), 0),
        _ => {},
    ))

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

            for (const unsub of unsubs) unsub();
            
            markdownSource.update(store => ({ source: "", renderers: store.renderers }));
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

            markdownSource.update(store => ({ source, renderers: store.renderers }));
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

        markdownSource.update(store => ({ source, renderers: store.renderers }));
    }
</script>

<main>
    <div class={divClass()}>
        <Render of={render} />
    </div>
</main>
