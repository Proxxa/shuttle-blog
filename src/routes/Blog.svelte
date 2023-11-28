<script>
    import SvelteMarkdown from 'svelte-markdown';

    export let blogId = null;
    let markdown = "# Loading Blog Article...";
    (async () => {
        if (blogId == null)
            return;


        let blog_importer = await import("../blogs/blog_getter");
        markdown = await blog_importer.import_blog(blogId);
    })(); 
    
</script>

<SvelteMarkdown source={markdown} />