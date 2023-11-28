<script lang="ts">
    import SvelteMarkdown from 'svelte-markdown';

    export let blogId = null;
    let markdown = "# Loading Blog Article...";
    const NO_ID_MARKDOWN = `
# Blogs

Welcome to the blogspace. As of right now, here's what there is:

* [Hello, Svelte!](/blog/0) A first blog post, detailing the site.

`;
    (async blogId => {
        if (blogId == null)
            return markdown = NO_ID_MARKDOWN;


        let blog_importer = await import("../blogs/blog_getter");
        markdown = await blog_importer.import_blog(blogId);
    })(blogId); 
    
</script>

<main>
    <div class="blog">
        <SvelteMarkdown source={markdown} />
    </div>
</main>