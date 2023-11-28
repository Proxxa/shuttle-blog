<script>
    // @ts-ignore
    import { Router, Link, Route } from "svelte-navigator";
	import Home from "./routes/Home.svelte";
</script>

<Router>
    <Route path="">
        <Home />
    </Route>
</Router>