<script lang="ts">
    import SvelteMarkdown from "svelte-markdown";
    import LinkList from "../lib/LinkList.svelte";
    import { ComponentRenderConfig, Render, createRender } from "svelte-render";
    import { import_blog, list_blogs } from "../blogs/blog_getter";
    import { writable, type Unsubscriber } from "svelte/store";
    import MarkdownCodeSpan from "../lib/markdown/MarkdownCodeSpan.svelte";
    import MarkdownCodeBlock from "../lib/markdown/MarkdownCodeBlock.svelte";

    import "highlight.js/styles/github-dark.min.css";
    import hljs from "highlight.js/lib/core";

    import javascript from "highlight.js/lib/languages/javascript";
    import json from "highlight.js/lib/languages/json";
    import css from "highlight.js/lib/languages/css";
    import toml from "highlight.js/lib/languages/ini";
    import svelte from "highlight.js/lib/languages/xml";
    import rust from "highlight.js/lib/languages/rust";
    import sh from "highlight.js/lib/languages/shell";
    hljs.registerLanguage("js", javascript);
    hljs.registerLanguage("ts", javascript);
    hljs.registerLanguage("json", json);
    hljs.registerLanguage("css", css);
    hljs.registerLanguage("toml", toml);
    hljs.registerLanguage("svelte", svelte);
    hljs.registerLanguage("rs", rust);
    hljs.registerLanguage("sh", sh);

    enum PageType {
        Post,
        List,
        NotFound,
    }

    export let blogId: string = null;
    let pageType = blogId ? PageType.Post : PageType.List;
    const divClass = () => (pageType == PageType.NotFound ? "" : "blog");
    const dummy = () => null;

    let blogMeta = null;

    let hack,
        renderLinkList = false,
        links = [],
        linkRenderStore = writable({ header: "", links: [], styles: ""}),
        linkRender = createRender(LinkList, linkRenderStore),
        createLinksUsed = true;
        
    
    let linksInterval = setInterval(() => {
        if (hack) {
            let newLinks = [];
            let linkSet = {};

            let linkElements = hack.getElementsByTagName("a");

            let t = document.createElement("textarea");
            for (const link of linkElements)
                if (!link.getAttribute("href").startsWith("#")) {
                    let href = link.getAttribute("href");

                    if (link.hasAttribute("title")) {
                        t.innerHTML = link.title
                        linkSet[href] = [t.value, href];
                    } else {
                        if (!(href in linkSet))
                            linkSet[href] = new Array(2).fill(
                                link.getAttribute("href"),
                            );
                    }
                }

            for (const link in linkSet) newLinks.push(linkSet[link]);
            
            renderLinkList = true;
            newLinks.sort();

            function arraysEqual(a, b, ignoreType = false) {
                return (!ignoreType || (Array.isArray(a) && 
                Array.isArray(b))) &&
                a.length == b.length &&
                a.every((c, i) => {
                    let d = b[i];
                    if (Array.isArray(c)) {
                        if (Array.isArray(d)) {
                            return arraysEqual(c, d, true)
                        } 
                        return false
                    }
                    return c == d
                })
            }

            
            if (newLinks.length > 0 && !arraysEqual(links, newLinks))
                links = newLinks;
            else
                clearInterval(linksInterval);

            linkRenderStore.set({ links, header: "Links Used", styles: "blog" });;
        }}, 100);

    

    let markdownSource = writable({
        source: "# Loading Blog Article...",
        renderers: {
            codespan: MarkdownCodeSpan,
            code: MarkdownCodeBlock,
            link: MarkdownLink,
        },
    });
    let unsubs: Unsubscriber[] = [];
    let render: ComponentRenderConfig<any> = createRender(
        SvelteMarkdown,
        markdownSource,
    );

    unsubs.push(
        markdownSource.subscribe(
            (_) =>
                setTimeout(() => {
                    hljs.highlightAll();
                }, 0),
            (_) => {},
        ),
    );

    if (pageType == PageType.List) default_page().then(dummy);
    else if (pageType == PageType.Post) blog_page().then(dummy);

    async function default_page() {
        try {
            let blogIter = Object.values(await list_blogs());
            let blogs = [];

            for (const blog of blogIter) blogs.push(blog.meta);

            blogs = blogs
                .sort((a, b) => a.ordering - b.ordering)
                .map((b) => [
                    b.title,
                    `/blog/${b.id}`,
                    `by ${b.author}. ${b.description}`,
                ]);

            for (const unsub of unsubs) unsub();

            markdownSource.update((store) => ({
                source: "",
                renderers: store.renderers,
            }));
            render = createRender(LinkList, {
                styles: "blog",
                header: "Blogs",
                links: blogs,
            });
        } catch (err) {
            pageType = PageType.NotFound;
            reportError(err);
            let source = `# Encountered an error.

It seems we've encountered an error:
<pre>
    <code>
        ${err.toString()}    
    </code>
</pre>
`;

            markdownSource.update((store) => ({
                source,
                renderers: store.renderers,
            }));
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
        blogMeta = meta;
        markdownSource.update((store) => ({
            source,
            renderers: store.renderers,
        }));
    }

    import { onMount } from "svelte";
    import MarkdownLink from "../lib/markdown/MarkdownLink.svelte";
    let url = null;
    onMount(() => (url = window.location.href));
</script>

<svelte:head>
    {#if blogMeta?.title}
        <title>{blogMeta.title} | Yori's Shuttle Blog</title>
    {/if}
    <meta name="viewport" content="width=device-width, initial-scale=1.0" />
    <meta
        name="description"
        content={blogMeta?.description ??
            "A blog on Shuttle.rs, about Shuttle.rs."}
    />

    <meta
        property="og:url"
        content={url ?? "https://proxxa-shuttleblog.shuttleapp.rs/blog"}
    />
    <meta property="og:type" content="website" />
    <meta
        property="og:title"
        content={blogMeta?.title ?? "Yori's Shuttle Blog"}
    />
    <meta
        property="og:description"
        content={blogMeta?.description ??
            "A blog on Shuttle.rs, about Shuttle.rs."}
    />
    <meta
        property="og:image"
        content="https://proxxa-shuttleblog.shuttleapp.rs/"
    />

    <meta name="twitter:card" content="summary_large_image" />
    <meta
        property="twitter:domain"
        content="proxxa-shuttleblog.shuttleapp.rs"
    />
    <meta
        property="twitter:url"
        content={url ?? "https://proxxa-shuttleblog.shuttleapp.rs/blog"}
    />
    <meta
        name="twitter:title"
        content={blogMeta?.title ?? "Yori's Shuttle Blog"}
    />
    <meta
        name="twitter:description"
        content={blogMeta?.description ??
            "A blog on Shuttle.rs, about Shuttle.rs."}
    />
    <meta
        name="twitter:image"
        content="https://proxxa-shuttleblog.shuttleapp.rs/"
    />
</svelte:head>

<div class={divClass()}>
    <div bind:this={hack}>
        <Render of={render} />
    </div>
    {#if pageType == PageType.Post}
    <Render of={linkRender} />
    {/if}
</div>
