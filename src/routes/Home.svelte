<script>
    import GithubLink from "../lib/GithubLink.svelte";
    import ShuttleLink from "../lib/ShuttleLink.svelte";
</script>

<main>
    <h1>
        Proxxa's Shuttle Blog
    </h1>
    <span class="paragraphHolder">
        <p>
            Hi, I'm Yori. This is my blog on things I do with <ShuttleLink />.
            This is probably not going to be updated much, but I might write a bit with the <ShuttleLink path="cch" text="Christmas Code Hunt"/>.
            You can expect this to change a bit if I start posting... a lot. This site is made with <a href="https://svelte.dev/">Svelte</a>, and
            this site is also an excuse to learn Svelte. You can see the source code on <GithubLink/>. 
        </p>
    </span>
</main>