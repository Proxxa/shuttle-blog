<script>
    import { Link } from "svelte-navigator";
    let output = "pending...";
    let duration;
    const start = Date.now();
    fetch("/api/hello")
        .then(res => res.text())
        .then(text => {
            output = text
            duration = (Date.now() - start) + "ms"
        });
</script>

<main>
    <h1>
        API
    </h1>

    <div class="paragraphHolder">
        <p>
            Hi! You shouldn't be here. It might be a good idea to <Link to="/">go home</Link>.
            The API is nothing right now. Come back later and I <i>might</i> have information on it.

            However, if you want to be sure that the routing for the API is still correct, here's a
            call to it: <br/>

            <b>{output}</b> {duration?`(${duration})`:""}
        </p>
    </div>
</main>