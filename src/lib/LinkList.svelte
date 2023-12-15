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
                        {#if !(link[1].startsWith('#') || link[1].includes("://")) }
                            <Link to={link[1]}>{link[0]}</Link>
                        {:else}
                            <a href={link[1]}>{link[0]}</a>
                        {/if}
                        {@html link[2] ?? ""}
                    </li>
                {/each}
            </ul>
        </nav>
    </Router>
</main>
