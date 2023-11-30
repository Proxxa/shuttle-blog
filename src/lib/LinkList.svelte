<script lang="ts">
    import { Router, Link } from "svelte-navigator";
    export let header: string = "Links";
    export let links: ([string, string] | [string, string, string])[] = [];
    export let styles: string = "";
</script>

<main class={styles}>
    <h1>
        {header}
    </h1>
    <Router>
        <nav>
            <ul>
                {#each links as link}
                    <li>
                        <Link to={link[1]}>{link[0]}</Link>
                        {@html link[2] ?? ""}
                    </li>
                {/each}
            </ul>
        </nav>
    </Router>
</main>
