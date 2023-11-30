<script>
    import GithubLink from "../lib/GithubLink.svelte";
    import ShuttleLink from "../lib/ShuttleLink.svelte";
</script>

<svelte:head>
    <meta name="description" content="A blog on Shuttle.rs, about Shuttle.rs.">
  
    <meta property="og:url" content="https://proxxa-shuttleblog.shuttleapp.rs/">
    <meta property="og:type" content="website">
    <meta property="og:title" content="Yori's Shuttle Blog">
    <meta property="og:description" content="A blog on Shuttle.rs, about Shuttle.rs.">
    <meta property="og:image" content="https://proxxa-shuttleblog.shuttleapp.rs/">
  
    <meta name="twitter:card" content="summary_large_image">
    <meta property="twitter:domain" content="proxxa-shuttleblog.shuttleapp.rs">
    <meta property="twitter:url" content="https://proxxa-shuttleblog.shuttleapp.rs/">
    <meta name="twitter:title" content="Yori's Shuttle Blog">
    <meta name="twitter:description" content="A blog on Shuttle.rs, about Shuttle.rs.">
    <meta name="twitter:image" content="https://proxxa-shuttleblog.shuttleapp.rs/">
</svelte:head>

<main>
    <div class="blog">
        
        <h1>
            Proxxa's Shuttle Blog
        </h1>
        <p>
            Hi, I'm Yori. This is my blog on things I do with <ShuttleLink />.
            This is probably not going to be updated much, but I might write a bit with the <ShuttleLink path="cch" text="Christmas Code Hunt"/>.
            You can expect this to change a bit if I start posting... a lot. This site is made with <a href="https://svelte.dev/">Svelte</a>, and
            this site is also an excuse to learn Svelte. You can see the source code on <GithubLink/>. 
        </p>
    </div>
</main>