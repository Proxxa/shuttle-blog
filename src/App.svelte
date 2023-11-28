<script>
    // @ts-ignore
    import { Router, Link, Route } from "svelte-navigator";
	import Home from "./routes/Home.svelte";
    import NotFound from "./routes/404.svelte";
    import GithubLink from "./lib/GithubLink.svelte";
    import favicon from "./assets/favicon.png";
    import ApiLandingPage from "./routes/ApiLandingPage.svelte";
    import Blog from "./routes/Blog.svelte";
</script>

<link rel="icon" type="image/png" href={favicon} />

<Router>
    <nav>
        <Link to="/">Home</Link>
        <Link to="blog">Blog</Link>
        <span class="navSpacer"></span>
        <GithubLink/>    
    </nav>

    <Route path="blog" component="{Blog}"/>
    <Route path="/api" component="{ApiLandingPage}"/>
    <Route path="/" component="{Home}"/>
    <Route path="" component="{NotFound}"/>
</Router>